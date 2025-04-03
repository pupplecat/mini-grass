use std::fmt::Display;

use crate::usecases::error::RollupError;

#[derive(Debug)]
pub struct SchedulerError(RollupError);

impl Display for SchedulerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<RollupError> for SchedulerError {
    fn from(error: RollupError) -> Self {
        SchedulerError(error)
    }
}
