use anchor_lang::prelude::*;

use crate::states::AdminAccount;

#[derive(Accounts)]
pub struct UpdateAuthority<'info> {

    // Make sure the current authority signed
    #[account(mut, has_one = authority)]
    pub admin: Account<'info, AdminAccount>,

    pub authority: Signer<'info>,

}

pub fn handler(ctx: Context<UpdateAuthority>, new_authority: Pubkey) -> Result<()> {
    ctx.accounts.admin.authority = new_authority;
    Ok(())
}