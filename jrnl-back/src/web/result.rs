use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub struct InternalError(anyhow::Error);
#[allow(clippy::module_name_repetitions)]
pub type InternalResult<T> = Result<T, InternalError>;

impl IntoResponse for InternalError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", self.0)).into_response()
    }
}

impl<E: Into<anyhow::Error>> From<E> for InternalError {
    fn from(err: E) -> Self {
        Self(err.into())
    }
}