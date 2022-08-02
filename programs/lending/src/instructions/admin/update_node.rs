use anchor_lang::prelude::*;

use crate::states::{AdminAccount, BorrowingNode, NodeParams};
use crate::instructions::utils;

#[derive(Accounts)]
pub struct UpdateNode<'info> {

    // Make sure the authority signed
    #[account(has_one = authority)]
    pub admin: Account<'info, AdminAccount>,

    pub authority: Signer<'info>,

    #[account(mut)]
    pub node: Box<Account<'info, BorrowingNode>>,

}

pub fn handler(ctx: Context<UpdateNode>, params: NodeParams) -> Result<()> {
    utils::validate_node_params(&params)?;

    let node = &mut ctx.accounts.node;
    node.params = params;
    Ok(())
}