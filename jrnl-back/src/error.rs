use axum::async_trait;
use axum::extract::rejection::JsonRejection;
use axum::extract::{FromRequest, Request};
use axum::http::StatusCode;
use thiserror::Error;
use thiserror_status::ErrorStatus;

pub type JrnlResult<T> = Result<T, JrnlError>;

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

    #[error("cannot create more than 10 groups")]
    #[status(StatusCode::FORBIDDEN)]
    CannotCreateMoreGroups,

    #[error("cannot join more than 20 groups")]
    #[status(StatusCode::FORBIDDEN)]
    CannotJoinMoreGroups,

    #[error("google oauth authentication error {0}")]
    #[status(StatusCode::UNAUTHORIZED)]
    GoogleAuthenticationError(#[from] GoogleAuthenticationError),

    #[error("apple oauth authentication error {0}")]
    #[status(StatusCode::UNAUTHORIZED)]
    AppleAuthenticationError(#[from] AppleAuthenticationError),

    #[deprecated(note = "use `DatabaseError` wrapper instead")]
    #[error("database error {0}")]
    #[status(StatusCode::INTERNAL_SERVER_ERROR)]
    DatabaseError(sqlx::Error),

    #[error("other error occurred {0}")]
    #[status(StatusCode::INTERNAL_SERVER_ERROR)]
    Other(#[from] anyhow::Error),

    #[error("bad authentication header")]
    #[status(StatusCode::UNAUTHORIZED)]
    BadAuthenticationHeader,

    #[error("invalid auth token")]
    #[status(StatusCode::UNAUTHORIZED)]
    InvalidToken,

    #[error("expired auth token")]
    #[status(StatusCode::UNAUTHORIZED)]
    ExpiredToken,

    #[error("failed to find profile")]
    #[status(StatusCode::NOT_FOUND)]
    ProfileNotFound,

    #[error("failed to encrypt journal entry {0}")]
    #[status(StatusCode::INTERNAL_SERVER_ERROR)]
    EntryEncryptionFailed(anyhow::Error),

    #[error("failed to decrypt journal entry {0}")]
    #[status(StatusCode::INTERNAL_SERVER_ERROR)]
    EntryDecryptionFailed(anyhow::Error),
}

#[derive(Debug, Error)]
pub enum GoogleAuthenticationError {
    #[error("bad callback state {0:?}")]
    BadCallbackState(Option<sqlx::Error>),
    #[error("code exchanged failed {0}")]
    CodeExchangeFailed(anyhow::Error),
}

#[derive(Debug, Error)]
pub enum AppleAuthenticationError {
    #[error("invalid callback data")]
    BadCallbackState,
    #[error("failed to verify apple id token {0}")]
    VerificationError(anyhow::Error),
}

pub struct DatabaseError(pub sqlx::Error);

#[allow(deprecated)]
impl From<DatabaseError> for JrnlError {
    fn from(err: DatabaseError) -> Self {
        if matches!(err, DatabaseError(sqlx::Error::RowNotFound)) {
            return Self::NoResultsFound;
        }

        Self::DatabaseError(err.0)
    }
}

impl From<sqlx::Error> for JrnlError {
    fn from(err: sqlx::Error) -> Self {
        DatabaseError(err).into()
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
            Err(why) => Err(JrnlError::BadRequestSyntax(why)),
        }
    }
}
