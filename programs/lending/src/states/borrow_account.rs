use anchor_lang::prelude::*;

#[account]
pub struct BorrowAccount {

    pub bump: u8,

    pub node: Pubkey,

    // Represents what portion of all debt belongs to this account
    pub shares: u64,

    // Represents amount borrowed which hasn't been paid back (excluding interest)
    pub outstanding: u64,

    pub collaterals: Pubkey,

}

impl BorrowAccount {
    pub const NAME: &'static [u8; 6] = b"borrow";
    pub const SIZE: usize = 8 + 1 + (2 * 8) + (2 * 32) + 119;

    pub fn close(&mut self) {
        self.shares = 0;
        self.outstanding = 0;
        self.collaterals = Pubkey::default();
    }
}