use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use anchor_spl::token::Transfer;

use crate::{
    AuthRole, AuthorityRole, ListingData, ListingEvent, ListingStatus, Market, MarketErrors, MarketStatus, LISTING_ACCOUNT, MARKET_ACCOUNT, OPERATOR_ROLE
};

#[derive(Accounts)]
pub struct Listing<'info> {
    #[account(
        seeds = [MARKET_ACCOUNT],
        bump=market.bump,
        constraint = market.operator == operator_account.key() @ MarketErrors::OperatorAccountInvalid,
    )]
    pub market: Box<Account<'info, Market>>,

    #[account(
        seeds = [OPERATOR_ROLE], 
        bump=operator_account.bump,
        constraint = operator_account.role == AuthRole::Operator @ MarketErrors::OnlyOperator,
        constraint = operator_account.status == true @ MarketErrors::OnlyOperator,
    )]
    pub operator_account:  Box<Account<'info, AuthorityRole>>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = authority
    )]
    pub from: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = market,
    )]
    pub to: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        space = 8 + 90,
        payer = authority,
        // seeds = [LISTING_ACCOUNT],
        seeds = [LISTING_ACCOUNT, mint.key().as_ref()],
        bump
    )]
    pub listing_account: Box<Account<'info, ListingData>>,

    pub mint: Box<Account<'info, Mint>>,
    #[account(mut, signer)]
    pub authority: Signer<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn listing_handler(ctx: Context<Listing>, currency: Pubkey, price: u64) -> Result<()> {
    let market = &mut ctx.accounts.market;
    let listing_account = &mut ctx.accounts.listing_account;
    let operator_account = &mut ctx.accounts.operator_account;
    let token_program = &ctx.accounts.token_program;
    let authority = &ctx.accounts.authority;

    msg!("Currency: {:}", currency);

    //check market status
    require!(
        (market.status == MarketStatus::Public) || 
        (market.status == MarketStatus::Private &&  //if private only operator have right to listing
            (operator_account.is_authority(authority.key) == true )
        ),
        MarketErrors::MatketNotOpen
    );

    //check currency supported
    // market.check_currency_support(&currency);
    require!(
        market.check_currency_support(&currency) == true,
        MarketErrors::CurrencyNotSupport
    );

    //check nft listed
    msg!("Listing status: {:?}", listing_account.status);
    require!(
        listing_account.status != ListingStatus::Listing,
        MarketErrors::ListingAlready
    );

    //transfer NFT to market
    msg!("Transfer NFT to market");
    anchor_spl::token::transfer(
        CpiContext::new(
            token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.from.to_account_info(),
                to: ctx.accounts.to.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        1,
    )?;

    //Update listing_account
    let current = Clock::get()?.unix_timestamp;

    listing_account.listing(
        &authority.key(),
        &currency,
        price,
        current,
        current + market.duration,
        ctx.bumps.listing_account,
    )?;

    //emit event
    emit!(ListingEvent {
        user: authority.key(),
        mint: ctx.accounts.mint.key(),
        currency: currency,
        price: price,
        listing_time: current,
        open_time: current + market.duration,
        slot: Clock::get()?.slot,
    });

    Ok(())
}

