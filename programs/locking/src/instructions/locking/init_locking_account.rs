use anchor_lang::prelude::*;

use crate::states::LockingAccount;

#[derive(Accounts)]
pub struct InitLockAccount<'info> {

    #[account(
        init,
        payer = signer,
        space = LockingAccount::SIZE,
        seeds = [LockingAccount::NAME, signer.key().as_ref()],
        bump
    )]
    pub locking_account: Account<'info, LockingAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitLockAccount>) -> Result<()> {
    ctx.accounts.locking_account.bump = *ctx.bumps.get("locking_account").unwrap();
    Ok(())
}
