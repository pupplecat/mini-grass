use chrono::Utc;
use std::{io::Result, time::Duration};

use job_scheduler::{Job, JobScheduler};

use crate::core::config::Config;

pub async fn serve(config: &Config) -> Result<()> {
    let mut sched = JobScheduler::new();

    println!("xxx config.job_schedule {}", &config.job_schedule);

    sched.add(Job::new(config.job_schedule.parse().unwrap(), || {
        let config_clone = config.clone();
        tokio::spawn(async move {
            run_schedule(&config_clone).await;
        });
    }));

    loop {
        sched.tick();

        std::thread::sleep(Duration::from_millis(500));
    }
}

pub async fn run_schedule(_config: &Config) {
    // Execute scheduled logic
    println!("run schedule {:?}", Utc::now());
}
