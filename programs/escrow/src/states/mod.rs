use anchor_lang::{ prelude::* };

#[account]
#[derive(InitSpace)]
pub struct PoolState {
    pub admin: Pubkey,
    pub round_id: u64,
    pub total_principal: u64,
    pub total_tickets: u64, // total halo tokens minted
    pub bump: u8,
    // ticket vars
    pub ticket_mint_bump: u8,
    pub ticket_count: u64,
    pub winning_id: Option<u64>,
}

#[account]
#[derive(InitSpace)]
pub struct Ticket {
    pub authority: Pubkey,
    pub id: u64,
    pub bump: u8,
}
