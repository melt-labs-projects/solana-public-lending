use anchor_lang::prelude::*;

use crate::states::{AdminAccount, MembershipAccount, LiquidationPerks};

#[derive(Accounts)]
#[instruction(membership_number: u64)]
pub struct InitMembership<'info> {

    #[account(
        mut,
        seeds = [AdminAccount::NAME],
        bump = admin.bump,
        has_one = authority
    )]
    pub admin: Account<'info, AdminAccount>,

    #[account(
        init,
        space = MembershipAccount::SIZE,
        payer = authority,
        seeds = [MembershipAccount::NAME, membership_number.to_le_bytes().as_ref()],
        bump
    )]
    pub membership: Account<'info, MembershipAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,

}
    
pub fn handler(ctx: Context<InitMembership>, membership_number: u64, perks: LiquidationPerks) -> Result<()> {
    ctx.accounts.membership.bump = *ctx.bumps.get("membership").unwrap();
    ctx.accounts.membership.number = membership_number;
    ctx.accounts.membership.liquidation_perks = perks;
    Ok(())
}
