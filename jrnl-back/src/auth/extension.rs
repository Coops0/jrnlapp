use crate::auth::jwt::{decode_jwt, Claims};
use crate::error::{AuthenticationError, JrnlError};
use crate::schemas::user::User;
use crate::AppState;
use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use chrono::Utc;

#[async_trait]
impl<S> FromRequestParts<S> for Claims {
    type Rejection = JrnlError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .and_then(|header| header.strip_prefix("Bearer "))
            .ok_or_else(|| {
                JrnlError::AuthenticationError(AuthenticationError::BadAuthenticationHeader)
            })?;

        let claims = decode_jwt(auth_header)
            .map_err(|_| JrnlError::AuthenticationError(AuthenticationError::InvalidToken))?;

        if claims.exp
            < usize::try_from(Utc::now().timestamp()).map_err(Into::<anyhow::Error>::into)?
        {
            return Err(JrnlError::AuthenticationError(
                AuthenticationError::ExpiredToken,
            ));
        }

        Ok(claims)
    }
}

#[async_trait]
impl FromRequestParts<AppState> for User {
    type Rejection = JrnlError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let Claims { sub, .. } = Claims::from_request_parts(parts, state).await?;

        sqlx::query_as::<_, Self>(
            // language=postgresql
            "
            SELECT * FROM users WHERE id = $1 LIMIT 1
            ",
        )
        .bind(sub)
        .fetch_one(&state.pool)
        .await
        .map_err(|_| AuthenticationError::ProfileNotFound.into())
    }
}
