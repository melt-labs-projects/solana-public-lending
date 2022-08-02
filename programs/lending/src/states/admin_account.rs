use anchor_lang::prelude::*;

#[account]
pub struct AdminAccount {
    
    pub bump: u8,

    pub authority: Pubkey,

}

impl AdminAccount {
    pub const NAME: &'static [u8; 5] = b"admin";
    pub const SIZE: usize = 8 + 1 + 32 + 259 + 33;
}