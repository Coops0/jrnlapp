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
    response::{IntoResponse, Redirect},
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
        .route("/google/callback", post(google_callback))
        .route("/apple/callback", post(apple_callback))
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
    user_service: UserService,
    auth_service: AuthService,
    headers: HeaderMap,
    Form(GoogleCallbackPayload { credential, g_csrf_token, state }): Form<GoogleCallbackPayload>,
) -> JrnlResult<JrnlTokenResponse> {
    let csrf_cookie = headers.get(COOKIE)
        .and_then(|c| c.to_str().ok())
        .and_then(|c| c.split_once("g_csrf_token="))
        .map(|(_, c)| c.trim().trim_matches(';'));

    if csrf_cookie != Some(&*g_csrf_token) {
        return Err(GoogleAuthenticationError::BadCallbackState(None).into());
    }

    let nonce = auth_service
        .delete_and_fetch_nonce(&state)
        .await
        .map_err(|_| GoogleAuthenticationError::BadCallbackState(None))?;

    let StrippedGoogleVerificationClaims { sub, name } = verify_google_credential(&credential, &nonce).await
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
) -> JrnlResult<impl IntoResponse> {
    let response = inner_google_callback(user_service, auth_service, headers, Form(payload)).await;
    serve_response_flash(response)
}

async fn init_session(auth_service: AuthService) -> JrnlResult<Json<TempAuthSession>> {
    auth_service.create_temp_auth_session()
        .await
        .map(Json)
        .map_err(Into::into)
}

async fn apple_callback_inner(
    auth_service: AuthService,
    user_service: UserService,
    Form(AppleCallbackPayload { id_token, user, state, .. }): Form<AppleCallbackPayload>,
) -> JrnlResult<JrnlTokenResponse> {
    let nonce = auth_service
        .delete_and_fetch_nonce(&state)
        .await
        .map_err(|_| AppleAuthenticationError::BadCallbackState)?;

    let subject = verify_apple_id_token(&id_token, &nonce)
        .await
        .map_err(AppleAuthenticationError::VerificationError)?;

    let name = user.map(|u| u.name.first_name);

    let user = user_service.create_or_get_user(&name, &None, &Some(subject)).await?;
    let jwt = jwt::encode_user_jwt(user.id)?;

    Ok(JrnlTokenResponse { token: jwt, user })
}

// no actual error should ever be triggered in the jrnlresult, they should just be serialized and passed
async fn apple_callback(auth_service: AuthService, user_service: UserService, Form(payload): Form<AppleCallbackPayload>) -> JrnlResult<Redirect> {
    let response = apple_callback_inner(auth_service, user_service, Form(payload)).await;
    serve_response_flash(response)
}

fn serve_response_flash(response: JrnlResult<JrnlTokenResponse>) -> JrnlResult<Redirect> {
    let json_string = match response {
        Ok(value) => serde_json::to_string(&value),
        Err(err) => serde_json::to_string(&json!({ "error": err.to_string() })),
    }.context("failed to serialize response")?;

    let encoded_string = STANDARD.encode(json_string.as_bytes());
    let redirect_uri = format!(
        "{}/cb?r={encoded_string}",
        env::var("FRONTEND_URL").context("missing frontend url?")?,
    );

    Ok(Redirect::temporary(&redirect_uri))
}