use std::{
    collections::HashMap,
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use axum::async_trait;

use crate::repository::RepositoryError;

use super::{CreateLabel, Label, LabelRepository};

type LabelData = HashMap<i32, Label>;

#[derive(Debug, Clone, Default)]
pub struct LabelRepositoryForMemory {
    store: Arc<RwLock<LabelData>>,
}

impl LabelRepositoryForMemory {
    pub fn new() -> Self {
        Self {
            store: Default::default(),
        }
    }

    fn write_store_ref(&self) -> RwLockWriteGuard<LabelData> {
        self.store.write().unwrap()
    }

    fn read_store_ref(&self) -> RwLockReadGuard<LabelData> {
        self.store.read().unwrap()
    }
}

#[async_trait]
impl LabelRepository for LabelRepositoryForMemory {
    async fn all(&self) -> Result<Vec<Label>, RepositoryError> {
        let store = self.read_store_ref();
        Ok(store.values().cloned().collect())
    }

    async fn create(&self, payload: CreateLabel) -> Result<Label, RepositoryError> {
        let mut store = self.write_store_ref();
        let id = (store.len() + 1) as i32;
        let label = Label::new(id, payload.name);
        store.insert(id, label.clone());
        Ok(label)
    }

    async fn delete(&self, id: i32) -> Result<(), RepositoryError> {
        let mut store = self.write_store_ref();
        store
            .remove(&id)
            .ok_or(RepositoryError::NotFound(id as u32))?;
        Ok(())
    }
}
