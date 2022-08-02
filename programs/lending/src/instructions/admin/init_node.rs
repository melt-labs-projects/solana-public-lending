use anchor_lang::prelude::*;

use crate::states::{AdminAccount, LendingPool, CollectionAccount, BorrowingNode, NodeParams};
use crate::instructions::utils;

#[derive(Accounts)]
pub struct InitNode<'info> {

    // Make sure the authority signed
    #[account(has_one = authority)]
    pub admin: Account<'info, AdminAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub pool: Account<'info, LendingPool>,

    pub collection: Account<'info, CollectionAccount>,

    #[account(
        init,
        space = BorrowingNode::SIZE,
        payer = authority,
        seeds = [BorrowingNode::NAME, pool.key().as_ref(), collection.key().as_ref()],
        bump
    )]
    pub node: Account<'info, BorrowingNode>,

    pub system_program: Program<'info, System>,

}

pub fn handler(ctx: Context<InitNode>, params: NodeParams) -> Result<()> {
    utils::validate_node_params(&params)?;

    let node = &mut ctx.accounts.node;
    node.bump = *ctx.bumps.get("node").unwrap();
    node.pool = ctx.accounts.pool.key();
    node.collection = ctx.accounts.collection.key();
    node.params = params; 
    Ok(())
}