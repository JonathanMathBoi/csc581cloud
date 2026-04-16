use std::{env, net::SocketAddr};

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
struct AppState {
    redis_client: redis::Client,
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

#[tokio::main]
async fn main() {
    init_tracing();

    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_owned());
    let bind_addr = env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".to_owned());

    let redis_client = redis::Client::open(redis_url.clone()).unwrap_or_else(|error| {
        panic!("failed to create Redis client from REDIS_URL='{redis_url}': {error}");
    });

    let app_state = AppState { redis_client };
    let app = Router::new()
        .nest("/api", api_router())
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .unwrap_or_else(|error| panic!("failed to bind API server to {bind_addr}: {error}"));

    let resolved: SocketAddr = listener
        .local_addr()
        .unwrap_or_else(|error| panic!("failed to read local socket address: {error}"));
    info!(%resolved, "API listening");

    axum::serve(listener, app)
        .await
        .unwrap_or_else(|error| panic!("API server crashed: {error}"));
}

fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "service_status_api=info,tower_http=info".into()),
        )
        .compact()
        .init();
}

fn api_router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health))
        .route("/counter", get(get_counter))
        .route("/counter/increment", post(increment_counter))
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

async fn get_counter(State(app_state): State<AppState>) -> AppResult<Json<CounterResponse>> {
    let mut conn = app_state
        .redis_client
        .get_multiplexed_async_connection()
        .await
        .map_err(internal_error)?;

    let counter: Option<i64> = conn.get("counter").await.map_err(internal_error)?;

    Ok(Json(CounterResponse {
        counter: counter.unwrap_or(0),
    }))
}

async fn increment_counter(State(app_state): State<AppState>) -> AppResult<Json<CounterResponse>> {
    let mut conn = app_state
        .redis_client
        .get_multiplexed_async_connection()
        .await
        .map_err(internal_error)?;

    let new_counter: i64 = conn.incr("counter", 1).await.map_err(internal_error)?;

    Ok(Json(CounterResponse {
        counter: new_counter,
    }))
}

fn internal_error(error: impl std::fmt::Display) -> (StatusCode, String) {
    error!(%error, "request failed");
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "internal server error".to_owned(),
    )
}
