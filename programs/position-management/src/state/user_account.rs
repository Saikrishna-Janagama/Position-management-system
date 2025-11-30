use anchor_lang::prelude::*;

#[account]
pub struct UserAccount {
    pub owner: Pubkey,
    pub bump: u8,
    pub total_collateral: u64,
    pub locked_collateral: u64,
    pub position_count: u32,
    pub total_pnl: i64,
    pub created_at: i64,
    pub last_activity: i64,
}

impl UserAccount {
    pub const LEN: usize = 8 + 32 + 1 + 8 + 8 + 4 + 8 + 8 + 8;
}