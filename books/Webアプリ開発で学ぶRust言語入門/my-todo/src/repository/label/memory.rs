use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use axum::async_trait;

use crate::repository::RepositoryError;

use super::{CreateLabel, Label, LabelRepository};

type LabelData = HashMap<u32, Label>;

#[derive(Debug, Clone, Default)]
pub struct LabelRepositoryForMemory {
    pub(crate) store: Arc<RwLock<LabelData>>,
}

impl LabelRepositoryForMemory {
    pub fn new() -> Self {
        Self {
            store: Default::default(),
        }
    }
}

#[async_trait]
impl LabelRepository for LabelRepositoryForMemory {
    async fn all(&self) -> Result<Vec<Label>, RepositoryError> {
        todo!()
    }

    async fn create(&self, payload: CreateLabel) -> Result<Label, RepositoryError> {
        todo!()
    }

    async fn delete(&self, id: i32) -> Result<(), RepositoryError> {
        todo!()
    }
}
