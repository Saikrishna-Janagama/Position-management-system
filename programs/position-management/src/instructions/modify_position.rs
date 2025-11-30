use anchor_lang::prelude::*;
use crate::state::{Position, UserAccount};
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct ModifyPosition<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,

    #[account(mut)]
    pub position: Account<'info, Position>,
}

pub fn handler(
    ctx: Context<ModifyPosition>,
    size_delta: i64,
    margin_delta: i64,
) -> Result<()> {
    let position = &mut ctx.accounts.position;
    let user = &mut ctx.accounts.user_account;

    require_eq!(position.status, 1, ErrorCode::PositionAlreadyClosed);

    if size_delta != 0 {
        let new_size = if size_delta > 0 {
            position.size.checked_add(size_delta as u64).ok_or(ErrorCode::CalculationOverflow)?
        } else {
            position.size.checked_sub((-size_delta) as u64).ok_or(ErrorCode::CalculationUnderflow)?
        };
        position.size = new_size;
    }

    if margin_delta != 0 {
        if margin_delta > 0 {
            position.margin = position.margin.checked_add(margin_delta as u64).ok_or(ErrorCode::CalculationOverflow)?;
            user.locked_collateral = user.locked_collateral.checked_add(margin_delta as u64).ok_or(ErrorCode::CalculationOverflow)?;
        } else {
            position.margin = position.margin.checked_sub((-margin_delta) as u64).ok_or(ErrorCode::CannotReduceMargin)?;
            user.locked_collateral = user.locked_collateral.checked_sub((-margin_delta) as u64).ok_or(ErrorCode::CalculationUnderflow)?;
        }
    }

    msg!("Position modified");
    Ok(())
}