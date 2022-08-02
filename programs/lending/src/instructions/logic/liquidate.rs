use anchor_lang::prelude::*;

use crate::errors::LendingError;
use crate::states::{
    LendingPool,
    BorrowingNode,
    BorrowAccount,
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

    // Find the total value of the borrower's collaterals
    let total_collateral_value = collection_oracle.value_of(collaterals.length())?;

    // Find the value at which liquidation can occur
    let liquidation_value = node.liquidation_value(total_collateral_value)?;

    let amount = pool.amount_from_debt_shares(borrower.shares)?;
    let debt_value = base_oracle.amount_to_value(amount)?;
    require!(debt_value > liquidation_value, LendingError::InvalidLiquidation);

    Ok(())
}

pub fn execute(
    pool: &mut LendingPool,
    node: &mut BorrowingNode,
    borrower: &mut BorrowAccount,
    collaterals: &mut CollateralList,
) -> Result<u64> {

    // Track all pending debt shares across the pool
    let pending_amount = pool.amount_from_debt_shares(borrower.shares)?;

    // Remove the shares from the debt shares so that interest doesn't get charged
    // on the amount now that it has been liquidated
    pool.debt_shares -= borrower.shares;
    node.debt_shares -= borrower.shares;

    // Move the amount from utilised to pending
    pool.utilised_tokens -= pending_amount;
    pool.pending_tokens += pending_amount;
    node.pending_tokens += pending_amount;

    // Track how much debt needs to be paid back per collateral
    collaterals.pending_tokens_per_collateral = pending_amount / collaterals.length();
    collaterals.is_pending_liquidation = true;

    // Zero out the data
    borrower.close();

    Ok(pending_amount)
}