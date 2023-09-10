pub mod todo;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("NotFound, id is {0}")]
    NotFound(u32),
}
