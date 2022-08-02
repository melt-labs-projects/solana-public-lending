use anchor_lang::prelude::*;

use crate::states::{AdminAccount, LendingPool, OracleAccount, NSlopeModel, PoolParams};
use crate::instructions::utils;

#[derive(Accounts)]
pub struct UpdatePool<'info> {

    // Make sure the authority signed
    #[account(has_one = authority)]
    pub admin: Account<'info, AdminAccount>,

    pub authority: Signer<'info>,

    #[account(mut)]
    pub pool: Box<Account<'info, LendingPool>>,

    pub n_slope_model: Account<'info, NSlopeModel>,

    pub base_oracle: Account<'info, OracleAccount>,

}

pub fn handler(ctx: Context<UpdatePool>, params: PoolParams) -> Result<()> {
    utils::validate_pool_params(&params)?;

    let pool = &mut ctx.accounts.pool;
    pool.base_oracle = ctx.accounts.base_oracle.key();
    pool.interest_rate_model = ctx.accounts.n_slope_model.key();
    pool.params = params;
    Ok(())
}