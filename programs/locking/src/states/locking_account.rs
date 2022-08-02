use anchor_lang::prelude::*;

use crate::instructions::utils;

#[account]
pub struct LockingAccount {

    pub bump: u8,

    pub membership: Pubkey,

    pub cube_mint: Pubkey,

    pub locked_token_account: Pubkey,

    pub locked_timestamp: u64,

    pub liquidation_data: LiquidationData,

}

impl LockingAccount {
    pub const NAME: &'static [u8; 6] = b"locked";
    pub const SIZE: usize = 8 + (2 * 1) + (3 * 32) + (4 * 8) + 370;

    pub fn discounted(&self, amount: u64) -> Result<u64> {
        utils::safe_fraction_mul(
            amount, 
            utils::ONE_HUNDRED_PERCENT - self.liquidation_data.discount,
            utils::ONE_HUNDRED_PERCENT
        )
    }

    pub fn is_cube_locked(&self) -> bool {
        self.cube_mint != Pubkey::default()
    }

    pub fn reset(&mut self) {
        self.membership = Pubkey::default();
        self.cube_mint = Pubkey::default();
        self.locked_token_account = Pubkey::default();
        self.locked_timestamp = 0;
        self.liquidation_data.active = false;
        self.liquidation_data.discount = 0;
        self.liquidation_data.uses = 0;
        self.liquidation_data.last_updated_timestamp = 0;
    }

}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct LiquidationData {
    pub active: bool,
    pub discount: u64,
    pub uses: u64,
    pub last_updated_timestamp: u64,
}