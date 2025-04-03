use anchor_lang::prelude::*;

use crate::state::{Recorder, Size};

pub fn initialize<'info>(_ctx: Context<'_, '_, '_, 'info, Initialize<'info>>) -> Result<()> {
    // State account created and initialize with default value
    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [b"state"],
        bump,
        payer = payer,
        space = Recorder::SIZE,
    )]
    pub recorder: Account<'info, Recorder>,

    // Signers
    #[account(mut)]
    pub payer: Signer<'info>,

    // programs
    pub system_program: Program<'info, System>,
}
