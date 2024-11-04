use sqlx::PgPool;
use std::time::Duration;
use tokio::time::interval;
use tracing::error;

mod providers;
pub mod extension;
pub mod routes;
mod jwt;

pub async fn clean_expired_sessions(pool: PgPool) {
    let mut ticker = interval(Duration::from_secs(60 * 60));
    loop {
        ticker.tick().await;

        let delete_temp_sessions_future = sqlx::query(
            // language=postgresql
            "DELETE FROM temp_auth_sessions WHERE expires_at < NOW()"
        ).execute(&pool);
        
        if let Err(why) = delete_temp_sessions_future.await {
            error!("Failed to clean expired temp sessions: {}", why);
        }
    }
}