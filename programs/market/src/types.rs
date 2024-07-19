use anchor_lang::prelude::*;

#[derive(PartialEq, Eq, AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum AuthRole {
    Admin,
    Operator,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct CurrencyParams {
    pub currency: Vec<Pubkey>,
}

#[derive(PartialEq, Eq, AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum ListingStatus {
    Close,
    Listing,
}

#[derive(PartialEq, Eq, AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum MarketStatus {
    Waiting,
    Private,
    Public,
    Close,
}

#[derive(PartialEq, Eq, AnchorSerialize, AnchorDeserialize, Clone, Debug, Copy)]
pub struct ListingDataParam {
    pub mint: Pubkey,     //32
    pub currency: Pubkey, //32
    pub price: u64,       //16
    pub listingtime: i64, //4
    pub opentime: i64,    //4
}
