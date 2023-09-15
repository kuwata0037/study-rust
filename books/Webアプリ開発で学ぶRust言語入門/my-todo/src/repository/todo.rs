mod memory;
mod postgres;

pub use memory::TodoRepositoryForMemory;
pub use postgres::TodoRepositoryForPostgres;

use super::RepositoryError;
use serde::Deserialize;
use serde::Serialize;

#[axum::async_trait]
pub trait TodoRepository: Send + Sync + 'static {
    async fn all(&self) -> Result<Vec<Todo>, RepositoryError>;
    async fn find(&self, id: u32) -> Result<Todo, RepositoryError>;
    async fn create(&self, payload: CreateTodo) -> Result<Todo, RepositoryError>;
    async fn update(&self, id: u32, payload: UpdateTodo) -> Result<Todo, RepositoryError>;
    async fn delete(&self, id: u32) -> Result<(), RepositoryError>;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateTodo {
    text: String,
}

impl CreateTodo {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateTodo {
    text: Option<String>,
    completed: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Todo {
    id: u32,
    text: String,
    completed: bool,
}

impl Todo {
    pub fn new(id: u32, text: String) -> Self {
        Self {
            id,
            text,
            completed: false,
        }
    }
}
