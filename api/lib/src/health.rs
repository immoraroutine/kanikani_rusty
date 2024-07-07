use axum::extract::State;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct MyState {
    pub pool: Arc<PgPool>,
}

pub async fn hello_world() -> &'static str {
    "Hello, world!"
}

pub async fn version(
    State(state): State<MyState>,
) -> String {
    tracing::info!("Getting version!");
    let result = sqlx::query_scalar("SELECT version()")
        .fetch_one(&*state.pool)
        .await;

    match result {
        Ok(version) => version,
        Err(err) => format!("Error: {}", err),
    }
}