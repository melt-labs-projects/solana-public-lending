use anchor_lang::prelude::*;

#[error_code]
pub enum LockingError {

    #[msg("Numerical overflow error")]
    NumericalError,

    #[msg("Cannot close locking account without withdawing cube")]
    CannotCloseLocking,

}