use rollup::{core::config::Config, transports::scheduler::error::SchedulerError};

#[tokio::main]
async fn main() -> Result<(), SchedulerError> {
    dotenv::dotenv().ok();

    let config = Config::from_env();
    rollup::transports::scheduler::server::serve(&config).await?;

    Ok(())
}
