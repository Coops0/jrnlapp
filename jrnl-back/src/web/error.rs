use axum::async_trait;
use axum::extract::rejection::JsonRejection;
use axum::extract::{FromRequest, Request};
use axum::http::StatusCode;
use oauth_axum::error::OauthError;
use thiserror::Error;
use thiserror_status::ErrorStatus;
use tracing::warn;

pub type JrnlResult<T> = Result<T, JrnlError>;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Error, ErrorStatus)]
pub enum JrnlError {
    #[error("bad json body {0}")]
    #[status(StatusCode::BAD_REQUEST)]
    BadRequestSyntax(JsonRejection),

    #[error("no results found")]
    #[status(StatusCode::NOT_FOUND)]
    NoResultsFound,

    #[error("already group member")]
    #[status(StatusCode::CONFLICT)]
    AlreadyGroupMember,

    #[error("cannot kick self")]
    #[status(StatusCode::BAD_REQUEST)]
    CannotKickSelf,

    #[error("authentication error {0}")]
    #[status(StatusCode::UNAUTHORIZED)]
    AuthenticationError(String),

    #[error("oauth error {0}")]
    #[status(StatusCode::INTERNAL_SERVER_ERROR)]
    OAuthError(String),

    #[deprecated(note = "use `DatabaseError` wrapper instead")]
    #[error("database error {0:?}")]
    #[status(StatusCode::INTERNAL_SERVER_ERROR)]
    DatabaseError(/*#[from]*/ sqlx::Error),

    #[error("other error occurred {0:}")]
    #[status(StatusCode::INTERNAL_SERVER_ERROR)]
    Other(#[from] anyhow::Error),
}

#[allow(clippy::module_name_repetitions)]
pub struct DatabaseError(pub sqlx::Error);

#[allow(deprecated)]
impl From<DatabaseError> for JrnlError {
    fn from(err: DatabaseError) -> Self {
        match err {
            DatabaseError(sqlx::Error::RowNotFound) => return Self::NoResultsFound,
            DatabaseError(sqlx::Error::Database(_)) => {
                warn!("Database error: {:?}", err.0);
            }
            _ => {}
        }

        Self::DatabaseError(err.0)
    }
}

impl From<sqlx::Error> for JrnlError {
    fn from(err: sqlx::Error) -> Self {
        DatabaseError(err).into()
    }
}

impl From<OauthError> for JrnlError {
    fn from(err: OauthError) -> Self {
        let e = match err {
            OauthError::TokenRequestFailed => "token request failed",
            OauthError::AuthUrlCreationFailed => "auth url creation failed",
        };

        Self::OAuthError(String::from(e))
    }
}

pub struct JsonExtractor<T>(pub T);

#[async_trait]
impl<S: Send + Sync, T> FromRequest<S> for JsonExtractor<T>
where
    axum::Json<T>: FromRequest<S, Rejection=JsonRejection>,
{
    type Rejection = JrnlError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let (parts, body) = req.into_parts();

        let req = Request::from_parts(parts, body);
        match axum::Json::<T>::from_request(req, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(why) => {
                warn!("got bad json syntax in req {why:?}");

                Err(JrnlError::BadRequestSyntax(why))
            }
        }
    }
}