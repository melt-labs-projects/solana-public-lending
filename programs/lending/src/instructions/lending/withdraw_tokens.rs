use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};

use crate::errors::LendingError;
use crate::instructions::{utils, logic};
use crate::states::{AdminAccount, LendingPool, NSlopeModel};

#[derive(Accounts)]
pub struct WithdrawTokens<'info> {

    pub admin: Account<'info, AdminAccount>,

    #[account(
        mut,
        has_one = treasury,
        has_one = credit_mint,
        has_one = interest_rate_model
    )]
    pub pool: Box<Account<'info, LendingPool>>,
    
    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>,

    #[account(mut)]
    pub credit_mint: Account<'info, Mint>,

    pub interest_rate_model: Account<'info, NSlopeModel>,

    pub signer: Signer<'info>,

    // The token account to return base tokens to, doesn't necessarily have
    // to belong to the signer.
    #[account(mut)]
    pub base_token_account: Account<'info, TokenAccount>,

    // The signer's credit token account. Credit tokens will be burned from here.
    // That will fail if the token account doesn't belong to the signer.
    #[account(mut)]
    pub credit_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,

}

impl<'info> WithdrawTokens<'info> {

    fn transfer_from_treasury(&self, amount: u64) -> Result<()> {
        utils::transfer_signed(
            self.token_program.to_account_info(),
            self.treasury.to_account_info(),
            self.base_token_account.to_account_info(),
            self.admin.to_account_info(),
            &self.admin,
            amount
        )
    }

    fn burn_credit_tokens(&self, amount: u64) -> Result<()> {
        utils::burn(
            self.token_program.to_account_info(),
            self.credit_mint.to_account_info(),
            self.credit_token_account.to_account_info(),
            self.signer.to_account_info(),
            amount
        )
    }

}

pub fn handler(ctx: Context<WithdrawTokens>, amount: u64) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let interest_rate_model = &ctx.accounts.interest_rate_model;

    require!(amount <= pool.available_tokens, LendingError::UtilisationTooHigh);

    // Update debt
    logic::debt::update(pool, interest_rate_model)?;

    // Calculate the number of shares to burn and update internal state
    let total_credit_shares = ctx.accounts.credit_token_account.amount;
    let shares_to_burn = logic::withdraw_tokens::execute(pool, amount, total_credit_shares)?;

    // Burn credit tokens from the user's token account
    ctx.accounts.burn_credit_tokens(shares_to_burn)?;

    // Transfer base tokens back to the user
    ctx.accounts.transfer_from_treasury(amount)?;
    
    Ok(())
}