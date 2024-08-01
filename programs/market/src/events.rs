use anchor_lang::prelude::*;

use crate::{AuthRole, MintListingParam};
// use solana_program::pubkey;

#[event]
pub struct ListingEvent {
    pub user: Pubkey,
    pub mint: Pubkey,
    pub currency: Pubkey,
    pub price: u64,
    pub listing_time: i64,
    pub open_time: i64,
    pub slot: u64,
}

#[event]
pub struct UnListingEvent {
    pub user: Pubkey,
    pub mint: Pubkey,
    pub time: i64,
    pub slot: u64,
}

#[event]
pub struct ChangePriceEvent {
    pub user: Pubkey,
    pub items: Vec<MintListingParam>,
    pub time: i64,
    pub slot: u64,
}

#[event]
pub struct BuyEvent {
    pub seller: Pubkey,
    pub buyer: Pubkey,
    pub mint: Pubkey,
    pub currency: Pubkey,
    pub price: u64,
    pub commission: u64,
    pub time: i64,
    pub slot: u64,
}

#[event]
pub struct SetAuthorityEvent {
    pub admin: Pubkey,
    pub role: AuthRole,
    pub operators: Vec<Pubkey>,
    pub time: i64,
}

#[event]
pub struct WithdrawEvent {
    pub admin: Pubkey,
    pub currency: Pubkey,
    pub amount: u64,
    pub time: i64,
}

/*
event Purchase(
        address indexed previousOwner,
        address indexed newOwner,
        address indexed nft,
        uint256 nftId,
        address currency,
        uint256 listingPrice,
        uint256 price,
        uint256 sellerAmount,
        uint256 commissionAmount,
        uint256 time
    );
*/

/*
event Listing(
        address indexed owner,
        address indexed nft,
        uint256 indexed nftId,
        address listingUser,
        address currency,
        uint256 listingPrice,
        uint256 listingTime,
        uint256 openTime
    );
*/
