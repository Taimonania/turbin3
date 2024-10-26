use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

declare_id!("Jp4mdejcrdQtiEgYqg1AZVo8MbzEnQZGwkB8kpdevdz");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)?;
        Ok(())
    }

    pub fn deposit(ctx: Context<Operations>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Operations>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)?;
        Ok(())
    }

    pub fn close(ctx: Context<Close>) -> Result<()> {
        ctx.accounts.close()?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init, 
        payer = user, 
        seeds = [b"state", user.key().as_ref()],
        bump,
        space = 8 + 1 + 1
    )]
    pub vault_state: Account<'info, VaultState>,
    #[account(
        seeds = [b"vault", vault_state.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        self.vault_state.vault_bump = bumps.vault_state;
        self.vault_state.state_bump = bumps.vault;
        Ok(())
    }
}

#[account]
pub struct VaultState {
    pub vault_bump: u8,
    pub state_bump: u8,
}

impl Space for VaultState {
    const INIT_SPACE: usize = 8 + 1 + 1;
}

#[derive(Accounts)]
pub struct Operations<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,
    #[account(
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.state_bump
    )]
    pub vault_state: Account<'info, VaultState>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient funds for rent exemption")]
    InsufficientFundsForRentExemption,
} 

impl<'info> Operations<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let data_len = self.vault.to_account_info().data_len();
        let min_rent_exemption = Rent::get()?.minimum_balance(data_len);

        if self.vault.lamports() + amount < min_rent_exemption {
            return Err(ErrorCode::InsufficientFundsForRentExemption.into());
        }

        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount)
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        let seeds = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];
        let signer_seeds = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(cpi_ctx, amount)
    }
}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.state_bump,
        close = user
    )]
    pub vault_state: Account<'info, VaultState>,
    pub system_program: Program<'info, System>,
}

impl<'info> Close<'info> {
    pub fn close(&mut self) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        let seeds = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];
        let signer_seeds = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(cpi_ctx, self.vault.lamports())
    }
}

// #[derive(Accounts)]
// pub struct Withdraw<'info> {
//     #[account(mut)]
//     pub user: Signer<'info>,
//     #[account(
//         mut,
//         seeds = [b"vault", vault_state.key().as_ref()],
//         bump = vault_state.vault_bump
//     )]
//     pub vault: SystemAccount<'info>,
//     #[account(
//         seeds = [b"state", user.key().as_ref()],
//         bump = vault_state.state_bump
//     )]
//     pub vault_state: Account<'info, VaultState>,
//     pub system_program: Program<'info, System>,
// }

// impl<'info> Withdraw<'info> {
//     pub fn withdraw(&mut self, amount: u64) -> Result<()> {
//         let cpi_program = self.system_program.to_account_info();

//         let cpi_accounts = Transfer {
//             from: self.vault.to_account_info(),
//             to: self.user.to_account_info(),
//         };

//         let seeds = &[
//             b"vault",
//             self.vault_state.to_account_info().key.as_ref(),
//             &[self.vault_state.vault_bump],
//         ];
//         let signer_seeds = &[&seeds[..]];
//         let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

//         transfer(cpi_ctx, amount)
//     }
// }

// #[derive(Accounts)]
// pub struct Deposit<'info> {
//     #[account(mut)]
//     pub user: Signer<'info>,
//     #[account(
//         mut,
//         seeds = [b"vault", vault_state.key().as_ref()],
//         bump = vault_state.vault_bump
//     )]
//     pub vault: SystemAccount<'info>,
//     #[account(
//         seeds = [b"state", user.key().as_ref()],
//         bump = vault_state.state_bump
//     )]
//     pub vault_state: Account<'info, VaultState>,
//     pub system_program: Program<'info, System>,
// }

// impl<'info> Deposit<'info> {
//     pub fn deposit(&mut self, amount: u64) -> Result<()> {
//         let cpi_program = self.system_program.to_account_info();

//         let cpi_accounts = Transfer {
//             from: self.user.to_account_info(),
//             to: self.vault.to_account_info(),
//         };

//         let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

//         transfer(cpi_ctx, amount)
//     }
// }
