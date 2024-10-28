use crate::schemas::entry::Entry;
use crate::schemas::profile::Profile;
use crate::web::auth::User;
use crate::web::result::InternalResult;
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow};
use uuid::Uuid;

pub fn entries_controller() -> Router<AppState> {
    Router::new()
        .route("/", get(get_entries_full))
        .route("/ratings", get(get_ratings))
        .route("/average", get(get_overall_average))
        .route("/today", get(get_today_entry).post(update_today_entry))
}

async fn get_entries_full(user: User, State(AppState { pool, .. }): State<AppState>) -> InternalResult<Json<Vec<Entry>>> {
    sqlx::query_as::<_, Entry>("SELECT * FROM entries WHERE author = $1 ORDER BY date DESC LIMIT 50")
        .bind(user.id)
        .fetch_all(&pool)
        .await
        .map(Json)
        .map_err(Into::into)
}

#[derive(FromRow, Serialize)]
struct Rating {
    id: Uuid,
    date: chrono::NaiveDate,
    emotion_scale: f32,
}

async fn get_ratings(user: User, State(AppState { pool, .. }): State<AppState>) -> InternalResult<Json<Vec<Rating>>> {
    sqlx::query_as::<_, Rating>(
        "SELECT id, date, emotion_scale FROM entries WHERE author = $1 ORDER BY date DESC LIMIT 100")
        .bind(user.id)
        .fetch_all(&pool)
        .await
        .map(Json)
        .map_err(Into::into)
}

async fn get_overall_average(user: User, State(AppState { pool, .. }): State<AppState>) -> InternalResult<Json<f64>> {
    sqlx::query_scalar("SELECT AVG(emotion_scale) FROM entries WHERE author = $1")
        .bind(user.id)
        .fetch_one(&pool)
        .await
        .map(Json)
        .map_err(Into::into)
}

async fn get_today_entry(profile: Profile, State(AppState { pool, .. }): State<AppState>) -> InternalResult<Json<Option<Entry>>> {
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

    // Intl.DateTimeFormat().resolvedOptions().timeZone
    tz: Option<String>,
}

fn clean_string<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<Option<String>, D::Error> {
    let s = String::deserialize(deserializer)?;
    let trimmed = s.trim();

    Ok(if trimmed.is_empty() { None } else { Some(trimmed.to_string()) })
}

async fn update_today_entry(
    mut profile: Profile,
    State(AppState { pool, .. }): State<AppState>,
    Json(payload): Json<UpdateEntryPayload>,
) -> Result<Json<Entry>, Response> {
    if profile.timezone == Tz::UTC.to_string() {
        // assume hasn't been set yet
        if let Some(tz) = payload.tz {
            let tz: Tz = tz.parse()
                .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid timezone identifier".to_string()).into_response())?;

            profile.timezone = tz.to_string();
            let _ = sqlx::query("UPDATE public.profiles SET timezone = $1 WHERE id = $2")
                .bind(tz.to_string())
                .bind(profile.id)
                .execute(&pool)
                .await;
        }
    }

    let entry = sqlx::query_as::<_, Entry>(
        "INSERT INTO entries (author, date, emotion_scale, text) VALUES ($1, $2, $3, $4) ON CONFLICT (author, date) DO UPDATE SET emotion_scale = $3, text = $4 RETURNING *"
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