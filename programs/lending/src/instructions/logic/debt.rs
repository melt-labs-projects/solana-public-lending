use anchor_lang::prelude::*;

use crate::instructions::utils;
use crate::states::{LendingPool, NSlopeModel};

fn calculate_accumulated_debt(interest_rate: u64, total_debt: u64, last_update: u64) -> Result<u64> {
    // Check last_update isn't ahead of current? TODO
    let seconds_since_update = utils::current_timestamp()? - last_update;
    let yearly_interest = utils::safe_fraction_mul(total_debt, interest_rate, utils::ONE)?;
    Ok(utils::safe_fraction_mul(yearly_interest, seconds_since_update, utils::YEAR)?)
}

pub fn update(
    pool: &mut LendingPool,
    interest_rate_model: &NSlopeModel
) -> Result<()> {

    // How much of the pool's base tokens are being utilised
    let utilisation = pool.utilisation()?;

    // Calculate the new interest rate
    pool.interest_rate = interest_rate_model.interest_rate(utilisation)?;

    let accumulated_debt = calculate_accumulated_debt(
        pool.interest_rate, 
        pool.utilised_tokens, 
        pool.last_update_timestamp
    )?;

    pool.last_update_timestamp = utils::current_timestamp()?;
    pool.utilised_tokens += accumulated_debt;

    Ok(())
}