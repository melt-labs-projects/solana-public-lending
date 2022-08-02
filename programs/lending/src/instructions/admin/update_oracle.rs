use anchor_lang::prelude::*;

use crate::instructions::utils;
use crate::states::{AdminAccount, OracleAccount};

#[derive(Accounts)]
pub struct UpdateOracle<'info> {

    // Make sure the authority signed
    #[account(has_one = authority)]
    pub admin: Account<'info, AdminAccount>,

    pub authority: Signer<'info>,

    #[account(mut)]
    pub oracle: Account<'info, OracleAccount>,

}

pub fn handler(ctx: Context<UpdateOracle>, price: u64) -> Result<()> {
    ctx.accounts.oracle.price = price;
    ctx.accounts.oracle.timestamp = utils::current_timestamp()?;
    Ok(())
}