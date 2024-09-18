// File: instructions/staking.rs

use anchor_lang::prelude::*;
use anchor_spl::token_interface;
use crate::context::*;
use crate::state::*;
use crate::errors::ErrorCode;

pub fn stake_coco_tokens(ctx: Context<StakeCocoTokens>, amount: u64) -> Result<()> {
    let stake_account = &mut ctx.accounts.stake_account;
    
    // Transfer tokens from user to stake account
    token_interface::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token_interface::Transfer {
                from: ctx.accounts.token_account.to_account_info(),
                to: stake_account.to_account_info(),
                authority: ctx.accounts.staker.to_account_info(),
            },
        ),
        amount,
    )?;

    stake_account.staked_amount = stake_account.staked_amount.checked_add(amount).ok_or(ErrorCode::Overflow)?;
    stake_account.last_stake_timestamp = Clock::get()?.unix_timestamp;

    emit!(TokensStaked {
        user: ctx.accounts.staker.key(),
        amount,
    });

    Ok(())
}

pub fn unstake_coco_tokens(ctx: Context<UnstakeCocoTokens>, amount: u64) -> Result<()> {
    let stake_account = &mut ctx.accounts.stake_account;
    require!(stake_account.staked_amount >= amount, ErrorCode::InsufficientStakedAmount);
    
    // Calculate rewards
    let rewards = calculate_rewards(stake_account);

    // Transfer staked tokens back to user
    token_interface::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token_interface::Transfer {
                from: stake_account.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                authority: stake_account.to_account_info(),
            },
        ),
        amount,
    )?;

    // Transfer rewards to user (assuming rewards are in the same token)
    token_interface::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token_interface::Transfer {
                from: stake_account.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                authority: stake_account.to_account_info(),
            },
        ),
        rewards,
    )?;

    stake_account.staked_amount = stake_account.staked_amount.checked_sub(amount).ok_or(ErrorCode::InsufficientStakedAmount)?;
    stake_account.last_stake_timestamp = Clock::get()?.unix_timestamp;

    emit!(TokensUnstaked {
        user: ctx.accounts.staker.key(),
        amount,
        rewards,
    });

    Ok(())
}

fn calculate_rewards(stake_account: &StakeAccount) -> u64 {
    let current_time = Clock::get().unwrap().unix_timestamp;
    let time_staked = (current_time - stake_account.last_stake_timestamp) as u64;
    
    // This is a simplified reward calculation. You might want to implement a more sophisticated model.
    (stake_account.staked_amount * time_staked) / 1_000_000_000 // 1 token per billion seconds per staked token
}

#[event]
pub struct TokensStaked {
    pub user: Pubkey,
    pub amount: u64,
}

#[event]
pub struct TokensUnstaked {
    pub user: Pubkey,
    pub amount: u64,
    pub rewards: u64,
}