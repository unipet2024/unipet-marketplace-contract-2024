use anchor_lang::prelude::*;

use crate::constants::*;
use crate::error::*;
use crate::events::*;
use crate::state::*;
use crate::types::*;

#[derive(Accounts)]
pub struct UpdatePrice<'info> {
    #[account(
        seeds = [MARKET_ACCOUNT],
        bump = market.bump,
        constraint = market.market_storage == market_storage.key() @ MarketErrors::MarketStorageInvalid,
    )]
    pub market: Box<Account<'info, Market>>,

    #[account(
        mut,
        seeds = [MARKET_STORAGE_ACCOUNT],
        bump = market_storage.bump
    )]
    pub market_storage: Box<Account<'info, MarketStorage>>,

    #[account(mut, signer)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl UpdatePrice<'_> {
    pub fn update_price_handler(
        ctx: Context<Self>,
        listing_params: Vec<MintListingParam>,
    ) -> Result<()> {
        let market = &mut ctx.accounts.market;
        let market_storage = &mut ctx.accounts.market_storage;

        let authority = &ctx.accounts.authority;

        for listing_param in listing_params.iter() {
            require_keys_neq!(
                listing_param.mint,
                Pubkey::default(),
                MarketErrors::InputInvalid
            );

            let mut listing_item = market_storage.get_item(listing_param.mint)?;

            require_keys_eq!(
                listing_item.owner,
                authority.key(),
                MarketErrors::OwnerInvalid
            );

            //check currency supported
            require!(
                market.check_currency_support(&listing_item.currency) == true,
                MarketErrors::CurrencyNotSupport
            );

            listing_item.currency = listing_param.currency;
            listing_item.price = listing_param.price;

            market_storage.update_item(listing_item)?;
        }

        let clock = Clock::get()?;

        emit!(ChangePriceEvent {
            user: authority.key(),
            items: listing_params,
            time: clock.unix_timestamp,
            slot: clock.slot
        });

        Ok(())
    }
}
