#![allow(unexpected_cfgs, deprecated)]
use anchor_lang::prelude::*;

pub mod instructions;
pub use instructions::*;

pub mod states;
pub use states::*;

declare_id!("FAB2f6QCzqNM5KW88ge7waUFTUzzF3xFALxMeNnstbcA");

#[program]
pub mod halopot {
    use super::*; // this basically brings all the definitions from above imports in the module body below

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)?;

        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount_sol: u64) -> Result<()> {
        ctx.accounts.deposit(amount_sol, ctx.bumps.ticket)?;

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount_sol: u64) -> Result<()> {
        ctx.accounts.withdraw(amount_sol)?;

        Ok(())
    }

    pub fn pick_winner(ctx: Context<PickWinner>) -> Result<()> {
        let winner_id = ctx.accounts.pick_winner()?;

        msg!("winner for the round {} is : {}", ctx.accounts.pool_state.round_id, winner_id);

        Ok(())
    }

    pub fn claim_prize(ctx: Context<ClaimPrize>) -> Result<()> {
        ctx.accounts.claim_prize()?;

        Ok(())
    }
}
