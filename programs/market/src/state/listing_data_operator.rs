use anchor_lang::prelude::*;

use crate::ListingDataParam;

#[account]
pub struct ListingDataOperator {
    pub listing_datas: Vec<ListingDataParam>, //4 + 88 * n
    pub bump: u8,                             //1
} // ==> max 1000 item => 37 + 88 * 1000 = 88037

impl ListingDataOperator {
    pub fn listing(&mut self, listing_datas: &Vec<ListingDataParam>, bump: u8) -> Result<()> {
        self.bump = bump;
        self.add_listings(listing_datas)?;

        Ok(())
    }

    pub fn add_listings(&mut self, listing_datas: &Vec<ListingDataParam>) -> Result<()> {
        for listing_data in listing_datas.iter() {
            self.listing_datas.push(*listing_data);
        }

        Ok(())
    }

    pub fn remote_listing(&mut self, mint: Pubkey) -> Result<()> {
        for i in 0..self.listing_datas.len() {
            if self.listing_datas[i].mint == mint {
                self.listing_datas.remove(i);
            }
        }
        Ok(())
    }
}
