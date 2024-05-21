use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use anchor_spl::token::transfer;
use anchor_spl::token::Transfer;
use solana_program::system_instruction;

use crate::{
    BuyEvent, ListingData, ListingStatus, Market, MarketErrors, LISTING_ACCOUNT, MARKET_ACCOUNT,
};

#[derive(Accounts)]
pub struct BuyWithSOL<'info> {
    #[account(
        mut,
        seeds = [MARKET_ACCOUNT],
        bump=market.bump,
    )]
    pub market: Box<Account<'info, Market>>,

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = nft_mint,
        associated_token::authority = buyer
    )]
    pub nft_to: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = market,
    )]
    pub nft_from: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [LISTING_ACCOUNT, nft_mint.key().as_ref()],
        bump=listing_account.bump,
        // constraint = listing_account.owner == seller.key() @ MarketErrors::InputInvalid,
        // constraint = listing_account.status == ListingStatus::Listing @ MarketErrors::ItemNotFound,
    )]
    pub listing_account: Box<Account<'info, ListingData>>,

    pub nft_mint: Box<Account<'info, Mint>>,
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
    let listing_account = &mut ctx.accounts.listing_account;
    msg!("Call to Listing account");

    validate(
        &listing_account,
        &seller.key(),
        buyer.to_account_info().lamports(),
    )?;

    msg!("Call to Validate");

    //calculate commisison
    let commission_amount = market.commission * listing_account.price / 100;
    let seller_amount = listing_account.price - commission_amount;

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
            ctx.accounts.nft_mint.to_account_info(),
            Transfer {
                from: ctx.accounts.nft_from.to_account_info(),
                to: ctx.accounts.nft_to.to_account_info(),
                authority: market.to_account_info(),
            },
        )
        .with_signer(signer),
        1,
    )?;

    //update listing account
    listing_account.status = ListingStatus::Close;

    emit!(BuyEvent {
        buyer: buyer.key(),
        seller: listing_account.owner,
        mint: ctx.accounts.nft_mint.key(),
        currency: ctx.accounts.market.key(),
        price: listing_account.price,
        commission: commission_amount,
        time: Clock::get()?.unix_timestamp
    });

    Ok(())
}

fn validate(listing_account: &Account<ListingData>, seller: &Pubkey, amount: u64) -> Result<()> {
    require!(
        listing_account.status == ListingStatus::Listing,
        MarketErrors::ItemNotFound
    );

    // SET currency = market.address in case SOL
    require_eq!(
        listing_account.currency,
        Pubkey::try_from("11111111111111111111111111111111").unwrap(),
        MarketErrors::InputInvalid
    );

    require_gte!(
        amount,
        listing_account.price,
        MarketErrors::InsufficientAmount
    );

    require_eq!(listing_account.owner, *seller, MarketErrors::InputInvalid);

    let current = Clock::get()?.unix_timestamp;
    msg!("Current:{:}", current);
    msg!("Open time:{:}", listing_account.opentime);

    require_gte!(
        current,
        listing_account.opentime,
        MarketErrors::ItemStillLock
    );
    Ok(())
}
