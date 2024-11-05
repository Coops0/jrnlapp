use crate::error::{JrnlResult, JsonExtractor};
use crate::schemas::user::User;
use crate::web::deserialize_empty_string;
use crate::AppState;
use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use chrono_tz::Tz;
use serde::{Deserialize, Deserializer};

pub fn users_controller() -> Router<AppState> {
    Router::new().route("/me", get(get_self_user).patch(update_self_user))
}

async fn get_self_user(user: User) -> Json<User> {
    Json(user)
}

#[derive(Debug, Deserialize)]
struct UpdateSelfPayload {
    // Intl.DateTimeFormat().resolvedOptions().timeZone
    #[serde(default, deserialize_with = "deserialize_tz")]
    tz: Option<Tz>,

    #[serde(default, deserialize_with = "deserialize_empty_string")]
    theme: Option<String>,
}

fn deserialize_tz<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<Tz>, D::Error> {
    let Ok(str) = String::deserialize(deserializer) else {
        return Ok(None);
    };

    if str.is_empty() {
        return Ok(None);
    }

    str.parse::<Tz>()
        .map(Some)
        .map_err(serde::de::Error::custom)
}

async fn update_self_user(
    user: User,
    State(AppState { pool }): State<AppState>,
    JsonExtractor(payload): JsonExtractor<UpdateSelfPayload>,
) -> JrnlResult<Json<User>> {
    sqlx::query_as::<_, User>(
        // language=postgresql
        "UPDATE users SET
            timezone = COALESCE($1, timezone),
            theme = COALESCE($2, theme)
            WHERE id = $3 RETURNING *
        ",
    )
    .bind(payload.tz.map(|tz| tz.to_string()))
    .bind(payload.theme)
    .bind(user.id)
    .fetch_one(&pool)
    .await
    .map(Json)
    .map_err(Into::into)
}
