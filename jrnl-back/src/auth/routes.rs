use crate::{
    error::{AuthenticationError, DatabaseError, JrnlResult, JsonExtractor},
    auth::{
        providers::{fetch_google_profile, google_provider},
        jwt
    },
    schemas::user::User,
    AppState
};
use axum::{
    http::StatusCode,
    extract::State,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Json,
    Router
};
use oauth2::{
    reqwest::async_http_client,
    AuthorizationCode,
    CsrfToken,
    PkceCodeChallenge,
    PkceCodeVerifier,
    Scope,
    TokenResponse
};
use serde::Deserialize;
use serde_json::json;

pub fn auth_controller() -> Router<AppState> {
    Router::new()
        .route("/google", get(google_url))
        .route("/google/callback", post(google_callback))
        .route("/logout", get(logout))
}

async fn google_url(State(AppState { pool }): State<AppState>) -> JrnlResult<impl IntoResponse> {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_token) = google_provider()
        .map_err(AuthenticationError::ProviderGenerationFailed)?
        .authorize_url(CsrfToken::new_random)
        .add_scopes(vec![
            Scope::new("https://www.googleapis.com/auth/userinfo.email".to_string()),
            Scope::new("https://www.googleapis.com/auth/userinfo.profile".to_string()),
            Scope::new("openid".to_string()),
        ])
        .set_pkce_challenge(pkce_challenge)
        .url();

    sqlx::query(
        // language=postgresql
        "
        INSERT INTO temp_auth_sessions (csrf_token, pkce_verifier, expires_at)
        VALUES ($1, $2, NOW() + INTERVAL '30 minutes')
        ",
    )
    .bind(csrf_token.secret())
    .bind(pkce_verifier.secret())
    .execute(&pool)
    .await
    .map_err(DatabaseError)?;

    Ok(Redirect::temporary(auth_url.as_str()))
}

#[derive(Deserialize)]
struct CallbackPayload {
    code: String,
    state: String,
}

async fn google_callback(
    State(AppState { pool }): State<AppState>,
    JsonExtractor(CallbackPayload { code, state }): JsonExtractor<CallbackPayload>,
) -> JrnlResult<impl IntoResponse> {
    let (pkce_verifier,) = sqlx::query_as::<_, (String,)>(
        // language=postgresql
        "
        SELECT pkce_verifier FROM temp_auth_sessions
        WHERE csrf_token = $1 AND expires_at > NOW() LIMIT 1
        ",
    )
    .bind(&state)
    .fetch_one(&pool)
    .await
    .map_err(AuthenticationError::BadCallbackState)?;

    let provider = google_provider()?;
    let google_token = provider
        .exchange_code(AuthorizationCode::new(code))
        .set_pkce_verifier(PkceCodeVerifier::new(pkce_verifier))
        .request_async(async_http_client)
        .await
        .map_err(|e| AuthenticationError::CodeExchangeFailed(e.into()))?;

    let (name, email) = fetch_google_profile(google_token.access_token().secret())
        .await
        .map_err(AuthenticationError::FetchGoogleProfileFailed)?;

    let user = sqlx::query_as::<_, User>(
        // language=postgresql
        "
        INSERT INTO users (name, email) VALUES ($1, $2)
        ON CONFLICT(email) DO UPDATE SET name = $1
        RETURNING *
        ",
    )
    .bind(name)
    .bind(email)
    .fetch_one(&pool)
    .await
    .map_err(DatabaseError)?;

    let jwt = jwt::encode_jwt(user.id)?;

    Ok(Json(json!({ "token": jwt, "user": user })))
}

async fn logout() -> JrnlResult<StatusCode> {
    Ok(StatusCode::OK)
}
