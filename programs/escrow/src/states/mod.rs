use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub seeds: u64,
    pub maker: Pubkey, // public key of the owner that wants to create the escrow
    pub mint_a: Pubkey, // public key of the token that the owner wants to sell
    pub mint_b: Pubkey, // public key of the token that the owner wants in return of mint_a
    pub receive: u64, // the amount of mint_b tokens expected in return of mint_a tokens
    pub bump: u8,
    // NOTE : notice we didnt declare the variable to store the amount of mint_a that owner wants to sell just like mint_b tokens in "receive"
    // thats because the vault account will eventually store that information for us
}
