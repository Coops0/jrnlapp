use crate::schemas::user::User;
use crate::error::{AuthenticationError, JrnlError};
use crate::AppState;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::{async_trait, RequestPartsExt};
use axum_extra::extract::CookieJar;

#[async_trait]
impl FromRequestParts<AppState> for User {
    type Rejection = JrnlError;

    async fn from_request_parts(parts: &mut Parts, AppState { pool }: &AppState) -> Result<Self, Self::Rejection> {
        let c_jar = parts.extract::<CookieJar>().await
            .map_err(|_| AuthenticationError::NoSessionCookie)?;

        let session_id = c_jar.get("session")
            .ok_or(AuthenticationError::NoSessionCookie)?
            .value();

        let user = sqlx::query_as::<_, Self>(
            // language=postgresql
            "
            SELECT * FROM users
                JOIN sessions s ON users.id = s.user_id
                WHERE s.id = $1 AND s.expires_at > NOW()
            LIMIT 1
            "
        )
            .bind(session_id)
            .fetch_one(pool)
            .await
            .map_err(|_| AuthenticationError::InvalidSessionId)?;

        Ok(user)
    }
}