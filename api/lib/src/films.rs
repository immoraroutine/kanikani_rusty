use crate::film_repository::FilmRepository;
use axum::{
    extract::{self, Extension, Path},
    response::{self, IntoResponse},
};
use shared::models::{CreateFilm, Film};
use std::sync::Arc;
use uuid::Uuid;

type Repository = Extension<Arc<Box<dyn FilmRepository>>>;

pub async fn list_films(repo: Repository) -> impl IntoResponse {
    match repo.get_films().await {
        Ok(films) => response::Json(films).into_response(),
        Err(e) => response::Json(format!("Internal server error: {:?}", e)).into_response(),
    }
}

pub async fn get_film(Path(film_id): Path<Uuid>, repo: Repository) -> impl IntoResponse {
    match repo.get_film(&film_id).await {
        Ok(film) => response::Json(film).into_response(),
        Err(_) => response::Json("Film not found").into_response(),
    }
}

pub async fn create_film(
    // なぜかrepoをさきに受け取らないとエラーになる
    // https://qiita.com/Sicut_study/items/5e5d6cce5ba48c225367
    repo: Repository,
    extract::Json(create_film): extract::Json<CreateFilm>,
) -> impl IntoResponse {
    match repo.create_film(&create_film).await {
        Ok(film) => response::Json(film).into_response(),
        Err(e) => response::Json(format!("Internal server error: {:?}", e)).into_response(),
    }
}

pub async fn update_film(
    // なぜかrepoをさきに受け取らないとエラーになる
    // https://qiita.com/Sicut_study/items/5e5d6cce5ba48c225367
    repo: Repository,
    extract::Json(film): extract::Json<Film>,
) -> impl IntoResponse {
    match repo.update_film(&film).await {
        Ok(film) => extract::Json(film).into_response(),
        Err(e) => extract::Json(format!("Internal server error: {:?}", e)).into_response(),
    }
}

pub async fn delete_film(Path(film_id): Path<Uuid>, repo: Repository) -> impl IntoResponse {
    match repo.delete_film(&film_id).await {
        Ok(film) => extract::Json(film).into_response(),
        Err(e) => extract::Json(format!("Internal server error: {:?}", e)).into_response(),
    }
}
