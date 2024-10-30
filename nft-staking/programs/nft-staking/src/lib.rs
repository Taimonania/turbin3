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

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
