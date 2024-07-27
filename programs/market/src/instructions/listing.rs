use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

// use anchor_spl::associated_token::get_associated_token_address;

use anchor_spl::token::Transfer;

use crate::{
    ListingData, ListingEvent, Market, MarketErrors, MarketStatus, LISTING_ACCOUNT, MARKET_ACCOUNT,
};

use crate::constants::*;
use crate::types::*;

// maximum 5 listing account
#[derive(Accounts)]
#[instruction(listing_params: Vec<ListingParam>)]
pub struct Listing<'info> {
    #[account(
        seeds = [MARKET_ACCOUNT],
        bump=market.bump,
    )]
    pub market: Box<Account<'info, Market>>,

    // #[account(
    //     init_if_needed,
    //     payer = authority,
    //     associated_token::mint = mint,
    //     associated_token::authority = market,
    // )]
    // pub to: Box<Account<'info, TokenAccount>>,

    // #[account(
    //     init,
    //     space = 8 + 90,
    //     payer = authority,
    //     // seeds = [LISTING_ACCOUNT],
    //     seeds = [LISTING_ACCOUNT, mint.key().as_ref()],
    //     bump
    // )]
    // pub listing_account: Box<Account<'info, ListingData>>,
    #[account(
        mut,
        associated_token::mint = mint_0,
        associated_token::authority = authority
    )]
    pub from_0: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer=authority,
        associated_token::mint = mint_0,
        associated_token::authority = market,
    )]
    pub to_0: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [LISTING_ACCOUNT, mint_0.key().as_ref()],
        bump
    )]
    pub listing_account_0: Box<Account<'info, ListingData>>,

    #[account(
        mut,
        associated_token::mint = mint_1,
        associated_token::authority = authority
    )]
    pub from_1: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer=authority,
        associated_token::mint = mint_1,
        associated_token::authority = market,
    )]
    pub to_1: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [LISTING_ACCOUNT, mint_1.key().as_ref()],
        bump
    )]
    pub listing_account_1: Box<Account<'info, ListingData>>,

    #[account(
        mut,
        associated_token::mint = mint_2,
        associated_token::authority = authority
    )]
    pub from_2: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer=authority,
        associated_token::mint = mint_2,
        associated_token::authority = market,
    )]
    pub to_2: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [LISTING_ACCOUNT, mint_2.key().as_ref()],
        bump
    )]
    pub listing_account_2: Box<Account<'info, ListingData>>,

    #[account(
        mut,
        associated_token::mint = mint_3,
        associated_token::authority = authority
    )]
    pub from_3: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer=authority,
        associated_token::mint = mint_3,
        associated_token::authority = market,
    )]
    pub to_3: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [LISTING_ACCOUNT, mint_3.key().as_ref()],
        bump
    )]
    pub listing_account_3: Box<Account<'info, ListingData>>,

    #[account(
        mut,
        associated_token::mint = mint_4,
        associated_token::authority = authority
    )]
    pub from_4: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer=authority,
        associated_token::mint = mint_4,
        associated_token::authority = market,
    )]
    pub to_4: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [LISTING_ACCOUNT, mint_4.key().as_ref()],
        bump
    )]
    pub listing_account_4: Box<Account<'info, ListingData>>,

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
        require_eq!(
            listing_params.len(),
            MAX_LISTING as usize,
            MarketErrors::InputInvalid
        );

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
        // let listing_account = &mut ctx.accounts.listing_account;
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

        let mut listing_accounts = vec![
            &mut ctx.accounts.listing_account_0,
            &mut ctx.accounts.listing_account_1,
            &mut ctx.accounts.listing_account_2,
            &mut ctx.accounts.listing_account_3,
            &mut ctx.accounts.listing_account_4,
        ];

        let bumps = vec![
            ctx.bumps.listing_account_0,
            ctx.bumps.listing_account_1,
            ctx.bumps.listing_account_2,
            ctx.bumps.listing_account_3,
            ctx.bumps.listing_account_4,
        ];

        let mints = vec![
            &ctx.accounts.mint_0,
            &ctx.accounts.mint_1,
            &ctx.accounts.mint_2,
            &ctx.accounts.mint_3,
            &ctx.accounts.mint_4,
        ];

        let current = Clock::get()?.unix_timestamp;

        for (index, param) in listing_params.iter().enumerate() {
            if mints[index].key() != Pubkey::default() {
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

                //Update listing_account
                ListingData::realloc_if_needed(
                    listing_accounts[index].to_account_info(),
                    ctx.accounts.authority.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                )?;

                listing_accounts[index].listing(
                    &authority.key(),
                    &param.currency,
                    param.price,
                    current,
                    current + market.duration,
                    bumps[index],
                )?;

                //emit event
                emit!(ListingEvent {
                    user: authority.key(),
                    mint: mints[index].key(),
                    currency: param.currency,
                    price: param.price,
                    listing_time: current,
                    open_time: current + market.duration,
                    slot: Clock::get()?.slot,
                });
            }
        }

        Ok(())
    }
}
