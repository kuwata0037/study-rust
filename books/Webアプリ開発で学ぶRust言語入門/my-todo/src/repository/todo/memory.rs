use std::{
    collections::HashMap,
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use crate::repository::RepositoryError;

use super::{CreateTodo, Todo, TodoRepository, UpdateTodo};

type TodoData = HashMap<u32, Todo>;

#[derive(Debug, Clone, Default)]
pub struct TodoRepositoryForMemory {
    store: Arc<RwLock<TodoData>>,
}

impl TodoRepositoryForMemory {
    pub fn new() -> Self {
        Self::default()
    }

    fn write_store_ref(&self) -> RwLockWriteGuard<TodoData> {
        self.store.write().unwrap()
    }

    fn read_store_ref(&self) -> RwLockReadGuard<TodoData> {
        self.store.read().unwrap()
    }
}

#[axum::async_trait]
impl TodoRepository for TodoRepositoryForMemory {
    async fn all(&self) -> Result<Vec<Todo>, RepositoryError> {
        let store = self.read_store_ref();
        Ok(store.values().cloned().collect())
    }

    async fn find(&self, id: u32) -> Result<Todo, RepositoryError> {
        let store = self.read_store_ref();
        let todo = store
            .get(&id)
            .cloned()
            .ok_or(RepositoryError::NotFound(id))?;
        Ok(todo)
    }

    async fn create(&self, payload: CreateTodo) -> Result<Todo, RepositoryError> {
        let mut store = self.write_store_ref();
        let id = (store.len() + 1) as u32;
        let todo = Todo::new(id, payload.text);
        store.insert(id, todo.clone());
        Ok(todo)
    }

    async fn update(&self, id: u32, payload: UpdateTodo) -> Result<Todo, RepositoryError> {
        let mut store = self.write_store_ref();
        let todo = store.get(&id).ok_or(RepositoryError::NotFound(id))?;

        let text = payload.text.unwrap_or_else(|| todo.text.clone());
        let completed = payload.completed.unwrap_or(todo.completed);
        let todo = Todo {
            id,
            text,
            completed,
        };
        store.insert(id, todo.clone());
        Ok(todo)
    }

    async fn delete(&self, id: u32) -> Result<(), RepositoryError> {
        let mut store = self.write_store_ref();
        store.remove(&id).ok_or(RepositoryError::NotFound(id))?;
        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    #[tokio::test]
    async fn todo_crud_scenario() {
        let id = 1;
        let text = "todo text".to_string();
        let expected = Todo::new(id, text.clone());

        // 1. create
        let repository = TodoRepositoryForMemory::new();
        let todo = repository
            .create(CreateTodo { text })
            .await
            .expect("failed create todo.");
        assert_eq!(expected, todo);

        // 2. find
        let todo = repository.find(id).await.expect("failed find todo.");
        assert_eq!(expected, todo);

        // 3. all
        let todo = repository.all().await.unwrap();
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
            .await
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
        let result = repository.delete(id).await;
        assert!(result.is_ok(), "failed delete todo: {result:?}")
    }
}
