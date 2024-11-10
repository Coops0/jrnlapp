mod auth;
mod controllers;
mod error;
mod schemas;
mod services;
mod web;

use crate::{auth::clean_expired_sessions, schemas::user::User};
use aes_gcm::{Aes256Gcm, Key};
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
use tracing::{info, warn};
use tracing_subscriber::{filter::LevelFilter, EnvFilter};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub master_key: Key<Aes256Gcm>,
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

    if let Err(why) = sqlx::migrate!().run(&pool).await {
        warn!("migrations failed: {:?}", why);
    } else {
        info!("migrations ran successfully / db connection valid");
    }

    let session_clean_task = tokio::task::spawn(clean_expired_sessions(pool.clone()));

    let master_key_env = env::var("MASTER_ENCRYPTION_KEY")?;
    let master_key = Key::<Aes256Gcm>::from_slice(master_key_env.as_bytes());

    let state = AppState {
        pool,
        master_key: *master_key,
    };

    let app = Router::new()
        .nest("/user", controllers::user_controller::users_controller())
        .nest(
            "/entries",
            controllers::entry_controller::entries_controller(),
        )
        .nest(
            "/groups",
            controllers::group_controller::groups_controller(),
        )
        // don't run middleware only for auth route
        .layer(axum::middleware::from_extractor_with_state::<User, AppState>(state.clone()))
        .nest("/auth", controllers::auth_controller::auth_controller())
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
                .layer(TimeoutLayer::new(Duration::from_secs(10))),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await?;
    info!("bound to {}", listener.local_addr()?);

    let axum_server = axum::serve(listener, app);

    let _ = join!(axum_server, session_clean_task);

    unreachable!();
}
