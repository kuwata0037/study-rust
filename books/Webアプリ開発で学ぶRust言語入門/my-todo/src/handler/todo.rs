use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::repository::todo::{CreateTodo, TodoRepository, UpdateTodo};

pub async fn all_todo<R: TodoRepository>(State(repository): State<Arc<R>>) -> impl IntoResponse {
    let todo = repository.all().unwrap();
    (StatusCode::OK, Json(todo))
}

pub async fn create_todo<R: TodoRepository>(
    State(repository): State<Arc<R>>,
    Json(payload): Json<CreateTodo>,
) -> impl IntoResponse {
    let todo = repository.create(payload).unwrap();

    (StatusCode::CREATED, Json(todo))
}

pub async fn find_todo<R: TodoRepository>(
    State(repository): State<Arc<R>>,
    Path(id): Path<u32>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository.find(id).map_err(|_e| StatusCode::NOT_FOUND)?;
    Ok((StatusCode::OK, Json(todo)))
}

pub async fn update_todo<R: TodoRepository>(
    State(repository): State<Arc<R>>,
    Path(id): Path<u32>,
    Json(payload): Json<UpdateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository
        .update(id, payload)
        .map_err(|_e| StatusCode::NOT_FOUND)?;
    Ok((StatusCode::CREATED, Json(todo)))
}

pub async fn delete_todo<R: TodoRepository>(
    State(repository): State<Arc<R>>,
    Path(id): Path<u32>,
) -> Result<impl IntoResponse, StatusCode> {
    repository.delete(id).map_err(|_e| StatusCode::NOT_FOUND)?;
    Ok(StatusCode::NO_CONTENT)
}
