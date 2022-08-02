use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use crate::states::{AdminAccount, CubeAccount, MembershipAccount};

#[derive(Accounts)]
pub struct InitCube<'info> {

    #[account(
        mut,
        seeds = [AdminAccount::NAME],
        bump = admin.bump,
        has_one = authority
    )]
    pub admin: Account<'info, AdminAccount>,

    #[account(
        init,
        space = CubeAccount::SIZE,
        payer = authority,
        seeds = [CubeAccount::NAME, cube_mint.key().as_ref()],
        bump
    )]
    pub cube: Account<'info, CubeAccount>,

    #[account(
        seeds = [MembershipAccount::NAME, membership.number.to_le_bytes().as_ref()],
        bump = membership.bump
    )]
    pub membership: Account<'info, MembershipAccount>,

    pub cube_mint: Account<'info, Mint>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,

}
    
pub fn handler(ctx: Context<InitCube>) -> Result<()> {
    let cube = &mut ctx.accounts.cube;
    cube.bump = *ctx.bumps.get("cube").unwrap();
    cube.membership = ctx.accounts.membership.key();
    cube.mint = ctx.accounts.cube_mint.key();
    Ok(())
}
