use crate::{
    auth::jwt,
    auth::providers::{verify_apple_id_token, verify_google_credential, AppleCallbackPayload, StrippedGoogleVerificationClaims},
    error::AppleAuthenticationError,
    error::{GoogleAuthenticationError, JrnlResult},
    services::{
        auth_service::{AuthService, TempAuthSession},
        user_service::UserService,
    },
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
use serde::Deserialize;
use serde_json::json;
use std::env;

pub fn auth_controller() -> Router<AppState> {
    Router::new()
        .route("/session", get(init_session))
        .route("/google/callback", post(google_callback))
        .route("/apple/callback", post(apple_callback))
}

#[derive(Deserialize)]
struct GoogleCallbackPayload {
    credential: String,
    g_csrf_token: String,
}

async fn inner_google_callback(
    user_service: UserService,
    headers: HeaderMap,
    Form(GoogleCallbackPayload { credential, g_csrf_token }): Form<GoogleCallbackPayload>,
) -> JrnlResult<serde_json::Value> {
    let csrf_cookie = headers.get(COOKIE)
        .and_then(|c| c.to_str().ok())
        .and_then(|c| c.split_once("g_csrf_token="))
        .map(|(_, c)| c.trim().trim_matches(';'));

    if csrf_cookie != Some(&*g_csrf_token) {
        return Err(GoogleAuthenticationError::BadCallbackState(None).into());
    }

    let StrippedGoogleVerificationClaims { sub, name } = verify_google_credential(&credential).await
        .map_err(GoogleAuthenticationError::CodeExchangeFailed)?;

    let user = user_service.create_or_get_user(&name, &Some(sub), &None).await?;
    let jwt = jwt::encode_user_jwt(user.id)?;

    Ok(json!({ "token": jwt, "user": user }))
}

async fn google_callback(
    user_service: UserService,
    headers: HeaderMap,
    Form(payload): Form<GoogleCallbackPayload>,
) -> JrnlResult<impl IntoResponse> {
    let response = inner_google_callback(user_service, headers, Form(payload)).await;
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
) -> JrnlResult<serde_json::Value> {
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

    Ok(json!({ "token": jwt, "user": user }))
}

// no actual error should ever be triggered in the jrnlresult, they should just be serialized and passed
async fn apple_callback(auth_service: AuthService, user_service: UserService, Form(payload): Form<AppleCallbackPayload>) -> JrnlResult<Redirect> {
    let response = apple_callback_inner(auth_service, user_service, Form(payload)).await;
    serve_response_flash(response)
}

fn serve_response_flash(response: JrnlResult<serde_json::Value>) -> JrnlResult<Redirect> {
    let json_value = response.unwrap_or_else(|err| json!({ "error": err.to_string() }));
    let encoded_string = STANDARD.encode(
        serde_json::to_string(&json_value).context("failed to serialize response")?.as_bytes(),
    );

    let redirect_uri = format!(
        "{}/cb?r={encoded_string}",
        env::var("FRONTEND_URL").context("missing frontend url?")?,
    );

    Ok(Redirect::temporary(&redirect_uri))
}