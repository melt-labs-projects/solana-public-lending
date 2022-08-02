use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Token};

use crate::events;
use crate::instructions::{utils, logic};
use crate::states::{LendingPool, BorrowingNode, BorrowAccount, NSlopeModel};

#[derive(Accounts)]
pub struct Repay<'info> {
    
    #[account(
        mut,
        has_one = treasury,
        has_one = interest_rate_model
    )]
    pub pool: Box<Account<'info, LendingPool>>,

    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>,
    
    pub interest_rate_model: Account<'info, NSlopeModel>,

    #[account(
        mut,
        seeds = [BorrowingNode::NAME, pool.key().as_ref(), node.collection.as_ref()],
        bump = node.bump
    )]
    pub node: Account<'info, BorrowingNode>,

    #[account(
        mut,
        seeds = [BorrowAccount::NAME, node.key().as_ref(), signer.key().as_ref()],
        bump = borrow_account.bump
    )]
    pub borrow_account: Box<Account<'info, BorrowAccount>>,

    pub signer: Signer<'info>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

impl<'info> Repay<'info> {

    pub fn transfer_to_treasury(&self, amount: u64) -> Result<()> {
        utils::transfer(
            self.token_program.to_account_info(),
            self.user_token_account.to_account_info(),
            self.treasury.to_account_info(),
            self.signer.to_account_info(),
            amount
        )
    }

}

pub fn handler(ctx: Context<Repay>, amount: u64) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let node = &mut ctx.accounts.node;
    let borrower = &mut ctx.accounts.borrow_account;
    let interest_rate_model = &ctx.accounts.interest_rate_model;
    
    logic::debt::update(pool, interest_rate_model)?;

    let repayment = logic::repay::execute(pool, node, borrower, amount)?;
    
    // Transfer the user's repayment to the treasury
    ctx.accounts.transfer_to_treasury(repayment)?;

    events::emit_repay_event(
        ctx.accounts.pool.key(),
        ctx.accounts.node.collection,
        ctx.accounts.signer.key(),
        amount
    )?;

    Ok(())
}