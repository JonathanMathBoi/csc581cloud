use axum::http::StatusCode;
use tracing::error;

#[derive(Clone)]
pub struct AppState {
    pub redis_client: redis::Client,
}

pub type AppResult<T> = Result<T, (StatusCode, String)>;

pub fn internal_error(error: impl std::fmt::Display) -> (StatusCode, String) {
    error!(%error, "request failed");
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "internal server error".to_owned(),
    )
}
