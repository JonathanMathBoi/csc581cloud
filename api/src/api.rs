use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use serde::Serialize;

use crate::{
    core::{AppResult, AppState},
    counter::{increment_counter_value, read_counter},
};

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

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

async fn get_counter(State(app_state): State<AppState>) -> AppResult<Json<CounterResponse>> {
    let counter = read_counter(&app_state).await?;

    Ok(Json(CounterResponse { counter }))
}

async fn increment_counter(State(app_state): State<AppState>) -> AppResult<Json<CounterResponse>> {
    let counter = increment_counter_value(&app_state).await?;

    Ok(Json(CounterResponse { counter }))
}
