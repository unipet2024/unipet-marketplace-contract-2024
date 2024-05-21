use crate::{ListingData, ListingStatus, Market, MarketErrors, LISTING_ACCOUNT, MARKET_ACCOUNT};
use anchor_lang::prelude::*;
use anchor_spl::token::transfer;
use anchor_spl::token::Transfer;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct UnListing<'info> {
    #[account(
        mut,
        seeds = [MARKET_ACCOUNT],
        bump=market.bump
    )]
    pub market: Box<Account<'info, Market>>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = market
    )]
    pub from: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = authority,
    )]
    pub to: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [LISTING_ACCOUNT, mint.key().as_ref()],
        bump=listing_account.bump,
        constraint = listing_account.owner == authority.key() @ MarketErrors::OnlyOwner,
        constraint = listing_account.status == ListingStatus::Listing @ MarketErrors::ItemNotFound,
    )]
    pub listing_account: Box<Account<'info, ListingData>>,

    pub mint: Box<Account<'info, Mint>>,
    #[account(mut, signer)]
    pub authority: Signer<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn un_listing_handler(ctx: Context<UnListing>) -> Result<()> {
    let market = &mut ctx.accounts.market;
    let listing_account = &mut ctx.accounts.listing_account;
    // let operator_account = &mut ctx.accounts.operator_account;
    let token_program = &ctx.accounts.token_program;
    // let authority = &ctx.accounts.authority;

    //check nft listed
    require!(
        listing_account.status == ListingStatus::Listing,
        MarketErrors::ItemNotFound
    );

    //transfer NFT back to seller

    let seeds: &[&[u8]] = &[MARKET_ACCOUNT, &[market.bump]];
    let signer = &[&seeds[..]];
    transfer(
        CpiContext::new(
            token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.from.to_account_info(),
                to: ctx.accounts.to.to_account_info(),
                authority: market.to_account_info(),
            },
        )
        .with_signer(signer),
        1,
    )?;

    //update listing account
    // listing_account.status = ListingStatus::Close;
    listing_account.un_listing()?;

    Ok(())
}
