mod memory;

pub use memory::TodoRepositoryForMemory;

use super::RepositoryError;
use serde::Deserialize;
use serde::Serialize;

pub trait TodoRepository: Send + Sync + 'static {
    fn all(&self) -> Result<Vec<Todo>, RepositoryError>;
    fn find(&self, id: u32) -> Result<Todo, RepositoryError>;
    fn create(&self, payload: CreateTodo) -> Result<Todo, RepositoryError>;
    fn update(&self, id: u32, payload: UpdateTodo) -> Result<Todo, RepositoryError>;
    fn delete(&self, id: u32) -> Result<(), RepositoryError>;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateTodo {
    pub(crate) text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateTodo {
    pub(crate) text: Option<String>,
    pub(crate) completed: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Todo {
    pub(crate) id: u32,
    pub(crate) text: String,
    pub(crate) completed: bool,
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
