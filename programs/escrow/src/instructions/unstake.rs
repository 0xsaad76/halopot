use anchor_lang::prelude::*;
use anchor_spl::{ associated_token::AssociatedToken, token::{ Mint, Token, TokenAccount } };
use marinade_cpi::{ cpi::accounts::LiquidUnstake, cpi::liquid_unstake, program::MarinadeFinance };

use crate::PoolState;

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(mut, seeds = [b"halopot"], bump = pool_state.bump, has_one = admin)]
    pub pool_state: Account<'info, PoolState>,

    /// HaloPot mSOL ATA that will be debited
    #[account(mut,
        associated_token::mint = msol_mint,
        associated_token::authority = pool_state
    )]
    pub pool_msol_account: Account<'info, TokenAccount>,

    /// CHECK: Marinade state account
    #[account(mut)]
    pub marinade_state: AccountInfo<'info>,

    #[account(mut)]
    pub msol_mint: Account<'info, Mint>,

    /// CHECK: Marinade liquidity pool sol leg
    #[account(mut)]
    pub liq_pool_sol_leg: AccountInfo<'info>,

    /// CHECK: Marinade liquidity pool msol leg
    #[account(mut)]
    pub liq_pool_msol_leg: AccountInfo<'info>,

    pub marinade_program: Program<'info, MarinadeFinance>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Unstake<'info> {
    /// Liquid-unstake mSOL -> SOL via Marinade CPI
    pub fn unstake(&mut self, amount_msol: u64) -> Result<()> {
        // PoolState PDA signs the CPI (it is authority over pool mSOL ATA)
        let pool_seeds = &[b"halopot".as_ref(), &[self.pool_state.bump]];
        let signer = &[&pool_seeds[..]];

        let cpi_accounts = LiquidUnstake {
            state: self.marinade_state.to_account_info(),
            msol_mint: self.msol_mint.to_account_info(),
            liq_pool_sol_leg_pda: self.liq_pool_sol_leg.to_account_info(),
            liq_pool_msol_leg: self.liq_pool_msol_leg.to_account_info(),
            treasury_msol_account: self.liq_pool_msol_leg.to_account_info(), // mSOL treasury
            get_msol_from: self.pool_msol_account.to_account_info(),
            get_msol_from_authority: self.pool_state.to_account_info(),
            transfer_sol_to: self.pool_state.to_account_info(),
            system_program: self.system_program.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            self.marinade_program.to_account_info(),
            cpi_accounts,
            signer
        );

        liquid_unstake(cpi_ctx, amount_msol)?;

        msg!("Liquid-unstaked {} mSOL for SOL", amount_msol);
        Ok(())
    }
}
