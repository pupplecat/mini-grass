use anchor_lang::prelude::*;

use crate::{error::ErrorCode, validate};

use super::Size;

#[account]
#[derive(Default, Eq, PartialEq, Debug)]
pub struct Contributor {
    pub total_bandwidth: u64, // 8 bytes
    pub last_timestamp: u64,  // 8 bytes
}

impl Contributor {
    pub fn record_bandwidth(&mut self, bandwidth: u64, timestamp: u64) -> Result<()> {
        // new timestamp must must be newer than the previous recorded
        validate!(
            timestamp > self.last_timestamp,
            ErrorCode::InvalidTimestamp,
            "Timestamp already pass",
        )?;

        // Update total_bandwidth and set last_timestamp
        self.total_bandwidth += bandwidth;
        self.last_timestamp = timestamp;

        Ok(())
    }

    pub const fn get_size() -> usize {
        8 + 8
    }
}

impl Size for Contributor {
    const SIZE: usize = Contributor::get_size() + 8;
}
