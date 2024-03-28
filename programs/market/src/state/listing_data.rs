use anchor_lang::prelude::*;

use crate::ListingStatus;

#[account]
pub struct ListingData {
    pub owner: Pubkey,         //32
    pub currency: Pubkey,      //32
    pub price: u64,           //16
    pub listingtime: i64,      //4
    pub opentime: i64,         //4
    pub status: ListingStatus, //1
    pub bump: u8,              //1
}

impl ListingData {
    pub fn listing(
        &mut self,
        owner: &Pubkey,
        currency: &Pubkey,
        price: u64,
        listingtime: i64,
        opentime: i64,
        bump: u8,
    ) -> Result<()> {
        self.owner = *owner;
        self.currency = *currency;
        self.price = price;
        self.listingtime = listingtime;
        self.opentime = opentime;
        self.status = ListingStatus::Listing;
        self.bump = bump;

        Ok(())
    }
}
