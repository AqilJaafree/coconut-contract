// File: errors.rs

use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient staked amount")]
    InsufficientStakedAmount,
    #[msg("Invalid room number")]
    InvalidRoomNumber,
    #[msg("Insufficient liquidity")]
    InsufficientLiquidity,
    #[msg("Slippage tolerance exceeded")]
    SlippageExceeded,
    #[msg("Invalid encrypted amount")]
    InvalidEncryptedAmount,
    #[msg("Unauthorized operation")]
    Unauthorized,
    #[msg("Arithmetic overflow")]
    Overflow,
    #[msg("Arithmetic overflow")]
    Overflow,
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("Invalid room count")]
    InvalidRoomCount,
    #[msg("Confidential transfer failed")]
    ConfidentialTransferFailed,
}