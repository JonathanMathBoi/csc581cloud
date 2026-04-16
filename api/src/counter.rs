use redis::AsyncCommands;
use tracing::info;

use crate::core::{AppResult, AppState, internal_error};

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

pub async fn read_counter(app_state: &AppState) -> AppResult<i64> {
    let mut conn = app_state
        .redis_client
        .get_multiplexed_async_connection()
        .await
        .map_err(internal_error)?;

    let counter: Option<i64> = conn.get("counter").await.map_err(internal_error)?;
    Ok(counter.unwrap_or(0))
}

pub async fn increment_counter_value(app_state: &AppState) -> AppResult<i64> {
    let mut conn = app_state
        .redis_client
        .get_multiplexed_async_connection()
        .await
        .map_err(internal_error)?;

    conn.incr("counter", 1).await.map_err(internal_error)
}

pub async fn reset_counter(app_state: &AppState) -> AppResult<i64> {
    let mut conn = app_state
        .redis_client
        .get_multiplexed_async_connection()
        .await
        .map_err(internal_error)?;

    let (): () = conn.set("counter", 0_i64).await.map_err(internal_error)?;
    Ok(0)
}
