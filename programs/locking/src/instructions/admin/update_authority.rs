use anchor_lang::prelude::*;

use crate::states::AdminAccount;

#[derive(Accounts)]
pub struct UpdateAuthority<'info> {

    #[account(
        mut,
        seeds = [AdminAccount::NAME],
        bump = admin.bump,
        has_one = authority
    )]
    pub admin: Account<'info, AdminAccount>,

    pub authority: Signer<'info>,

}
    
    
pub fn handler(ctx: Context<UpdateAuthority>, authority: Pubkey) -> Result<()> {
    ctx.accounts.admin.authority = authority;
    Ok(())
}
