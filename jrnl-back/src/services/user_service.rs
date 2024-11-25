use crate::{impl_service, schemas::user::User};
use sqlx::{Error, PgPool};
use uuid::Uuid;

pub struct UserService(PgPool);
impl_service!(UserService);

impl UserService {
    pub async fn create_or_get_user(
        &self,
        name: Option<&str>,
        google_subject: &Option<String>,
        apple_subject: &Option<String>,
    ) -> Result<User, Error> {
        sqlx::query_as(
            // language=postgresql
            "
                INSERT INTO users (name, google_subject, apple_subject) VALUES ($1, $2, $3)
                ON CONFLICT ON CONSTRAINT unique_auth
                DO UPDATE SET name = COALESCE($1, users.name)
                RETURNING *
            ",
        )
        .bind(name)
        .bind(google_subject)
        .bind(apple_subject)
        .fetch_one(&self.0)
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
            "
                UPDATE users SET
                timezone = COALESCE($1, timezone),
                theme = COALESCE($2, theme)
                WHERE id = $3 RETURNING *
            ",
        )
        .bind(tz)
        .bind(theme)
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
