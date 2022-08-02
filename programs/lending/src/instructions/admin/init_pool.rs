use anchor_lang::prelude::*;
use anchor_spl::token::{Token, Mint, TokenAccount};

use crate::states::{AdminAccount, LendingPool, OracleAccount, NSlopeModel, PoolParams};
use crate::instructions::utils;

#[constant]
const CREDIT_MINT_DECIMALS: u8 = 9;

#[derive(Accounts)]
pub struct InitPool<'info> {

    // Make sure the authority signed
    #[account(has_one = authority)]
    pub admin: Account<'info, AdminAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        space = LendingPool::SIZE,
        payer = authority,
    )]
    pub pool: Box<Account<'info, LendingPool>>,

    pub n_slope_model: Account<'info, NSlopeModel>,

    pub base_oracle: Account<'info, OracleAccount>,

    pub base_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = authority,
        token::mint = base_mint,
        token::authority = admin, 
    )]
    pub treasury: Box<Account<'info, TokenAccount>>,

    #[account(
        init,
        payer = authority,
        token::mint = base_mint,
        token::authority = admin, 
    )]
    pub reserve: Box<Account<'info, TokenAccount>>,

    #[account(
        init,
        payer = authority,
        token::mint = base_mint,
        token::authority = admin, 
    )]
    pub fee_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        init,
        payer = authority,
        mint::authority = admin, 
        mint::decimals = CREDIT_MINT_DECIMALS
    )]
    pub credit_mint: Account<'info, Mint>,
    
    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub rent: Sysvar<'info, Rent>,

}

pub fn handler(ctx: Context<InitPool>, params: PoolParams) -> Result<()> {
    utils::validate_pool_params(&params)?;

    let pool = &mut ctx.accounts.pool;
    pool.treasury = ctx.accounts.treasury.key();
    pool.reserve = ctx.accounts.reserve.key();
    pool.fee_token_account = ctx.accounts.fee_token_account.key();
    pool.credit_mint = ctx.accounts.credit_mint.key();
    pool.base_oracle = ctx.accounts.base_oracle.key();
    pool.base_mint = ctx.accounts.base_mint.key();
    pool.interest_rate_model = ctx.accounts.n_slope_model.key();
    pool.params = params;
    Ok(())
}