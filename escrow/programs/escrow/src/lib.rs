use anchor_lang::prelude::*;

declare_id!("8UH4wi7RRHEA2sbUWi7VBreSrHLvRqXZgW7Bs8dDzuZg");

pub mod contexts;
use contexts::*;

pub mod state;
// use state::*;

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, deposit: u64, receive: u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, receive, ctx.bumps.escrow)?;
        ctx.accounts.deposit_to_vault(deposit)?;
        Ok(())
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.transfer_to_maker()?;
        ctx.accounts.withdraw_and_close()?;
        Ok(())
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.withdraw_and_close()?;
        Ok(())
    }
}
