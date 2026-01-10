use anchor_lang::prelude::*;

use crate::PoolState;

#[derive(Accounts)]
pub struct PickWinner<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds=[b"halopot"],
        bump= pool_state.bump,
        has_one=admin
    )]
    pub pool_state: Account<'info, PoolState>,

    /// the winner account is only used to receive SOL via transfer and does not need further constraints.
    #[account(mut)]
    pub winner: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> PickWinner<'info> {
    pub fn pick_winner(&mut self) -> Result<u64> {
        let pool = &mut self.pool_state;

        let current_balance = pool.to_account_info().lamports();
        let total_principal = pool.total_principal;

        // checking yeild in pool
        if current_balance <= total_principal {
            return err!(ErrorCode::NoYieldToDistribute);
        }

        let clock = Clock::get()?;
        let time_seed = clock.unix_timestamp as u64;

        // the Math: Random % TotalTickets
        // example: If time is 12345 and tickets are 100. Winner is #45.
        let random_winner_id = time_seed % pool.ticket_count;

        // saving winner in winning id
        pool.winning_id = Some(random_winner_id);

        msg!("The Winning Ticket ID is: {}", random_winner_id);
        Ok(random_winner_id)
    }
}

#[error_code]
enum ErrorCode {
    #[msg("No Yeild generated yet!")]
    NoYieldToDistribute,
}
