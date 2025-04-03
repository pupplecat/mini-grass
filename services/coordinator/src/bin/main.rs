use coordinator::core::config::Config;
use std::io::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    // load config from environment variables
    let config = Config::from_env();

    // start actix server
    coordinator::transports::actix::server::serve(&config).await
}
