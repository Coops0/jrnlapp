use crate::{
    error::{JrnlResult, JsonExtractor},
    schemas::user::User,
    services::user_service::UserService,
    web::deserialize_empty_string,
    AppState,
};
use axum::{routing::get, Json, Router};
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
    user_service: UserService,
    JsonExtractor(payload): JsonExtractor<UpdateSelfPayload>,
) -> JrnlResult<Json<User>> {
    user_service
        .update_user(&user, &payload.theme, &payload.tz.as_ref().map(Tz::to_string))
        .await
        .map(Json)
        .map_err(Into::into)
}
