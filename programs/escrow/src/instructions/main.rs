use anchor_lang::prelude::*;

use anchor_spl::{ token::{ Mint, Token } };

pub use crate::states::PoolState;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(init, payer = admin, space = 8 + PoolState::INIT_SPACE, seeds = [b"halopot"], bump)]
    pub pool_state: Account<'info, PoolState>,

    #[account(
        init,
        payer = admin,
        seeds = [b"ticket_mint"],
        bump,
        mint::decimals = 9,
        mint::authority = pool_state.key()
        // mint::token_program = token_program
    )]
    pub ticket_mint: Account<'info, Mint>,

    // #[account(init, payer = admin, space = 8)]
    // pub pool_vault: Account<'info, TokenAccount>,

    // #[account(
    //     mut,
    //     associated_token::mint=ticket_mint,
    //     associated_token::authority = admin,
    //     associated_token::token_program=token_program,
    // )]
    // pub signer_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(
        &mut self,
        bumps: &InitializeBumps // on writing this : Anchor generates this struct automatically for you to access the bumps calculated in the #[derive(Accounts)].
    ) -> Result<()> {
        self.pool_state.set_inner(PoolState {
            admin: self.admin.key(),

            round_id: 1,
            total_principal: 0,
            total_tickets: 0,

            bump: bumps.pool_state,
            ticket_mint_bump: bumps.ticket_mint,
        });
        msg!("HaloPot Initialized !");
        Ok(())
    }
}
