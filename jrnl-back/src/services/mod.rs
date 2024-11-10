#![allow(clippy::crate_in_macro_def)]

pub mod auth_service;
pub mod entry_service;
pub mod group_service;
pub mod user_service;

#[macro_export]
macro_rules! impl_service {
    ($service:ident) => {
        impl $service {
            pub const fn new(pool: sqlx::PgPool) -> Self {
                Self(pool)
            }
        }

        #[axum::async_trait]
        impl axum::extract::FromRequestParts<crate::AppState> for $service {
            type Rejection = ();

            async fn from_request_parts(
                _req: &mut axum::http::request::Parts,
                state: &crate::AppState,
            ) -> Result<Self, Self::Rejection> {
                Ok(Self::new(sqlx::PgPool::clone(&state.pool)))
            }
        }
    };
}
