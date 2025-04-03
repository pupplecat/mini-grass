use derive_more::Display;

use crate::repositories::error::RepoError;

#[derive(Debug, Clone, Display)]
pub enum RollupError {
    RepositoryError(String),
    BwRecordClientError(String),
}

impl From<RepoError> for RollupError {
    fn from(e: RepoError) -> Self {
        match e {
            RepoError::IoError(e) => RollupError::RepositoryError(e),
            RepoError::SerializeError(e) => RollupError::RepositoryError(e),
            RepoError::DeserializeError(e) => RollupError::RepositoryError(e),
        }
    }
}

impl From<Box<dyn std::error::Error>> for RollupError {
    fn from(e: Box<dyn std::error::Error>) -> Self {
        RollupError::BwRecordClientError(e.to_string())
    }
}
