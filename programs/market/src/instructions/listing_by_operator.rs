use crate::constants::*;
use crate::error::*;
use crate::state::*;
use crate::types::*;
use crate::ListingEvent;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(listing_params: Vec<MintListingParam>)]
pub struct ListingByOperator<'info> {
    #[account(
        seeds = [MARKET_ACCOUNT],
        bump=market.bump,
        constraint = market.operator == operator_account.key() @ MarketErrors::MarketStorageInvalid,
    )]
    pub market: Box<Account<'info, Market>>,

    #[account(
        mut,
        seeds = [MARKET_STORAGE_ACCOUNT],
        bump= market_storage.bump
    )]
    pub market_storage: Box<Account<'info, MarketStorage>>,

    #[account(
        seeds = [OPERATOR_ROLE],
        bump=operator_account.bump,
        constraint = operator_account.role == AuthRole::Operator @ MarketErrors::OnlyOperator,
        constraint = operator_account.is_authority(authority.key) == true @ MarketErrors::OnlyOperator,
        constraint = operator_account.status == true @ MarketErrors::OnlyOperator,
    )]
    pub operator_account: Box<Account<'info, AuthorityRole>>,

    #[account(mut, signer)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl ListingByOperator<'_> {
    fn validate(&self, listing_params: &Vec<MintListingParam>) -> Result<()> {
        require_gte!(listing_params.len(), 0, MarketErrors::InputInvalid);

        for listing_param in listing_params.iter() {
            // check currency supported
            require!(
                self.market.check_currency_support(&listing_param.currency) == true,
                MarketErrors::CurrencyNotSupport
            );

            // check mint
            match self.market_storage.get_item_index(listing_param.mint) {
                Some(_) => return err!(MarketErrors::ItemAlreadyExist),
                _ => 0,
            };
        }
        Ok(())
    }

    #[access_control(ctx.accounts.validate(&listing_params))]
    pub fn listing_by_operator_handler(
        ctx: Context<Self>,
        listing_params: Vec<MintListingParam>,
    ) -> Result<()> {
        let market = &mut ctx.accounts.market;
        let market_storage = &mut ctx.accounts.market_storage;
        let authority = &ctx.accounts.authority;

        let mut listing_items = vec![];

        let current = Clock::get()?.unix_timestamp;

        for (_, param) in listing_params.iter().enumerate() {
            if param.mint != Pubkey::default() {
                listing_items.push(ListingItem {
                    owner: authority.key(),
                    mint: param.mint,
                    currency: param.currency,
                    price: param.price,
                    listingtime: current,
                    opentime: current + market.duration,
                });
            }

            //emit event
            emit!(ListingEvent {
                user: authority.key(),
                mint: param.mint,
                currency: param.currency,
                price: param.price,
                listing_time: current,
                open_time: current + market.duration,
                slot: Clock::get()?.slot,
            });
        }

        //Update listing_account
        MarketStorage::realloc_if_needed(
            market_storage.to_account_info(),
            market_storage.items.len() + listing_items.len(),
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        )?;

        market_storage.add_items(listing_items)?;

        Ok(())
    }
}
