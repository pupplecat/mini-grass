use anchor_lang::prelude::*;

#[event]
#[derive(Default)]
pub struct BandwidthRecorded {
    pub node_id: u64,
    pub bandwidth: u64,
    pub timestamp: u64,
    pub total_contributor_bandwidth: u64,
    pub total_bandwidth: u64,
}

impl BandwidthRecorded {
    pub fn emit_event(
        node_id: u64,
        bandwidth: u64,
        timestamp: u64,
        total_contributor_bandwidth: u64,
        total_bandwidth: u64,
    ) {
        msg!("Bandwidth recorded for node_id: {}", node_id);
        emit!(Self {
            node_id,
            bandwidth,
            timestamp,
            total_contributor_bandwidth,
            total_bandwidth,
        });
    }
}
