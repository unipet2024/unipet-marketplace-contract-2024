use anchor_lang::prelude::*;

#[event]
pub struct ListingEvent {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub token_account: Pubkey,
    pub currency: Pubkey,
    pub price: u64,
    pub listingtime: i64,
    pub opentime: i64,
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
