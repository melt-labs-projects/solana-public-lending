use anchor_lang::prelude::*;

use crate::errors::LendingError;
use crate::states::{
    LendingPool,
    BorrowingNode,
    BorrowAccount,
    CollectionAccount,
    CollateralAccount,
    OracleAccount,
    CollateralList,
};

pub fn validate(
    pool: &LendingPool,
    node: &BorrowingNode,
    borrower: &mut BorrowAccount,
    collaterals: &CollateralList,
    base_oracle: &OracleAccount,
    collection_oracle: &OracleAccount,
) -> Result<()> {

    // If they have no collateral clearly they cannot withdraw
    require!(collaterals.length() > 0, LendingError::CannotWithdrawCollateral);

    // What is the total value of the collateral after one has been withdrawn
    let post_total_collaterval_value = collection_oracle.value_of(collaterals.length() - 1)?;

    // What is the maximum that can be borrowed against this amount
    let max_borrow_value = node.max_borrow(post_total_collaterval_value)?;

    // How much debt does the borrower currently have
    let amount = pool.amount_from_debt_shares(borrower.shares)?;
    let debt_value = base_oracle.amount_to_value(amount)?;

    // Fail if their debt would be larger than the maximum allowed borrow
    require!(debt_value <= max_borrow_value, LendingError::CannotWithdrawCollateral);

    Ok(())
}

pub fn execute(
    collaterals: &mut CollateralList,
    collection: &mut CollectionAccount,
    collateral: &mut CollateralAccount,
    collateral_mint: &Pubkey,
) -> Result<()> {

    // Keep track of how many collaterals are deposited for this collection
    collection.collaterals_in_use -= 1;

    // Remove the collateral from the borrower's list
    collaterals.remove(*collateral_mint)?;

    // Reset the collateral, so it's available to be used by another borrower
    collateral.reset();

    Ok(())
}