use coordinator::core::config::Config;
use std::io::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let config = Config::from_env();
    coordinator::transports::actix::server::serve(&config).await
}
