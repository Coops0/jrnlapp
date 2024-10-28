mod controllers;
mod schemas;
mod web;

use axum::Router;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use std::env;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::EnvFilter;
use crate::web::auth::User;

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
        .with_default_directive(LevelFilter::WARN.into())
        .from_env()?
        .add_directive("jrnl-back=debug".parse()?);

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .pretty()
        .init();

    let pool = PgPoolOptions::new().connect_lazy_with(env::var("DATABASE_URL")?.parse::<PgConnectOptions>()?);

    let state = AppState {
        pool: pool.clone(),
        supabase_url: env::var("SUPABASE_URL")?,
        supabase_anon_key: env::var("SUPABASE_ANON_KEY")?,
        supabase_key: env::var("SUPABASE_KEY")?,
    };

    let app = Router::new()
        .nest("/users", controllers::user::users_controller())
        .nest("/entries", controllers::entry::entries_controller())
        .layer(axum::middleware::from_extractor_with_state::<User, AppState>(state.clone()))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
