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

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct ListingParam {
    // pub mint: Pubkey,
    pub currency: Pubkey,
    pub price: u64,
}
