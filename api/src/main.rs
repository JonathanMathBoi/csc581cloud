mod api;
mod core;
mod counter;

use std::{env, net::SocketAddr};

use axum::Router;
use tracing::info;

use crate::core::AppState;

#[tokio::main]
async fn main() {
    init_tracing();

    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_owned());
    let bind_addr = env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".to_owned());

    let redis_client = redis::Client::open(redis_url.clone()).unwrap_or_else(|error| {
        panic!("failed to create Redis client from REDIS_URL='{redis_url}': {error}");
    });

    let app_state = AppState { redis_client };
    counter::initialize_counter(&app_state)
        .await
        .unwrap_or_else(|error| panic!("failed to initialize counter: {error:?}"));

    let app = Router::new()
        .nest("/api", api::router())
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
