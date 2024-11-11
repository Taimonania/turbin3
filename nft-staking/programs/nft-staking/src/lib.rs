use anchor_lang::prelude::*;

pub mod state;
use state::*;
pub mod instructions;
use instructions::*;
pub mod errors;
use errors::*;

declare_id!("7eXd3QXce4DrWH8B2S8KgufRw5ooiJGgPGJPqkRDLqx9");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        ctx.accounts.initialize_user_account(&ctx.bumps)?;
        Ok(())
    }

    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        points_per_stake: u8,
        max_stake: u8,
        freeze_period: u32,
    ) -> Result<()> {
        ctx.accounts
            .initialize_stake_config(points_per_stake, max_stake, freeze_period, &ctx.bumps)?;
        Ok(())
    }

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.stake(&ctx.bumps)
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        ctx.accounts.unstake()
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        ctx.accounts.claim()
    }
}
