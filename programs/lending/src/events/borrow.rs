use anchor_lang::prelude::*;

use crate::instructions::utils;

#[event]
pub struct Borrow {
    pub pool: Pubkey,
    pub collection: Pubkey,
    pub borrower: Pubkey,
    pub amount: u64,
    pub timestamp: u64,
}

pub fn emit_borrow_event(
    pool: Pubkey,
    collection: Pubkey,
    borrower: Pubkey,
    amount: u64,
) -> Result<()> {
    emit!(Borrow { 
        pool,
        collection,
        borrower,
        amount,
        timestamp: utils::current_timestamp()? 
    });
    Ok(())
}