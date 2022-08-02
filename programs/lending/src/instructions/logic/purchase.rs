use anchor_lang::prelude::*;
use std::cmp;
use locking::states::LockingAccount;

use crate::states::{
    LendingPool,
    OracleAccount,
    CollateralAccount,
    CollateralList,
    BorrowingNode
};

pub fn execute(
    pool: &mut LendingPool,
    node: &mut BorrowingNode,
    collateral: &mut CollateralAccount,
    collaterals: &CollateralList,
    base_oracle: &OracleAccount,
    collateral_oracle: &OracleAccount,
    locking_account: &LockingAccount
) -> Result<(u64, u64)> {

    // Calculate minimum payment
    let base_floor_price = collateral_oracle.value_in(base_oracle)?;
    let sale_amount = locking_account.discounted(base_floor_price)?;

    // Check how much was made or lost.
    // If net is negative some lender tokens were lost
    // If net is positive, the lenders tokens were all recovered and there is some extra
    let pending_tokens = collaterals.pending_tokens_per_collateral;
    let net = sale_amount as i64 - pending_tokens as i64;
    let lost_tokens = cmp::min(0, net).abs() as u64;
    let recovered_tokens = pending_tokens - lost_tokens;

    pool.lost_tokens += lost_tokens;
    pool.available_tokens += recovered_tokens;
    pool.pending_tokens -= pending_tokens;
    node.pending_tokens -= pending_tokens;

    // msg!("PENDING {}", pending_tokens);
    // msg!("SALE {}", sale_amount);
    // msg!("NET {}", net);
    // msg!("LOST {}", lost_tokens);
    // msg!("POOL LOST {}", pool.lost_tokens);

    // TODO think about this
    // If we do this then the lender (available + utilised) never goes down
    // which means that the loss is not split amongst the lenders, instead,
    // those who manage to withdraw while there are still available tokens
    // will not lose any money. But then the debt gets split among the borrowers...
    //
    // pool.utilised_tokens += lost_tokens;

    let amount_for_treasury = recovered_tokens;
    let amount_for_reserve = sale_amount - amount_for_treasury;

    collateral.reset();

    Ok((amount_for_treasury, amount_for_reserve))
}

// pub fn recover_tokens(pool: &mut LendingPool) -> Result<u64> {
//     if pool.lost_tokens > 0 {
//         let recoverable_amount = cmp::min(pool.lost_tokens, pool.reserve_tokens);
//         pool.lost_tokens -= recoverable_amount;
//         return Ok(recoverable_amount)
//     }
//     Ok(0)
// }