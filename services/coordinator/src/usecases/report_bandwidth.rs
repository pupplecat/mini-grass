use std::sync::Arc;

use async_trait::async_trait;

use crate::repositories::bandwidth_record_store::BandwidthRecordRepository;

use super::error::CoordinatorError;

#[async_trait]
pub trait ReportBandwidthUsecase: Send + Sync {
    async fn report(&self, node_id: u64, bandwidth: u64) -> Result<u64, CoordinatorError>;
}

pub struct ReportBandwidthUsecaseImpl {
    bandwidth_record_repo: Arc<dyn BandwidthRecordRepository>,
}

pub fn new(
    bandwidth_record_repo: Arc<dyn BandwidthRecordRepository>,
) -> impl ReportBandwidthUsecase {
    ReportBandwidthUsecaseImpl {
        bandwidth_record_repo,
    }
}

#[async_trait]
impl ReportBandwidthUsecase for ReportBandwidthUsecaseImpl {
    async fn report(&self, node_id: u64, bandwidth: u64) -> Result<u64, CoordinatorError> {
        let pending_sync = self
            .bandwidth_record_repo
            .record(node_id, bandwidth)
            .await?;

        Ok(pending_sync)
    }
}
