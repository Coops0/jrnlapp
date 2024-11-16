use crate::impl_service;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::postgres::PgQueryResult;
use sqlx::{Error, FromRow, PgPool};
use uuid::Uuid;

pub struct AuthService(PgPool);
impl_service!(AuthService);

#[derive(Serialize, FromRow)]
pub struct TempAuthSession {
    pub csrf_token: Uuid,
    pub nonce: Uuid,
    pub expiry: DateTime<Utc>,
}

#[derive(Serialize, FromRow)]
pub struct MobileNonceOneshot {
    pub nonce: Uuid,
    pub payload: String,
    pub expiry: DateTime<Utc>,
}

impl AuthService {
    pub async fn create_temp_auth_session(&self) -> Result<TempAuthSession, Error> {
        sqlx::query_as(
            // language=postgresql
            "
                INSERT INTO temp_auth_sessions (expiry)
                VALUES (NOW() + INTERVAL '30 minutes')
                RETURNING *
            ",
        )
            .fetch_one(&self.0)
            .await
    }

    pub async fn create_mobile_nonce_oneshot(&self, nonce: &Uuid, payload: &str) -> Result<PgQueryResult, Error> {
        sqlx::query(
            // language=postgresql
            "
                INSERT INTO mobile_nonce_oneshots (nonce, payload, expiry)
                VALUES ($1, $2, NOW() + INTERVAL '5 minutes')
            ",
        )
            .bind(nonce)
            .bind(payload)
            .execute(&self.0)
            .await
    }

    pub async fn delete_and_fetch_nonce(&self, csrf: &Uuid) -> Result<Uuid, Error> {
        sqlx::query_as::<_, (Uuid,)>(
            // language=postgresql
            "
                DELETE FROM temp_auth_sessions 
                WHERE csrf_token = $1 AND expiry > NOW()
                RETURNING nonce
            ",
        )
            .bind(csrf)
            .fetch_one(&self.0)
            .await
            .map(|(nonce, )| nonce)
    }

    pub async fn delete_and_fetch_mobile_nonce_oneshot(&self, nonce: &Uuid) -> Result<String, Error> {
        sqlx::query_as::<_, (String,)>(
            // language=postgresql
            "
                DELETE FROM mobile_nonce_oneshots
                WHERE nonce = $1 AND expiry > NOW()
                RETURNING payload
            ",
        )
            .bind(nonce)
            .fetch_one(&self.0)
            .await
            .map(|(payload, )| payload)
    }
}
