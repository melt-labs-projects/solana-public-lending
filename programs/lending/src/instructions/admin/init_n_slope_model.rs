use anchor_lang::prelude::*;

use crate::states::{AdminAccount, NSlopeModel};

#[derive(Accounts)]
#[instruction(_length: u64)]
pub struct InitNSlopeModel<'info> {

    // Make sure the authority signed
    #[account(has_one = authority)]
    pub admin: Account<'info, AdminAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        space = NSlopeModel::space(_length),
        payer = authority,
    )]
    pub model: Account<'info, NSlopeModel>,

    pub system_program: Program<'info, System>,

}

pub fn handler(ctx: Context<InitNSlopeModel>, slopes: Vec<[u64; 3]>) -> Result<()> {
    ctx.accounts.model.slopes = slopes;
    Ok(())
}