use anchor_lang::prelude::*;

use crate::{ AuthRole, AuthorityRole, Market, MarketErrors, MarketStatus, SetAuthorityEvent, ADMIN_ROLE, MARKET_ACCOUNT, OPERATOR_ROLE};

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
        constraint = admin_account.is_authority(admin.key) == true @ MarketErrors::OnlyAdmin,
        constraint = admin_account.role == AuthRole::Admin @ MarketErrors::OnlyAdmin,
        constraint = admin_account.status == true @ MarketErrors::OnlyAdmin,
    )]
    pub admin_account:  Box<Account<'info, AuthorityRole>>,

    #[account(
        mut,
        seeds = [OPERATOR_ROLE], 
        bump=operator_account.bump,
        constraint = operator_account.role == AuthRole::Operator @ MarketErrors::OnlyOperator,
        constraint = operator_account.status == true @ MarketErrors::OnlyOperator,
    )]
    pub operator_account:  Box<Account<'info, AuthorityRole>>,

    #[account(mut, signer)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>, 
}

pub fn set_authority_handler(ctx: Context<AdminInstruction>, role: AuthRole, operators: Vec<Pubkey>) -> Result<()> {
    match role {
        AuthRole::Operator => set_operator_handler(ctx, operators),
        AuthRole::Admin => set_admin_handler(ctx, operators),
    }
}

fn set_operator_handler(ctx: Context<AdminInstruction>, operators: Vec<Pubkey>) -> Result<()> {
    let operator_account = &mut ctx.accounts.operator_account;

    for operator in operators.iter(){
        msg!("{:},", *operator)
    }

    operator_account.set_authorities(&operators)?;

    emit!(SetAuthorityEvent{
        admin: ctx.accounts.admin.key(),
        role: AuthRole::Operator,
        operators,
        time: Clock::get()?.unix_timestamp
    });

    Ok(())
}

fn set_admin_handler(ctx: Context<AdminInstruction>, admins: Vec<Pubkey>) -> Result<()> {
    let admin_account = &mut ctx.accounts.admin_account;

    admin_account.set_authorities(&admins)?;

    emit!(SetAuthorityEvent{
        admin: ctx.accounts.admin.key(),
        role: AuthRole::Admin,
        operators: admins,
        time: Clock::get()?.unix_timestamp
    });

    Ok(())
}
pub fn set_status_handler(ctx: Context<AdminInstruction>, status: MarketStatus) -> Result<()> {
    let market = &mut ctx.accounts.market;

    market.set_status(status);
    Ok(())
}