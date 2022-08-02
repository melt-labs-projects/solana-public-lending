use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};

use crate::states::{AdminAccount, LendingPool, NSlopeModel};
use crate::instructions::{logic, utils};

#[derive(Accounts)]
pub struct DepositTokens<'info> {

    // We need the admin account to mint credit tokens to the lender
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

    // The signer's base token account. Base tokens will be debited from here.
    // That will fail if the token account doesn't belong to the signer.
    #[account(mut)]
    pub base_token_account: Account<'info, TokenAccount>,

    // The token account to issue credit tokens to, doesn't necessarily have
    // to belong to the signer.
    #[account(mut)]
    pub credit_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,

}

impl<'info> DepositTokens<'info> {

    fn transfer_to_treasury(&self, amount: u64) -> Result<()> {
        utils::transfer(
            self.token_program.to_account_info(), 
            self.base_token_account.to_account_info(),
            self.treasury.to_account_info(),
            self.signer.to_account_info(),
            amount
        )
    }

    fn mint_credit_tokens(&self, amount: u64) -> Result<()> {
        utils::mint_to(
            self.token_program.to_account_info(), 
            self.credit_mint.to_account_info(),
            self.credit_token_account.to_account_info(),
            self.admin.to_account_info(),
            &self.admin,
            amount
        )
    }

}

pub fn handler(ctx: Context<DepositTokens>, amount: u64) -> Result<()> {

    // Transfer the lender's tokens to the treasury
    ctx.accounts.transfer_to_treasury(amount)?;

    let pool = &mut ctx.accounts.pool;
    let interest_rate_model = &ctx.accounts.interest_rate_model;

    // Update debt
    logic::debt::update(pool, interest_rate_model)?;

    // Update the internal state with the deposited tokens
    let shares = logic::deposit_tokens::execute(pool, amount)?;

    // Mint credit tokens to the user
    ctx.accounts.mint_credit_tokens(shares)?;

    Ok(())

}