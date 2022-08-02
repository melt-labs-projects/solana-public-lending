use anchor_lang::prelude::*;
use std::cmp;

use crate::errors::LendingError;
use crate::states::{
    LendingPool,
    BorrowAccount,
    BorrowingNode,
};

pub fn execute(
    pool: &mut LendingPool, 
    node: &mut BorrowingNode,
    borrower: &mut BorrowAccount, 
    amount: u64
) -> Result<u64> {

    // Figure out how many shares the borrower is paying off.
    // We make sure to not pay off more than they actually have.
    let shares_to_repay = cmp::min(
        pool.debt_shares_from_amount(amount)?,
        borrower.shares
    );

    // Base on the shares, calculate the actual repayment amount
    let repayment = pool.amount_from_debt_shares(shares_to_repay)?;

    // Just check the borrower isn't paying more than they specified
    require!(repayment <= amount, LendingError::SomethingWentWrong);

    // Subtract the repayment from their outstanding balance
    borrower.outstanding = borrower.outstanding.saturating_sub(repayment);

    // Remove the repaid debt shares from the total
    pool.debt_shares -= shares_to_repay;
    node.debt_shares -= shares_to_repay;

    // Remove the repaid debt shares from the borrower's total
    borrower.shares -= shares_to_repay;

    // Move the repaid tokens from utilised to available 
    pool.available_tokens += repayment;
    pool.utilised_tokens -= repayment;

    Ok(repayment)
}