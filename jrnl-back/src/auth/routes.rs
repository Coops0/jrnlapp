use crate::auth::providers::{verify_apple_id_token, AppleCallbackPayload, StrippedVerificationClaims};
use crate::{
    auth::{
        jwt,
        providers::{fetch_google_profile, google_provider},
    },
    error::{AuthenticationError, DatabaseError, JrnlResult, JsonExtractor},
    schemas::user::User,
    AppState,
};
use anyhow::{anyhow, Context};
use axum::{extract::State, http::StatusCode, response::{IntoResponse, Redirect}, routing::{get, post}, Form, Json, Router};
use oauth2::{
    reqwest::async_http_client,
    AuthorizationCode,
    CsrfToken,
    PkceCodeChallenge,
    PkceCodeVerifier,
    Scope,
    TokenResponse,
};
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

pub fn auth_controller() -> Router<AppState> {
    Router::new()
        .route("/google", get(google_url))
        .route("/google/callback", post(google_callback))
        .route("/apple", post(init_apple_session))
        .route("/apple/callback", post(apple_callback))
        .route("/logout", get(logout))
}

async fn google_url(State(AppState { pool, .. }): State<AppState>) -> JrnlResult<impl IntoResponse> {
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
        INSERT INTO temp_auth_sessions (csrf_token, nonce, expires_at)
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
    State(AppState { pool, .. }): State<AppState>,
    JsonExtractor(CallbackPayload { code, state }): JsonExtractor<CallbackPayload>,
) -> JrnlResult<impl IntoResponse> {
    let (pkce_verifier, ) = sqlx::query_as::<_, (String,)>(
        // language=postgresql
        "
        SELECT nonce FROM temp_auth_sessions
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

#[derive(Deserialize)]
struct AppleSession {
    code: String,
    state: String,
}

async fn init_apple_session(State(AppState { pool, .. }): State<AppState>) -> JrnlResult<impl IntoResponse> {
    let verifier = CsrfToken::new_random();
    let nonce = Uuid::new_v4().to_string();

    sqlx::query(
        // language=postgresql
        "
        INSERT INTO temp_auth_sessions (csrf_token, nonce, expires_at)
        VALUES ($1, $2, NOW() + INTERVAL '30 minutes')
        ",
    )
        .bind(verifier.secret())
        .bind(&nonce)
        .execute(&pool)
        .await
        .map_err(DatabaseError)?;

    Ok(AppleSession { code: nonce, state: verifier.secret().to_owned() })
}

async fn apple_callback(
    State(AppState { pool, .. }): State<AppState>,
    Form(AppleCallbackPayload { id_token, user, state, .. }): Form<AppleCallbackPayload>,
) -> JrnlResult<impl IntoResponse> {
    let (nonce, ) = sqlx::query_as::<_, (String,)>(
        // language=postgresql
        "
        SELECT nonce FROM temp_auth_sessions
        WHERE csrf_token = $1 AND expires_at > NOW() LIMIT 1
        ",
    )
        .bind(&state)
        .fetch_one(&pool)
        .await
        .map_err(AuthenticationError::BadCallbackState)?;

    let claims = verify_apple_id_token(&id_token, &nonce).await?;
    let name = user.map(|u| format!("{} {}", u.name.first_name, u.name.last_name));

    let existing_user = sqlx::query_as::<_, User>(
        // language=postgresql
        "
       SELECT * FROM users WHERE email = $1 OR apple_subject = $2 LIMIT 1
        ",
    )
        .bind(&claims.email)
        .bind(&claims.sub)
        .fetch_optional(&pool)
        .await
        .map_err(DatabaseError)?;


    let user = match existing_user {
        Some(user) => {
            if user.apple_subject.is_none() {
                migrate_google_account(&user, &claims, &pool, name).await?;
            }

            user
        }
        None => {
            let name = name.context("missing name for initial signup")?;

            sqlx::query_as::<_, User>(
                // language=postgresql
                "INSERT INTO users (name, email, apple_subject) VALUES ($1, $2, $3) RETURNING *",
            )
                .bind(&name)
                .bind(&claims.email)
                .fetch_one(&pool)
                .await
                .map_err(DatabaseError)?
        }
    };

    // todo response needs to go back to frontend since client is redirected here i think
    let jwt = jwt::encode_jwt(user.id)?;
    Ok(Json(json!({ "token": jwt, "user": user })))
}

async fn migrate_google_account(
    user: &User,
    claims: &StrippedVerificationClaims,
    pool: &PgPool,
    name: Option<String>,
) -> JrnlResult<()> {
    if user.email != claims.email {
        return Err(anyhow!("email mismatch").into());
    }

    sqlx::query(
        // language=postgresql
        "
                UPDATE users
                SET apple_subject = $1, name = COALESCE($2, name)
                WHERE id = $3
                ",
    )
        .bind(&claims.sub)
        .bind(&name) // if first sign in thru apple, update name
        .bind(&user.id)
        .execute(pool)
        .await
        .map(|_| ())
        .map_err(|e| DatabaseError(e).into())
}

async fn logout() -> JrnlResult<StatusCode> {
    Ok(StatusCode::OK)
}
