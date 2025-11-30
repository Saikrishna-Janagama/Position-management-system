use anchor_lang::prelude::*;
use crate::state::{Position, UserAccount};
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct LiquidatePosition<'info> {
    #[account(mut)]
    pub liquidator: Signer<'info>,

    #[account(mut)]
    pub position: Account<'info, Position>,

    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
}

pub fn handler(
    ctx: Context<LiquidatePosition>,
) -> Result<()> {
    let position = &mut ctx.accounts.position;
    let user = &mut ctx.accounts.user_account;

    require_eq!(position.status, 1, ErrorCode::PositionAlreadyClosed);

    position.status = 3;
    position.closed_at = Clock::get()?.unix_timestamp;

    user.locked_collateral = user.locked_collateral.checked_sub(position.margin).ok_or(ErrorCode::CalculationUnderflow)?;
    user.position_count = user.position_count.checked_sub(1).ok_or(ErrorCode::CalculationUnderflow)?;

    msg!("Position liquidated");
    Ok(())
}