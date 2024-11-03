use crate::web::error::{DatabaseError, JrnlError, JrnlResult};
use crate::web::providers::google_provider;
use crate::AppState;
use axum::extract::{Query, State};
use axum::response::Redirect;
use axum::routing::get;
use axum::Router;
use oauth_axum::{CustomProvider, OAuthClient};
use serde::Deserialize;
use tower_sessions::Session;

pub fn auth_controller() -> Router<AppState> {
    Router::new()
        .route("/google", get(google_url))
        .route("/google/callback", get(google_callback))
}

#[derive(Deserialize)]
struct CallbackQuery {
    code: String,
    state: String,
}

async fn google_url(session: Session) -> JrnlResult<Redirect> {
    let generation_response: Box<CustomProvider> = google_provider()?
        .generate_url(
            vec![String::from("openid"), String::from("userinfo.email"), String::from("userinfo.profile")],
            |s| async move {
                let _ = session.insert(&s.state, s.verifier).await;
            },
        )
        .await
        .map_err(Into::<JrnlError>::into)?;

    let url = generation_response.state
        .and_then(|state| state.url_generated)
        .ok_or_else(|| JrnlError::AuthenticationError(String::from("failed to generate url")))?;

    Ok(Redirect::temporary(&url))
}

async fn google_callback(
    session: Session,
    State(AppState { pool }): State<AppState>,
    Query(CallbackQuery { code, state }): Query<CallbackQuery>,
) -> JrnlResult<Redirect> {
    let verifier = session
        .remove::<String>(&state)
        .await
        .ok()
        .flatten()
        .ok_or_else(|| JrnlError::AuthenticationError(String::from("error fetching verifier from session")))?;

    let provider = google_provider()?;
    let token = provider
        .generate_token(code, verifier)
        .await?;

    let user = client
        .get_user(&token)
        .await
        .map_err(Into::<JrnlError>::into)?;
    
    let user = sqlx::query_as::<_, User>(
        // language=postgresql
        "INSERT INTO users (email, name) VALUES ($1, $2) ON CONFLICT DO NOTHING RETURNING *"
    )
        .bind(user.email)
        .bind(user.name)
        .fetch_one(&pool)
        .await
        .map_err(DatabaseError)?;
    
    session.insert("user", user.id).await?;

    Ok(Redirect::temporary("/"))
}