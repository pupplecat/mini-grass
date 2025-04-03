use anchor_lang::prelude::*;

pub mod error;
mod instructions;
mod macros;
mod state;

use instructions::*;
pub use state::*;

declare_id!("Pwr6Zo12iYxEqqeaLsWXcaCuw5bw5M3QdZxFnULhjmU");

#[program]
pub mod bw_recorder {

    use super::*;

    pub fn initialize<'info>(ctx: Context<'_, '_, '_, 'info, Initialize<'info>>) -> Result<()> {
        instructions::initialize(ctx)
    }

    pub fn record_bandwidth<'info>(
        ctx: Context<'_, '_, '_, 'info, RecordBandwidth<'info>>,
        params: RecordBandwidthParams,
    ) -> Result<()> {
        instructions::record_bandwidth(ctx, params)
    }
}
