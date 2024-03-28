use anchor_lang::prelude::*;

use crate::{ AuthRole, AuthorityRole, Market, MarketErrors, ADMIN_ROLE, MARKET_ACCOUNT, OPERATOR_ROLE, MarketStatus};

#[derive(Accounts)]
pub struct AdminInstruction<'info> {
    #[account( 
        mut,
        seeds = [MARKET_ACCOUNT],
        bump,
        constraint = market.admin == admin_account.key() @ MarketErrors::AdminAccountInvalid,
        constraint = market.operator == operator_account.key() @ MarketErrors::OperatorAccountInvalid,
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
        mut,
        seeds = [OPERATOR_ROLE], 
        bump=operator_account.bump,
        constraint = operator_account.role == AuthRole::Operator @ MarketErrors::OnlyOperator,
        constraint = operator_account.status == true @ MarketErrors::OnlyOperator,
    )]
    pub operator_account:  Account<'info, AuthorityRole>,

    #[account(mut, signer)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>, 
}

pub fn update_operator(ctx: Context<AdminInstruction>, operator: Pubkey) -> Result<()> {
    let operator_account = &mut ctx.accounts.operator_account;

    require_keys_neq!(
        operator_account.authority,
        operator,
        MarketErrors::OperatorNotChange
    );

    operator_account.set_authority(operator);

    Ok(())
}

pub fn set_status(ctx: Context<AdminInstruction>, status: MarketStatus) -> Result<()> {
    let market = &mut ctx.accounts.market;

    market.set_status(status);
    Ok(())
}

// pub fn set_public(ctx: Context<AdminInstruction>, public: bool) -> Result<()> {
//     let market = &mut ctx.accounts.market;

//     market.set_public(public);
//     Ok(())
// }