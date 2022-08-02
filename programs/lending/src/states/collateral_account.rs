use anchor_lang::prelude::*;

#[account]
pub struct CollateralAccount {
    
    pub bump: u8,

    pub mint: Pubkey,

    pub borrower: Pubkey,

    pub collateral_token_account: Pubkey,

    // Collateral list this collateral is a part of
    pub collaterals: Pubkey,

}

impl CollateralAccount {
    pub const NAME: &'static [u8; 10] = b"collateral";
    pub const SIZE: usize = 8 + (1 * 1) + (4 * 32);

    pub fn reset(&mut self) {
        self.borrower = Pubkey::default();
        self.collateral_token_account = Pubkey::default();
        self.collaterals = Pubkey::default();
    }

}