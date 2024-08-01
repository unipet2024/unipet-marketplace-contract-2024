use anchor_lang::prelude::*;

use crate::id;
use crate::MarketErrors;
use anchor_lang::system_program;

#[account]
pub struct MarketStorage {
    pub items: Vec<ListingItem>,
    pub bump: u8, //1
}

impl MarketStorage {
    pub fn size(items_length: usize) -> usize {
        8 +  // anchor account discriminator
        1 +  // bump 
        4 +  // vector init
        items_length * ListingItem::INIT_SPACE
    }
    pub fn init(&mut self, bump: u8) -> Result<()> {
        self.bump = bump;

        Ok(())
    }

    pub fn add_item(&mut self, item: ListingItem) -> Result<()> {
        self.items.push(item);

        self.items.sort_by_key(|m| m.mint);
        Ok(())
    }

    pub fn add_items(&mut self, items: Vec<ListingItem>) -> Result<()> {
        for item in items.iter() {
            self.items.push(*item);
        }

        self.items.sort_by_key(|m| m.mint);
        Ok(())
    }

    pub fn update_item(&mut self, item: ListingItem) -> Result<()> {
        let index = match self.get_item_index(item.mint) {
            Some(index) => index,
            None => return err!(MarketErrors::ItemNotFound),
        };

        self.items[index].currency = item.currency;
        self.items[index].price = item.price;

        Ok(())
    }

    pub fn get_item_index(&self, mint: Pubkey) -> Option<usize> {
        self.items.binary_search_by_key(&mint, |m| m.mint).ok()
    }

    pub fn get_item(&self, mint: Pubkey) -> Result<ListingItem> {
        let item = match self.get_item_index(mint) {
            Some(item_index) => self.items[item_index],
            None => return err!(MarketErrors::ItemNotFound),
        };

        Ok(item)
    }

    pub fn remove_item(&mut self, mint: Pubkey) -> Result<()> {
        let old_item_index = match self.get_item_index(mint) {
            Some(old_item_index) => old_item_index,
            None => return err!(MarketErrors::ItemNotFound),
        };

        self.items.remove(old_item_index);

        Ok(())
    }

    /// Check if the market_storage account space needs to be reallocated .
    /// Returns `true` if the account was reallocated.
    pub fn realloc_if_needed<'a>(
        market_storage: AccountInfo<'a>,
        items_length: usize,
        rent_payer: AccountInfo<'a>,
        system_program: AccountInfo<'a>,
    ) -> Result<bool> {
        require_keys_eq!(
            *market_storage.owner,
            id(),
            MarketErrors::IllegalAccountOwner
        );

        let current_account_size = market_storage.data.borrow().len();
        let account_size_to_fit_items = MarketStorage::size(items_length);

        // Check if we need to reallocate space.
        if current_account_size >= account_size_to_fit_items {
            return Ok(false);
        }

        // Reallocate more space.
        AccountInfo::realloc(&market_storage, account_size_to_fit_items, false)?;

        // If more lamports are needed, transfer them to the account.
        let rent_exempt_lamports = Rent::get()
            .unwrap()
            .minimum_balance(account_size_to_fit_items)
            .max(1);
        let top_up_lamports =
            rent_exempt_lamports.saturating_sub(market_storage.to_account_info().lamports());

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
                        to: market_storage,
                    },
                ),
                top_up_lamports,
            )?;
        }

        Ok(true)
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, InitSpace, Eq, PartialEq, Clone, Copy)]
pub struct ListingItem {
    pub owner: Pubkey,    //32
    pub mint: Pubkey,     //32
    pub currency: Pubkey, //32
    pub price: u64,       //16
    pub listingtime: i64, //4
    pub opentime: i64,    //4
}

impl ListingItem {
    pub fn add_item(
        &mut self,
        owner: &Pubkey,
        mint: &Pubkey,
        currency: &Pubkey,
        price: u64,
        listingtime: i64,
        opentime: i64,
    ) -> Result<()> {
        self.owner = *owner;
        self.mint = *mint;
        self.currency = *currency;
        self.price = price;
        self.listingtime = listingtime;
        self.opentime = opentime;

        Ok(())
    }
}
