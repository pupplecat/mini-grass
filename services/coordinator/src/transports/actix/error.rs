use std::fmt::Display;

use actix_web::{error::ErrorInternalServerError, Error};

use crate::usecases::error::CoordinatorError;

#[derive(Debug)]
pub struct ApiError(CoordinatorError);

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<ApiError> for Error {
    fn from(e: ApiError) -> Self {
        let err = e.0;
        ErrorInternalServerError(err)
    }
}

impl From<CoordinatorError> for ApiError {
    fn from(error: CoordinatorError) -> Self {
        ApiError(error)
    }
}
