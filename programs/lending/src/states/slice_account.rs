use anchor_lang::prelude::*;

use crate::errors::LendingError;

#[account]
pub struct LSlice {

    pub bump: u8,

    pub number: u64,

    pub vector: Vec<Pubkey>,

}

#[constant]
pub const MAX_SLICE_SIZE: usize = 10;

impl LSlice {
    pub const NAME: &'static [u8; 5] = b"slice";
    pub const SIZE: usize = 8 + (1 * 1) + (1 * 8) + (4 + (10 * 32));
    
    pub fn add(&mut self, mint: Pubkey) -> Result<()> {
        require!(self.vector.len() < MAX_SLICE_SIZE, LendingError::NoSpaceLeft);
        self.vector.push(mint);
        Ok(())
    }

    pub fn remove(&mut self, index: usize) -> Result<()> {
        require!(index < self.vector.len(), LendingError::IndexOutOfBounds);
        self.vector.remove(index);
        Ok(())
    }

    pub fn index_of(&self, key: Pubkey) -> Result<usize> {
        let index = self.vector.iter().position(|x| *x == key).ok_or(LendingError::ElementNotFound)?;
        Ok(index)
    }

}