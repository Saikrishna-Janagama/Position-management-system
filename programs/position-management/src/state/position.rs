use anchor_lang::prelude::*;

#[account]
pub struct Position {
    pub owner: Pubkey,
    pub symbol: [u8; 16],
    pub bump: u8,
    pub side: u8,
    pub size: u64,
    pub entry_price: u64,
    pub leverage: u16,
    pub status: u8,
    pub margin: u64,
    pub liquidation_price: u64,
    pub mark_price: u64,
    pub unrealized_pnl: i64,
    pub realized_pnl: i64,
    pub opened_at: i64,
    pub closed_at: i64,
    pub close_price: u64,
}

impl Position {
    pub const LEN: usize = 8 + 32 + 16 + 1 + 1 + 8 + 8 + 2 + 1 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8;
}