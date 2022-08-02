use anchor_lang::prelude::*;

use crate::states::LockingAccount;

#[derive(Accounts)]
pub struct LiquidationLog<'info> {

    #[account(
        mut,
        seeds = [LockingAccount::NAME, locker.key().as_ref()],
        bump = locking_account.bump,
    )]
    pub locking_account: Account<'info, LockingAccount>,

    pub locker: Signer<'info>,
    
}


pub fn handler(ctx: Context<LiquidationLog>) -> Result<()> {
    let locking = &mut ctx.accounts.locking_account;
    if locking.is_cube_locked() {
        locking.liquidation_data.uses += 1;
    }
    Ok(())
}
