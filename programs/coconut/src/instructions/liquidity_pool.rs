// File: instructions/liquidity_pool.rs

use anchor_lang::prelude::*;
use anchor_spl::token_interface;
use crate::context::*;
use crate::state::*;
use crate::errors::ErrorCode;

pub fn create_liquidity_pool(ctx: Context<CreateLiquidityPool>, initial_liquidity: u64) -> Result<()> {
    let pool = &mut ctx.accounts.liquidity_pool;
    pool.total_liquidity = initial_liquidity;
    pool.coco_token_reserve = 0;
    pool.usdc_reserve = 0;

    emit!(LiquidityPoolCreated {
        pool: pool.key(),
        creator: ctx.accounts.creator.key(),
        initial_liquidity,
    });

    Ok(())
}

pub fn add_liquidity(ctx: Context<AddLiquidity>, coco_amount: u64, usdc_amount: u64) -> Result<()> {
    let pool = &mut ctx.accounts.liquidity_pool;

    // Transfer COCO tokens to the pool
    token_interface::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token_interface::Transfer {
                from: ctx.accounts.coco_token_account.to_account_info(),
                to: pool.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        coco_amount,
    )?;

    // Transfer USDC to the pool
    token_interface::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token_interface::Transfer {
                from: ctx.accounts.usdc_token_account.to_account_info(),
                to: pool.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        usdc_amount,
    )?;

    pool.coco_token_reserve = pool.coco_token_reserve.checked_add(coco_amount).ok_or(ErrorCode::Overflow)?;
    pool.usdc_reserve = pool.usdc_reserve.checked_add(usdc_amount).ok_or(ErrorCode::Overflow)?;
    pool.total_liquidity = pool.total_liquidity.checked_add(coco_amount.min(usdc_amount)).ok_or(ErrorCode::Overflow)?;

    emit!(LiquidityAdded {
        pool: pool.key(),
        user: ctx.accounts.user.key(),
        coco_amount,
        usdc_amount,
    });

    Ok(())
}

pub fn swap_tokens(ctx: Context<SwapTokens>, amount_in: u64, min_amount_out: u64) -> Result<()> {
    let pool = &mut ctx.accounts.liquidity_pool;
    
    let (reserve_in, reserve_out) = if ctx.accounts.token_account_in.mint == pool.coco_token_reserve {
        (pool.coco_token_reserve, pool.usdc_reserve)
    } else {
        (pool.usdc_reserve, pool.coco_token_reserve)
    };

    // Calculate the output amount using the constant product formula
    let amount_out = (amount_in as u128 * reserve_out as u128 / (reserve_in as u128 + amount_in as u128)) as u64;
    require!(amount_out >= min_amount_out, ErrorCode::SlippageExceeded);

    // Transfer input tokens to the pool
    token_interface::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token_interface::Transfer {
                from: ctx.accounts.token_account_in.to_account_info(),
                to: pool.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        amount_in,
    )?;

    // Transfer output tokens from the pool to the user
    token_interface::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token_interface::Transfer {
                from: pool.to_account_info(),
                to: ctx.accounts.token_account_out.to_account_info(),
                authority: pool.to_account_info(),
            },
        ),
        amount_out,
    )?;

    // Update pool reserves
    if ctx.accounts.token_account_in.mint == pool.coco_token_reserve {
        pool.coco_token_reserve = pool.coco_token_reserve.checked_add(amount_in).ok_or(ErrorCode::Overflow)?;
        pool.usdc_reserve = pool.usdc_reserve.checked_sub(amount_out).ok_or(ErrorCode::InsufficientFunds)?;
    } else {
        pool.usdc_reserve = pool.usdc_reserve.checked_add(amount_in).ok_or(ErrorCode::Overflow)?;
        pool.coco_token_reserve = pool.coco_token_reserve.checked_sub(amount_out).ok_or(ErrorCode::InsufficientFunds)?;
    }

    emit!(TokensSwapped {
        pool: pool.key(),
        user: ctx.accounts.user.key(),
        amount_in,
        amount_out,
    });

    Ok(())
}

#[event]
pub struct LiquidityPoolCreated {
    pub pool: Pubkey,
    pub creator: Pubkey,
    pub initial_liquidity: u64,
}

#[event]
pub struct LiquidityAdded {
    pub pool: Pubkey,
    pub user: Pubkey,
    pub coco_amount: u64,
    pub usdc_amount: u64,
}

#[event]
pub struct TokensSwapped {
    pub pool: Pubkey,
    pub user: Pubkey,
    pub amount_in: u64,
    pub amount_out: u64,
}