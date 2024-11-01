mod controllers;
mod schemas;
mod web;

use crate::web::auth::User;
use axum::Router;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use std::env;
use tower::ServiceBuilder;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};
use tracing::info;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::EnvFilter;

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::PgPool,
    pub supabase_url: String,
    pub supabase_anon_key: String,
    pub supabase_key: String,
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

    let state = AppState {
        pool,
        supabase_url: env::var("SUPABASE_URL")?,
        supabase_anon_key: env::var("SUPABASE_ANON_KEY")?,
        supabase_key: env::var("SUPABASE_KEY")?,
    };

    let app = Router::new()
        .nest("/users", controllers::user::users_controller())
        .nest("/entries", controllers::entry::entries_controller())
        .nest("/groups", controllers::group::groups_controller())
        .layer(ServiceBuilder::new()
            .layer(CorsLayer::new()
                .allow_origin(AllowOrigin::any()) // todo
                .allow_methods(AllowMethods::any())
                .allow_headers(AllowHeaders::any())
            )
            .layer(axum::middleware::from_extractor_with_state::<User, AppState>(state.clone()))
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await?;

    info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
