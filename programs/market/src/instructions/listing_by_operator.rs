use anchor_lang::prelude::*;
// use anchor_spl::{associated_token::get_associated_token_address, token::Mint};
// use solana_client::rpc_client::RpcClient;

use crate::{
    AuthRole, AuthorityRole, ListingDataOperator, ListingDataParam, ListingEventOperator, Market,
    MarketErrors, LISTING_ACCOUNT_OPERATOR, MARKET_ACCOUNT, OPERATOR_ROLE,
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
    pub operator_account: Box<Account<'info, AuthorityRole>>,

    #[account(
        init,
        space = 8 + 90,
        payer = authority,
        // seeds = [LISTING_ACCOUNT, mint.key().as_ref()],
        seeds = [LISTING_ACCOUNT_OPERATOR],
        bump
    )]
    pub listing_account_operator: Box<Account<'info, ListingDataOperator>>,

    // pub mint: Box<Account<'info, Mint>>,
    #[account(mut, signer)]
    pub authority: Signer<'info>,
    // pub associated_token_program: Program<'info, AssociatedToken>,
    // pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn listing_by_operator_handler(
    ctx: Context<ListingByOperator>,
    listing_datas: Vec<ListingDataParam>,
) -> Result<()> {
    validate(&ctx, &listing_datas)?;

    // let market = &mut ctx.accounts.market;
    let listing_account_operator = &mut ctx.accounts.listing_account_operator;
    // let operator_account = &mut ctx.accounts.operator_account;
    // // let token_program = &ctx.accounts.token_program;
    // let authority = &ctx.accounts.authority;

    //Update listing_account
    // let current = Clock::get()?.unix_timestamp;

    listing_account_operator.listing(&listing_datas, ctx.bumps.listing_account_operator)?;

    emit!(ListingEventOperator {
        operator: ctx.accounts.operator_account.key(),
        listing_datas,
        slot: Clock::get()?.slot
    });
    
    Ok(())
}

fn validate(ctx: &Context<ListingByOperator>, listing_datas: &Vec<ListingDataParam>) -> Result<()> {
    for listing_data in listing_datas.iter() {
        //validate currency
        require_eq!(
            ctx.accounts
                .market
                .check_currency_support(&listing_data.currency),
            true,
            MarketErrors::CurrencyNotSupport
        );
        //validate balance
        // let ata = get_associated_token_address(
        //     ctx.accounts.market.to_account_info().key,
        //     &listing_data.mint,
        // );

        // let connection = RpcClient::new("https://intensive-dimensional-shape.solana-devnet.quiknode.pro/9f13ebb0af09474b28825684a57cdd891b7734d9/".to_string());

        // let account_data = connection.get_token_account_balance(&ata).unwrap();
        // msg!("Token amount: {:}", account_data.ui_amount_string);
    }
    Ok(())
}
