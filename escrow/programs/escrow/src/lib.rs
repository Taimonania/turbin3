use anchor_lang::prelude::*;

declare_id!("8UH4wi7RRHEA2sbUWi7VBreSrHLvRqXZgW7Bs8dDzuZg");

pub mod state;
pub mod instructions;

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}
