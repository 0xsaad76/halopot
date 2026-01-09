use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{ transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked }, // functions and structs that are used when interacting with token program
};

use crate::states::Escrow;

#[derive(Accounts)] // this validates the accounts that we pass below.
#[instruction(seed: u64)] // this gets in the seed values from the user when calling the make instruction
pub struct Make<'info> {
    #[account(mut)] // mutable because we need to deduct some lamports for acccount fees
    pub maker: Signer<'info>,
    #[account(mint::token_program = token_program)]
    pub mint_a: InterfaceAccount<'info, Mint>, // The InterfaceAccount type is a wrapper that allows the account to work with both the Token Program and Token Extension Program.
    #[account(mint::token_program = token_program)]
    pub mint_b: InterfaceAccount<'info, Mint>, // the account type here is Mint which acts as the mint account for the token_progarm specified
    #[account(
        mut, 
        associated_token::mint=mint_a,
        associated_token::authority = maker,
        associated_token::token_program=token_program,
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,
    // escrow account
    #[account(
        init,
        payer = maker,
        space = 8 + Escrow::INIT_SPACE,
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,
    // vault account which hol// DOUBTS
    // 1. what exactly is the Mint ?  in general sense it is a mintting a token to lets say 1000million supply. why is there a Mint type and a TokenAccount type what is the difference ?
    // 2. what is the purpose of InterfaceAccount as the type to the mint token program ? is it only a type or does it actually create the mint account for the seller's token ?
    // 3. what happens when we pass the account macro with the argument mint::token_program = token_program
    // 4. for the variable maker_ata_a is it determining where could be this ata of lets say solana of user a be by passing the necccesary fields like mint address, authority and token_program ?
    // 5. why are we passing the associated token program what is the use and why exactly ?
    // 6. why passing the token_program and why the types are interface and tokenInterface ? ds the user's tokens from the escrow account
    #[account(
        init,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>, // WE need this program to create (init) the Vault account that is passing the associated_token as an argument, the associated_token_program will do the heavy lifting in creating the finding the appropriate associated token accounts
    // simple explanatoin for associated_token_program : The logic to calculate the address of an ATA and create it lives inside the Associated Token Program. Your contract calls this program to do the heavy lifting of creating the vault.
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Make<'info> {
    pub fn init_escrow(&mut self, seed: u64, receive: u64, bumps: &MakeBumps) -> Result<()> {
        self.escrow.set_inner(Escrow {
            seeds: seed,
            receive,
            maker: self.maker.key(),
            mint_a: self.mint_a.key(),
            mint_b: self.mint_b.key(),
            bump: bumps.escrow,
        });
        Ok(())
    }

    pub fn deposit(&mut self, deposit: u64) -> Result<()> {
        let transfer_accounts = TransferChecked {
            authority: self.maker.to_account_info(),
            mint: self.mint_a.to_account_info(),
            from: self.maker_ata_a.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), transfer_accounts);

        transfer_checked(cpi_ctx, deposit, self.mint_a.decimals)?;

        Ok(())
    }
}

// DOUBTS
// 1. what exactly is the Mint ?  in general sense it is a mintting a token to lets say 1000million supply. why is there a Mint type and a TokenAccount type what is the difference ?
// 2. what is the purpose of InterfaceAccount as the type to the mint token program ? is it only a type or does it actually create the mint account for the seller's token ?
// 3. what happens when we pass the account macro with the argument mint::token_program = token_program
// 4. for the variable maker_ata_a is it determining where could be this ata of lets say solana of user a be by passing the necccesary fields like mint address, authority and token_program ?
// 5. why are we passing the associated token program what is the use and why exactly ?
// 6. why passing the token_program and why the types are interface and tokenInterface ?

// DOUBT : when passing the 'Mint' as the type in the InterfaceAccount, will this create a new tokken sjadfjklasjdflsadlkf
