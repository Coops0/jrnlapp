use axum::Router;
use crate::AppState;

pub fn users_controller() -> Router<AppState> {
    Router::new()
}