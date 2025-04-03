use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use tokio::fs;

use super::error::RepoError;

#[async_trait]
pub trait BandwidthRecordRepository: Send + Sync {
    async fn record(&self, node_id: u64, bandwidth: u64) -> Result<u64, RepoError>;

    async fn read_all(&self) -> Result<HashMap<u64, u64>, RepoError>;

    async fn clear(&self) -> Result<(), RepoError>;
}

pub struct JsonFileBandwidthRecordRepositoryImpl {
    filename: String,
}

pub fn new(filename: String) -> Arc<dyn BandwidthRecordRepository> {
    Arc::from(JsonFileBandwidthRecordRepositoryImpl { filename })
}

#[async_trait]
impl BandwidthRecordRepository for JsonFileBandwidthRecordRepositoryImpl {
    async fn record(&self, node_id: u64, bandwidth: u64) -> Result<u64, RepoError> {
        let file_contents = fs::read_to_string(&self.filename).await;
        let mut records: HashMap<u64, u64> = match file_contents {
            Ok(contents) => serde_json::from_str(&contents)
                .map_err(|e| RepoError::DeserializeError(e.to_string()))?,
            Err(_e) => HashMap::new(),
        };

        let current_bandwidth = records.get(&node_id).copied().unwrap_or(0);

        let updated_bandwidth = current_bandwidth + bandwidth;
        records.insert(node_id, updated_bandwidth);

        let serialized = serde_json::to_string_pretty(&records)
            .map_err(|e| RepoError::SerializeError(e.to_string()))?;

        fs::write(&self.filename, serialized)
            .await
            .map_err(|e| RepoError::IoError(e.to_string()))?;

        Ok(updated_bandwidth)
    }

    async fn read_all(&self) -> Result<HashMap<u64, u64>, RepoError> {
        let file_contents = fs::read_to_string(&self.filename).await;
        let records: HashMap<u64, u64> = match file_contents {
            Ok(contents) => serde_json::from_str(&contents)
                .map_err(|e| RepoError::DeserializeError(e.to_string()))?,
            Err(_e) => HashMap::new(),
        };

        Ok(records)
    }

    async fn clear(&self) -> Result<(), RepoError> {
        fs::write(&self.filename, "{}".to_string())
            .await
            .map_err(|e| RepoError::IoError(e.to_string()))?;
        Ok(())
    }
}
