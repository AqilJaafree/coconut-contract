// File: instructions/hotel.rs

use anchor_lang::prelude::*;
use crate::context::*;
use crate::state::*;
use crate::errors::ErrorCode;

pub fn initialize_hotel(ctx: Context<InitializeHotel>, name: String, room_count: u16) -> Result<()> {
    require!(room_count > 0, ErrorCode::InvalidRoomCount);

    let hotel = &mut ctx.accounts.hotel;
    hotel.owner = ctx.accounts.owner.key();
    hotel.name = name;
    hotel.room_count = room_count;
    hotel.is_verified = false;

    Ok(())
}

pub fn verify_hotel(ctx: Context<VerifyHotel>) -> Result<()> {
    let hotel = &mut ctx.accounts.hotel;
    
    // In a real-world scenario, you'd want to check if the authority is actually allowed to verify hotels
    // For simplicity, we're not implementing that check here
    
    hotel.is_verified = true;

    // Here you would implement the logic to mint a Soulbound NFT to the hotel owner
    // For brevity, we're not implementing the full NFT minting logic

    emit!(HotelVerified {
        hotel: hotel.key(),
        owner: hotel.owner,
    });

    Ok(())
}

#[event]
pub struct HotelVerified {
    pub hotel: Pubkey,
    pub owner: Pubkey,
}