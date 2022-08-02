use anchor_lang::prelude::*;
use std::cmp;

use crate::states::LendingPool;

pub fn execute(pool: &mut LendingPool, amount: u64, total_credit_shares: u64) -> Result<u64> {
    let shares = cmp::min(
        pool.credit_shares_from_amount(amount)?, 
        total_credit_shares
    );
    pool.available_tokens -= amount;
    pool.credit_shares -= shares;
    Ok(shares)
}