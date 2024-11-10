use crate::auth::providers::{verify_apple_id_token, AppleCallbackPayload};
use crate::error::AppleAuthenticationError;
use crate::services::auth_service::AuthService;
use crate::services::user_service::UserService;
use crate::{
    auth::{
        jwt,
        providers::{fetch_google_profile, google_provider},
    },
    error::{GoogleAuthenticationError, JrnlResult, JsonExtractor},
    AppState,
};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Form, Json, Router,
};
use oauth2::{
    reqwest::async_http_client, AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier,
    Scope, TokenResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

pub fn auth_controller() -> Router<AppState> {
    Router::new()
        .route("/google", get(google_url))
        .route("/google/callback", post(google_callback))
        .route("/apple", post(init_apple_session))
        .route("/apple/callback", post(apple_callback))
        .route("/logout", get(logout))
}

async fn google_url(auth_service: AuthService) -> JrnlResult<impl IntoResponse> {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_token) = google_provider()
        .map_err(GoogleAuthenticationError::ProviderGenerationFailed)?
        .authorize_url(CsrfToken::new_random)
        .add_scopes(vec![
            Scope::new("https://www.googleapis.com/auth/userinfo.email".to_string()),
            Scope::new("https://www.googleapis.com/auth/userinfo.profile".to_string()),
            Scope::new("openid".to_string()),
        ])
        .set_pkce_challenge(pkce_challenge)
        .url();

    auth_service
        .create_temp_auth_session(csrf_token.secret(), pkce_verifier.secret())
        .await?;

    Ok(Redirect::temporary(auth_url.as_str()))
}

#[derive(Deserialize)]
struct CallbackPayload {
    code: String,
    state: String,
}

async fn google_callback(
    auth_service: AuthService,
    user_service: UserService,
    JsonExtractor(CallbackPayload { code, state }): JsonExtractor<CallbackPayload>,
) -> JrnlResult<impl IntoResponse> {
    let pkce_verifier = auth_service
        .get_temp_auth_session(&state)
        .await
        .map_err(GoogleAuthenticationError::BadCallbackState)?;

    let provider = google_provider()?;
    let google_token = provider
        .exchange_code(AuthorizationCode::new(code))
        .set_pkce_verifier(PkceCodeVerifier::new(pkce_verifier))
        .request_async(async_http_client)
        .await
        .map_err(|e| GoogleAuthenticationError::CodeExchangeFailed(e.into()))?;

    let (name, email) = fetch_google_profile(google_token.access_token().secret())
        .await
        .map_err(GoogleAuthenticationError::FetchGoogleProfileFailed)?;

    let user = user_service.create_user_from_google(&name, &email).await?;

    let jwt = jwt::encode_user_jwt(user.id)?;
    Ok(Json(json!({ "token": jwt, "user": user })))
}

#[derive(Serialize)]
struct AppleSession {
    code: String,
    state: String,
}

async fn init_apple_session(auth_service: AuthService) -> JrnlResult<Json<AppleSession>> {
    let verifier = CsrfToken::new_random();
    let nonce = Uuid::new_v4().to_string();

    auth_service
        .create_temp_auth_session(verifier.secret(), &nonce)
        .await?;

    Ok(Json(AppleSession {
        code: nonce,
        state: verifier.secret().to_owned(),
    }))
}

async fn apple_callback(
    auth_service: AuthService,
    user_service: UserService,
    Form(AppleCallbackPayload {
        id_token,
        user,
        state,
        ..
    }): Form<AppleCallbackPayload>,
) -> JrnlResult<impl IntoResponse> {
    let nonce = auth_service
        .get_temp_auth_session(&state)
        .await
        .map_err(|_| AppleAuthenticationError::BadCallbackState)?;

    let claims = verify_apple_id_token(&id_token, &nonce)
        .await
        .map_err(AppleAuthenticationError::VerificationError)?;

    let name = user.map(|u| format!("{} {}", u.name.first_name, u.name.last_name));

    let user = match user_service
        .get_user_by_email_or_apple_id(&claims.email, &claims.sub)
        .await?
    {
        Some(user) => {
            if user.apple_subject.is_none() {
                user_service
                    .migrate_google_account_to_apple(&user, &claims.sub, name.as_deref())
                    .await
                    .map_err(AppleAuthenticationError::FailedGoogleMigration)?;
            }

            user
        }
        None => {
            user_service
                .create_user_from_apple(
                    &name.ok_or(AppleAuthenticationError::NoNameOnSignup)?,
                    &claims.email,
                    &claims.sub,
                )
                .await?
        }
    };

    // todo response needs to go back to frontend since client is redirected here i think
    let jwt = jwt::encode_user_jwt(user.id)?;
    Ok(Json(json!({ "token": jwt, "user": user })))
}

async fn logout() -> JrnlResult<StatusCode> {
    Ok(StatusCode::OK)
}
