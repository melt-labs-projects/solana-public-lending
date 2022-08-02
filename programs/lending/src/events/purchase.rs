use anchor_lang::prelude::*;

use crate::instructions::utils;

#[event]
pub struct Purchase {
    pub pool: Pubkey,
    pub collection: Pubkey,
    pub buyer: Pubkey,
    pub mint: Pubkey,
    pub price: u64,
    pub timestamp: u64,
}

pub fn emit_purchase_event(
    pool: Pubkey,
    collection: Pubkey,
    buyer: Pubkey,
    mint: Pubkey,
    price: u64,
) -> Result<()> {
    emit!(Purchase { 
        pool,
        collection,
        buyer,
        mint,
        price,
        timestamp: utils::current_timestamp()? 
    });
    Ok(())
}