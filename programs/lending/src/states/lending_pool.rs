use anchor_lang::prelude::*;

use crate::instructions::utils;

#[account]
pub struct LendingPool {

     /*
    ===================
         IMMUTABLE
    ===================
    */

    // The mint of the token which is lent and borrowed.
    pub base_mint: Pubkey, 

    // The mint of the token given to lenders to represent their share
    // of the tokens in the lending pool.
    pub credit_mint: Pubkey, 

    // Holds the lending pool's available tokens 
    pub treasury: Pubkey, 

    // Holds accumulated backup tokens to account for liquidation inefficiencies
    pub reserve: Pubkey, 

    // Token account to the facilitator fee is paid to when loan repayments are made
    pub fee_token_account: Pubkey, 

    /*
    ===================
         INTERNALS
    ===================
    */
    
    /// `available_tokens` is the number of BASE tokens available to be borrowed or withdrawn
    /// from the lending pool. `utilised_tokens` on the other hand is the number of tokens that
    /// are currently borrowed from the lending pool. Adding these two numbers together will
    /// give you the total BASE token supply to the lending pool.
    pub available_tokens: u64,
    pub utilised_tokens: u64, 

    /// `pending_debt_shares` is the number of debt shares which in the process of being reacquired
    /// through the protocol's liquidation mechanism. 
    /// 
    /// When a liquidation is not timely, the protocol may be unable to reacquire the full number 
    /// of BASE tokens. In this scenario, the number of tokens which were not accounted for by the 
    /// liquidation are added to the `lost_tokens` count.
    pub pending_tokens: u64, 
    pub lost_tokens: u64,

    /// `credit_shares` is the total number of shares that have been issued to lenders to represent 
    /// their supply. While `debt_shares` is the total number of shares that have been issued to 
    /// borrowers and represent their portion of the total debt.
    pub credit_shares: u64,
    pub debt_shares: u64,
    
    /// The last timestamp when the interest rate and interest accumulated on the debt was 
    /// calculated and updated internally.
    pub last_update_timestamp: u64,

    /// The interest rate as per the `last_updated_timestamp`.
    pub interest_rate: u64,

    pub interest_rate_model: Pubkey, 

        /*
    ===================
         UPDATABLE
    ===================
    */

    // The oracle for the pricing the pool mint
    pub base_oracle: Pubkey, 

    pub params: PoolParams,

}

impl LendingPool {
    pub const SIZE: usize = 8 + (10 * 8) + (7 * 32) + 288;

    pub fn total_tokens(&self) -> u64 {
        self.available_tokens + self.utilised_tokens + self.pending_tokens
    }

    pub fn utilisation(&self) -> Result<u64> {
        if self.total_tokens() == 0 { return Ok(0) }
        utils::safe_fraction_mul(utils::ONE, self.utilised_tokens, self.total_tokens())
    }

    pub fn credit_shares_from_amount(&self, amount: u64) -> Result<u64> {
        if self.total_tokens() == 0 { return Ok(amount) }
        utils::safe_fraction_mul(self.credit_shares, amount, self.total_tokens())
    }

    pub fn debt_shares_from_amount(&self, amount: u64) -> Result<u64> {
        if self.utilised_tokens == 0 { return Ok(amount) }
        utils::safe_fraction_mul(self.debt_shares, amount, self.utilised_tokens)
    }

    pub fn amount_from_debt_shares(&self, shares: u64) -> Result<u64> {
        if shares == 0 { return Ok(0) }
        utils::safe_fraction_mul(self.utilised_tokens, shares, self.debt_shares)
    }

    pub fn has_available(&self, amount: u64) -> bool {
        self.available_tokens >= amount
    }

}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PoolParams {

    // The proportion of debt repayments to be charged as a fee.
    pub facilitator_fee: u64, 

}