use anchor_lang::prelude::*;

use crate::instructions::utils;

#[account]
pub struct BorrowingNode {

    pub bump: u8,
    
    pub pool: Pubkey,

    pub collection: Pubkey,

    pub debt_shares: u64,

    pub pending_tokens: u64, 

    pub params: NodeParams,

}

impl BorrowingNode {
    pub const NAME: &'static [u8; 4] = b"node";
    pub const SIZE: usize = 8 + (1 * 1) + (2 * 8) + (2 * 32) + NodeParams::SIZE + 194;

    pub fn max_borrow(&self, amount: u64) -> Result<u64> {
        utils::safe_fraction_mul(amount, self.params.max_borrow_ratio, utils::ONE_HUNDRED_PERCENT)
    }

    pub fn liquidation_value(&self, amount: u64) -> Result<u64> {
        utils::safe_fraction_mul(amount, self.params.liquidation_ratio, utils::ONE_HUNDRED_PERCENT)
    }

    pub fn is_enabled(&self) -> bool {
        self.params.enabled
    }

}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct NodeParams {

    // Whether the node is accepting new borrowers
    // (Repayment + withdrawing collateral is possible even if not enabled)
    pub enabled: bool,

    // The max amount proportion of collateral value that can be borrowed
    pub max_borrow_ratio: u64, 

    // The max debt to collateral ratio before liquidation
    pub liquidation_ratio: u64, 

}

impl NodeParams {
    pub const SIZE: usize = 1 + (2 * 8);
}