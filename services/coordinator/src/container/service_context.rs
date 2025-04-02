use crate::core::config::Config;

#[derive(Clone)]
pub struct ServiceContext {
    config: Config,
}

impl ServiceContext {
    pub fn new(config: &Config) -> Self {
        Self {
            config: config.clone(),
        }
    }
}
