mod memory;
mod postgres;

use axum::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::RepositoryError;

pub use memory::LabelRepositoryForMemory;
pub use postgres::LabelRepositoryForPostgres;

#[async_trait]
pub trait LabelRepository: Send + Sync + 'static {
    async fn all(&self) -> Result<Vec<Label>, RepositoryError>;
    async fn create(&self, payload: CreateLabel) -> Result<Label, RepositoryError>;
    async fn delete(&self, id: i32) -> Result<(), RepositoryError>;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateLabel {
    name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateLabel {
    id: i32,
    name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct Label {
    id: i32,
    name: String,
}

impl Label {
    fn new(id: i32, name: String) -> Self {
        Self { id, name }
    }
}
