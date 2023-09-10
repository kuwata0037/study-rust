use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::repository::{CreateTodo, TodoRepository};

pub async fn create_todo<R: TodoRepository>(
    State(repository): State<Arc<R>>,
    Json(payload): Json<CreateTodo>,
) -> impl IntoResponse {
    let todo = repository.create(payload).unwrap();

    (StatusCode::CREATED, Json(todo))
}
