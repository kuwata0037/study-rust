use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};

use crate::{
    handler::handle_error,
    repository::label::{CreateLabel, LabelRepository},
};

pub async fn create_label<T: LabelRepository>(
    Extension(repository): Extension<Arc<T>>,
    Json(payload): Json<CreateLabel>,
) -> Result<impl IntoResponse, StatusCode> {
    let label = repository.create(payload).await.map_err(handle_error)?;

    Ok((StatusCode::CREATED, Json(label)))
}

pub async fn all_label<T: LabelRepository>(
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let labels = repository.all().await.map_err(handle_error)?;

    Ok((StatusCode::OK, Json(labels)))
}

pub async fn delete_label<T: LabelRepository>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    repository.delete(id).await.map_err(handle_error)?;

    Ok(StatusCode::NO_CONTENT)
}
