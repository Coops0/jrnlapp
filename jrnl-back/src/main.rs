mod auth;
mod controllers;
mod error;
mod schemas;
mod web;

use crate::auth::clean_expired_sessions;
use crate::schemas::user::User;
use axum::extract::DefaultBodyLimit;
use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};
use axum::Router;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::PgPool;
use std::env;
use tower::ServiceBuilder;
use tower_http::cors::{AllowCredentials, AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};
use tracing::info;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::EnvFilter;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
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

    sqlx::migrate!().run(&pool).await?;

    #[allow(clippy::let_underscore_future)]
    let _ = tokio::task::spawn(clean_expired_sessions(pool.clone()));

    let state = AppState { pool };

    let app = Router::new()
        .nest("/user", controllers::user::users_controller())
        .nest("/entries", controllers::entry::entries_controller())
        .nest("/groups", controllers::group::groups_controller())
        // don't run middleware only for auth route
        .layer(axum::middleware::from_extractor_with_state::<User, AppState>(state.clone()))
        .nest("/auth", auth::routes::auth_controller())
        .layer(
            ServiceBuilder::new()
                .layer(
                    CorsLayer::new()
                        .allow_origin(AllowOrigin::exact(env::var("FRONTEND_URL")?.parse()?))
                        // .allow_origin(AllowOrigin::any()) // todo
                        .allow_methods(AllowMethods::mirror_request())
                        .allow_headers(AllowHeaders::list([AUTHORIZATION, CONTENT_TYPE]))
                        .allow_credentials(AllowCredentials::yes()),
                )
                .layer(DefaultBodyLimit::max(8192)),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await?;

    info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
