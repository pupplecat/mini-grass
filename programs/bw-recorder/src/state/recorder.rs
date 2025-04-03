use anchor_lang::prelude::*;

use super::Size;

#[account]
#[derive(Default, Eq, PartialEq, Debug)]
pub struct Recorder {
    pub total_bandwidth: u64, // 8 bytes
}

impl Recorder {
    pub fn record_bandwidth(&mut self, bandwidth: u64) -> Result<()> {
        self.total_bandwidth += bandwidth;

        Ok(())
    }
    pub const fn get_size() -> usize {
        8
    }
}

impl Size for Recorder {
    const SIZE: usize = Recorder::get_size() + 8;
}
