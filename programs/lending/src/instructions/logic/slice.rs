use anchor_lang::prelude::*;


use crate::errors::LendingError;
use crate::states::{
    CollateralAccount,
    CollateralList,
    CollectionAccount,
    LSlice,
};


pub fn add(
    collection: &mut CollectionAccount,
    collaterals: &mut CollateralList,
    collaterals_key: Pubkey,
    slice: &mut LSlice,
) -> Result<()> {

    // Make sure this is the last slice
    require!(collection.last_slice() == slice.number, LendingError::InvalidSlice);

    // Helpful for the front end to know which slice the collateral is stored in
    collaterals.slice_number = slice.number;

    // Track the number of pending liquidations for this collection
    collection.pending_liquidations += collaterals.length();

    collection.total_slices_length += 1;
    slice.add(collaterals_key)?;

    Ok(())
}

pub fn remove(
    collaterals: &mut CollateralList,
    collaterals_key: Pubkey,
    collection: &mut CollectionAccount,
    collateral: &CollateralAccount,
    a_slice: &mut LSlice,
    last_slice: &mut LSlice,
) -> Result<()> {

    // Make sure we've been given the correct last slice
    require!(collection.last_slice() == last_slice.number, LendingError::InvalidSlice);

    // Get the index of the collaterals key is in the `a_slice`
    let collaterals_index = a_slice.index_of(collaterals_key)?;

    // Remove the collateral from the list
    collaterals.remove(collateral.mint)?;

    // If the collateral list is empty we can remove it from the liquidation slice
    if collaterals.is_empty() {

        // If the slices are the same just remove the element
        if a_slice.number == last_slice.number {
            last_slice.remove(collaterals_index)?;

        // Otherwise pop one of the end of the last slice and replace the element we
        // want to remove with this popped element
        } else {
            let replacer = last_slice.vector.pop().ok_or(LendingError::InvalidSlice)?;
            a_slice.vector[collaterals_index] = replacer;
        }

        collection.total_slices_length -= 1;

    }

    // Track the number of collaterals in use for this collection
    collection.collaterals_in_use -= 1;

    // Track the number of pending liquidations for this collection
    collection.pending_liquidations -= 1;

    // Track the total number of liquidations for this collection
    collection.total_liquidations += 1;

    Ok(())
}