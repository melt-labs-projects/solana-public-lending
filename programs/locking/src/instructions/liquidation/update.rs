use anchor_lang::prelude::*;


use crate::states::{LockingAccount, MembershipAccount};
use crate::instructions::utils;

#[derive(Accounts)]
pub struct LiquidationUpdate<'info> {

    #[account(mut)]
    pub locking_account: Account<'info, LockingAccount>,

    pub membership: Account<'info, MembershipAccount>,

}
    
pub fn handler(ctx: Context<LiquidationUpdate>) -> Result<()> {

    let locking = &mut ctx.accounts.locking_account;
    locking.liquidation_data.discount = utils::DEFAULT_DISCOUNT;

    // If the locking account doesn't have a locked cube then return
    if !locking.is_cube_locked() { return Ok(()) };

    // Make sure the membership account is associated with the locking account
    require_keys_eq!(locking.membership, ctx.accounts.membership.key());

    let perks = &ctx.accounts.membership.liquidation_perks;
    let current_timestamp = utils::current_timestamp()?;

    // Check if the liquidation perks need to be activated
    if !locking.liquidation_data.active {
        let time_since_locked = current_timestamp - locking.locked_timestamp;
        if time_since_locked >= perks.warmup_period {
            locking.liquidation_data.active = true;
        }
    }

    if locking.liquidation_data.active {

        // Reset uses if enough times has passed since the last update
        let time_since_update = current_timestamp - locking.liquidation_data.last_updated_timestamp;
        if time_since_update >= perks.refresh_period {
            locking.liquidation_data.uses = 0;
            locking.liquidation_data.last_updated_timestamp = current_timestamp; 
        }

        // Set the locker's current discount
        if locking.liquidation_data.uses < perks.max_uses {
            locking.liquidation_data.discount = perks.discount;
        }

    }

    Ok(())
}
