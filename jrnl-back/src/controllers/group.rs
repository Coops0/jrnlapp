use crate::{
    error::{DatabaseError, JrnlError, JrnlResult, JsonExtractor},
    schemas::group::Group,
    schemas::user::User,
    AppState,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post},
    Json,
    Router,
};
use base64::{
    engine::general_purpose::STANDARD,
    Engine,
};
use chrono::{Duration, NaiveDate};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
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
    State(AppState { pool, .. }): State<AppState>,
    JsonExtractor(CreateGroupPayload { name }): JsonExtractor<CreateGroupPayload>,
) -> JrnlResult<Json<Group>> {
    let existing_owned_groups = sqlx::query_scalar::<_, i64>(
        // language=postgresql
        "SELECT COUNT(*) FROM groups WHERE owner_id = $1",
    )
        .bind(user.id)
        .fetch_one(&pool)
        .await
        .map_err(DatabaseError)?;

    if existing_owned_groups >= 10 {
        return Err(JrnlError::CannotCreateMoreGroups);
    }

    let code = Group::generate_code();
    sqlx::query_as::<_, Group>(
        // language=postgresql
        "INSERT INTO groups (name, code, owner_id) VALUES ($1, $2, $3) RETURNING *",
    )
        .bind(name)
        .bind(code)
        .bind(user.id)
        .fetch_one(&pool)
        .await
        .map(Json)
        .map_err(Into::into)
}

#[derive(Serialize, FromRow)]
struct GetGroupBody {
    id: Uuid,
    name: String,
    members: i64, // doesn't support unsigned ints
}

async fn get_group(
    Path(code): Path<String>,
    State(AppState { pool, .. }): State<AppState>,
) -> JrnlResult<Json<Option<GetGroupBody>>> {
    sqlx::query_as::<_, GetGroupBody>(
        // language=postgresql
        "
    SELECT g.id, g.name, g.owner_id, COUNT(gm.user_id) as members
       FROM groups g
           LEFT JOIN group_memberships gm ON g.id = gm.group_id
     WHERE g.code = $1
     GROUP BY g.id LIMIT 1
        ",
    )
        .bind(code)
        .fetch_optional(&pool)
        .await
        .map(Json)
        .map_err(Into::into)
}

async fn join_group(
    Path(code): Path<String>,
    user: User,
    State(AppState { pool, .. }): State<AppState>,
) -> JrnlResult<StatusCode> {
    let joined_groups = sqlx::query_scalar::<_, i64>(
        // language=postgresql
        "
        SELECT COUNT(*) FROM group_memberships WHERE user_id = $1
        ",
    )
        .bind(user.id)
        .fetch_one(&pool)
        .await
        .map_err(DatabaseError)?;

    if joined_groups >= 20 {
        return Err(JrnlError::CannotJoinMoreGroups);
    }

    sqlx::query(
        // language=postgresql
        "
        INSERT INTO group_memberships (group_id, user_id) VALUES (
                (SELECT id FROM groups WHERE code = $1 LIMIT 1),
                $2
            )
        ",
    )
        .bind(code)
        .bind(user.id)
        .execute(&pool)
        .await
        .map(|_| StatusCode::OK)
        .map_err(|why| match &why {
            sqlx::Error::Database(d) if d.is_unique_violation() => JrnlError::AlreadyGroupMember,
            _ => DatabaseError(why).into(),
        })
}

#[derive(FromRow)]
struct TrimmedUser {
    id: Uuid,
    name: String,
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
    State(AppState { pool, .. }): State<AppState>,
) -> JrnlResult<Json<Vec<TrimmedUserWithOwner>>> {
    let group = sqlx::query_as::<_, Group>(
        // language=postgresql
        "
        SELECT * FROM groups WHERE code = $1
        AND EXISTS (
            SELECT 1
            FROM group_memberships gm
            WHERE gm.group_id = groups.id
            AND gm.user_id = $2
        )
        LIMIT 1
        ",
    )
        .bind(code)
        .bind(user.id)
        .fetch_one(&pool)
        .await
        .map_err(DatabaseError)?;

    let members = sqlx::query_as::<_, TrimmedUser>(
        // language=postgresql
        "
        SELECT u.id, u.name FROM users u
        JOIN group_memberships gm ON u.id = gm.user_id
        WHERE gm.group_id = $1

    ",
    )
        .bind(group.id)
        .fetch_all(&pool)
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
    State(AppState { pool, .. }): State<AppState>,
) -> JrnlResult<StatusCode> {
    let group = sqlx::query_as::<_, Group>(
        // language=postgresql
        "SELECT * FROM groups WHERE code = $1",
    )
        .bind(&code)
        .fetch_one(&pool)
        .await
        .map_err(DatabaseError)?;

    sqlx::query(
        // language=postgresql
        "
        DELETE FROM group_memberships
        WHERE group_id = $1
        AND user_id = $2
    ",
    )
        .bind(group.id)
        .bind(user.id)
        .execute(&pool)
        .await?;

    if group.owner_id != user.id {
        return Ok(StatusCode::OK);
    }

    let members = sqlx::query_scalar::<_, i64>(
        // language=postgresql
        "
        SELECT COUNT(*) FROM group_memberships WHERE group_id = $1
        ",
    )
        .bind(group.id)
        .fetch_one(&pool)
        .await?;

    if members == 0 {
        sqlx::query("DELETE FROM groups WHERE id = $1")
            .bind(group.id)
            .execute(&pool)
            .await?;
    } else {
        sqlx::query(
            // language=postgresql
            "
            UPDATE groups SET owner_id = (
               SELECT user_id FROM group_memberships WHERE group_id = $1 LIMIT 1
            )
            WHERE id = $1
            ",
        )
            .bind(group.id)
            .execute(&pool)
            .await?;
    }

    Ok(StatusCode::OK)
}

async fn kick_member(
    user: User,
    Path((code, target_user_id)): Path<(String, Uuid)>,
    State(AppState { pool, .. }): State<AppState>,
) -> JrnlResult<StatusCode> {
    if user.id == target_user_id {
        return Err(JrnlError::CannotKickSelf);
    }

    sqlx::query(
        // language=postgresql
        "
        DELETE FROM group_memberships
        WHERE group_id = (SELECT id FROM groups WHERE code = $1 AND owner_id = $2)
        AND user_id = $3
    ",
    )
        .bind(code)
        .bind(user.id)
        .bind(target_user_id)
        .execute(&pool)
        .await
        .map(|_| StatusCode::OK)
        .map_err(Into::into)
}

#[derive(Serialize)]
struct DayData {
    scales: Vec<f32>,
    day: NaiveDate,
}

#[derive(FromRow)]
struct DayDataRow {
    emotion_scale: f32,
    date: NaiveDate,
}

#[derive(Debug, Deserialize)]
struct GetDaysDataParams {
    day_limit: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_base_date")]
    before: Option<NaiveDate>,
}

fn deserialize_base_date<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<Option<NaiveDate>, D::Error> {
    let Some(encoded_date) = Option::<String>::deserialize(deserializer)? else {
        return Ok(None)
    };

    let decoded_bytes = STANDARD.decode(encoded_date).map_err(serde::de::Error::custom)?;
    let date_string = String::from_utf8(decoded_bytes).map_err(serde::de::Error::custom)?;

    NaiveDate::parse_from_str(&date_string, "%m/%d/%Y")
        .map(Some)
        .map_err(serde::de::Error::custom)
}

async fn get_days_data_paginated(
    user: User,
    Query(params): Query<GetDaysDataParams>,
    Path(code): Path<String>,
    State(AppState { pool, .. }): State<AppState>,
) -> JrnlResult<Json<Vec<DayData>>> {
    let day_limit = params.day_limit.unwrap_or(7).clamp(1, 30);
    let before_date = params
        .before
        .unwrap_or_else(|| chrono::Utc::now().naive_utc().date());

    let group_id = sqlx::query_scalar::<_, Uuid>(
        // language=postgresql
        "
        SELECT id FROM groups WHERE code = $1
        AND EXISTS (
            SELECT 1
            FROM group_memberships gm
            WHERE gm.group_id = groups.id
            AND gm.user_id = $2
        )
        LIMIT 1
        ",
    )
        .bind(code)
        .bind(user.id)
        .fetch_one(&pool)
        .await
        .map_err(DatabaseError)?;

    let group_members = sqlx::query_scalar::<_, Uuid>(
        // language=postgresql
        "
        SELECT user_id FROM group_memberships WHERE group_id = $1
        ",
    )
        .bind(group_id)
        .fetch_all(&pool)
        .await?;


    let start_date = before_date - Duration::days(day_limit - 1);

    let all_entries = sqlx::query_as::<_, DayDataRow>(
        // language=postgresql
        "
        SELECT date, emotion_scale FROM entries
        WHERE author = ANY($1)
        AND date >= $2
        AND date <= $3
        ORDER BY date DESC
        LIMIT 500
        ",
    )
        .bind(&group_members)
        .bind(start_date)
        .bind(before_date)
        .fetch_all(&pool)
        .await
        .map_err(DatabaseError)?;

    let entries_grouped_by_day = all_entries
        .into_iter()
        .fold(
            HashMap::<NaiveDate, Vec<f32>>::new(),
            |mut acc, entry| {
                acc.entry(entry.date).or_default().push(entry.emotion_scale);
                acc
            },
        )
        .into_iter()
        .map(|(day, scales)| DayData { scales, day })
        .collect::<Vec<_>>();

    Ok(Json(entries_grouped_by_day))
}

#[derive(Serialize, FromRow)]
struct SelfGroup {
    id: Uuid,
    name: String,
    code: String,
}

async fn joined_groups(
    user: User,
    State(AppState { pool, .. }): State<AppState>,
) -> JrnlResult<Json<Vec<SelfGroup>>> {
    sqlx::query_as::<_, SelfGroup>(
        // language=postgresql
        "
        SELECT *, gm.group_id as group_id FROM group_memberships gm
        JOIN groups g ON gm.group_id = g.id
        WHERE gm.user_id = $1
    "
    )
        .bind(user.id)
        .fetch_all(&pool)
        .await
        .map(Json)
        .map_err(Into::into)
}
