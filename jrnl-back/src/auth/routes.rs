use crate::auth::providers::google_provider;
use crate::error::{AuthenticationError, DatabaseError, JrnlResult, JsonExtractor};
use crate::schemas::user::User;
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use axum::routing::{get, post};
use axum::{Json, Router};
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;
use oauth2::reqwest::async_http_client;
use oauth2::{AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, Scope};
use serde::Deserialize;
use uuid::Uuid;

pub fn auth_controller() -> Router<AppState> {
    Router::new()
        .route("/google", get(google_url))
        .route("/google/callback", post(google_callback))
        .route("/logout", get(logout))
}

#[derive(Deserialize)]
struct CallbackPayload {
    code: String,
    state: String,
}

async fn google_url(State(AppState { pool }): State<AppState>) -> JrnlResult<impl IntoResponse> {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_token) = google_provider().map_err(AuthenticationError::ProviderGenerationFailed)?
        .authorize_url(CsrfToken::new_random)
        .add_scopes(
            vec![
                Scope::new("https://www.googleapis.com/auth/userinfo.email".to_string()),
                Scope::new("https://www.googleapis.com/auth/userinfo.profile".to_string()),
                Scope::new("https://www.googleapis.com/auth/openid".to_string()),
            ]
        )
        .set_pkce_challenge(pkce_challenge)
        .url();

    let (temp_session_id, ) = sqlx::query_as::<_, (String,)>(
        // language=postgresql
        "
        INSERT INTO temp_auth_sessions (csrf_token, pkce_verifier, expires_at)
        VALUES ($1, $2, NOW() + INTERVAL '5 minutes')
        RETURNING id
        "
    )
        .bind(csrf_token.secret())
        .bind(pkce_verifier.secret())
        .fetch_one(&pool)
        .await
        .map_err(DatabaseError)?;

    let c_jar = CookieJar::new().add(
        Cookie::build(("g-a-id", temp_session_id))
            .http_only(true)
            .secure(false) // todo
            .build()
    );


    Ok((c_jar, Redirect::temporary(auth_url.as_str())))
}

async fn google_callback(
    c_jar: CookieJar,
    State(AppState { pool }): State<AppState>,
    JsonExtractor(CallbackPayload { code, state }): JsonExtractor<CallbackPayload>,
) -> JrnlResult<impl IntoResponse> {
    let temp_session_id = c_jar.get("g-a-id").ok_or(AuthenticationError::NoCookieId)?.value().to_string();
    let c_jar = c_jar.remove("g-a-id");

    let (csrf, pkce_verifier) = sqlx::query_as::<_, (String, String)>(
        // language=postgresql
        "
        DELETE FROM temp_auth_sessions
        WHERE id = $1 AND expires_at > NOW()
        RETURNING csrf_token, pkce_verifier
        "
    )
        .bind(&temp_session_id)
        .fetch_one(&pool)
        .await
        .map_err(DatabaseError)?;

    if csrf != state {
        return Err(AuthenticationError::BadCsrfToken.into());
    }

    let provider = google_provider()?;
    let token_result = provider
        .exchange_code(AuthorizationCode::new(code))
        .set_pkce_verifier(PkceCodeVerifier::new(pkce_verifier))
        .request_async(async_http_client)
        .await
        .map_err(|e| AuthenticationError::CodeExchangeFailed(e.into()))?;

    dbg!(token_result);

    let user = sqlx::query_as::<_, User>(
        // language=postgresql
        "INSERT INTO users (email, name) VALUES ($1, $2) ON CONFLICT DO NOTHING RETURNING *"
    )
        // .bind(user.email)
        // .bind(user.name)
        .bind("test@test.com")
        .bind("test")
        .fetch_one(&pool)
        .await
        .map_err(DatabaseError)?;

    let (session_id, ) = sqlx::query_as::<_, (Uuid,)>(
        // language=postgresql
        "
        INSERT INTO sessions (user_id, expires_at)
        VALUES ($1, NOW() + INTERVAL '30 days')
        RETURNING id
        "
    )
        .bind(user.id)
        .fetch_one(&pool)
        .await
        .map_err(DatabaseError)?;

    let c_jar = CookieJar::new().add(
        Cookie::build(("session", session_id.to_string()))
            .http_only(true)
            .secure(false) // todo
            .build()
    );

    Ok((c_jar, Json(user)))
}

async fn logout(c_jar: CookieJar, State(AppState { pool }): State<AppState>) -> JrnlResult<StatusCode> {
    let session_id = c_jar.get("session").ok_or(AuthenticationError::NoSessionCookie)?.value().to_string();
    let _ = c_jar.remove("session");

    sqlx::query(
        // language=postgresql
        "DELETE FROM sessions WHERE id = $1"
    )
        .bind(&session_id)
        .execute(&pool)
        .await
        .map(|_| StatusCode::OK)
        .map_err(Into::into)
}