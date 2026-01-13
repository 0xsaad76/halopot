use anchor_lang::{ prelude::*, system_program::{ self, Transfer } };
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ self, Mint, MintTo, Token, TokenAccount },
};

use crate::{ PoolState, states::Ticket };

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

    #[account(
        init, // a new ticket pda will be created for every 1 sol deposit
        payer = user,
        space = 8 + Ticket::INIT_SPACE,
        seeds = [b"ticket", pool_state.ticket_count.to_le_bytes().as_ref()],
        bump
    )]
    pub ticket: Account<'info, Ticket>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, amount: u64, ticket_bump: u8) -> Result<()> {
        const TICKET_PRICE: u64 = 1_000_000_000; // 1 SOL

        if amount != TICKET_PRICE {
            return err!(ErrorCode::IncorrectTicketPrice);
        }

        let ticket = &mut self.ticket;

        ticket.authority = self.user.key();
        ticket.id = self.pool_state.ticket_count;
        ticket.bump = ticket_bump;

        // increment the global pool_state ticket counter
        self.pool_state.ticket_count += 1;

        let sol_transfer_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.pool_state.to_account_info(),
        };

        let sol_transfer_cpi = CpiContext::new(
            self.system_program.to_account_info(),
            sol_transfer_accounts
        );

        system_program::transfer(sol_transfer_cpi, amount)?;

        // Mint Tickets from Program -> User
        // We need to sign with the PDA seeds because the PDA owns the Mint
        // PoolState is the mint authority, so sign with its PDA seeds
        let pool_seeds = &[b"halopot".as_ref(), &[self.pool_state.bump]];
        let signer = &[&pool_seeds[..]];

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

#[error_code]
enum ErrorCode {
    #[msg("Ticket price must be exactly 1 SOL")]
    IncorrectTicketPrice,
    // ...
}
