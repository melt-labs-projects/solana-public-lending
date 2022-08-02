use anchor_lang::prelude::*;


#[account]
pub struct CubeAccount {

    pub bump: u8,

    pub mint: Pubkey,

    pub locker: Pubkey,

    pub membership: Pubkey

}

impl CubeAccount {
    pub const NAME: &'static [u8; 4] = b"cube";
    pub const SIZE: usize = 8 + 1 + (3 * 32) + 45;
}