use anchor_lang::prelude::*;

use crate::errors::LendingError;
use crate::states::{BorrowingNode, CollectionAccount, CollateralAccount, CollateralList};

pub fn execute(
    node: &BorrowingNode,
    collaterals: &mut CollateralList,
    collection: &mut CollectionAccount,
    collateral: &mut CollateralAccount,
    collateral_token_account: &Pubkey,
    collaterals_key: Pubkey,
    collateral_mint: &Pubkey,
    signer: &Pubkey
) -> Result<()> {

    // TODO: Change this for multi-collateral borrowing
    // For now don't let anyone deposit more than 1 collateral
    require!(collaterals.length() == 0, LendingError::MaxCollaterals);

    // Make sure the borrowing node is currently enabled
    require!(node.is_enabled(), LendingError::DisabledNode);

    // Make sure the collection is willing to accept more collateral
    require!(collection.is_accepting_collateral(), LendingError::MaxCollaterals);

    // Make sure there is space for more collateral in the borrower's account
    require!(collaterals.length() < collaterals.max_size(), LendingError::MaxCollaterals);

    // Add the collateral to the borrower's list
    collaterals.add(*collateral_mint)?;

    // Keep track of how much collateral is currently deposited for this collection
    collection.collaterals_in_use += 1;

    // Set the who the borrower is and where the collateral token is stored
    collateral.collaterals = collaterals_key;
    collateral.collateral_token_account = *collateral_token_account;
    collateral.borrower = *signer;

    Ok(())
}