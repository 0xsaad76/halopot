use anchor_lang::{ prelude::* };
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ self, Burn, Mint, Token, TokenAccount },
};

use crate::PoolState;

#[derive(Accounts)]
// #[instruction()]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut, seeds = [b"halopot"], bump = pool_state.bump)]
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

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

// withdrawal required accounts
// signer/user, pda signer, pool_state, user ata, system_program, token_program, associated_token

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self, amount_sol: u64) -> Result<()> {
        let burn_accounts = Burn {
            authority: self.user.to_account_info(),
            from: self.user_ticket_ata.to_account_info(),
            mint: self.ticket_mint.to_account_info(),
        };

        let burn_cpi_context = CpiContext::new(self.token_program.to_account_info(), burn_accounts);

        token::burn(burn_cpi_context, amount_sol)?;

        // here we cant transfer the old fasioned cpi_context way because the pda(sender) is not owned by system_program rather we just send it ourselves from pda account
        **self.pool_state.to_account_info().try_borrow_mut_lamports()? -= amount_sol;

        // increasing User Balance
        **self.user.to_account_info().try_borrow_mut_lamports()? += amount_sol;

        // updating the pool_state
        self.pool_state.total_principal -= amount_sol;
        self.pool_state.total_tickets -= amount_sol;

        msg!("Token Burned Succesfully !");

        Ok(())
    }
}

#[error_code]
pub enum TokenError {
    InsufficientBalance,
}
