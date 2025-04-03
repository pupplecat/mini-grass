use std::sync::Arc;

use async_trait::async_trait;
use tracing::info;

use crate::clients::bw_recorder_client::BwRecorderClient;

use super::error::RollupError;

#[async_trait]
pub trait AccountUsecase: Send + Sync {
    async fn prepare_contracts(&self) -> Result<(), RollupError>;
}

pub struct AccountUsecaseImpl {
    bw_recorder_client: Arc<BwRecorderClient>,
}

pub fn new(bw_recorder_client: Arc<BwRecorderClient>) -> impl AccountUsecase {
    AccountUsecaseImpl { bw_recorder_client }
}

#[async_trait]
impl AccountUsecase for AccountUsecaseImpl {
    async fn prepare_contracts(&self) -> Result<(), RollupError> {
        info!("Checking balance, if airdrop required");
        self.bw_recorder_client.airdrop_payer().await?;

        if self.bw_recorder_client.is_initialized().await? {
            info!("Contract initialized");
            return Ok(());
        }

        info!("Initializing contract");
        let tx = self.bw_recorder_client.initialize_contract().await?;

        info!("Contract initialized: {}", tx);

        Ok(())
    }
}
