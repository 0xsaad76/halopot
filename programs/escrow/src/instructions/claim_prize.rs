use anchor_lang::prelude::*;

use crate::{ PoolState, states::Ticket };

#[derive(Accounts)]
pub struct ClaimPrize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut,
    seeds=[b"halopot"],
    bump=pool_state.bump
    )]
    pub pool_state: Account<'info, PoolState>,

    #[account(
        seeds = [b"ticket", pool_state.winning_id.unwrap().to_le_bytes().as_ref()],
        bump = ticket.bump
    )]
    pub ticket: Account<'info, Ticket>,

    pub system_program: Program<'info, System>,
}

impl<'info> ClaimPrize<'info> {
    pub fn claim_prize(&mut self) -> Result<()> {
        let pool = &mut self.pool_state;
        let ticket = &self.ticket;

        // check: if we actually have the winner in the round
        require!(pool.winning_id.is_some(), ErrorCode::NoWinnerSelected);

        if ticket.id != pool.winning_id.unwrap() {
            return err!(ErrorCode::WrongTicket);
        }

        if self.user.key() != ticket.authority {
            return err!(ErrorCode::WrongWinnerAddress);
        }

        let total_principal = pool.total_principal;
        let total_balance = pool.to_account_info().try_lamports()?;

        let prize_amount = total_balance - total_principal;

        **pool.to_account_info().try_borrow_mut_lamports()? -= prize_amount;
        **self.user.to_account_info().try_borrow_mut_lamports()? += prize_amount;

        // reset the winner to none for the 2nd round
        pool.winning_id = None;

        Ok(())
    }
}

#[error_code]
enum ErrorCode {
    NoWinnerSelected,
    WrongTicket,
    WrongWinnerAddress,
}
