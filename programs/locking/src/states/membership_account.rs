use anchor_lang::prelude::*;

#[account]
pub struct MembershipAccount {

    pub bump: u8,

    pub number: u64,

    pub locked_count: u64, 

    pub liquidation_perks: LiquidationPerks,

}

impl MembershipAccount {
    pub const NAME: &'static [u8; 10] = b"membership";
    pub const SIZE: usize = 8 + 1 + (6 * 8) + 443;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct LiquidationPerks {
    pub discount: u64,
    pub max_uses: u64,
    pub warmup_period: u64,
    pub refresh_period: u64,
}