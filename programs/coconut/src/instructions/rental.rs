// File: instructions/rental.rs

use anchor_lang::prelude::*;
use anchor_spl::token_interface;
use crate::context::*;
use crate::state::*;
use crate::errors::ErrorCode;

pub fn rent_room(ctx: Context<RentRoom>, room_number: u16, duration: i64, usdc_amount: u64) -> Result<()> {
    let hotel = &mut ctx.accounts.hotel;
    require!(room_number <= hotel.room_count, ErrorCode::InvalidRoomNumber);

    // Transfer USDC from renter to hotel owner
    token_interface::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token_interface::Transfer {
                from: ctx.accounts.renter_usdc_account.to_account_info(),
                to: ctx.accounts.hotel_owner_usdc_account.to_account_info(),
                authority: ctx.accounts.renter.to_account_info(),
            },
        ),
        usdc_amount,
    )?;

    emit!(RoomRented {
        hotel: hotel.key(),
        renter: ctx.accounts.renter.key(),
        room_number,
        duration,
        amount: usdc_amount,
    });

    Ok(())
}

pub fn create_rental_listing(ctx: Context<CreateRentalListing>, room_number: u16, price: u64) -> Result<()> {
    let listing = &mut ctx.accounts.rental_listing;
    listing.owner = ctx.accounts.owner.key();
    listing.room_number = room_number;
    listing.price = price;
    listing.is_active = true;

    emit!(RentalListingCreated {
        listing: listing.key(),
        owner: listing.owner,
        room_number,
        price,
    });

    Ok(())
}

#[event]
pub struct RoomRented {
    pub hotel: Pubkey,
    pub renter: Pubkey,
    pub room_number: u16,
    pub duration: i64,
    pub amount: u64,
}

#[event]
pub struct RentalListingCreated {
    pub listing: Pubkey,
    pub owner: Pubkey,
    pub room_number: u16,
    pub price: u64,
}