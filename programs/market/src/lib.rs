pub mod constants;
pub mod error;
pub mod events;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use error::*;
pub use events::*;
pub use instructions::*;
pub use state::*;

declare_id!("7ScnRwX7fYPQbc126PPtMYdgHSE9zhbXLAcYY6rqgAEx");

#[program]
pub mod market {
    use super::*;

    pub fn init(
        ctx: Context<Init>,
        duration: i64,
        currencies: CurrencyParams,
        commission: u64,
    ) -> Result<()> {
        init::init_handler(ctx, duration, currencies, commission)
    }

    pub fn set_duration(ctx: Context<OperatorInstruction>, duration: i64) -> Result<()> {
        operator_instruction::set_duration_handler(ctx, duration)
    }

    pub fn set_currencies(
        ctx: Context<OperatorInstruction>,
        currencies: CurrencyParams,
    ) -> Result<()> {
        operator_instruction::set_currencies_handler(ctx, currencies)
    }

    pub fn set_operator(ctx: Context<AdminInstruction>, operator: Pubkey) -> Result<()> {
        admin_instruction::update_operator(ctx, operator)
    }

    pub fn set_status(ctx: Context<AdminInstruction>, status: MarketStatus) -> Result<()> {
        admin_instruction::set_status_handler(ctx, status)
    }

    pub fn listing(ctx: Context<Listing>, currency: Pubkey, price: u64) -> Result<()> {
        listing::listing_handler(ctx, currency, price)
    }

    pub fn buy_with_spl(ctx: Context<BuyWithSPL>) -> Result<()> {
        buy_with_spl::buy_with_spl_hanlder(ctx)
    }

    pub fn buy_with_sol(ctx: Context<BuyWithSOL>) -> Result<()> {
        buy_with_sol::buy_with_sol_hanlder(ctx)
    }

    pub fn update_price(ctx: Context<UpdatePrice>, currency: Pubkey, price: u64) -> Result<()> {
        update_price::update_price_handler(ctx, currency, price)
    }

    pub fn un_listing(ctx: Context<UnListing>) -> Result<()> {
        un_listing::un_listing_handler(ctx)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        withdraw::withdraw_handler(ctx, amount)
    }
}
