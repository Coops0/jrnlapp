mod auth;
mod controllers;
mod error;
mod schemas;
mod web;

use crate::{
    auth::clean_expired_sessions,
    schemas::user::User,
};
use axum::{
    extract::DefaultBodyLimit,
    http::header::{AUTHORIZATION, CONTENT_TYPE},
    Router,
};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};
use std::{env, time::Duration};
use tokio::join;
use tower::ServiceBuilder;
use tower_http::{
    cors::{AllowCredentials, AllowHeaders, AllowMethods, AllowOrigin, CorsLayer},
    timeout::TimeoutLayer,
};
use tracing::info;
use tracing_subscriber::{
    filter::LevelFilter,
    EnvFilter,
};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool
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
    info!("migrations ran successfully / db connection valid");

    let session_clean_task = tokio::task::spawn(clean_expired_sessions(pool.clone()));

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
                        .allow_methods(AllowMethods::mirror_request())
                        .allow_headers(AllowHeaders::list([AUTHORIZATION, CONTENT_TYPE]))
                        .allow_credentials(AllowCredentials::yes()),
                )
                .layer(DefaultBodyLimit::max(1024 * 12))
                .layer(TimeoutLayer::new(Duration::from_secs(10)))
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await?;
    info!("bound to {}", listener.local_addr()?);

    let axum_server = axum::serve(listener, app);

    let _ = join!(
        axum_server,
        session_clean_task
    );

    unreachable!();
}
