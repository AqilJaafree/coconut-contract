// File: instructions/token.rs

use anchor_lang::prelude::*;
use anchor_spl::token_interface;
use crate::context::*;
use crate::state::*;
use crate::errors::ErrorCode;

pub fn issue_coco_tokens(ctx: Context<IssueCococTokens>, amount: u64) -> Result<()> {
    let coco_mint = &mut ctx.accounts.coco_mint;
    require!(ctx.accounts.authority.key() == coco_mint.authority, ErrorCode::Unauthorized);

    coco_mint.total_supply = coco_mint.total_supply.checked_add(amount).ok_or(ErrorCode::Overflow)?;

    token_interface::mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token_interface::MintTo {
                mint: ctx.accounts.coco_token_mint.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        amount,
    )?;

    emit!(CocoTokensIssued {
        amount,
        recipient: ctx.accounts.token_account.key(),
    });

    Ok(())
}

#[event]
pub struct CocoTokensIssued {
    pub amount: u64,
    pub recipient: Pubkey,
}