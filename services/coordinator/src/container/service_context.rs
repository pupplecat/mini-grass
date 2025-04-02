use std::sync::Arc;

use crate::{
    core::config::Config,
    repositories::bandwidth_record_store::{self, BandwidthRecordRepository},
    usecases::report_bandwidth::{self, ReportBandwidthUsecase},
};

#[derive(Clone)]
pub struct ServiceContext {
    pub config: Config,
}

impl ServiceContext {
    pub fn new(config: &Config) -> Self {
        Self {
            config: config.clone(),
        }
    }

    fn json_file_bandwidth_record_repo(&self) -> Arc<dyn BandwidthRecordRepository> {
        let repo: Arc<dyn BandwidthRecordRepository> =
            Arc::from(bandwidth_record_store::new(self.config.bw_filename.clone()));

        repo
    }

    pub fn report_bandwidth_usecase(&self) -> Arc<dyn ReportBandwidthUsecase> {
        let usecase: Arc<dyn ReportBandwidthUsecase> = Arc::from(report_bandwidth::new(
            self.json_file_bandwidth_record_repo(),
        ));

        usecase
    }
}
