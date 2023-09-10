use thiserror::Error;
pub mod todo;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("NotFound, id is {0}")]
    NotFound(u32),
}
