use anchor_lang::prelude::*;

#[derive(Debug, Clone, Copy, AnchorSerialize, AnchorDeserialize, PartialEq, Eq)]
pub struct RecordBandwidthParams {
    pub node_id: u64,
    pub bandwidth: u64,
    pub timestamp: u64,
}
