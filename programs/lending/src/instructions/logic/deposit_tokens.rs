use anchor_lang::prelude::*;

use crate::states::LendingPool;

pub fn execute(pool: &mut LendingPool, amount: u64) -> Result<u64> {
    let shares = pool.credit_shares_from_amount(amount)?;
    pool.available_tokens += amount;
    pool.credit_shares += shares;
    Ok(shares)
}