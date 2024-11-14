use crate::auth::providers::AppleCallbackUser;
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
use anyhow::Context;
use axum::{
    http::{header::COOKIE, HeaderMap},
    response::Redirect,
    routing::{get, post},
    Form,
    Json,
    Router,
};
use base64::{engine::general_purpose::STANDARD, Engine};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use uuid::Uuid;

pub fn auth_controller() -> Router<AppState> {
    Router::new()
        .route("/session", get(init_session))
        .route("/take-session", get(take_oneshot_session))
        .route("/google/callback", post(google_callback))
        .route("/google/callback/mobile", post(google_callback_mobile))
        .route("/apple/callback", post(apple_callback))
        .route("/apple/callback/mobile", post(apple_callback_mobile))
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
    g_csrf_token: String,
    state: Uuid,
}

async fn inner_google_callback(
    user_service: &UserService,
    headers: &HeaderMap,
    credential: &str,
    g_csrf_token: &str,
    nonce: &Uuid,
) -> JrnlResult<JrnlTokenResponse> {
    let csrf_cookie = headers.get(COOKIE)
        .and_then(|c| c.to_str().ok())
        .and_then(|c| c.split_once("g_csrf_token="))
        .map(|(_, c)| c.trim().trim_matches(';'));

    if csrf_cookie != Some(g_csrf_token) {
        return Err(GoogleAuthenticationError::BadCallbackState(None).into());
    }

    let StrippedGoogleVerificationClaims { sub, name } = verify_google_credential(credential, nonce).await
        .map_err(GoogleAuthenticationError::CodeExchangeFailed)?;

    let user = user_service.create_or_get_user(&name, &Some(sub), &None).await?;
    let jwt = jwt::encode_user_jwt(user.id)?;

    Ok(JrnlTokenResponse { token: jwt, user })
}

async fn google_callback(
    user_service: UserService,
    auth_service: AuthService,
    headers: HeaderMap,
    Form(payload): Form<GoogleCallbackPayload>,
) -> JrnlResult<Redirect> {
    let nonce = auth_service
        .delete_and_fetch_nonce(&payload.state)
        .await
        .map_err(|_| GoogleAuthenticationError::BadCallbackState(None))?;

    let response = inner_google_callback(&user_service, &headers, &payload.credential, &payload.g_csrf_token, &nonce).await;
    serve_response_flash(response)
}

async fn google_callback_mobile(
    user_service: UserService,
    auth_service: AuthService,
    headers: HeaderMap,
    Form(payload): Form<GoogleCallbackPayload>,
) -> JrnlResult<()> {
    let nonce = auth_service
        .delete_and_fetch_nonce(&payload.state)
        .await
        .map_err(|_| GoogleAuthenticationError::BadCallbackState(None))?;

    let response = inner_google_callback(&user_service, &headers, &payload.credential, &payload.g_csrf_token, &nonce).await;

    let encoded_string = encode_jrnl_token_response(response)?;
    auth_service.create_mobile_nonce_oneshot(&nonce, &encoded_string).await?;

    Ok(())
}

async fn apple_callback_inner(
    user_service: &UserService,
    id_token: &str,
    nonce: &Uuid,
    user: &Option<AppleCallbackUser>,
) -> JrnlResult<JrnlTokenResponse> {
    let subject = verify_apple_id_token(id_token, nonce)
        .await
        .map_err(AppleAuthenticationError::VerificationError)?;

    let name = user.as_ref().map(|u| u.name.first_name.clone());

    let user = user_service.create_or_get_user(&name, &None, &Some(subject)).await?;
    let jwt = jwt::encode_user_jwt(user.id)?;

    Ok(JrnlTokenResponse { token: jwt, user })
}

async fn apple_callback(
    auth_service: AuthService,
    user_service: UserService,
    Form(payload): Form<AppleCallbackPayload>,
) -> JrnlResult<Redirect> {
    let nonce = auth_service
        .delete_and_fetch_nonce(&payload.state)
        .await
        .map_err(|_| AppleAuthenticationError::BadCallbackState)?;

    let response = apple_callback_inner(&user_service, &payload.id_token, &nonce, &payload.user).await;
    serve_response_flash(response)
}

async fn apple_callback_mobile(
    auth_service: AuthService,
    user_service: UserService,
    Form(payload): Form<AppleCallbackPayload>,
) -> JrnlResult<()> {
    let nonce = auth_service
        .delete_and_fetch_nonce(&payload.state)
        .await
        .map_err(|_| AppleAuthenticationError::BadCallbackState)?;

    let response = apple_callback_inner(&user_service, &payload.id_token, &nonce, &payload.user).await;

    let encoded_string = encode_jrnl_token_response(response)?;
    auth_service.create_mobile_nonce_oneshot(&nonce, &encoded_string).await?;

    Ok(())
}

#[derive(Deserialize)]
struct OneshotSessionPayload {
    nonce: Uuid,
}

async fn take_oneshot_session(
    auth_service: AuthService,
    JsonExtractor(OneshotSessionPayload { nonce }): JsonExtractor<OneshotSessionPayload>,
) -> JrnlResult<String> {
    auth_service
        .delete_and_fetch_mobile_nonce_oneshot(&nonce)
        .await
        .map_err(Into::into)
}

fn encode_jrnl_token_response(response: JrnlResult<JrnlTokenResponse>) -> JrnlResult<String> {
    let json_string = match response {
        Ok(value) => serde_json::to_string(&value),
        Err(err) => serde_json::to_string(&json!({ "error": err.to_string() })),
    }.context("failed to serialize response")?;

    Ok(STANDARD.encode(json_string.as_bytes()))
}

fn serve_response_flash(response: JrnlResult<JrnlTokenResponse>) -> JrnlResult<Redirect> {
    let encoded_string = encode_jrnl_token_response(response)?;
    let redirect_uri = format!(
        "{}/cb?r={encoded_string}",
        env::var("FRONTEND_URL").context("missing frontend url?")?,
    );

    Ok(Redirect::temporary(&redirect_uri))
}