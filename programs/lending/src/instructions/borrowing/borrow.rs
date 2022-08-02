use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::events;
use crate::errors::LendingError;
use crate::instructions::{utils, logic};
use crate::states::{
    AdminAccount, 
    LendingPool,
    BorrowingNode,
    BorrowAccount,
    OracleAccount,
    CollectionAccount,
    NSlopeModel,
    CollateralList,
};

#[derive(Accounts)]
pub struct Borrow<'info> {

    pub admin: Account<'info, AdminAccount>,

    #[account(
        mut,
        has_one = treasury,
        has_one = base_oracle,
        has_one = interest_rate_model
    )]
    pub pool: Box<Account<'info, LendingPool>>,

    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>,

    pub base_oracle: Account<'info, OracleAccount>,
    
    pub interest_rate_model: Account<'info, NSlopeModel>,

    #[account(
        seeds = [BorrowingNode::NAME, pool.key().as_ref(), node.collection.as_ref()],
        bump = node.bump,
        has_one = collection
    )]
    pub node: Account<'info, BorrowingNode>,

    #[account(has_one = collection_oracle)]
    pub collection: Account<'info, CollectionAccount>,

    pub collection_oracle: Account<'info, OracleAccount>,

    #[account(
        mut,
        seeds = [BorrowAccount::NAME, node.key().as_ref(), signer.key().as_ref()],
        bump = borrow_account.bump,
        has_one = collaterals,
    )]
    pub borrow_account: Account<'info, BorrowAccount>,

    #[account(mut)]
    pub collaterals: Box<Account<'info, CollateralList>>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    pub signer: Signer<'info>,

    pub token_program: Program<'info, Token>,

}

impl<'info> Borrow<'info> {

    fn transfer_from_treasury(&self, amount: u64) -> Result<()> {
        utils::transfer_signed(
            self.token_program.to_account_info(), 
            self.treasury.to_account_info(),
            self.user_token_account.to_account_info(), 
            self.admin.to_account_info(), 
            &self.admin,
            amount
        )
    }

}
    
pub fn handler(ctx: Context<Borrow>, amount: u64) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let node = &mut ctx.accounts.node;
    let interest_rate_model = &ctx.accounts.interest_rate_model;

    require!(node.is_enabled(), LendingError::DisabledNode);
    require!(pool.has_available(amount), LendingError::InsufficientAvailability);

    logic::debt::update(pool, interest_rate_model)?;
    
    logic::borrow::execute(
        pool,
        node,
        &mut ctx.accounts.borrow_account,
        &ctx.accounts.collaterals,
        &ctx.accounts.base_oracle,
        &ctx.accounts.collection_oracle,
        amount
    )?;

    ctx.accounts.transfer_from_treasury(amount)?;

    events::emit_borrow_event(
        ctx.accounts.pool.key(),
        ctx.accounts.node.collection,
        ctx.accounts.signer.key(),
        amount
    )?;
    
    Ok(())
}
