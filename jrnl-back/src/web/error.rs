use axum::extract::rejection::JsonRejection;
use axum::extract::{FromRequest, Request};
use axum::http::StatusCode;
use axum::{async_trait, RequestPartsExt};
use thiserror::Error;
use thiserror_status::ErrorStatus;
use tracing::error;

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

    #[error("database error {0:?}")]
    #[status(StatusCode::INTERNAL_SERVER_ERROR)]
    DatabaseError(#[from] sqlx::Error),

    #[error("other error occurred {0:}")]
    #[status(StatusCode::INTERNAL_SERVER_ERROR)]
    Other(#[from] anyhow::Error),
}

pub type JrnlResult<T> = Result<T, JrnlError>;

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