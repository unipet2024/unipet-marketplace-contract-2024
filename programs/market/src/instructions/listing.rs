use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use anchor_spl::token::Transfer;

use crate::constants::*;
use crate::error::*;
use crate::events::*;
use crate::state::*;
use crate::types::*;

#[derive(Accounts)]
#[instruction(listing_params: Vec<ListingParam>)]
pub struct Listing<'info> {
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

    #[account(
        mut,
        associated_token::mint = mint_0,
        associated_token::authority = authority
    )]
    pub from_0: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = mint_1,
        associated_token::authority = authority
    )]
    pub from_1: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = mint_2,
        associated_token::authority = authority
    )]
    pub from_2: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = mint_3,
        associated_token::authority = authority
    )]
    pub from_3: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = mint_4,
        associated_token::authority = authority
    )]
    pub from_4: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer=authority,
        associated_token::mint = mint_0,
        associated_token::authority = market,
    )]
    pub to_0: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer=authority,
        associated_token::mint = mint_1,
        associated_token::authority = market,
    )]
    pub to_1: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer=authority,
        associated_token::mint = mint_2,
        associated_token::authority = market,
    )]
    pub to_2: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer=authority,
        associated_token::mint = mint_3,
        associated_token::authority = market,
    )]
    pub to_3: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer=authority,
        associated_token::mint = mint_4,
        associated_token::authority = market,
    )]
    pub to_4: Box<Account<'info, TokenAccount>>,

    pub mint_0: Box<Account<'info, Mint>>,
    pub mint_1: Box<Account<'info, Mint>>,
    pub mint_2: Box<Account<'info, Mint>>,
    pub mint_3: Box<Account<'info, Mint>>,
    pub mint_4: Box<Account<'info, Mint>>,
    #[account(mut, signer)]
    pub authority: Signer<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl Listing<'_> {
    fn validate(&self, listing_params: &Vec<ListingParam>) -> Result<()> {
        require_gte!(
            MAX_LISTING as usize,
            listing_params.len(),
            MarketErrors::InputInvalid
        );

        require_gte!(listing_params.len(), 0, MarketErrors::InputInvalid);

        //check market status
        require!(
            self.market.status == MarketStatus::Public,
            MarketErrors::MatketNotOpen
        );
        for listing_param in listing_params.iter() {
            //check currency supported
            require!(
                self.market.check_currency_support(&listing_param.currency) == true,
                MarketErrors::CurrencyNotSupport
            );
        }
        Ok(())
    }

    #[access_control(ctx.accounts.validate(&listing_params))]
    pub fn listing_handler(ctx: Context<Self>, listing_params: Vec<ListingParam>) -> Result<()> {
        let market = &mut ctx.accounts.market;
        let market_storage = &mut ctx.accounts.market_storage;
        let token_program = &ctx.accounts.token_program;
        let authority = &ctx.accounts.authority;

        let froms = vec![
            &ctx.accounts.from_0,
            &ctx.accounts.from_1,
            &ctx.accounts.from_2,
            &ctx.accounts.from_3,
            &ctx.accounts.from_4,
        ];

        let tos = vec![
            &ctx.accounts.to_0,
            &ctx.accounts.to_1,
            &ctx.accounts.to_2,
            &ctx.accounts.to_3,
            &ctx.accounts.to_4,
        ];

        let mints = vec![
            &ctx.accounts.mint_0,
            &ctx.accounts.mint_1,
            &ctx.accounts.mint_2,
            &ctx.accounts.mint_3,
            &ctx.accounts.mint_4,
        ];

        let mut listing_items = vec![];

        let clock = Clock::get()?;

        for (index, param) in listing_params.iter().enumerate() {
            if mints[index].key() != Pubkey::default() {
                // check mint
                match market_storage.get_item_index(mints[index].key()) {
                    Some(_) => return err!(MarketErrors::ItemAlreadyExist),
                    _ => 0,
                };

                //transfer NFT to market
                msg!("Transfer NFT to market");

                anchor_spl::token::transfer(
                    CpiContext::new(
                        token_program.to_account_info(),
                        Transfer {
                            from: froms[index].to_account_info(),
                            to: tos[index].to_account_info(),
                            authority: ctx.accounts.authority.to_account_info(),
                        },
                    ),
                    1,
                )?;

                listing_items.push(ListingItem {
                    owner: authority.key(),
                    mint: mints[index].key(),
                    currency: param.currency,
                    price: param.price,
                    listingtime: clock.unix_timestamp,
                    opentime: clock.unix_timestamp + market.duration,
                });

                //emit event
                emit!(ListingEvent {
                    user: authority.key(),
                    mint: mints[index].key(),
                    currency: param.currency,
                    price: param.price,
                    listing_time: clock.unix_timestamp,
                    open_time: clock.unix_timestamp + market.duration,
                    slot: clock.slot,
                });
            }
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
