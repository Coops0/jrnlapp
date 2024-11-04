use sqlx::PgPool;
use std::time::Duration;
use tokio::time::interval;
use tracing::error;

mod providers;
pub mod extension;
pub mod routes;

pub async fn clean_expired_sessions(pool: PgPool) {
    let mut ticker = interval(Duration::from_secs(60 * 60));
    loop {
        ticker.tick().await;

        let delete_sessions_future = sqlx::query(
            // language=postgresql
            "DELETE FROM sessions WHERE expires_at < NOW()"
        ).execute(&pool);

        let delete_temp_sessions_future = sqlx::query(
            // language=postgresql
            "DELETE FROM temp_auth_sessions WHERE expires_at < NOW()"
        ).execute(&pool);
        
        let (dl_sessions_result, dl_temp_sessions_result) = tokio::join!(delete_sessions_future, delete_temp_sessions_future);

        if let Err(why) = dl_sessions_result {
            error!("Failed to clean expired sessions: {}", why);
        }
        
        if let Err(why) = dl_temp_sessions_result {
            error!("Failed to clean expired temp sessions: {}", why);
        }
    }
}