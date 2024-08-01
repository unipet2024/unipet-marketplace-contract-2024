use anchor_lang::prelude::*;

use crate::{AuthRole, AuthorityRole, CurrencyParams, Market, MarketStorage};
use crate::constants::*;

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(
        init,  
        payer = authority, 
        space =8 + 300,
        seeds = [MARKET_ACCOUNT],
        bump
    )]
    pub market: Box<Account<'info, Market>>,
    #[account(
        init,  
        payer = authority, 
        space = MarketStorage::size(100), // init with 100 items
        seeds = [MARKET_STORAGE_ACCOUNT],
        bump
    )]
    pub market_storage: Box<Account<'info, MarketStorage>>,
    #[account(
        init,
        space = 8 + 40, // 1 admin
        payer = authority,
        seeds = [ADMIN_ROLE], 
        bump,
    )]
    pub admin_account:  Box<Account<'info, AuthorityRole>>,
    #[account(
        init,
        space = 8+170, // max 5 operator
        payer = authority,
        seeds = [OPERATOR_ROLE], 
        bump,
    )]
    pub operator_account:  Box<Account<'info, AuthorityRole>>,

    #[account(mut, signer)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>, 
}

pub fn init_handler(ctx: Context<Init>,duration: i64, currencies: CurrencyParams, commission: u64) -> Result<()> {
    let market = &mut ctx.accounts.market;
    let market_storage = &mut ctx.accounts.market_storage;
    let admin_account = &mut ctx.accounts.admin_account;
    let operator_account = &mut ctx.accounts.operator_account;


    market.init(
        admin_account.key(),
        operator_account.key(),
        market_storage.key(),
        ctx.bumps.market,
        duration,
        &currencies.currency,
        commission
    )?;

    // SET MARKET STORAGE
    market_storage.init(ctx.bumps.market_storage)?;
    
   // SET ADMIN
   let authorities = vec![ctx.accounts.authority.key()];
   admin_account.initialize(
       &authorities,
       ctx.bumps.admin_account,
       AuthRole::Admin,
   )?;

   // SET OPERATOR
   operator_account.initialize(
       &authorities,
       ctx.bumps.operator_account,
       AuthRole::Operator,
   )?;


    Ok(())
}
