use api_lib::health::{health, hello_world, version, MyState};
use axum::{routing::get, Router};
use shuttle_runtime::CustomError;
use sqlx::{Executor, PgPool};

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
        .route("/health", get(health))
        .with_state(state);

    Ok(router.into())
}
