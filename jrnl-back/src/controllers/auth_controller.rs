use crate::error::JsonExtractor;
use crate::{
    auth::{
        jwt,
        providers::{verify_apple_id_token, verify_google_credential, AppleCallbackPayload, StrippedGoogleVerificationClaims},
    },
    error::{AppleAuthenticationError, GoogleAuthenticationError, JrnlResult},
    schemas::user::User,
    services::{auth_service::{AuthService, TempAuthSession}, user_service::UserService},
    AppState,
};
use axum::{
    routing::{get, post}
    ,
    Json,
    Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub fn auth_controller() -> Router<AppState> {
    Router::new()
        .route("/session", get(init_session))
        .route("/google", post(google_callback))
        .route("/apple", post(apple_callback))
}

async fn init_session(auth_service: AuthService) -> JrnlResult<Json<TempAuthSession>> {
    auth_service.create_temp_auth_session()
        .await
        .map(Json)
        .map_err(Into::into)
}

#[derive(Serialize)]
struct JrnlTokenResponse {
    token: String,
    user: User,
}

#[derive(Deserialize)]
struct GoogleCallbackPayload {
    credential: String,
    state: Uuid,
}

async fn google_callback(
    user_service: UserService,
    auth_service: AuthService,
    JsonExtractor(payload): JsonExtractor<GoogleCallbackPayload>,
) -> JrnlResult<Json<JrnlTokenResponse>> {
    let nonce = auth_service
        .delete_and_fetch_nonce(&payload.state)
        .await
        .map_err(|_| GoogleAuthenticationError::BadCallbackState(None))?;

    let StrippedGoogleVerificationClaims { sub, name } = verify_google_credential(&payload.credential, &nonce).await
        .map_err(GoogleAuthenticationError::CodeExchangeFailed)?;

    let name = name.as_deref().unwrap_or("no name");

    let user = user_service.create_or_get_user(name, &Some(sub), &None).await?;
    let token = jwt::encode_user_jwt(user.id)?;

    Ok(Json(JrnlTokenResponse { token, user }))
}

async fn apple_callback(
    auth_service: AuthService,
    user_service: UserService,
    JsonExtractor(payload): JsonExtractor<AppleCallbackPayload>,
) -> JrnlResult<Json<JrnlTokenResponse>> {
    let nonce = auth_service
        .delete_and_fetch_nonce(&payload.authorization.state)
        .await
        .map_err(|_| AppleAuthenticationError::BadCallbackState)?;

    let subject = verify_apple_id_token(&payload.authorization.id_token, &nonce)
        .await
        .map_err(AppleAuthenticationError::VerificationError)?;

    let name = payload.user.map_or_else(|| "no name".to_string(), |u| u.name.first_name);


    let user = user_service.create_or_get_user(&name, &None, &Some(subject)).await?;
    let jwt = jwt::encode_user_jwt(user.id)?;

    Ok(Json(JrnlTokenResponse { token: jwt, user }))
}