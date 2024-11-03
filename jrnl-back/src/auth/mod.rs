use std::time::Duration;
use sqlx::PgPool;
use tokio::time::interval;
use tracing::error;

mod providers;
pub mod extension;
pub mod routes;

pub async fn clean_expired_sessions(pool: PgPool) {
    let mut ticker = interval(Duration::from_secs(60 * 60));
    loop {
        ticker.tick().await;

        let deletion_result = sqlx::query(
            // language=postgresql
            "
            DELETE FROM sessions WHERE expires_at < NOW();
            DELETE FROM temp_auth_sessions WHERE expires_at < NOW();
            "
        )
            .execute(&pool)
            .await;

        if let Err(why) = deletion_result {
            error!("Failed to clean expired sessions: {}", why);
        }
    }
}