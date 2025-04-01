use anchor_lang::prelude::*;

pub mod error;
mod instructions;
mod macros;
mod state;

use instructions::*;
pub use state::*;

declare_id!("qL45T6VenxJA8RNaNupKW2K5am8KF1jWTJHEmHqzhGf");

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
