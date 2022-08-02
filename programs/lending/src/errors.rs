use anchor_lang::prelude::*;

#[error_code]
pub enum LendingError {

    #[msg("Numerical overflow error")]
    NumericalError,

    #[msg("Not enough tokens in token account")]
    InsufficientFunds,

    #[msg("Cannot withdraw due to high utilisation")]
    UtilisationTooHigh,

    #[msg("You don't have sufficient collateral to borrow the requested amount")]
    InsufficientCollateral,

    #[msg("Debt has not been fully repaid")]
    OutstandingDebt,    
    
    #[msg("Cannot liquidate this position")]
    InvalidLiquidation,

    #[msg("Not enough funds to borrow")]
    InsufficientAvailability,

    #[msg("This collateral has not been liquidated")]
    NotPurchasable,

    #[msg("Specified fee was too large")]
    InvalidFee,

    #[msg("Specified rate was too large")]
    InvalidRate,

    #[msg("Cannot withdraw collateral")]
    CannotWithdrawCollateral,

    #[msg("Cannot close borrow account unless all associated collateral has been withdrawn")]
    CannotCloseBorrowAccount,

    #[msg("Cannot deposit more collateral")]
    MaxCollaterals,

    // This is the error, when the programs sees something isn't quite right,
    // potentially protecting users in the cases it's used.
    #[msg("Something went wrong")]
    SomethingWentWrong,

    #[msg("Incorrect slice was provided")]
    InvalidSlice,

    #[msg("This borrowing node is currently disabled")]
    DisabledNode,
    
    #[msg("No space left in the vector")]
    NoSpaceLeft,

    #[msg("Element not found in the vector")]
    ElementNotFound,

    #[msg("Index out of vector bounds")]
    IndexOutOfBounds,

}
