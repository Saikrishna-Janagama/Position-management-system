use anchor_lang::prelude::*;
use crate::state::{Position, UserAccount};
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct OpenPosition<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        init,
        payer = owner,
        space = Position::LEN,
        seeds = [b"position", owner.key().as_ref(), &user_account.position_count.to_le_bytes()],
        bump
    )]
    pub position: Account<'info, Position>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<OpenPosition>,
    symbol: String,
    side: u8,
    size: u64,
    leverage: u16,
    entry_price: u64,
) -> Result<()> {
    let position = &mut ctx.accounts.position;
    let user = &mut ctx.accounts.user_account;
    let owner = &ctx.accounts.owner;

    require!(size > 0, ErrorCode::InvalidPositionSize);
    require!(leverage >= 1 && leverage <= 100, ErrorCode::InvalidLeverageValue);
    require_neq!(entry_price, 0, ErrorCode::InvalidPrice);

    let initial_margin = entry_price.checked_div(leverage as u64).ok_or(ErrorCode::CalculationUnderflow)?;

    let mut sym_bytes = [0u8; 16];
    let s = symbol.as_bytes();
    let len = s.len().min(16);
    sym_bytes[..len].copy_from_slice(&s[..len]);

    position.owner = owner.key();
    position.symbol = sym_bytes;
    position.side = side;
    position.size = size;
    position.entry_price = entry_price;
    position.leverage = leverage;
    position.margin = initial_margin;
    position.status = 1;
    position.opened_at = Clock::get()?.unix_timestamp;
    position.liquidation_price = if side == 1 { entry_price * 90 / 100 } else { entry_price * 110 / 100 };
    position.bump = ctx.bumps.position;

    user.locked_collateral = user.locked_collateral.checked_add(initial_margin).ok_or(ErrorCode::CalculationOverflow)?;
    user.position_count = user.position_count.checked_add(1).ok_or(ErrorCode::CalculationOverflow)?;
    user.last_activity = Clock::get()?.unix_timestamp;

    msg!("Position opened");
    Ok(())
}