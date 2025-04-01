use anchor_lang::prelude::*;

declare_id!("qL45T6VenxJA8RNaNupKW2K5am8KF1jWTJHEmHqzhGf");

#[program]
pub mod bw_recorder {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
