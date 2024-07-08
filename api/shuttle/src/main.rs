use api_lib::films::{create_film, delete_film, get_film, list_films, update_film};
use axum::{
    extract::Extension,
    routing::{delete, get, post, put},
    Router,
};
use shuttle_runtime::CustomError;
use sqlx::{Executor, PgPool};
use std::sync::Arc;
use tower_http::services::{ServeDir, ServeFile};

#[shuttle_runtime::main]
async fn axum(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    pool.execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(CustomError::new)?;
    let film_repository = api_lib::film_repository::PostgresFilmRepository::new(pool);
    let film_repository: Arc<Box<dyn api_lib::film_repository::FilmRepository>> =
        Arc::new(Box::new(film_repository));

    let films_router = Router::new()
        .route("/", get(list_films))
        .route("/:id", get(get_film))
        .route("/", post(create_film))
        .route("/:id", put(update_film))
        .route("/:id", delete(delete_film));

    let api_router = Router::new()
        .nest("/v1/films", films_router)
        .layer(Extension(film_repository));

    let static_dir = ServeDir::new("static");
    let index_file = ServeFile::new("static/index.html");

    println!("{:?}", static_dir);

    let router = Router::new()
        // .route("/version", get(version))
        // .route("/health", get(health))
        .nest("/api", api_router)
        .nest_service("/", index_file)
        .nest_service("/static", static_dir);

    Ok(router.into())
}
