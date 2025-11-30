use anchor_lang::prelude::*;
use crate::state::UserAccount;

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = UserAccount::LEN,
        seeds = [b"user", owner.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeUser>) -> Result<()> {
    let user = &mut ctx.accounts.user_account;
    let owner = &ctx.accounts.owner;

    user.owner = owner.key();
    user.bump = ctx.bumps.user_account;
    user.total_collateral = 0;
    user.locked_collateral = 0;
    user.position_count = 0;
    user.total_pnl = 0;
    user.created_at = Clock::get()?.unix_timestamp;
    user.last_activity = Clock::get()?.unix_timestamp;

    msg!("User initialized");
    Ok(())
}