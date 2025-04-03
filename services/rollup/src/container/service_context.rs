use std::sync::Arc;

use crate::{
    clients::bw_recorder_client::BwRecorderClient,
    core::config::Config,
    repositories::bandwidth_record_store::{self, BandwidthRecordRepository},
    usecases::{
        account::{self, AccountUsecase},
        sync_bandwidth::{self, SyncBandwidthUsecase},
    },
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
        // create a new BandwidthRecordRepository
        let repo: Arc<dyn BandwidthRecordRepository> =
            bandwidth_record_store::new(self.config.bw_filename.clone());

        repo
    }

    fn bw_recorder_client(&self) -> Arc<BwRecorderClient> {
        // create a new BwRecorderClient
        let bw_recorder_client: Arc<BwRecorderClient> = Arc::from(BwRecorderClient::new(
            self.config.rpc_url.clone(),
            self.config.payer_keypair_filename.clone(),
        ));

        bw_recorder_client
    }

    pub fn account_usecase(&self) -> Arc<dyn AccountUsecase> {
        // create a new AccountUsecase
        let usecase: Arc<dyn AccountUsecase> = Arc::from(account::new(self.bw_recorder_client()));

        usecase
    }

    pub fn sync_bandwidth_usecase(&self) -> Arc<dyn SyncBandwidthUsecase> {
        // create a new SyncBandwidthUsecase
        let usecase: Arc<dyn SyncBandwidthUsecase> = Arc::from(sync_bandwidth::new(
            self.bw_recorder_client(),
            self.json_file_bandwidth_record_repo(),
        ));

        usecase
    }
}
