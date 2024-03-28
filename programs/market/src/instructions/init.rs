use anchor_lang::prelude::*;

use crate::{AuthRole, AuthorityRole, Market, CurrencyParams, ADMIN_ROLE, MARKET_ACCOUNT, OPERATOR_ROLE};


#[derive(Accounts)]
pub struct Init<'info> {
    #[account(
        init_if_needed,  
        payer = authority, 
        space =8 + 250,
        seeds = [MARKET_ACCOUNT],
        bump
    )]
    pub market: Box<Account<'info, Market>>,
    #[account(
        init_if_needed,
        space = 60,
        payer = authority,
        seeds = [ADMIN_ROLE], 
        bump,
    )]
    pub admin_account:  Account<'info, AuthorityRole>,
    #[account(
        init_if_needed,
        space = 60,
        payer = authority,
        seeds = [OPERATOR_ROLE], 
        bump,
    )]
    pub operator_account:  Account<'info, AuthorityRole>,

    #[account(mut, signer)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>, 
}

pub fn handle(ctx: Context<Init>,duration: i64, currencies: CurrencyParams, commission: u64) -> Result<()> {
    let market = &mut ctx.accounts.market;
    let admin_account = &mut ctx.accounts.admin_account;
    let operator_account = &mut ctx.accounts.operator_account;


    market.init(
        admin_account.key(),
        operator_account.key(),
        ctx.bumps.market,
        duration,
        &currencies.currency,
        commission
    )?;

    //SET ADMIN
    admin_account.initialize(
        ctx.accounts.authority.key(),
        ctx.bumps.admin_account,
        AuthRole::Admin,
    )?;

    //SET OPERATOR ROLE FOR ADMIN
    operator_account.initialize(
        ctx.accounts.authority.key(),
        ctx.bumps.operator_account,
        AuthRole::Operator,
    )?;

    Ok(())
}
