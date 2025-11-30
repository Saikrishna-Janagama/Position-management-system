use anchor_lang::prelude::*;

declare_id!("6NnMPpT6i5SmsV1cUSjDwUx5QxGKsUSTEeVzngCEm1Ma");

pub mod instructions;
pub mod state;
pub mod utils;
pub mod errors;

use instructions::*;

#[program]
pub mod position_management {
    use super::*;

    pub fn initialize_user(
        ctx: Context<InitializeUser>
    ) -> Result<()> {
        instructions::initialize_user::handler(ctx)
    }

    pub fn open_position(
        ctx: Context<OpenPosition>,
        symbol: String,
        side: u8,
        size: u64,
        leverage: u16,
        entry_price: u64,
    ) -> Result<()> {
        instructions::open_position::handler(
            ctx, symbol, side, size, leverage, entry_price
        )
    }

    pub fn modify_position(
        ctx: Context<ModifyPosition>,
        size_delta: i64,
        margin_delta: i64,
    ) -> Result<()> {
        instructions::modify_position::handler(ctx, size_delta, margin_delta)
    }

    pub fn close_position(
        ctx: Context<ClosePosition>,
        exit_price: u64,
    ) -> Result<()> {
        instructions::close_position::handler(ctx, exit_price)
    }

    pub fn liquidate_position(
        ctx: Context<LiquidatePosition>,
    ) -> Result<()> {
        instructions::liquidate_position::handler(ctx)
    }
}