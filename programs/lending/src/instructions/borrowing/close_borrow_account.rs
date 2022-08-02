use anchor_lang::prelude::*;

use crate::errors::LendingError;
use crate::states::{BorrowingNode, BorrowAccount, CollateralList};

#[derive(Accounts)]
pub struct CloseBorrowAccount<'info> {

    #[account(mut)]
    pub signer: Signer<'info>,

    pub node: Account<'info, BorrowingNode>,

    #[account(
        mut, 
        seeds = [BorrowAccount::NAME, node.key().as_ref(), signer.key().as_ref()],
        bump = borrow_account.bump,
        has_one = collaterals,
        close = receiver,
    )]
    pub borrow_account: Account<'info, BorrowAccount>,

    #[account(mut, close = receiver)]
    pub collaterals: Box<Account<'info, CollateralList>>,

    #[account(mut)]
    pub receiver: SystemAccount<'info>,

}

pub fn handler(ctx: Context<CloseBorrowAccount>) -> Result<()> {
    require!(ctx.accounts.collaterals.length() == 0, LendingError::CannotCloseBorrowAccount);
    Ok(())
}