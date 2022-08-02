use anchor_lang::prelude::*;

use crate::instructions::utils;

#[event]
pub struct Locked {
    pub mint: Pubkey,
    pub timestamp: u64,
}

#[event]
pub struct Unlocked {
    pub mint: Pubkey,
    pub timestamp: u64,
}

pub fn emit_locked_event(mint: Pubkey) -> Result<()> {
    emit!(Locked { mint, timestamp: utils::current_timestamp()? });
    Ok(())
}

pub fn emit_unlocked_event(mint: Pubkey) -> Result<()> {
    emit!(Unlocked { mint, timestamp: utils::current_timestamp()? });
    Ok(())
}
    
    