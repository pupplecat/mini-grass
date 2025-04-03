use std::env;

#[derive(Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub bw_filename: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            host: env::var("HOST").unwrap(),
            port: str::parse::<u16>(env::var("PORT").unwrap().as_str()).unwrap(),
            bw_filename: env::var("BW_FILENAME").unwrap_or("./bw_file.json".to_string()),
        }
    }

    pub fn listening_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
