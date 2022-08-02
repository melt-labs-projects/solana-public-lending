use anchor_lang::prelude::*;

use crate::errors::LendingError;
use crate::states::{
    LendingPool,
    BorrowingNode,
    BorrowAccount,
    OracleAccount,
    CollateralList
};

pub fn execute(
    pool: &mut LendingPool, 
    node: &mut BorrowingNode,
    borrower: &mut BorrowAccount,
    collaterals: &CollateralList,
    base_oracle: &OracleAccount,
    collection_oracle: &OracleAccount,
    new_debt_amount: u64,
) -> Result<()> {

    // Calculate the maximum amount that can be borrowed against the collateral
    let total_collateral_value = collection_oracle.value_of(collaterals.length())?;

    let max_borrow_value = node.max_borrow(total_collateral_value)?;

    // Calculate the amount of debt the borrower would have
    let current_debt_amount = pool.amount_from_debt_shares(borrower.shares)?;
    let proposed_debt_value = base_oracle.amount_to_value(current_debt_amount + new_debt_amount)?;

    // Make sure the debt doesn't exceed the maximum
    require!(proposed_debt_value <= max_borrow_value, LendingError::InsufficientCollateral);

    // Update the internal state with the new shares
    let new_shares = pool.debt_shares_from_amount(new_debt_amount)?;

    // Add the new debt shares to the total
    pool.debt_shares += new_shares;
    node.debt_shares += new_shares;

    // Add the new debt shares to the borrower's total
    borrower.shares += new_shares;

    // Add the original borrow amount to outstanding
    // Note: `outstanding` just tracks how much of original borrows is still to be paid back.
    //       This is useful for taking facilitator fees, as you only charge those fees
    //       On any interest paid back, rather than the original borrowed amount.
    borrower.outstanding += new_debt_amount;

    // Move the borrowed tokens from availabe to utilised
    pool.available_tokens -= new_debt_amount;
    pool.utilised_tokens += new_debt_amount;

    Ok(())
}