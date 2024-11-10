use crate::impl_service;
use sqlx::postgres::PgQueryResult;
use sqlx::{Error, PgPool};

pub struct AuthService(PgPool);
impl_service!(AuthService);

impl AuthService {
    pub async fn create_temp_auth_session(
        &self,
        csrf_token: &str,
        nonce: &str,
    ) -> Result<PgQueryResult, Error> {
        sqlx::query(
            // language=postgresql
            "
            INSERT INTO temp_auth_sessions (csrf_token, nonce, expires_at)
            VALUES ($1, $2, NOW() + INTERVAL '30 minutes')
            ",
        )
        .bind(csrf_token)
        .bind(nonce)
        .execute(&self.0)
        .await
    }

    pub async fn get_temp_auth_session(&self, csrf: &str) -> Result<String, Error> {
        sqlx::query_as::<_, (String,)>(
            // language=postgresql
            "
        SELECT nonce FROM temp_auth_sessions
        WHERE csrf_token = $1 AND expires_at > NOW() LIMIT 1
        ",
        )
        .bind(csrf)
        .fetch_one(&self.0)
        .await
        .map(|(nonce,)| nonce)
    }
}
