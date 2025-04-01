use anchor_lang::prelude::*;

use crate::{
    state::{Contributor, Recorder, Size},
    BandwidthRecorded, RecordBandwidthParams,
};

pub fn record_bandwidth<'info>(
    ctx: Context<'_, '_, '_, 'info, RecordBandwidth<'info>>,
    params: RecordBandwidthParams,
) -> Result<()> {
    let recorder = &mut ctx.accounts.recorder;
    let contributor = &mut ctx.accounts.contributor;

    contributor.record_bandwidth(params.bandwidth, params.timestamp)?;
    recorder.record_bandwidth(params.bandwidth)?;

    BandwidthRecorded::emit_event(
        params.node_id,
        params.bandwidth,
        params.timestamp,
        contributor.total_bandwidth,
        recorder.total_bandwidth,
    );

    Ok(())
}

#[derive(Accounts)]
#[instruction(params: RecordBandwidthParams)]
pub struct RecordBandwidth<'info> {
    #[account(mut,
        seeds = [b"state"],
        bump
    )]
    pub recorder: Account<'info, Recorder>,

    #[account(
        init_if_needed,
        seeds = [b"contributor", bytemuck::bytes_of(&params.node_id)],
        bump,
        payer = payer,
        space = Contributor::SIZE,
    )]
    pub contributor: Account<'info, Contributor>,

    // Signers
    #[account(mut)]
    pub payer: Signer<'info>,

    // programs
    pub system_program: Program<'info, System>,
}
