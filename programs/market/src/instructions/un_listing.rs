use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use anchor_spl::token::{transfer, Transfer};

use crate::constants::*;
use crate::error::*;
use crate::events::*;
use crate::state::*;
// use crate::types::*;

#[derive(Accounts)]
pub struct UnListing<'info> {
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
        mut,
        associated_token::mint = mint_0,
        associated_token::authority = market,
    )]
    pub to_0: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = mint_1,
        associated_token::authority = market,
    )]
    pub to_1: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = mint_2,
        associated_token::authority = market,
    )]
    pub to_2: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = mint_3,
        associated_token::authority = market,
    )]
    pub to_3: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
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

impl UnListing<'_> {
    pub fn unlisting_handler(ctx: Context<Self>) -> Result<()> {
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

        let clock = Clock::get()?;

        for (index, _) in mints.iter().enumerate() {
            if mints[index].key() != Pubkey::default() {
                // check mint
                // let item_index = match market_storage.get_item_index(mints[index].key()) {
                //     Some(item_index) => item_index,
                //     None => return err!(MarketErrors::ItemNotFound),
                // };

                //check owner
                let listing_item = match market_storage.get_item(mints[index].key()) {
                    Ok(listing_item) => listing_item,
                    Err(_) => return err!(MarketErrors::ItemNotFound),
                };
                require_keys_eq!(listing_item.owner, authority.key(), MarketErrors::OnlyOwner);

                // let listing_item = market_storage

                // Transfer NFt from market to user
                let seeds: &[&[u8]] = &[MARKET_ACCOUNT, &[market.bump]];
                let signer = &[&seeds[..]];
                transfer(
                    CpiContext::new(
                        token_program.to_account_info(),
                        Transfer {
                            from: froms[index].to_account_info(),
                            to: tos[index].to_account_info(),
                            authority: market.to_account_info(),
                        },
                    )
                    .with_signer(signer),
                    1,
                )?;

                //emit event UnListing
                emit!(UnListingEvent {
                    user: ctx.accounts.authority.key(),
                    mint: mints[index].key(),
                    time: clock.unix_timestamp,
                    slot: clock.slot,
                });

                market_storage.remove_item(mints[index].key())?;
            }
        }

        Ok(())
    }
}
