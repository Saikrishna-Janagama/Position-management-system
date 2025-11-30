use anchor_lang::prelude::*;
use crate::state::{Position, UserAccount};
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct ClosePosition<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,

    #[account(mut)]
    pub position: Account<'info, Position>,
}

pub fn handler(
    ctx: Context<ClosePosition>,
    exit_price: u64,
) -> Result<()> {
    let position = &mut ctx.accounts.position;
    let user = &mut ctx.accounts.user_account;

    require_eq!(position.status, 1, ErrorCode::PositionAlreadyClosed);
    require_neq!(exit_price, 0, ErrorCode::InvalidPrice);

    let is_long = position.side == 1;
    let pnl = if is_long {
        ((exit_price as i64) - (position.entry_price as i64)) * (position.size as i64)
    } else {
        ((position.entry_price as i64) - (exit_price as i64)) * (position.size as i64)
    };

    position.status = 2;
    position.close_price = exit_price;
    position.realized_pnl = pnl;
    position.closed_at = Clock::get()?.unix_timestamp;

    user.locked_collateral = user.locked_collateral.checked_sub(position.margin).ok_or(ErrorCode::CalculationUnderflow)?;
    user.total_pnl = (user.total_pnl as i128).checked_add(pnl as i128).ok_or(ErrorCode::CalculationOverflow)? as i64;
    user.position_count = user.position_count.checked_sub(1).ok_or(ErrorCode::CalculationUnderflow)?;
    user.last_activity = Clock::get()?.unix_timestamp;

    msg!("Position closed");
    Ok(())
}