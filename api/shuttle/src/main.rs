use api_lib::films::{create_film, delete_film, get_film, list_films, update_film};
use axum::{routing::{get, post, put, delete}, Router};
use shuttle_runtime::CustomError;
use sqlx::{Executor, PgPool};

#[shuttle_runtime::main]
async fn axum(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    pool.execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(CustomError::new)?;
    let film_repository = api_lib::film_repository::PostgresFilmRepository::new(pool);

    let films_router = Router::new()
        .route("/", get(list_films))
        .route("/:id", get(get_film))
        .route("/", post(create_film))
        .route("/:id", put(update_film))
        .route("/:id", delete(delete_film));

    let router = Router::new()
        // .route("/version", get(version))
        // .route("/health", get(health))
        .nest("/v1/films", films_router)
        .with_state(film_repository);

    Ok(router.into())
}
