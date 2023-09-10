use std::{
    collections::HashMap,
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
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

type TodoData = HashMap<i32, Todo>;

#[derive(Debug, Clone)]
pub struct TodoRepositoryForMemory {
    store: Arc<RwLock<TodoData>>,
}

impl TodoRepositoryForMemory {
    pub fn new() -> Self {
        Self {
            store: Arc::default(),
        }
    }

    fn write_store_ref(&self) -> RwLockWriteGuard<TodoData> {
        self.store.write().unwrap()
    }

    fn read_store_ref(&self) -> RwLockReadGuard<TodoData> {
        self.store.read().unwrap()
    }
}

impl TodoRepository for TodoRepositoryForMemory {
    fn all(&self) -> Vec<Todo> {
        let store = self.read_store_ref();
        store.values().cloned().collect()
    }

    fn find(&self, id: i32) -> Option<Todo> {
        let store = self.read_store_ref();
        store.get(&id).cloned()
    }

    fn create(&self, payload: CreateTodo) -> Result<Todo, RepositoryError> {
        let mut store = self.write_store_ref();
        let id = (store.len() + 1) as i32;
        let todo = Todo::new(id, payload.text);
        store.insert(id, todo.clone());
        Ok(todo)
    }

    fn update(&self, id: i32, payload: UpdateTodo) -> Result<Todo, RepositoryError> {
        let mut store = self.write_store_ref();
        let todo = store
            .get(&id)
            .ok_or_else(|| RepositoryError::NotFound(id))?;

        let text = payload.text.unwrap_or_else(|| todo.text.clone());
        let completed = payload.completed.unwrap_or_else(|| todo.completed);
        let todo = Todo {
            id,
            text,
            completed,
        };
        store.insert(id, todo.clone());
        Ok(todo)
    }

    fn delete(&self, id: i32) -> Result<(), RepositoryError> {
        let mut store = self.write_store_ref();
        store
            .remove(&id)
            .ok_or_else(|| RepositoryError::NotFound(id))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn todo_crud_scenario() {
        let id = 1;
        let text = "todo text".to_string();
        let expected = Todo::new(id, text.clone());

        // 1. create
        let repository = TodoRepositoryForMemory::new();
        let todo = repository
            .create(CreateTodo { text })
            .expect("failed create todo.");
        assert_eq!(expected, todo);

        // 2. find
        let todo = repository.find(id).expect("failed find todo.");
        assert_eq!(expected, todo);

        // 3. all
        let todo = repository.all();
        assert_eq!(vec![expected], todo);

        // 4. update
        let text = "update todo text".to_string();
        let todo = repository
            .update(
                id,
                UpdateTodo {
                    text: Some(text.clone()),
                    completed: Some(true),
                },
            )
            .expect("failed update todo.");
        assert_eq!(
            Todo {
                id,
                text,
                completed: true
            },
            todo
        );

        // 5. delete
        let result = repository.delete(id);
        assert!(result.is_ok(), "failed delete todo: {result:?}")
    }
}
