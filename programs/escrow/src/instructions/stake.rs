use anchor_lang::prelude::*;
use anchor_spl::{ associated_token::AssociatedToken, token::{ Mint, Token, TokenAccount } };
use marinade_cpi::{
    cpi::accounts::{ Deposit, LiquidUnstake },
    cpi::{ deposit, liquid_unstake },
    program::MarinadeFinance,
};

use crate::PoolState;

#[derive(Accounts)]
pub struct Staking<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    // halopot payes the sol
    #[account(mut, seeds = [b"halopot"], bump = pool_state.bump, has_one = admin)]
    pub pool_state: Account<'info, PoolState>,

    // The HaloPot mSOL Account (receives the mSOL)
    #[account(
        init_if_needed,
        payer = admin,
        associated_token::mint = msol_mint,
        associated_token::authority = pool_state
    )]
    pub pool_msol_account: Account<'info, TokenAccount>,

    /// CHECK: the account is already verified by marinade program
    #[account(mut)]
    pub marinade_state: AccountInfo<'info>,

    #[account(mut)]
    pub msol_mint: Account<'info, Mint>,

    #[account(mut)]

    /// CHECK: Marinade verifeie SOL leg.
    pub liq_pool_sol_leg: AccountInfo<'info>,

    // CHECK: Marinade verified MSOl
    #[account(mut)]
    pub liq_pool_msol_leg: AccountInfo<'info>,

    /// CHECK: Marinade verified Authority.
    pub liq_pool_msol_leg_authority: AccountInfo<'info>,

    // stores long-term staked sol pegged to msol
    #[account(mut)]
    /// CHECK: Safe. Marinade Reserve.
    pub reserve_pda: AccountInfo<'info>,

    /// CHECK: Marinade Verified Mint Auth.
    pub msol_mint_authority: AccountInfo<'info>,

    pub marinade_program: Program<'info, MarinadeFinance>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

// 6. STAKE ASSETS (Admin converts SOL -> mSOL)
impl<'info> Staking<'info> {
    pub fn stake(&mut self, amount_sol: u64) -> Result<()> {
        // PoolState PDA signs (it owns the SOL that's transferred)
        let pool_seeds = &[b"halopot".as_ref(), &[self.pool_state.bump]];
        let signer = &[&pool_seeds[..]];

        // Build Marinade CPI accounts and context
        let cpi_accounts = Deposit {
            state: self.marinade_state.to_account_info(),
            msol_mint: self.msol_mint.to_account_info(),
            liq_pool_sol_leg_pda: self.liq_pool_sol_leg.to_account_info(),
            liq_pool_msol_leg: self.liq_pool_msol_leg.to_account_info(),
            liq_pool_msol_leg_authority: self.liq_pool_msol_leg_authority.to_account_info(),
            reserve_pda: self.reserve_pda.to_account_info(),
            transfer_from: self.pool_state.to_account_info(), // SOL comes from PoolState
            mint_to: self.pool_msol_account.to_account_info(), // mSOL goes into pool's mSOL ATA
            msol_mint_authority: self.msol_mint_authority.to_account_info(),
            system_program: self.system_program.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            self.marinade_program.to_account_info(),
            cpi_accounts,
            signer
        );

        // perform Marinade deposit
        deposit(cpi_ctx, amount_sol)?;

        msg!("Staked {} lamports for mSOL", amount_sol);
        Ok(())
    }

    /// Liquid-unstake mSOL -> SOL via Marinade CPI
    pub fn unstake(&mut self, amount_msol: u64) -> Result<()> {
        // PoolState PDA signs
        let pool_seeds = &[b"halopot".as_ref(), &[self.pool_state.bump]];
        let signer = &[&pool_seeds[..]];

        // Map to Marinade CPI LiquidUnstake account layout
        let cpi_accounts = LiquidUnstake {
            state: self.marinade_state.to_account_info(),
            msol_mint: self.msol_mint.to_account_info(),
            liq_pool_sol_leg_pda: self.liq_pool_sol_leg.to_account_info(),
            liq_pool_msol_leg: self.liq_pool_msol_leg.to_account_info(),
            treasury_msol_account: self.liq_pool_msol_leg.to_account_info(), // Marinade's mSOL treasury
            get_msol_from: self.pool_msol_account.to_account_info(), // pool's mSOL ATA (source)
            get_msol_from_authority: self.pool_state.to_account_info(), // authority over pool mSOL ATA (PDA)
            transfer_sol_to: self.pool_state.to_account_info(), // SOL destination (PoolState PDA)
            system_program: self.system_program.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            self.marinade_program.to_account_info(),
            cpi_accounts,
            signer
        );

        // Call Marinade liquid unstake (amount in mSOL units)
        liquid_unstake(cpi_ctx, amount_msol)?;

        msg!("Liquid-unstaked {} mSOL for SOL", amount_msol);
        Ok(())
    }
}
