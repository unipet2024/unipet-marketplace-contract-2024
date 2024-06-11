use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use crate::{
    AuthRole, AuthorityRole, ListingData, ListingEvent, Market, MarketErrors, LISTING_ACCOUNT,
    MARKET_ACCOUNT, OPERATOR_ROLE,
};

#[derive(Accounts)]
pub struct ListingByOperator<'info> {
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
        constraint = operator_account.is_authority(authority.key) == true @ MarketErrors::OnlyOperator,
        constraint = operator_account.status == true @ MarketErrors::OnlyOperator,
    )]
    pub operator_account:  Box<Account<'info, AuthorityRole>>,

    #[account(
        init,
        space = 8 + 90,
        payer = authority,
        seeds = [LISTING_ACCOUNT, mint.key().as_ref()],
        bump
    )]
    pub listing_account: Box<Account<'info, ListingData>>,

    pub mint: Box<Account<'info, Mint>>,
    #[account(mut, signer)]
    pub authority: Signer<'info>,
    // pub associated_token_program: Program<'info, AssociatedToken>,
    // pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn listing_by_operator_handler(
    ctx: Context<ListingByOperator>,
    currency: Pubkey,
    price: u64,
) -> Result<()> {
    let market = &mut ctx.accounts.market;
    let listing_account = &mut ctx.accounts.listing_account;
    // let operator_account = &mut ctx.accounts.operator_account;
    // let token_program = &ctx.accounts.token_program;
    let authority = &ctx.accounts.authority;

    msg!("Currency: {:}", currency);

    //check currency supported
    require!(
        market.check_currency_support(&currency) == true,
        MarketErrors::CurrencyNotSupport
    );

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
