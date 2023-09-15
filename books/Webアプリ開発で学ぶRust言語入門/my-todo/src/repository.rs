pub mod todo;

use thiserror::Error;

type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("NotFound, id is {0}")]
    NotFound(u32),
    #[error(transparent)]
    Unexpected(BoxError),
}
