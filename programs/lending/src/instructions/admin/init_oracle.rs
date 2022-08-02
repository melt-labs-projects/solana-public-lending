use anchor_lang::prelude::*;

use crate::instructions::utils;
use crate::states::{AdminAccount, OracleAccount};

#[derive(Accounts)]
pub struct InitOracle<'info> {

    // Make sure the authority signed
    #[account(has_one = authority)]
    pub admin: Account<'info, AdminAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        space = OracleAccount::SIZE,
        payer = authority,
    )]
    pub oracle: Account<'info, OracleAccount>,

    pub system_program: Program<'info, System>,

}

pub fn handler(ctx: Context<InitOracle>, decimals: u64, price: u64) -> Result<()> {
    ctx.accounts.oracle.decimals = decimals;
    ctx.accounts.oracle.price = price;
    ctx.accounts.oracle.timestamp = utils::current_timestamp()?;
    Ok(())
}