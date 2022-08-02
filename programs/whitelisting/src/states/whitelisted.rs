use anchor_lang::prelude::*;

#[account]
pub struct Whitelisted {
    
    pub bump: u8,

    pub mint: Pubkey,

    pub collection: Pubkey,

}

impl Whitelisted {
    pub const NAME: &'static [u8; 4] = b"wnft";
    pub const SIZE: usize = 8 + 1 + (2 * 32) + 77;
}