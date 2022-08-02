use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, MintTo, Burn, CloseAccount};
use anchor_lang::solana_program::sysvar::clock::Clock;

use crate::states::{AdminAccount, PoolParams, NodeParams};
use crate::errors::LendingError;

pub const YEAR: u64 = (60 * 60 * 24 * 365) as u64;
pub const ONE_HUNDRED_PERCENT: u64 = 10_000u64;
pub const ONE: u64 = 1_000_000_000u64;

pub fn validate_pool_params(params: &PoolParams) -> Result<()> {
    assert_valid_fee(params.facilitator_fee)?;
    Ok(())
}

pub fn validate_node_params(params: &NodeParams) -> Result<()> {
    assert_valid_rate(params.max_borrow_ratio)?;
    assert_valid_rate(params.liquidation_ratio)?;
    Ok(())
}

pub fn current_timestamp() -> Result<u64> {
    Ok(Clock::get()?.unix_timestamp as u64)
}

pub fn safe_mul(a: u64, b: u64) -> Result<u64> {
    Ok(match (a as u128).checked_mul(b as u128) {
        Some(x) => x as u64,
        None => return err!(LendingError::NumericalError)
    })
}

pub fn safe_fraction_mul(a: u64, nominator: u64, denominator: u64) -> Result<u64> {
    Ok(match (a as u128).checked_mul(nominator as u128) {
        Some(x) => match x.checked_div(denominator as u128) {
            Some(y) => y as u64,
            None => return err!(LendingError::NumericalError)
        },
        None => return err!(LendingError::NumericalError)
    })
}

pub fn assert_valid_fee(value: u64) -> Result<()> {
    require!(value <= ONE, LendingError::InvalidFee);
    Ok(())
}

pub fn assert_valid_rate(value: u64) -> Result<()> {
    require!(value <= ONE_HUNDRED_PERCENT, LendingError::InvalidRate);
    Ok(())
}

// ==== SPL instructions ====

pub fn transfer<'info>(
    token_program: AccountInfo<'info>,
    from: AccountInfo<'info>,
    to: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    amount: u64
) -> Result<()> {
    token::transfer(
        CpiContext::new(
            token_program, 
            Transfer { from, to, authority}
        ), 
        amount
    )
}

pub fn transfer_signed<'info>(
    token_program: AccountInfo<'info>,
    from: AccountInfo<'info>,
    to: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    admin: &AdminAccount,
    amount: u64
) -> Result<()> {
    token::transfer(
        CpiContext::new_with_signer(
            token_program, 
            Transfer { from, to, authority},
            &[&[AdminAccount::NAME, &[admin.bump]]]
        ), 
        amount
    )
}

pub fn mint_to<'info>(
    token_program: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    to: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    admin: &AdminAccount,
    amount: u64
) -> Result<()> {
    token::mint_to(
        CpiContext::new_with_signer(
            token_program, 
            MintTo { mint, to, authority, },
            &[&[AdminAccount::NAME, &[admin.bump]]]
        ), 
        amount
    )
}

pub fn burn<'info>(
    token_program: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    from: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    amount: u64
) -> Result<()> {
    token::burn(
        CpiContext::new(
            token_program, 
            Burn { mint, from, authority },
        ), 
        amount
    )
}

pub fn close_token_account<'info>(
    token_program: AccountInfo<'info>,
    account: AccountInfo<'info>,
    destination: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    admin: &AdminAccount,
) -> Result<()> {
    token::close_account(
        CpiContext::new_with_signer(
            token_program, 
            CloseAccount { account, destination, authority },
            &[&[AdminAccount::NAME, &[admin.bump]]]
        )
    )
}

pub fn update_locking<'info>(
    locking_program: AccountInfo<'info>,
    locking_account: AccountInfo<'info>,
    membership: AccountInfo<'info>,
) -> Result<()> {
    locking::cpi::liquidation_update(
        CpiContext::new(
            locking_program,
            locking::cpi::accounts::LiquidationUpdate { locking_account, membership }
        )
    )
}

pub fn log_liquidation<'info>(
    locking_program: AccountInfo<'info>,
    locking_account: AccountInfo<'info>,
    locker: AccountInfo<'info>,
) -> Result<()> {
    locking::cpi::liquidation_log(
        CpiContext::new(
            locking_program,
            locking::cpi::accounts::LiquidationLog { locking_account, locker }
        )
    )
}