use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use redis::AsyncCommands;
use serde::Serialize;
use tracing::{error, info};

#[derive(Clone)]
pub struct AppState {
    pub redis_client: redis::Client,
}

type AppResult<T> = Result<T, (StatusCode, String)>;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

#[derive(Serialize)]
struct CounterResponse {
    counter: i64,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health))
        .route("/counter", get(get_counter))
        .route("/counter/increment", post(increment_counter))
}

pub async fn initialize_counter(app_state: &AppState) -> AppResult<()> {
    let mut conn = app_state
        .redis_client
        .get_multiplexed_async_connection()
        .await
        .map_err(internal_error)?;

    let initialized: bool = conn
        .set_nx("counter", 0_i64)
        .await
        .map_err(internal_error)?;
    if initialized {
        info!("initialized counter to 0");
    }

    Ok(())
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

async fn get_counter(State(app_state): State<AppState>) -> AppResult<Json<CounterResponse>> {
    let counter = read_counter(&app_state).await?;

    Ok(Json(CounterResponse { counter }))
}

async fn read_counter(app_state: &AppState) -> AppResult<i64> {
    let mut conn = app_state
        .redis_client
        .get_multiplexed_async_connection()
        .await
        .map_err(internal_error)?;

    let counter: Option<i64> = conn.get("counter").await.map_err(internal_error)?;
    Ok(counter.unwrap_or(0))
}

async fn increment_counter(State(app_state): State<AppState>) -> AppResult<Json<CounterResponse>> {
    let counter = increment_counter_value(&app_state).await?;

    Ok(Json(CounterResponse { counter }))
}

async fn increment_counter_value(app_state: &AppState) -> AppResult<i64> {
    let mut conn = app_state
        .redis_client
        .get_multiplexed_async_connection()
        .await
        .map_err(internal_error)?;

    conn.incr("counter", 1).await.map_err(internal_error)
}

fn internal_error(error: impl std::fmt::Display) -> (StatusCode, String) {
    error!(%error, "request failed");
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "internal server error".to_owned(),
    )
}
