#![allow(unexpected_cfgs, deprecated)]
use anchor_lang::prelude::*;

pub mod instructions;
pub use instructions::*;

pub mod states;
// pub use states::*;

declare_id!("D4KtPYLxqfVjdwt3JA2esEnDRkiFpcVHWYTSbuUAPzhQ");

#[program]
pub mod escrow {
    use super::*; // this basically brings all the definitions from above imports in the module body below

    pub fn make(ctx: Context<Make>, seed: u64, deposit: u64, receive: u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, receive, &ctx.bumps)?;
        ctx.accounts.deposit(deposit)?;

        Ok(())
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund_and_close_vault()
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.deposit()?;
        ctx.accounts.withdraw_and_close_vault()
    }
}

#[derive(Accounts)]
pub struct Initialize {}
