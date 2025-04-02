use std::env;

#[derive(Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            host: env::var("HOST").unwrap(),
            port: str::parse::<u16>(env::var("PORT").unwrap().as_str()).unwrap(),
        }
    }

    pub fn listening_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
