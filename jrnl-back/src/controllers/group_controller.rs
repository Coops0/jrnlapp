use crate::{
    error::{DatabaseError, JrnlError, JrnlResult, JsonExtractor},
    schemas::{group::Group, user::User},
    services::{
        entry_service::EntryService,
        group_service::{GetGroupAndMembersBody, GroupService, SelfGroup},
    },
    AppState,
};
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    routing::{delete, get, post},
    Json, Router,
};
use base64::{engine::general_purpose::STANDARD, Engine};
use chrono::{Duration, NaiveDate};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

pub fn groups_controller() -> Router<AppState> {
    Router::new()
        .route("/", post(create_group).get(joined_groups))
        .route(
            "/:group",
            get(get_group).post(join_group).delete(leave_group),
        )
        .route("/:group/:user", delete(kick_member))
        .route("/:group/members", get(get_group_members))
        .route("/:group/day", get(get_days_data_paginated))
}

#[derive(Deserialize)]
struct CreateGroupPayload {
    name: String,
}

async fn create_group(
    user: User,
    group_service: GroupService,
    JsonExtractor(CreateGroupPayload { name }): JsonExtractor<CreateGroupPayload>,
) -> JrnlResult<Json<Group>> {
    let existing_owned_groups = group_service
        .get_owned_groups_count(&user)
        .await
        .map_err(DatabaseError)?;

    if existing_owned_groups >= 10 {
        return Err(JrnlError::CannotCreateMoreGroups);
    }

    group_service
        .create_group(&user, &name)
        .await
        .map(Json)
        .map_err(Into::into)
}

async fn get_group(
    Path(code): Path<String>,
    group_service: GroupService,
) -> JrnlResult<Json<Option<GetGroupAndMembersBody>>> {
    group_service
        .get_group_and_members_maybe_by_code(&code)
        .await
        .map(Json)
        .map_err(Into::into)
}

async fn join_group(
    Path(code): Path<String>,
    user: User,
    group_service: GroupService,
) -> JrnlResult<StatusCode> {
    let joined_groups = group_service
        .get_joined_groups_count(&user)
        .await
        .map_err(DatabaseError)?;

    if joined_groups >= 20 {
        return Err(JrnlError::CannotJoinMoreGroups);
    }

    group_service
        .join_group(&code, &user)
        .await
        .map(|_| StatusCode::OK)
        .map_err(|why| match &why {
            sqlx::Error::Database(d) if d.is_unique_violation() => JrnlError::AlreadyGroupMember,
            _ => DatabaseError(why).into(),
        })
}

#[derive(Serialize)]
struct TrimmedUserWithOwner {
    id: Uuid,
    name: String,
    owner: bool,
}

async fn get_group_members(
    user: User,
    Path(code): Path<String>,
    group_service: GroupService,
) -> JrnlResult<Json<Vec<TrimmedUserWithOwner>>> {
    let group = group_service
        .get_joined_group_by_code(&user, &code)
        .await
        .map_err(DatabaseError)?;

    let members = group_service
        .get_group_members(&group)
        .await
        .map_err(DatabaseError)?;

    let members_with_owners = members
        .into_iter()
        .map(|user| TrimmedUserWithOwner {
            id: user.id,
            name: user.name,
            owner: user.id.eq(&group.owner_id),
        })
        .collect::<Vec<_>>();

    Ok(Json(members_with_owners))
}

async fn leave_group(
    user: User,
    Path(code): Path<String>,
    group_service: GroupService,
) -> JrnlResult<StatusCode> {
    let group = group_service
        .get_group_by_code(&code)
        .await
        .map_err(DatabaseError)?;

    group_service.leave_group(&user, &group).await?;

    if group.owner_id != user.id {
        return Ok(StatusCode::OK);
    }

    let members = group_service.get_group_members_count(&group).await?;
    if members == 0 {
        group_service.delete_group(&group).await?;
    } else {
        group_service.assign_group_new_owner(&group).await?;
    }

    Ok(StatusCode::OK)
}

async fn kick_member(
    user: User,
    Path((code, target_user_id)): Path<(String, Uuid)>,
    group_service: GroupService,
) -> JrnlResult<StatusCode> {
    if user.id == target_user_id {
        return Err(JrnlError::CannotKickSelf);
    }

    group_service
        .kick_group_by_code_member(&code, &user, &target_user_id)
        .await
        .map(|_| StatusCode::OK)
        .map_err(Into::into)
}

#[derive(Serialize)]
struct DayData {
    scales: Vec<f32>,
    day: NaiveDate,
}

#[derive(Debug, Deserialize)]
struct GetDaysDataParams {
    day_limit: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_base_date")]
    before: Option<NaiveDate>,
}

fn deserialize_base_date<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<NaiveDate>, D::Error> {
    let Some(encoded_date) = Option::<String>::deserialize(deserializer)? else {
        return Ok(None);
    };

    let decoded_bytes = STANDARD
        .decode(encoded_date)
        .map_err(serde::de::Error::custom)?;
    let date_string = String::from_utf8(decoded_bytes).map_err(serde::de::Error::custom)?;

    NaiveDate::parse_from_str(&date_string, "%m/%d/%Y")
        .map(Some)
        .map_err(serde::de::Error::custom)
}

async fn get_days_data_paginated(
    user: User,
    Query(params): Query<GetDaysDataParams>,
    Path(code): Path<String>,
    group_service: GroupService,
    entry_service: EntryService,
) -> JrnlResult<Json<Vec<DayData>>> {
    let day_limit = params.day_limit.unwrap_or(7).clamp(1, 30);
    let before_date = params
        .before
        .unwrap_or_else(|| chrono::Utc::now().naive_utc().date());

    let group = group_service
        .get_joined_group_by_code(&user, &code)
        .await
        .map_err(DatabaseError)?;

    let group_member_ids = group_service
        .get_group_members(&group)
        .await
        .map_err(DatabaseError)?
        .into_iter()
        .map(|user| user.id)
        .collect::<Vec<_>>();

    let start_date = before_date - Duration::days(day_limit - 1);
    let all_entries = entry_service
        .get_multiple_users_entries_between_dates(&group_member_ids, &start_date, &before_date)
        .await
        .map_err(DatabaseError)?;

    let entries_grouped_by_day = all_entries
        .into_iter()
        .fold(HashMap::<NaiveDate, Vec<f32>>::new(), |mut acc, entry| {
            acc.entry(entry.date).or_default().push(entry.emotion_scale);
            acc
        })
        .into_iter()
        .map(|(day, scales)| DayData { scales, day })
        .collect::<Vec<_>>();

    Ok(Json(entries_grouped_by_day))
}

async fn joined_groups(
    user: User,
    group_service: GroupService,
) -> JrnlResult<Json<Vec<SelfGroup>>> {
    group_service
        .get_joined_groups(&user)
        .await
        .map(Json)
        .map_err(Into::into)
}
