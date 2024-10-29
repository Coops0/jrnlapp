use crate::schemas::entry::Entry;
use crate::schemas::profile::Profile;
use crate::web::auth::User;
use crate::web::cursor::{Cursor, CursorPaginatedResponse, CursorParams};
use crate::web::result::InternalResult;
use crate::AppState;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use base64::Engine;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

pub fn entries_controller() -> Router<AppState> {
    Router::new()
        .route("/", get(get_entries_paginated))
        .route("/ratings", get(get_ratings_paginated))
        .route("/average", get(get_overall_average))
        .route("/today", get(get_today_entry).put(update_today_entry))
}

async fn get_entries_paginated(
    user: User,
    Query(params): Query<CursorParams>,
    State(AppState { pool, .. }): State<AppState>,
) -> InternalResult<Json<CursorPaginatedResponse<Entry>>> {
    let limit = params.limit.unwrap_or(20).clamp(1, 100);
    let limit_plus_one = i64::from(limit) + 1;

    let cursor = params.cursor.unwrap_or_default();

    let mut entries = sqlx::query_as::<_, Entry>(
        // language=postgresql
        "
            SELECT * FROM entries
            WHERE author = $1
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
        .await?;

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

#[derive(FromRow, Serialize)]
struct Rating {
    id: Uuid,
    date: NaiveDate,
    emotion_scale: f32,
}

async fn get_ratings_paginated(
    user: User,
    Query(params): Query<CursorParams>,
    State(AppState { pool, .. }): State<AppState>,
) -> InternalResult<Json<CursorPaginatedResponse<Rating>>> {
    let limit = params.limit.unwrap_or(50).clamp(1, 200);
    let limit_plus_one = i64::from(limit) + 1;

    let cursor = params.cursor.unwrap_or_default();

    let mut ratings = sqlx::query_as::<_, Rating>(
        // language=postgresql
        "
            SELECT (id, date, emotion_scale) FROM entries
            WHERE author = $1
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
        .await?;

    let has_more = ratings.len() > limit as usize;
    if has_more {
        ratings.pop();
    }

    let next_cursor = match (has_more, ratings.last()) {
        (true, Some(last_rating)) => Some(Cursor { id: last_rating.id, date: last_rating.date }),
        _ => None
    };

    Ok(Json(CursorPaginatedResponse {
        items: ratings,
        next_cursor,
        has_more,
    }))
}

async fn get_overall_average(
    user: User,
    State(AppState { pool, .. }): State<AppState>,
) -> InternalResult<Json<f64>> {
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
) -> InternalResult<Json<Option<Entry>>> {
    sqlx::query_as::<_, Entry>("SELECT * FROM entries WHERE author = $1 AND date = $2")
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

fn clean_string<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<String>, D::Error> {
    let s = String::deserialize(deserializer)?;
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
    Json(payload): Json<UpdateEntryPayload>,
) -> Result<Json<Entry>, Response> {
    let entry = sqlx::query_as::<_, Entry>(
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
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response())?;

    Ok(Json(entry))
}
