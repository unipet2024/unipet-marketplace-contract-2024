use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use anchor_spl::token::Transfer;
use anchor_spl::token::{transfer, Transfer as SplTransfer};

use crate::constants::*;
use crate::error::*;
use crate::events::*;
use crate::state::*;

#[derive(Accounts)]
pub struct BuyWithSPL<'info> {
    #[account(
        mut,
        seeds = [MARKET_ACCOUNT],
        bump=market.bump,
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
        init_if_needed,
        payer = buyer,
        associated_token::mint = currency,
        associated_token::authority = market
    )]
    pub currency_market: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = mint,
        associated_token::authority = buyer
    )]
    pub nft_to: Box<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = currency,
        associated_token::authority = seller
    )]
    pub currency_to: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = market,
    )]
    pub nft_from: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = currency,
        associated_token::authority = buyer
    )]
    pub currency_from: Box<Account<'info, TokenAccount>>,

    pub mint: Box<Account<'info, Mint>>,
    pub currency: Box<Account<'info, Mint>>,
    #[account(mut, signer)]
    pub buyer: Signer<'info>,

    /// CHECK: read only
    pub seller: UncheckedAccount<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn buy_with_spl_hanlder(ctx: Context<BuyWithSPL>) -> Result<()> {
    // let mint = &mut ctx.accounts.mint;
    let market = &ctx.accounts.market;
    let buyer = &ctx.accounts.buyer;
    let currency = &ctx.accounts.currency;
    let currency_from = &ctx.accounts.currency_from;
    let currency_to = &ctx.accounts.currency_to;
    let currency_market = &ctx.accounts.currency_market;
    let seller = &ctx.accounts.seller;
    let market_storage = &mut ctx.accounts.market_storage;

    let listing_item = match market_storage.get_item(ctx.accounts.mint.key()) {
        Ok(listing_item) => listing_item,
        Err(_) => return err!(MarketErrors::ItemNotFound),
    };

    validate(
        &listing_item,
        &seller.key(),
        &ctx.accounts.mint.key(),
        &currency.key(),
        currency_from.amount,
    )?;

    //calculate commisison
    let commission_amount = market.commission * listing_item.price / 100;
    let seller_amount = listing_item.price - commission_amount;

    //transfer amount to seller

    msg!("Transfer currency from seller to buyer");
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            SplTransfer {
                authority: buyer.to_account_info(),
                from: currency_from.to_account_info(),
                to: currency_to.to_account_info(),
            },
        ),
        seller_amount,
    )?;

    //transfer commistion to market
    msg!("Transfer commission from seller to market");
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            SplTransfer {
                authority: buyer.to_account_info(),
                from: currency_from.to_account_info(),
                to: currency_market.to_account_info(),
            },
        ),
        commission_amount,
    )?;

    //transfer NFT to buyer
    msg!("Transfer NFT tobuyer");
    let seeds: &[&[u8]] = &[MARKET_ACCOUNT, &[market.bump]];
    let signer = &[&seeds[..]];
    transfer(
        CpiContext::new(
            ctx.accounts.mint.to_account_info(),
            Transfer {
                from: ctx.accounts.nft_from.to_account_info(),
                to: ctx.accounts.nft_to.to_account_info(),
                authority: market.to_account_info(),
            },
        )
        .with_signer(signer),
        1,
    )?;

    // Remove item
    market_storage.remove_item(ctx.accounts.mint.key())?;

    let clock = Clock::get()?;
    emit!(BuyEvent {
        buyer: buyer.key(),
        seller: listing_item.owner,
        mint: ctx.accounts.mint.key(),
        currency: ctx.accounts.currency.key(),
        price: listing_item.price,
        commission: commission_amount,
        time: clock.unix_timestamp,
        slot: clock.slot,
    });

    Ok(())
}

fn validate(
    listing_item: &ListingItem,
    seller: &Pubkey,
    mint: &Pubkey,
    currency: &Pubkey,
    amount: u64,
) -> Result<()> {
    // Check owner of item
    require!(listing_item.owner == *seller, MarketErrors::InputInvalid);

    //check mint
    require_keys_eq!(listing_item.mint, *mint, MarketErrors::InputInvalid);

    // SET currency = market.address in case SOL
    require_eq!(listing_item.currency, *currency, MarketErrors::InputInvalid);

    require_gte!(amount, listing_item.price, MarketErrors::InsufficientAmount);

    let current = Clock::get()?.unix_timestamp;
    msg!("Current:{:}", current);
    msg!("Open time:{:}", listing_item.opentime);

    require_gte!(current, listing_item.opentime, MarketErrors::ItemStillLock);
    Ok(())
}
