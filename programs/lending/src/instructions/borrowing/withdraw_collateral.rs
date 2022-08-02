use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::instructions::{utils, logic};
use crate::states::{
    AdminAccount,
    LendingPool,
    BorrowingNode,
    BorrowAccount,
    CollateralAccount,
    OracleAccount,
    CollectionAccount,
    NSlopeModel,
    CollateralList,
};

#[derive(Accounts)]
pub struct WithdrawCollateral<'info> {

    pub admin: Account<'info, AdminAccount>,

    // Make sure the base oracle and the interest rate model are the ones
    // associated with this lending pool.
    #[account(
        mut,
        has_one = base_oracle,
        has_one = interest_rate_model
    )]
    pub pool: Box<Account<'info, LendingPool>>,

    pub base_oracle: Account<'info, OracleAccount>,

    pub interest_rate_model: Box<Account<'info, NSlopeModel>>,

    // Make sure the node is associated with the lending pool and collection.
    //
    // Also check that the collection is associated with the node, this is
    // likely redundant given that it's checked as a part of the seeds.
    #[account(
        seeds = [BorrowingNode::NAME, pool.key().as_ref(), collection.key().as_ref()],
        bump = node.bump,
        has_one = collection
    )]
    pub node: Box<Account<'info, BorrowingNode>>,

    // Check that we've got the right oracle to price the collateral.
    #[account(mut, has_one = collection_oracle)]
    pub collection: Box<Account<'info, CollectionAccount>>,

    pub collection_oracle: Account<'info, OracleAccount>,

    pub borrower: Signer<'info>,

    // Make sure we're using the borrow account for this borrowing node
    // and for this borrower who signed the instruction.
    //
    // And make sure the collaterals account is the one associated with
    // this borrow account.
    #[account(
        mut, 
        seeds = [BorrowAccount::NAME, node.key().as_ref(), borrower.key().as_ref()],
        bump = borrow_account.bump,
        has_one = collaterals,
    )]
    pub borrow_account: Box<Account<'info, BorrowAccount>>,

    #[account(mut)]
    pub collaterals: Box<Account<'info, CollateralList>>,

    // Make sure this collateral is associated with the correct collection by
    // checking the seeds.
    //
    // Make sure the collateral token account and the borrower are associated with this 
    // collateral account.
    #[account(
        mut,
        seeds = [CollateralAccount::NAME, node.collection.as_ref(), collateral_account.mint.as_ref()],
        bump = collateral_account.bump,
        has_one = borrower,
        has_one = collateral_token_account,
        close = receiver
    )]
    pub collateral_account: Account<'info, CollateralAccount>,

    #[account(mut)]
    pub collateral_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub receiver: SystemAccount<'info>,

    pub token_program: Program<'info, Token>,

}

impl<'info> WithdrawCollateral<'info> {

    pub fn transfer_collateral(&self) -> Result<()> {
        utils::transfer_signed(
            self.token_program.to_account_info(),
            self.collateral_token_account.to_account_info(),
            self.user_token_account.to_account_info(),
            self.admin.to_account_info(),
            &self.admin,
            1
        )
    }

    pub fn close_collateral_token_account(&self) -> Result<()> {
        utils::close_token_account(
            self.token_program.to_account_info(),
            self.collateral_token_account.to_account_info(),
            self.receiver.to_account_info(),
            self.admin.to_account_info(),
            &self.admin
        )
    }

}

pub fn handler(ctx: Context<WithdrawCollateral>) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let interest_rate_model = &ctx.accounts.interest_rate_model;

    logic::debt::update(pool, interest_rate_model)?;

    logic::withdraw_collateral::validate(
        pool,
        &ctx.accounts.node,
        &mut ctx.accounts.borrow_account,
        &ctx.accounts.collaterals,
        &ctx.accounts.base_oracle,
        &ctx.accounts.collection_oracle,
    )?;

    // Transfer collateral to borrower
    ctx.accounts.transfer_collateral()?;
    ctx.accounts.close_collateral_token_account()?;

    logic::withdraw_collateral::execute(
        &mut ctx.accounts.collaterals,
        &mut ctx.accounts.collection,
        &mut ctx.accounts.collateral_account,
        &ctx.accounts.collateral_token_account.mint
    )?;

    Ok(())
}