use crate::{
    auth::jwt::{decode_user_jwt, Claims},
    error::JrnlError,
    schemas::user::User,
    services::user_service::UserService,
    AppState,
};
use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
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
            .ok_or(JrnlError::BadAuthenticationHeader)?;

        let claims = decode_user_jwt(auth_header).map_err(|_| JrnlError::InvalidToken)?;

        let parsed_exp =
            usize::try_from(Utc::now().timestamp()).map_err(Into::<anyhow::Error>::into)?;
        if claims.exp < parsed_exp {
            return Err(JrnlError::ExpiredToken);
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
        let user_service = UserService::from_request_parts(parts, state).await.unwrap();

        user_service
            .get_user_by_id(&sub)
            .await
            .map_err(|_| JrnlError::ProfileNotFound)
    }
}
