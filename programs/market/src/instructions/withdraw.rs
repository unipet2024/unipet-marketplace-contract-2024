use anchor_lang::prelude::*;

use anchor_spl::associated_token::AssociatedToken;
// use anchor_spl::token::{transfer, Transfer as SplTransfer};
use anchor_spl::token::{transfer,Mint, Token, TokenAccount, Transfer as SplTransfer};

use crate::{
    AuthRole, AuthorityRole, Market, MarketErrors, WithdrawEvent, ADMIN_ROLE, MARKET_ACCOUNT
};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account( 
        seeds = [MARKET_ACCOUNT],
        bump,
        constraint = market.admin == admin_account.key() @ MarketErrors::AdminAccountInvalid,
    )]
    pub market: Box<Account<'info, Market>>,

    #[account(
        seeds = [ADMIN_ROLE], 
        bump=admin_account.bump,
        constraint = admin_account.authority == admin.key() @ MarketErrors::OnlyAdmin,
        constraint = admin_account.role == AuthRole::Admin @ MarketErrors::OnlyAdmin,
        constraint = admin_account.status == true @ MarketErrors::OnlyAdmin,
    )]
    pub admin_account:  Account<'info, AuthorityRole>,

    #[account(
        init_if_needed,
        payer=admin,
        associated_token::mint = mint,
        associated_token::authority = admin
    )]
    pub currency_admin_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = market
    )]
    pub currency_market_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,

    #[account(mut, signer)]
    pub admin: Signer<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}


pub fn withdraw_handler(ctx: Context<Withdraw>, amount: u64) -> Result<()>{
    //transfer commistion to market
    msg!("Transfer commission from seller to market");
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            SplTransfer {
                authority: ctx.accounts.market.to_account_info(),
                from: ctx.accounts.currency_market_account.to_account_info(),
                to: ctx.accounts.currency_admin_account.to_account_info(),
            },
        ),
        amount,
    )?;

    emit!(WithdrawEvent{
        admin: ctx.accounts.admin.key(),
        currency: ctx.accounts.mint.key(),
        amount,
        time: Clock::get()?.unix_timestamp
    });

    Ok(())
}
