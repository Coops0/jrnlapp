use crate::schemas::profile::Profile;
use crate::web::auth::User;
use crate::web::error::{JrnlResult, JsonExtractor};
use crate::AppState;
use axum::extract::State;
use axum::routing::{get, put};
use axum::{Json, Router};
use chrono_tz::Tz;
use reqwest::StatusCode;
use serde::{Deserialize, Deserializer};

pub fn users_controller() -> Router<AppState> {
    Router::new()
        .route("/tz", put(update_timezone))
        .route("/me", get(get_self_profile))
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
    JsonExtractor(UpdateTimezonePayload { tz }): JsonExtractor<UpdateTimezonePayload>,
) -> JrnlResult<StatusCode> {
    sqlx::query("UPDATE profiles SET timezone = $1 WHERE id = $2")
        .bind(tz.to_string())
        .bind(user.id)
        .execute(&pool)
        .await
        .map(|_| StatusCode::OK)
        .map_err(Into::into)
}

async fn get_self_profile(profile: Profile) -> Json<Profile> {
    Json(profile)
}