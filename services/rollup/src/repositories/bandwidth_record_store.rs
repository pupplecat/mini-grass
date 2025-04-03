use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use tokio::fs;

use super::error::RepoError;

#[async_trait]
pub trait BandwidthRecordRepository: Send + Sync {
    // Read all records
    async fn read_all(&self) -> Result<HashMap<u64, u64>, RepoError>;

    // Clear all records
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
    async fn read_all(&self) -> Result<HashMap<u64, u64>, RepoError> {
        // rea content from file
        let file_contents = fs::read_to_string(&self.filename).await;

        // if file exists, deserialize and return data, otherwise return a new HashMap
        let records: HashMap<u64, u64> = match file_contents {
            Ok(contents) => serde_json::from_str(&contents)
                .map_err(|e| RepoError::DeserializeError(e.to_string()))?,
            Err(_e) => HashMap::new(),
        };

        Ok(records)
    }

    async fn clear(&self) -> Result<(), RepoError> {
        // Write empty json object to file
        fs::write(&self.filename, "{}".to_string())
            .await
            .map_err(|e| RepoError::IoError(e.to_string()))?;
        Ok(())
    }
}
