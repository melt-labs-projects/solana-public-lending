use anchor_lang::prelude::*;

use crate::instructions::utils;

#[account]
pub struct OracleAccount {
    
    pub price: u64,

    pub decimals: u64,

    pub timestamp: u64, 

}

impl OracleAccount {
    pub const NAME: &'static [u8; 6] = b"oracle";
    pub const SIZE: usize = 8 + (3 * 8);

    pub fn value_of(&self, amount: u64) -> Result<u64> {
        utils::safe_mul(self.price, amount)
    }

    // This converts an amount of the base token to its USD value. 
    // As such, self.decimals of the base oracle should represent the decimals of the base mint.
    pub fn amount_to_value(&self, amount: u64) -> Result<u64> {
        utils::safe_fraction_mul(amount, self.price, 10u64.pow(self.decimals as u32)) // Should divide by decimals
    }

    // This outputs the price of one collateral according to the number of base tokens with base mint decimals
    pub fn value_in(&self, denomination: &OracleAccount) -> Result<u64> {
        utils::safe_fraction_mul(denomination.price, self.price, 10u64.pow(self.decimals as u32)) // should divide by decimals
    }

}