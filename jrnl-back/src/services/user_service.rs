use crate::impl_service;
use crate::schemas::user::User;
use sqlx::postgres::PgQueryResult;
use sqlx::{Error, PgPool};
use uuid::Uuid;

pub struct UserService(PgPool);
impl_service!(UserService);

impl UserService {
    pub async fn create_user_from_google(&self, name: &str, email: &str) -> Result<User, Error> {
        sqlx::query_as(
            // language=postgresql
            "
        INSERT INTO users (name, email) VALUES ($1, $2)
        ON CONFLICT(email) DO UPDATE SET name = $1
        RETURNING *
        ",
        )
        .bind(name)
        .bind(email)
        .fetch_one(&self.0)
        .await
    }

    pub async fn create_user_from_apple(
        &self,
        name: &str,
        email: &str,
        apple_id: &str,
    ) -> Result<User, Error> {
        sqlx::query_as(
            // language=postgresql
            "
        INSERT INTO users (name, email, apple_subject) VALUES ($1, $2, $3)
        ON CONFLICT(email) DO UPDATE SET apple_subject = $3
        RETURNING *
        ",
        )
        .bind(name)
        .bind(email)
        .bind(apple_id)
        .fetch_one(&self.0)
        .await
    }

    pub async fn get_user_by_email_or_apple_id(
        &self,
        email: &str,
        apple_id: &str,
    ) -> Result<Option<User>, Error> {
        sqlx::query_as(
            // language=postgresql
            "SELECT * FROM users WHERE email = $1 OR apple_subject = $2 LIMIT 1",
        )
        .bind(email)
        .bind(apple_id)
        .fetch_optional(&self.0)
        .await
    }

    pub async fn migrate_google_account_to_apple(
        &self,
        user: &User,
        apple_subject: &str,
        name: Option<&str>,
    ) -> Result<PgQueryResult, Error> {
        sqlx::query(
            // language=postgresql
            "
                UPDATE users
                SET apple_subject = $1, name = COALESCE($2, name)
                WHERE id = $3
                ",
        )
        .bind(apple_subject)
        .bind(name) // if first sign in thru apple, update name
        .bind(user.id)
        .execute(&self.0)
        .await
    }

    pub async fn update_user(
        &self,
        user: &User,
        theme: &Option<String>,
        tz: &Option<String>,
    ) -> Result<User, Error> {
        sqlx::query_as(
            // language=postgresql
            "UPDATE users SET
            timezone = COALESCE($1, timezone),
            theme = COALESCE($2, theme)
            WHERE id = $3 RETURNING *
        ",
        )
        .bind(theme)
        .bind(tz)
        .bind(user.id)
        .fetch_one(&self.0)
        .await
    }

    pub async fn get_user_by_id(&self, id: &Uuid) -> Result<User, Error> {
        sqlx::query_as(
            // language=postgresql
            "SELECT * FROM users WHERE id = $1 LIMIT 1",
        )
        .bind(id)
        .fetch_one(&self.0)
        .await
    }
}
