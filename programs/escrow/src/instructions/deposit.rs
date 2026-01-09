use anchor_lang::{ prelude::*, system_program::{ self, Transfer } };
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ self, Mint, MintTo, Token, TokenAccount },
};

use crate::PoolState;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut, seeds=[b"halopot"], bump = pool_state.bump)]
    pub pool_state: Account<'info, PoolState>,

    #[account(mut, seeds=[b"ticket_mint"], bump = pool_state.ticket_mint_bump)]
    pub ticket_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = ticket_mint,
        associated_token::authority = user
    )]
    pub user_ticket_ata: Account<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let sol_transfer_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.pool_state.to_account_info(),
        };

        let sol_transfer_cpi = CpiContext::new(
            self.system_program.to_account_info(),
            sol_transfer_accounts
        );

        system_program::transfer(sol_transfer_cpi, amount)?;

        // Step B: Mint Tickets from Program -> User
        // We need to sign with the PDA seeds because the PDA owns the Mint
        let mint_ticket_seeds = &[b"ticket_mint".as_ref(), &[self.pool_state.ticket_mint_bump]];
        let signer = &[&mint_ticket_seeds[..]];

        let mint_accounts = MintTo {
            authority: self.pool_state.to_account_info(),
            mint: self.ticket_mint.to_account_info(),
            to: self.user_ticket_ata.to_account_info(),
        };

        let mint_cpi_context = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            mint_accounts,
            signer
        );

        token::mint_to(mint_cpi_context, amount)?;

        let pool = &mut self.pool_state;
        pool.total_principal += amount;
        pool.total_tickets += amount;

        msg!("Deposited {} lamports to user", amount);

        Ok(())
    }
}
