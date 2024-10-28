use crate::web::auth::User;
use crate::web::result::InternalResult;
use crate::AppState;
use axum::extract::State;
use axum::routing::put;
use axum::{Json, Router};
use chrono_tz::Tz;
use reqwest::StatusCode;
use serde::{Deserialize, Deserializer};

pub fn users_controller() -> Router<AppState> {
    Router::new()
        .route("/tz", put(update_timezone))
}

#[derive(Deserialize)]
struct UpdateTimezonePayload {
    // Intl.DateTimeFormat().resolvedOptions().timeZone
    #[serde(deserialize_with = "deserialize_tz")]
    tz: Tz,
}

fn deserialize_tz<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Tz, D::Error> {
    String::deserialize(deserializer)?
        .parse::<Tz>()
        .map_err(serde::de::Error::custom)
}

async fn update_timezone(
    user: User,
    State(AppState { pool, .. }): State<AppState>,
    Json(UpdateTimezonePayload { tz }): Json<UpdateTimezonePayload>,
) -> InternalResult<StatusCode> {
    sqlx::query("UPDATE public.profiles SET timezone = $1 WHERE id = $2")
        .bind(tz.to_string())
        .bind(user.id)
        .execute(&pool)
        .await
        .map(|_| StatusCode::OK)
        .map_err(Into::into)
}