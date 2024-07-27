use anchor_lang::prelude::*;
use anchor_lang::system_program;

use crate::ListingStatus;

use crate::id;
use crate::MarketErrors;

#[account]
pub struct ListingData {
    pub owner: Pubkey,         //32
    pub currency: Pubkey,      //32
    pub price: u64,            //8
    pub listingtime: i64,      //8
    pub opentime: i64,         //8
    pub status: ListingStatus, //1
    pub bump: u8,              //1
}

impl ListingData {
    pub const SIZE: usize = 8 // anchor account discriminator
    + 32 // owner
    + 32 // currency
    + 8  // price
    + 8  // listingtime
    + 8  // opentime
    + 1  // status
    + 1; // bump
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

    /// Check if the listing_data account space needs to be reallocated .
    /// Returns `true` if the account was reallocated.
    pub fn realloc_if_needed<'a>(
        listing_data: AccountInfo<'a>,
        rent_payer: AccountInfo<'a>,
        system_program: AccountInfo<'a>,
    ) -> Result<bool> {
        require_keys_eq!(*listing_data.owner, id(), MarketErrors::IllegalAccountOwner);

        let current_account_size = listing_data.data.borrow().len();
        let account_size_to_fit = ListingData::SIZE;

        // Check if we need to reallocate space.
        if current_account_size >= account_size_to_fit {
            return Ok(false);
        }

        // Reallocate more space.
        AccountInfo::realloc(&listing_data, account_size_to_fit, false)?;

        // If more lamports are needed, transfer them to the account.
        let rent_exempt_lamports = Rent::get()
            .unwrap()
            .minimum_balance(account_size_to_fit)
            .max(1);
        let top_up_lamports =
            rent_exempt_lamports.saturating_sub(listing_data.to_account_info().lamports());

        if top_up_lamports > 0 {
            require_keys_eq!(
                *system_program.key,
                system_program::ID,
                MarketErrors::InvalidAccount
            );

            system_program::transfer(
                CpiContext::new(
                    system_program,
                    system_program::Transfer {
                        from: rent_payer,
                        to: listing_data,
                    },
                ),
                top_up_lamports,
            )?;
        }

        Ok(true)
    }
}
