use crate::schemas::entry::Entry;
use crate::schemas::profile::Profile;
use crate::web::auth::User;
use crate::web::cursor::{Cursor, CursorPaginatedResponse, CursorParams};
use crate::web::error::{DatabaseError, JrnlResult, JsonExtractor};
use crate::AppState;
use axum::extract::{Path, Query, State};
use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

pub fn entries_controller() -> Router<AppState> {
    Router::new()
        .route("/", get(get_trimmed_entries_paginated))
        .route("/:id", get(get_entry))
        .route("/average", get(get_overall_average))
        .route("/today", get(get_today_entry).put(update_today_entry))
}

#[derive(Serialize, FromRow)]
struct StrippedEntry {
    emotion_scale: f32,
    date: chrono::NaiveDate,
    id: Uuid,
}

async fn get_trimmed_entries_paginated(
    user: User,
    Query(params): Query<CursorParams>,
    State(AppState { pool, .. }): State<AppState>,
) -> JrnlResult<Json<CursorPaginatedResponse<StrippedEntry>>> {
    let limit = params.limit.unwrap_or(20).clamp(1, 100);
    let limit_plus_one = i64::from(limit) + 1;

    let cursor = params.cursor.unwrap_or_default();

    let mut entries = sqlx::query_as::<_, StrippedEntry>(
        // language=postgresql
        "
            SELECT emotion_scale, date, id FROM entries
            WHERE entries.author = $1
            AND (date, id) < ($2, $3)
            ORDER BY date DESC, id DESC
            LIMIT $4
            "
    )
        .bind(user.id)
        .bind(cursor.date)
        .bind(cursor.id)
        .bind(limit_plus_one)
        .fetch_all(&pool)
        .await
        .map_err(DatabaseError)?;

    let has_more = entries.len() > limit as usize;
    if has_more {
        entries.pop();
    }

    let next_cursor = match (has_more, entries.last()) {
        (true, Some(last_entry)) => Some(Cursor { id: last_entry.id, date: last_entry.date }),
        _ => None
    };

    Ok(Json(CursorPaginatedResponse {
        items: entries,
        next_cursor,
        has_more,
    }))
}

async fn get_entry(
    user: User,
    Path(id): Path<Uuid>,
    State(AppState { pool, .. }): State<AppState>,
) -> JrnlResult<Json<Option<Entry>>> {
    sqlx::query_as::<_, Entry>(
        // language=postgresql
        "SELECT * FROM entries WHERE author = $1 AND id = $2 LIMIT 1"
    )
        .bind(user.id)
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map(Json)
        .map_err(Into::into)
}

async fn get_overall_average(
    user: User,
    State(AppState { pool, .. }): State<AppState>,
) -> JrnlResult<Json<f64>> {
    sqlx::query_scalar("SELECT AVG(emotion_scale) FROM entries WHERE author = $1")
        .bind(user.id)
        .fetch_one(&pool)
        .await
        .map(Json)
        .map_err(Into::into)
}

async fn get_today_entry(
    profile: Profile,
    State(AppState { pool, .. }): State<AppState>,
) -> JrnlResult<Json<Option<Entry>>> {
    sqlx::query_as::<_, Entry>("SELECT * FROM entries WHERE author = $1 AND date = $2 LIMIT 1")
        .bind(profile.id)
        .bind(profile.current_date_by_timezone())
        .fetch_optional(&pool)
        .await
        .map(Json)
        .map_err(Into::into)
}

#[derive(Deserialize)]
struct UpdateEntryPayload {
    emotion_scale: f32,
    #[serde(deserialize_with = "clean_string")]
    text: Option<String>,
}

#[allow(clippy::unnecessary_wraps)]
fn clean_string<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<String>, D::Error> {
    let Ok(s) = String::deserialize(deserializer) else {
        return Ok(None);
    };

    let trimmed = s.trim();

    Ok(if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    })
}

async fn update_today_entry(
    profile: Profile,
    State(AppState { pool, .. }): State<AppState>,
    JsonExtractor(payload): JsonExtractor<UpdateEntryPayload>,
) -> JrnlResult<Json<Entry>> {
    sqlx::query_as::<_, Entry>(
        // language=postgresql
        "
            INSERT INTO entries (author, date, emotion_scale, text) VALUES ($1, $2, $3, $4) 
            ON CONFLICT (author, date) 
            DO UPDATE SET emotion_scale = $3, text = $4 
            RETURNING *
        "
    )
        .bind(profile.id)
        .bind(profile.current_date_by_timezone())
        .bind(payload.emotion_scale)
        .bind(payload.text)
        .fetch_one(&pool)
        .await
        .map(Json)
        .map_err(Into::into)
}
