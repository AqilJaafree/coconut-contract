// File: src/instructions/mod.rs

pub mod hotel;
pub mod token;
pub mod liquidity_pool;
pub mod staking;
pub mod rental;
pub mod confidential_transfer;

pub use hotel::*;
pub use token::*;
pub use liquidity_pool::*;
pub use staking::*;
pub use rental::*;
pub use confidential_transfer::*;