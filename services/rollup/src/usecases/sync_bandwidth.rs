use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use tracing::info;

use crate::{
    clients::bw_recorder_client::BwRecorderClient,
    repositories::bandwidth_record_store::BandwidthRecordRepository,
};

use super::error::RollupError;

#[async_trait]
pub trait SyncBandwidthUsecase: Send + Sync {
    async fn sync(&self) -> Result<(), RollupError>;
}

pub struct SyncBandwidthUsecaseImpl {
    bw_recorder_client: Arc<BwRecorderClient>,
    bandwidth_record_repo: Arc<dyn BandwidthRecordRepository>,
}

pub fn new(
    bw_recorder_client: Arc<BwRecorderClient>,
    bandwidth_record_repo: Arc<dyn BandwidthRecordRepository>,
) -> impl SyncBandwidthUsecase {
    SyncBandwidthUsecaseImpl {
        bw_recorder_client,
        bandwidth_record_repo,
    }
}

#[async_trait]
impl SyncBandwidthUsecase for SyncBandwidthUsecaseImpl {
    async fn sync(&self) -> Result<(), RollupError> {
        // read all records from repository
        let records = self.bandwidth_record_repo.read_all().await?;

        info!("record counts: {}", records.len());

        // No new data, skip the execution
        if records.is_empty() {
            return Ok(());
        }

        // call record bandwidth to submit onchain
        let timestamp = Utc::now().timestamp() as u64;
        let tx = self
            .bw_recorder_client
            .record_bandwidth(records, timestamp)
            .await?;

        info!("Record synced: {}", tx);

        // clear previous recorded information
        self.bandwidth_record_repo.clear().await?;
        info!("Record cleared");

        Ok(())
    }
}
