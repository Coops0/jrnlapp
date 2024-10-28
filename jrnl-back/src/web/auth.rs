use crate::schemas::profile::Profile;
use crate::AppState;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::{async_trait, RequestPartsExt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub role: String,
}

#[async_trait]
impl FromRequestParts<AppState> for User {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .and_then(|header| header.strip_prefix("Bearer "))
            .ok_or_else(|| {
                (
                    StatusCode::UNAUTHORIZED,
                    String::from("Missing or invalid authorization header"),
                )
            })?;

        let client = reqwest::Client::new();
        let user_response = client
            .get(format!("{}/auth/v1/user", state.supabase_url))
            .header("Authorization", format!("Bearer {auth_header}"))
            // .header("apikey", &state.supabase_key)
            .header("apikey", auth_header)
            .send()
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to verify token: {e}"),
                )
            })?;

        if !user_response.status().is_success() {
            return Err((StatusCode::UNAUTHORIZED, String::from("invalid token")));
        }

        user_response.json::<Self>().await.map_or_else(
            |e| {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to parse user data: {e}"),
                ))
            },
            Ok,
        )
    }
}

#[async_trait]
impl FromRequestParts<AppState> for Profile {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let user = parts.extract_with_state::<User, AppState>(state).await?;

        let profile = sqlx::query_as::<_, Self>("SELECT * FROM profiles WHERE id = $1")
            .bind(user.id)
            .fetch_optional(&state.pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        profile.map_or_else(
            // this shouldn't happen hopefully
            || Err((StatusCode::NOT_FOUND, String::from("Profile not found"))),
            Ok,
        )
    }
}
