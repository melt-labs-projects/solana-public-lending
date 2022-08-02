use anchor_lang::prelude::*;

use crate::states::{BorrowingNode, BorrowAccount, CollateralList};

#[derive(Accounts)]
#[instruction(max_collaterals: u64)]
pub struct InitBorrowAccount<'info> {

    #[account(mut)]
    pub signer: Signer<'info>,

    pub node: Account<'info, BorrowingNode>,

    // We create the borrow account as a PDA using the public key of the node
    // and the borrower, which conveniently limits the borrower to one borrow 
    // account per borrowing node.
    #[account(
        init, 
        payer = signer,
        space = BorrowAccount::SIZE,
        seeds = [BorrowAccount::NAME, node.key().as_ref(), signer.key().as_ref()],
        bump
    )]
    pub borrow_account: Account<'info, BorrowAccount>,

    // This account stores a list of the tokens the borrower has deposited
    // to be used as collateral.
    #[account(
        init, 
        payer = signer,
        space = CollateralList::space(max_collaterals),
    )]
    pub collaterals: Box<Account<'info, CollateralList>>,

    pub system_program: Program<'info, System>,

}

pub fn handler(ctx: Context<InitBorrowAccount>, max_collaterals: u64) -> Result<()> {

    let borrow_account = &mut ctx.accounts.borrow_account;
    borrow_account.bump = *ctx.bumps.get("borrow_account").unwrap();
    borrow_account.node = ctx.accounts.node.key();
    borrow_account.collaterals = ctx.accounts.collaterals.key();

    let collaterals = &mut ctx.accounts.collaterals;
    collaterals.size = max_collaterals;
    collaterals.borrower = ctx.accounts.signer.key();

    Ok(())
}