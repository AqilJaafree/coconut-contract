// File: lib.rs

use anchor_lang::prelude::*;

pub mod state;
pub mod context;
pub mod instructions;
pub mod error;

use instructions::*;

declare_id!("FSudCsBKGDQShx9orrZHWTq7pXF14NnPqw6MLxC47uXM");

#[program]
pub mod coconut_rwa {
    use super::*;

    pub fn initialize_hotel(ctx: Context<InitializeHotel>, name: String, room_count: u16) -> Result<()> {
        hotel::initialize_hotel(ctx, name, room_count)
    }

    pub fn verify_hotel(ctx: Context<VerifyHotel>) -> Result<()> {
        hotel::verify_hotel(ctx)
    }

    pub fn issue_coco_tokens(ctx: Context<IssueCococTokens>, amount: u64) -> Result<()> {
        token::issue_coco_tokens(ctx, amount)
    }

    pub fn create_liquidity_pool(ctx: Context<CreateLiquidityPool>, initial_liquidity: u64) -> Result<()> {
        liquidity_pool::create_liquidity_pool(ctx, initial_liquidity)
    }

    pub fn add_liquidity(ctx: Context<AddLiquidity>, coco_amount: u64, usdc_amount: u64) -> Result<()> {
        liquidity_pool::add_liquidity(ctx, coco_amount, usdc_amount)
    }

    pub fn swap_tokens(ctx: Context<SwapTokens>, amount_in: u64, min_amount_out: u64) -> Result<()> {
        liquidity_pool::swap_tokens(ctx, amount_in, min_amount_out)
    }

    pub fn stake_coco_tokens(ctx: Context<StakeCocoTokens>, amount: u64) -> Result<()> {
        staking::stake_coco_tokens(ctx, amount)
    }

    pub fn unstake_coco_tokens(ctx: Context<UnstakeCocoTokens>, amount: u64) -> Result<()> {
        staking::unstake_coco_tokens(ctx, amount)
    }

    pub fn rent_room(ctx: Context<RentRoom>, room_number: u16, duration: i64, usdc_amount: u64) -> Result<()> {
        rental::rent_room(ctx, room_number, duration, usdc_amount)
    }

    pub fn create_rental_listing(ctx: Context<CreateRentalListing>, room_number: u16, price: u64) -> Result<()> {
        rental::create_rental_listing(ctx, room_number, price)
    }

    pub fn confidential_transfer(ctx: Context<ConfidentialTransfer>, encrypted_amount: [u8; 32]) -> Result<()> {
        confidential_transfer::confidential_transfer(ctx, encrypted_amount)
    }
}