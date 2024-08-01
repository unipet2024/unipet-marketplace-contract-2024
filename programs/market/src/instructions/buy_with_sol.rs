use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use anchor_spl::token::transfer;
use anchor_spl::token::Transfer;
use solana_program::system_instruction;

use crate::constants::*;
use crate::error::*;
use crate::events::*;
use crate::state::*;
// use crate::types::*;

#[derive(Accounts)]
pub struct BuyWithSOL<'info> {
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
        associated_token::mint = mint,
        associated_token::authority = buyer
    )]
    pub nft_to: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = market,
    )]
    pub nft_from: Box<Account<'info, TokenAccount>>,

    pub mint: Box<Account<'info, Mint>>,
    #[account(mut, signer)]
    pub buyer: Signer<'info>,

    /// CHECK: for sending sol
    #[account(mut)]
    pub seller: AccountInfo<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn buy_with_sol_hanlder(ctx: Context<BuyWithSOL>) -> Result<()> {
    // let mint = &mut ctx.accounts.mint;
    let market = &ctx.accounts.market;
    msg!("Call to Market");
    let seller = &ctx.accounts.seller;
    msg!("Call to Seller");
    let buyer = &ctx.accounts.buyer;
    msg!("Call to Buyer");
    let market_storage = &mut ctx.accounts.market_storage;

    let listing_item = match market_storage.get_item(ctx.accounts.mint.key()) {
        Ok(listing_item) => listing_item,
        Err(_) => return err!(MarketErrors::ItemNotFound),
    };

    validate(
        &listing_item,
        &ctx.accounts.mint.key(),
        &seller.key(),
        buyer.to_account_info().lamports(),
    )?;

    msg!("Call to Validate");

    // market_storage.close(sol_destination);

    //calculate commisison
    let commission_amount = market.commission * listing_item.price / 100;
    let seller_amount = listing_item.price - commission_amount;

    msg!("Commisison : {:} - {:}", commission_amount, seller_amount);

    //transfer amount to seller

    msg!("Transfer SOL from seller to buyer");
    let mut transfer_instruction =
        system_instruction::transfer(buyer.key, seller.key, seller_amount);

    // Invoke the transfer instruction
    anchor_lang::solana_program::program::invoke_signed(
        &transfer_instruction,
        &[
            buyer.to_account_info(),
            seller.clone(),
            ctx.accounts.system_program.to_account_info(),
        ],
        &[],
    )?;

    //transfer commistion to market
    msg!("Transfer commission from buyer to market");
    transfer_instruction =
        system_instruction::transfer(buyer.key, &market.key(), commission_amount);

    // Invoke the transfer instruction
    anchor_lang::solana_program::program::invoke_signed(
        &transfer_instruction,
        &[
            buyer.to_account_info(),
            market.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        &[],
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
        currency: ctx.accounts.market.key(),
        price: listing_item.price,
        commission: commission_amount,
        time: clock.unix_timestamp,
        slot: clock.slot,
    });

    Ok(())
}

fn validate(listing_item: &ListingItem, mint: &Pubkey, seller: &Pubkey, amount: u64) -> Result<()> {
    // Check owner of item
    require!(listing_item.owner == *seller, MarketErrors::InputInvalid);

    //check mint
    require_keys_eq!(listing_item.mint, *mint, MarketErrors::InputInvalid);

    // SET currency = market.address in case SOL
    require_eq!(
        listing_item.currency,
        Pubkey::try_from("11111111111111111111111111111111").unwrap(),
        MarketErrors::InputInvalid
    );

    require_gte!(amount, listing_item.price, MarketErrors::InsufficientAmount);

    let current = Clock::get()?.unix_timestamp;
    msg!("Current:{:}", current);
    msg!("Open time:{:}", listing_item.opentime);

    require_gte!(current, listing_item.opentime, MarketErrors::ItemStillLock);
    Ok(())
}
