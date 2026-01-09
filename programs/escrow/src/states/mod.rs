use anchor_lang::{ prelude::* };

#[account]
#[derive(InitSpace)]
pub struct PoolState {
    pub admin: Pubkey,
    pub round_id: u64,
    pub total_principal: u64,
    pub total_tickets: u64, // total halo tokens minted
    pub bump: u8,
    // Add this to help the Mint logic later!
    pub ticket_mint_bump: u8,
}
