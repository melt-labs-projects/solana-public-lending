use anchor_lang::prelude::*;

use crate::states::{AdminAccount, MembershipAccount, LiquidationPerks};

#[derive(Accounts)]
pub struct UpdateMembership<'info> {

    #[account(
        seeds = [AdminAccount::NAME],
        bump = admin.bump,
        has_one = authority
    )]
    pub admin: Account<'info, AdminAccount>,

    #[account(mut)]
    pub membership: Account<'info, MembershipAccount>,

    pub authority: Signer<'info>,

}
    
    
pub fn handler(ctx: Context<UpdateMembership>, perks: LiquidationPerks) -> Result<()> {
    ctx.accounts.membership.liquidation_perks = perks;
    Ok(())
}
