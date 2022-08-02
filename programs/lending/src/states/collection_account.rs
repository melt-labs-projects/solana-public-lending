use anchor_lang::prelude::*;

use crate::states::slice_account::MAX_SLICE_SIZE;

#[account]
pub struct CollectionAccount {

    pub collection_oracle: Pubkey,

    // The maximum number of collaterals that can be deposited
    pub max_collaterals: u64,

    // The number of collaterals that are deposited
    pub collaterals_in_use: u64,

    // The number of collaterals which are pending liquidation
    pub pending_liquidations: u64,

    // The total number of liquidations that have occurred
    pub total_liquidations: u64,

    // If you put all the slice vectors together and add their length,
    // that is this number...
    pub total_slices_length: u64,

}

impl CollectionAccount {
    pub const SIZE: usize = 8 + (5 * 8) + (1 * 32) + 120;

    pub fn is_accepting_collateral(&self) -> bool {
        self.collaterals_in_use < self.max_collaterals
    }

    pub fn last_slice(&self) -> u64 {
        self.total_slices_length / MAX_SLICE_SIZE as u64
    }

}