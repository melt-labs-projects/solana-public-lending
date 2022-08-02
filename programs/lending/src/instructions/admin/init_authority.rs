use anchor_lang::prelude::*;

use crate::states::AdminAccount;

#[derive(Accounts)]
pub struct InitAuthority<'info> {

    #[account(
        init,
        space = AdminAccount::SIZE,
        payer = authority,
        seeds = [AdminAccount::NAME],
        bump
    )]
    pub admin: Account<'info, AdminAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,

}

pub fn handler(ctx: Context<InitAuthority>) -> Result<()> {
    ctx.accounts.admin.authority = ctx.accounts.authority.key();
    ctx.accounts.admin.bump = *ctx.bumps.get("admin").unwrap();
    Ok(())
}