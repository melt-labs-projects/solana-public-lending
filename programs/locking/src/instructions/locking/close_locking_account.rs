use anchor_lang::prelude::*;

use crate::errors::LockingError;
use crate::states::LockingAccount;

#[derive(Accounts)]
pub struct CloseLockAccount<'info> {

    #[account(
        mut,
        seeds = [LockingAccount::NAME, signer.key().as_ref()],
        bump = locking_account.bump,
        close = receiver,
    )]
    pub locking_account: Account<'info, LockingAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub receiver: SystemAccount<'info>,

}

pub fn handler(ctx: Context<CloseLockAccount>) -> Result<()> {
    let locking = &mut ctx.accounts.locking_account;
    require!(!locking.is_cube_locked(), LockingError::CannotCloseLocking);
    Ok(())
}
