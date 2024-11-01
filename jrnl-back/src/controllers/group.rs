use crate::schemas::group::Group;
use crate::web::auth::User;
use crate::web::error::{JrnlError, JrnlResult, JsonExtractor};
use crate::AppState;
use axum::extract::{Path, Query, State};
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use reqwest::StatusCode;
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
    members: i32, // doesn't support unsigned ints
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
     GROUP BY g.id
",
    )
        .bind(code)
        .fetch_optional(&pool)
        .await
        .map(Json)
        .map_err(|_| JrnlError::NoResultsFound)
}

async fn join_group(
    Path(code): Path<String>,
    user: User,
    State(AppState { pool, .. }): State<AppState>,
) -> JrnlResult<StatusCode> {
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
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => JrnlError::NoResultsFound,
            sqlx::Error::Database(_) => JrnlError::AlreadyGroupMember,
            _ => JrnlError::DatabaseError(e)
        })
}

#[derive(FromRow, Serialize)]
struct TrimmedUser {
    id: Uuid,
    name: String,
}

async fn get_group_members(
    user: User,
    Path(code): Path<String>,
    State(AppState { pool, .. }): State<AppState>,
) -> JrnlResult<Json<Vec<TrimmedUser>>> {
    sqlx::query_as::<_, TrimmedUser>(
        // language=postgresql
        "
        SELECT p.id, p.name 
        FROM profiles p
        JOIN group_memberships gm ON p.id = gm.user_id
        JOIN groups g ON gm.group_id = g.id
        WHERE g.code = $1 LIMIT 1
        AND EXISTS (
            SELECT 1
            FROM group_memberships gm2
            JOIN groups g2 ON gm2.group_id = g2.id
            WHERE g2.code = $1 
            AND gm2.user_id = $2
        )
    ",
    )
        .bind(code)
        .bind(user.id)
        .fetch_all(&pool)
        .await
        .map(Json)
        .map_err(|_| JrnlError::NoResultsFound)
}

async fn leave_group(
    user: User,
    Path(code): Path<String>,
    State(AppState { pool, .. }): State<AppState>,
) -> JrnlResult<StatusCode> {
    let group = sqlx::query_as::<_, Group>(
        // language=postgresql
        "SELECT * FROM groups WHERE code = $1"
    )
        .bind(&code)
        .fetch_one(&pool)
        .await
        .map_err(|_| JrnlError::NoResultsFound)?;

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

    let members = sqlx::query_scalar::<_, i32>(
        // language=postgresql
        "
        SELECT COUNT(*) FROM group_memberships WHERE group_id = $1
        "
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
            "
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
        .map_err(|_| JrnlError::NoResultsFound)
}

#[derive(Serialize)]
struct DayData {
    scales: Vec<f32>,
    day: chrono::NaiveDate,
}

#[derive(FromRow)]
struct DayDataRow {
    emotion_scale: f32,
    date: chrono::NaiveDate,
}

#[derive(Debug, Deserialize)]
struct GetDaysDataParams {
    day_limit: Option<i64>,
    before: Option<chrono::NaiveDate>,
}

async fn get_days_data_paginated(
    user: User,
    Query(params): Query<GetDaysDataParams>,
    Path(code): Path<String>,
    State(AppState { pool, .. }): State<AppState>,
) -> JrnlResult<Json<Vec<DayData>>> {
    let day_limit = params.day_limit.unwrap_or(7).clamp(1, 30);
    let before = params.before.unwrap_or_else(|| chrono::Utc::now().naive_utc().date());

    let group = sqlx::query_as::<_, Group>(
        // language=postgresql
        "
        SELECT id FROM groups WHERE code = $1 LIMIT 1
        AND EXISTS (
            SELECT 1
            FROM group_memberships gm
            WHERE gm.group_id = groups.id
            AND gm.user_id = $2
        )
        "
    )
        .bind(code)
        .bind(user.id)
        .fetch_one(&pool)
        .await
        .map_err(|_| JrnlError::NoResultsFound)?;

    let group_members = sqlx::query_scalar::<_, Uuid>(
        // language=postgresql
        "
        SELECT user_id FROM group_memberships WHERE group_id = $1
        "
    )
        .bind(group.id)
        .fetch_all(&pool)
        .await?;


    let dates = (0..day_limit)
        .map(|i| before - chrono::Duration::days(i))
        .collect::<Vec<_>>();

    // todo make sure this date checking works
    let all_entries = sqlx::query_as::<_, DayDataRow>(
        // language=postgresql
        "
        SELECT date, emotion_scale FROM entries
        WHERE author IN ($1) AND date IN ($2)
        ORDER BY date DESC
        LIMIT 500
        "
    )
        .bind(&group_members)
        .bind(&dates)
        .fetch_all(&pool)
        .await
        .map_err(JrnlError::DatabaseError)?;

    let entries_grouped_by_day = all_entries
        .into_iter()
        .fold(HashMap::<chrono::NaiveDate, Vec<f32>>::new(), |mut acc, entry| {
            acc
                .entry(entry.date)
                .or_default()
                .push(entry.emotion_scale);
            acc
        })
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
    ",
    )
        .bind(user.id)
        .fetch_all(&pool)
        .await
        .map(Json)
        .map_err(Into::into)
}