// File: state.rs

use anchor_lang::prelude::*;

#[account]
pub struct Hotel {
    pub owner: Pubkey,
    pub name: String,
    pub room_count: u16,
    pub is_verified: bool,
}

#[account]
pub struct LiquidityPool {
    pub total_liquidity: u64,
    pub coco_token_reserve: u64,
    pub usdc_reserve: u64,
}

#[account]
pub struct StakeAccount {
    pub owner: Pubkey,
    pub staked_amount: u64,
    pub last_stake_timestamp: i64,
}

#[account]
pub struct RentalListing {
    pub owner: Pubkey,
    pub room_number: u16,
    pub price: u64,
    pub is_active: bool,
}

#[account]
pub struct CocoMint {
    pub authority: Pubkey,
    pub total_supply: u64,
}