use std::io::Result;

use rollup::core::config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let config = Config::from_env();
    rollup::transports::scheduler::server::serve(&config).await
}
