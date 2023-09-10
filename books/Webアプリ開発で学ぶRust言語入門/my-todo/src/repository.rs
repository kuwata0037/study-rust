use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("NotFound, id is {0}")]
    NotFound(i32),
}

pub trait TodoRepository: Clone + Send + Sync + 'static {
    fn all(&self) -> Vec<Todo>;
    fn find(&self, id: i32) -> Option<Todo>;
    fn create(&self, payload: CreateTodo) -> Result<Todo, RepositoryError>;
    fn update(&self, id: i32, payload: UpdateTodo) -> Result<Todo, RepositoryError>;
    fn delete(&self, id: i32) -> Result<(), RepositoryError>;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Todo {
    id: i32,
    text: String,
    completed: bool,
}

impl Todo {
    pub fn new(id: i32, text: String) -> Self {
        Self {
            id,
            text,
            completed: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateTodo {
    text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateTodo {
    text: Option<String>,
    completed: Option<bool>,
}

type TodoDatas = HashMap<i32, Todo>;

#[derive(Debug, Clone)]
pub struct TodoRepositoryForMemory {
    store: Arc<RwLock<TodoDatas>>,
}

impl TodoRepositoryForMemory {
    pub fn new() -> Self {
        Self {
            store: Arc::default(),
        }
    }
}

impl TodoRepository for TodoRepositoryForMemory {
    fn all(&self) -> Vec<Todo> {
        todo!()
    }

    fn find(&self, id: i32) -> Option<Todo> {
        todo!()
    }

    fn create(&self, payload: CreateTodo) -> Result<Todo, RepositoryError> {
        todo!()
    }

    fn update(&self, id: i32, payload: UpdateTodo) -> Result<Todo, RepositoryError> {
        todo!()
    }

    fn delete(&self, id: i32) -> Result<(), RepositoryError> {
        todo!()
    }
}
