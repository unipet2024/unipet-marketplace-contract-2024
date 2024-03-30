use anchor_lang::prelude::*;

use crate::{ListingData, ListingStatus, Market, MarketErrors, LISTING_ACCOUNT, MARKET_ACCOUNT};

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token},
};

#[derive(Accounts)]
pub struct UpdatePrice<'info> {
    #[account(
        seeds = [MARKET_ACCOUNT],
        bump=market.bump,
    )]
    pub market: Box<Account<'info, Market>>,

    #[account(
        mut,
        seeds = [LISTING_ACCOUNT, mint.key().as_ref()],
        bump=listing_account.bump,
        constraint = listing_account.owner == authority.key() @ MarketErrors::OnlyOwner,
    )]
    pub listing_account: Account<'info, ListingData>,

    pub mint: Account<'info, Mint>,
    #[account(mut, signer)]
    pub authority: Signer<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn update_price_handler(ctx: Context<UpdatePrice>, currency: Pubkey, price: u64) -> Result<()> {
    let market = &mut ctx.accounts.market;
    let listing_account = &mut ctx.accounts.listing_account;
    // let operator_account = &mut ctx.accounts.operator_account;
    // let token_program = &ctx.accounts.token_program;
    let authority = &ctx.accounts.authority;

    //check authority is seller
    require_eq!(
        listing_account.owner,
        authority.key(),
        MarketErrors::InputInvalid
    );

    //check nft listed
    require!(
        listing_account.status == ListingStatus::Listing,
        MarketErrors::ItemNotFound
    );

    //check currency supported
    require!(
        market.check_currency_support(&currency) == true,
        MarketErrors::CurrencyNotSupport
    );

    //update listing account
    listing_account.currency = currency;
    listing_account.price = price;

    Ok(())
}
