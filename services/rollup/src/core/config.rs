use std::env;

#[derive(Clone)]
pub struct Config {
    pub job_schedule: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            job_schedule: env::var("JOB_SCHEDULE").unwrap(),
        }
    }
}
