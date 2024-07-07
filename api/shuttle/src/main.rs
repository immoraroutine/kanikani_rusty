use axum::{routing::get, Router};
use sqlx::{Executor, PgPool};
use shuttle_runtime::CustomError;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> shuttle_axum::ShuttleAxum {
    pool.execute(include_str!("../../db/schema.sql")).await.map_err(CustomError::new)?;
    let router = Router::new().route("/", get(hello_world));

    Ok(router.into())
}
