use std::time::Duration;
use tracing::{error, info};
use tracing_bunyan_formatter::JsonStorageLayer;
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

use job_scheduler::{Job, JobScheduler};

use crate::{container::ServiceContext, core::config::Config};

use super::error::SchedulerError;

pub async fn serve(config: &Config) -> Result<(), SchedulerError> {
    initial_logger();

    // initialize contract and fund payer account
    initialize_contract(config).await?;

    // Create a new JobScheduler instance
    let mut scheduler = JobScheduler::new();

    // Add a new Job
    scheduler.add(Job::new(config.job_schedule.parse().unwrap(), || {
        let config_clone = config.clone();
        tokio::spawn(async move {
            run_schedule(&config_clone).await;
        });
    }));

    info!("Scheduler started: {}", &config.job_schedule);
    // Keep the service alive
    loop {
        scheduler.tick();

        std::thread::sleep(Duration::from_millis(500));
    }
}

pub async fn initialize_contract(config: &Config) -> Result<(), SchedulerError> {
    info!("Initialize contracts");

    let ctx = ServiceContext::new(config);

    // perform prepare contracts
    ctx.account_usecase().prepare_contracts().await?;

    Ok(())
}

pub async fn run_schedule(config: &Config) {
    info!("Start executing task");

    let ctx = ServiceContext::new(config);

    // perform sync recorded bandwidth
    let ret = ctx.sync_bandwidth_usecase().sync().await;

    if ret.is_ok() {
        info!("Execution successfully");
        return;
    }

    // Log failed case
    error!("Execution failed: {}", ret.err().unwrap());
}

pub fn initial_logger() {
    let env_filter_layer = EnvFilter::new("INFO");

    let _ = LogTracer::init();
    let formatter_layer = tracing_subscriber::fmt::layer().with_test_writer();
    let subscriber = Registry::default()
        .with(env_filter_layer)
        .with(JsonStorageLayer)
        .with(formatter_layer);

    // panic here is expected, in e2e, subscriber my already been set by prior test case.
    let _ = tracing::subscriber::set_global_default(subscriber);
}
