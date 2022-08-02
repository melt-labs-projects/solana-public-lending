use anchor_lang::prelude::*;

use crate::events;
use crate::instructions::logic;
use crate::states::{
    LendingPool,
    BorrowingNode,
    BorrowAccount,
    OracleAccount,
    CollectionAccount,
    LSlice,
    NSlopeModel,
    CollateralList,
};

#[derive(Accounts)]
pub struct Liquidate<'info> {

    #[account(
        mut, 
        has_one = base_oracle,
        has_one = interest_rate_model
    )]
    pub pool: Box<Account<'info, LendingPool>>,

    pub base_oracle: Account<'info, OracleAccount>,
    
    pub interest_rate_model: Account<'info, NSlopeModel>,

    #[account(
        mut,
        seeds = [BorrowingNode::NAME, pool.key().as_ref(), node.collection.as_ref()],
        bump = node.bump,
        has_one = collection
    )]
    pub node: Account<'info, BorrowingNode>,

    #[account(mut, has_one = collection_oracle)]
    pub collection: Account<'info, CollectionAccount>,

    pub collection_oracle: Account<'info, OracleAccount>,

    #[account(
        mut, 
        seeds = [BorrowAccount::NAME, node.key().as_ref(), collaterals.borrower.as_ref()],
        bump = borrow_account.bump,
        has_one = collaterals,
        close = receiver,
    )]
    pub borrow_account: Account<'info, BorrowAccount>,

    #[account(mut)]
    pub collaterals: Box<Account<'info, CollateralList>>,

    pub signer: Signer<'info>,

    #[account(mut)]
    pub receiver: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [LSlice::NAME, node.collection.as_ref(), last_slice.number.to_le_bytes().as_ref()],
        bump = last_slice.bump
    )]
    pub last_slice: Box<Account<'info, LSlice>>,

}

pub fn handler(ctx: Context<Liquidate>) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let node = &mut ctx.accounts.node;
    let borrower = &mut ctx.accounts.borrow_account;
    let interest_rate_model = &ctx.accounts.interest_rate_model;

    logic::debt::update(pool, interest_rate_model)?;

    logic::liquidate::validate(
        pool,
        node,
        borrower,
        &ctx.accounts.collaterals,
        &ctx.accounts.base_oracle,
        &ctx.accounts.collection_oracle
    )?;

    let pending_tokens = logic::liquidate::execute(
        pool,
        node,
        borrower,
        &mut ctx.accounts.collaterals,
    )?;

    let collaterals_key = ctx.accounts.collaterals.key();
    logic::slice::add(
        &mut ctx.accounts.collection,
        &mut ctx.accounts.collaterals,
        collaterals_key,
        &mut ctx.accounts.last_slice,
    )?;

    events::emit_liquidate_event(
        ctx.accounts.pool.key(),
        ctx.accounts.node.collection,
        ctx.accounts.collaterals.borrower,
        pending_tokens
    )?;

    Ok(())
}