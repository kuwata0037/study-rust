use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::repository::{
    todo::{CreateTodo, TodoRepository, UpdateTodo},
    RepositoryError,
};

pub async fn all_todo<R: TodoRepository>(
    State(repository): State<Arc<R>>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository.all().await.map_err(handle_error)?;
    Ok((StatusCode::OK, Json(todo)))
}

pub async fn create_todo<R: TodoRepository>(
    State(repository): State<Arc<R>>,
    Json(payload): Json<CreateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository.create(payload).await.map_err(handle_error)?;

    Ok((StatusCode::CREATED, Json(todo)))
}

pub async fn find_todo<R: TodoRepository>(
    State(repository): State<Arc<R>>,
    Path(id): Path<u32>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository.find(id).await.map_err(handle_error)?;

    Ok((StatusCode::OK, Json(todo)))
}

pub async fn update_todo<R: TodoRepository>(
    State(repository): State<Arc<R>>,
    Path(id): Path<u32>,
    Json(payload): Json<UpdateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository.update(id, payload).await.map_err(handle_error)?;
    Ok((StatusCode::CREATED, Json(todo)))
}

pub async fn delete_todo<R: TodoRepository>(
    State(repository): State<Arc<R>>,
    Path(id): Path<u32>,
) -> Result<impl IntoResponse, StatusCode> {
    repository.delete(id).await.map_err(handle_error)?;
    Ok(StatusCode::NO_CONTENT)
}

fn handle_error(error: RepositoryError) -> StatusCode {
    match error {
        RepositoryError::NotFound(_id) => StatusCode::NOT_FOUND,
        RepositoryError::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
