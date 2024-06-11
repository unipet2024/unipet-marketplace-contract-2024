use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use anchor_spl::token::Transfer;
use anchor_spl::token::{transfer, Transfer as SplTransfer};

use crate::{
    BuyEvent, ListingData, ListingStatus, Market, MarketErrors, LISTING_ACCOUNT, MARKET_ACCOUNT,
};

#[derive(Accounts)]
pub struct BuyWithSPL<'info> {
    #[account(
        mut,
        seeds = [MARKET_ACCOUNT],
        bump=market.bump,
    )]
    pub market: Box<Account<'info, Market>>,
    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = currency_mint,
        associated_token::authority = market
    )]
    pub currency_market: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = nft_mint,
        associated_token::authority = buyer
    )]
    pub nft_to: Box<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = currency_mint,
        associated_token::authority = seller
    )]
    pub currency_to: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = market,
    )]
    pub nft_from: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = currency_mint,
        associated_token::authority = buyer
    )]
    pub currency_from: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [LISTING_ACCOUNT, nft_mint.key().as_ref()],
        bump=listing_account.bump,
        close= seller,
        // constraint = listing_account.owner == seller.key() @ MarketErrors::InputInvalid,
        // constraint = listing_account.status == ListingStatus::Listing @ MarketErrors::ItemNotFound,
    )]
    pub listing_account: Box<Account<'info, ListingData>>,

    pub nft_mint: Box<Account<'info, Mint>>,
    pub currency_mint: Box<Account<'info, Mint>>,
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
    let currency_mint = &ctx.accounts.currency_mint;
    let currency_from = &ctx.accounts.currency_from;
    let currency_to = &ctx.accounts.currency_to;
    let currency_market = &ctx.accounts.currency_market;
    let seller = &ctx.accounts.seller;
    let listing_account = &mut ctx.accounts.listing_account;

    validate(
        &listing_account,
        seller.key(),
        currency_mint.key(),
        currency_from.amount,
    )?;

    //calculate commisison
    let commission_amount = market.commission * listing_account.price / 100;
    let seller_amount = listing_account.price - commission_amount;

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

    let clock = Clock::get()?;
    emit!(BuyEvent {
        buyer: buyer.key(),
        seller: listing_account.owner,
        mint: ctx.accounts.nft_mint.key(),
        currency: ctx.accounts.currency_mint.key(),
        price: listing_account.price,
        commission: commission_amount,
        time: clock.unix_timestamp,
        slot: clock.slot,
    });

    Ok(())
}

fn validate(
    listing_account: &Account<ListingData>,
    seller: Pubkey,
    currency_mint: Pubkey,
    amount: u64,
) -> Result<()> {
    require!(
        listing_account.status == ListingStatus::Listing,
        MarketErrors::ItemNotFound
    );

    require_eq!(
        listing_account.currency,
        currency_mint,
        MarketErrors::InputInvalid
    );

    require_gte!(
        amount,
        listing_account.price,
        MarketErrors::InsufficientAmount
    );

    require_eq!(listing_account.owner, seller, MarketErrors::InputInvalid);

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
