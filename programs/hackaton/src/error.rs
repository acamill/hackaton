//! Error types

use anchor_lang::prelude::*;

#[error_code]
pub enum HackatonError {
    #[msg("Overflow in arithmetic operation")]
    Overflow = 0,
    #[msg("Limited String can store 64 chars at most")]
    LimitedStringLengthExceeded,
}
