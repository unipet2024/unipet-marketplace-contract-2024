use anchor_lang::prelude::*;

use crate::{AuthRole, AuthorityRole, CurrencyParams, Market, MarketErrors, ADMIN_ROLE, MARKET_ACCOUNT};

#[derive(Accounts)]
pub struct OperatorInstruction<'info> {
    #[account( 
        mut,
        seeds = [MARKET_ACCOUNT],
        bump,
        constraint = market.admin == admin_account.key() @ MarketErrors::AdminAccountInvalid,
    )]
    pub market: Box<Account<'info, Market>>,

    #[account(
        seeds = [ADMIN_ROLE], 
        bump,
        constraint = admin_account.authority == admin.key() @ MarketErrors::OnlyAdmin,
        constraint = admin_account.role == AuthRole::Admin @ MarketErrors::OnlyAdmin,
        constraint = admin_account.status == true @ MarketErrors::OnlyAdmin,
    )]
    pub admin_account:  Account<'info, AuthorityRole>,

    #[account(mut, signer)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>, 
}

pub fn set_duration_handler(ctx: Context<OperatorInstruction>, duration: i64) -> Result<()> {
    let market = &mut ctx.accounts.market;

    market.duration = duration;
    Ok(())
}

pub fn set_currencies_handler(ctx: Context<OperatorInstruction>, currencies: CurrencyParams) -> Result<()> {
    let market = &mut ctx.accounts.market;

    market.currencies = currencies.currency;
    Ok(())
}