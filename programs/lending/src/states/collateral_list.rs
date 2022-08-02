use anchor_lang::prelude::*;

use crate::errors::LendingError;

#[account]
pub struct CollateralList {

    pub borrower: Pubkey,

    // Maximum size of the array
    pub size: u64,

    // The number of base tokens that need to be paid back to lenders
    pub pending_tokens_per_collateral: u64,

    // Which slice the collaterals are stored in
    pub slice_number: u64, 

    // If these collaterals are up for liquidation
    pub is_pending_liquidation: bool,

    // Stores the collateral mints
    pub vector: Vec<Pubkey>,

}

impl CollateralList {
    pub const NAME: &'static [u8; 15] = b"collateral-list";

    pub fn space(size: u64) -> usize {
        8 + (1 * 1) + (3 * 8) + (1 * 32) + (4 + (size as usize) * 32)
    }

    pub fn length(&self) -> u64 {
        self.vector.len() as u64
    }

    pub fn max_size(&self) -> u64 {
        self.size
    }

    pub fn add(&mut self, mint: Pubkey) -> Result<()> {
        require!(self.length() < self.max_size(), LendingError::NoSpaceLeft);
        self.vector.push(mint);
        Ok(())
    }

    pub fn remove(&mut self, mint: Pubkey) -> Result<()> {
        let index = self.vector.iter().position(|x| *x == mint).ok_or(LendingError::ElementNotFound)?;
        self.vector.remove(index);
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.vector.is_empty()
    }

}