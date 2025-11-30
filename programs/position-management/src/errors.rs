use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient collateral")]
    InsufficientCollateral = 2001,

    #[msg("Position already closed")]
    PositionAlreadyClosed = 3002,

    #[msg("Invalid position size")]
    InvalidPositionSize = 3004,

    #[msg("Invalid leverage")]
    InvalidLeverageValue = 4001,

    #[msg("Invalid price")]
    InvalidPrice = 4002,

    #[msg("Calculation overflow")]
    CalculationOverflow = 7001,

    #[msg("Calculation underflow")]
    CalculationUnderflow = 7002,

    #[msg("Cannot modify others position")]
    CannotModifyOthersPosition = 5002,

    #[msg("Unauthorized")]
    Unauthorized = 5001,

    #[msg("Cannot reduce margin")]
    CannotReduceMargin = 2003,

    #[msg("Position not found")]
    PositionNotFound = 3001,
}