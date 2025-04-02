use derive_more::Display;

use crate::repositories::error::RepoError;

#[derive(Debug, Clone, Display)]
pub enum CoordinatorError {
    RepositoryError(String),
}

impl From<RepoError> for CoordinatorError {
    fn from(e: RepoError) -> Self {
        match e {
            RepoError::IoError(e) => CoordinatorError::RepositoryError(e),
            RepoError::SerializeError(e) => CoordinatorError::RepositoryError(e),
            RepoError::DeserializeError(e) => CoordinatorError::RepositoryError(e),
        }
    }
}
