mod controllers;
mod schemas;
mod web;

use crate::web::auth::User;
use axum::extract::DefaultBodyLimit;
use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};
use axum::Router;
use oauth_axum::providers::google::GoogleProvider;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::Pool;
use std::env;
use tokio::time::Duration;
use tower::ServiceBuilder;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};
use tower_sessions::{ExpiredDeletion, Expiry, SessionManagerLayer};
use tower_sessions_sqlx_store::PostgresStore;
use tracing::info;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::EnvFilter;

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::PgPool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();

    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env()?
        .add_directive("jrnl-back=debug".parse()?)
        .add_directive("app=debug".parse()?);

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .compact()
        .init();

    let pool = PgPoolOptions::new()
        .connect_lazy_with(env::var("DATABASE_URL")?.parse::<PgConnectOptions>()?);

    let session_store = PostgresStore::new(Pool::clone(&pool));
    session_store.migrate().await?;

    let _ = tokio::task::spawn(
        session_store
            .clone()
            .continuously_delete_expired(Duration::from_secs(60)),
    );

    let state = AppState { pool };

    let app = Router::new()
        .nest("/user", controllers::user::users_controller())
        .nest("/entries", controllers::entry::entries_controller())
        .nest("/groups", controllers::group::groups_controller())
        // don't run middleware only for auth route
        .layer(axum::middleware::from_extractor_with_state::<User, AppState>(state.clone()))
        .nest("/auth", controllers::auth::auth_controller())
        .layer(ServiceBuilder::new()
            .layer(CorsLayer::new()
                .allow_origin(AllowOrigin::any()) // todo
                .allow_methods(AllowMethods::any())
                .allow_headers(AllowHeaders::list([AUTHORIZATION, CONTENT_TYPE]))
            )
            .layer(SessionManagerLayer::new(session_store).with_secure(false))
            .layer(DefaultBodyLimit::max(1024))
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await?;

    info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
