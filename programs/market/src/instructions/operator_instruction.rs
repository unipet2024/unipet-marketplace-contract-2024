use anchor_lang::prelude::*;

use crate::{AuthRole, AuthorityRole, CurrencyParams, Market, MarketErrors, MARKET_ACCOUNT, OPERATOR_ROLE};

#[derive(Accounts)]
pub struct OperatorInstruction<'info> {
    #[account( 
        mut,
        seeds = [MARKET_ACCOUNT],
        bump,
        constraint = market.operator == operator_account.key() @ MarketErrors::OperatorAccountInvalid,
    )]
    pub market: Box<Account<'info, Market>>,

    #[account(
        seeds = [OPERATOR_ROLE], 
        bump,
        constraint = operator_account.is_authority(operator.key) == true @ MarketErrors::OnlyOperator,
        constraint = operator_account.role == AuthRole::Operator @ MarketErrors::OnlyOperator,
        constraint = operator_account.status == true @ MarketErrors::OnlyOperator,
    )]
    pub operator_account:  Account<'info, AuthorityRole>,

    #[account(mut, signer)]
    pub operator: Signer<'info>,
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