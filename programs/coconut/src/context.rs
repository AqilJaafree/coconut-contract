// File: context.rs

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};
use crate::state::*;

#[derive(Accounts)]
pub struct InitializeHotel<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 100 + 2 + 1)]
    pub hotel: Account<'info, Hotel>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VerifyHotel<'info> {
    #[account(mut, has_one = owner)]
    pub hotel: Account<'info, Hotel>,
    pub owner: Signer<'info>,
    /// CHECK: This is the authority that can verify hotels
    pub authority: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct IssueCococTokens<'info> {
    #[account(mut)]
    pub coco_mint: Account<'info, CocoMint>,
    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = coco_token_mint,
        associated_token::authority = authority
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub coco_token_mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateLiquidityPool<'info> {
    #[account(init, payer = creator, space = 8 + 8 + 8 + 8)]
    pub liquidity_pool: Account<'info, LiquidityPool>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddLiquidity<'info> {
    #[account(mut)]
    pub liquidity_pool: Account<'info, LiquidityPool>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub coco_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub usdc_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
}

#[derive(Accounts)]
pub struct SwapTokens<'info> {
    #[account(mut)]
    pub liquidity_pool: Account<'info, LiquidityPool>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub token_account_in: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub token_account_out: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
}

#[derive(Accounts)]
pub struct StakeCocoTokens<'info> {
    #[account(mut)]
    pub staker: Signer<'info>,
    #[account(mut)]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = staker,
        space = 8 + 32 + 8 + 8,
        seeds = [b"stake", staker.key().as_ref()],
        bump
    )]
    pub stake_account: Account<'info, StakeAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UnstakeCocoTokens<'info> {
    #[account(mut)]
    pub staker: Signer<'info>,
    #[account(mut)]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"stake", staker.key().as_ref()],
        bump,
        has_one = owner
    )]
    pub stake_account: Account<'info, StakeAccount>,
    pub token_program: Interface<'info, TokenInterface>,
}

#[derive(Accounts)]
pub struct RentRoom<'info> {
    #[account(mut)]
    pub hotel: Account<'info, Hotel>,
    #[account(mut)]
    pub renter: Signer<'info>,
    #[account(mut)]
    pub renter_usdc_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub hotel_owner_usdc_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
}

#[derive(Accounts)]
pub struct CreateRentalListing<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 2 + 8 + 1
    )]
    pub rental_listing: Account<'info, RentalListing>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ConfidentialTransfer<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(mut)]
    pub sender_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub recipient_token_account: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
}