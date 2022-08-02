use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::clock::Clock;
use anchor_spl::token::{self, Transfer};

use crate::errors::LockingError;
use crate::states::{LockingAccount};

pub const DEFAULT_DISCOUNT: u64 = 1_000;
pub const ONE_HUNDRED_PERCENT: u64 = 10_000;

pub fn current_timestamp() -> Result<u64> {
    Ok(Clock::get()?.unix_timestamp as u64)
}

pub fn safe_fraction_mul(a: u64, nominator: u64, denominator: u64) -> Result<u64> {
    Ok(match (a as u128).checked_mul(nominator as u128) {
        Some(x) => match x.checked_div(denominator as u128) {
            Some(y) => y as u64,
            None => return err!(LockingError::NumericalError)
        },
        None => return err!(LockingError::NumericalError)
    })
}

pub fn transfer<'info>(
    token_program: AccountInfo<'info>,
    from: AccountInfo<'info>,
    to: AccountInfo<'info>,
    authority: AccountInfo<'info>,
) -> Result<()> {
    token::transfer(
        CpiContext::new(
            token_program, 
            Transfer { from, to, authority}
        ), 
        1
    )
}

pub fn transfer_signed<'info>(
    token_program: AccountInfo<'info>,
    from: AccountInfo<'info>,
    to: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    locking_account: &LockingAccount,
    locker: &Pubkey,
) -> Result<()> {
    token::transfer(
        CpiContext::new_with_signer(
            token_program, 
            Transfer { from, to, authority},
            &[&[LockingAccount::NAME, locker.as_ref(), &[locking_account.bump]]]
        ), 
        1
    )
}