use axum::{routing::get, Router, extract::State};
use shuttle_runtime::CustomError;
use sqlx::{Executor, PgPool};
use std::sync::Arc;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn version(
    State(state): State<MyState>,
) -> String {
    let result = sqlx::query_scalar("SELECT version()")
        .fetch_one(&*state.pool)
        .await;

    match result {
        Ok(version) => version,
        Err(err) => format!("Error: {}", err),
    }
}

#[derive(Clone)]
struct MyState {
    pool: Arc<PgPool>,
}

#[shuttle_runtime::main]
async fn axum(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    pool.execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(CustomError::new)?;
    let pool = std::sync::Arc::new(pool);

    let state = MyState { pool };
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/version", get(version))
        .with_state(state);

    Ok(router.into())
}
