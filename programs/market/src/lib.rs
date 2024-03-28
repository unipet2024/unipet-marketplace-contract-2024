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

    pub fn set_duration(ctx: Context<OperatorInstruction>, duration: i64) -> Result<()> {
        operator_instruction::set_duration(ctx, duration)
    }

    pub fn set_currencies(
        ctx: Context<OperatorInstruction>,
        currencies: CurrencyParams,
    ) -> Result<()> {
        operator_instruction::set_currencies(ctx, currencies)
    }

    pub fn set_operator(ctx: Context<AdminInstruction>, operator: Pubkey) -> Result<()> {
        admin_instruction::update_operator(ctx, operator)
    }

    pub fn set_status(ctx: Context<AdminInstruction>, status: MarketStatus) -> Result<()> {
        admin_instruction::set_status(ctx, status)
    }

    pub fn listing(ctx: Context<Listing>, currency: Pubkey, price: u64) -> Result<()> {
        listing::listing(ctx, currency, price)
    }

    pub fn buy(ctx: Context<Buy>) -> Result<()> {
        buy::handler(ctx)
    }

    pub fn update_price(ctx: Context<UpdatePrice>) -> Result<()> {
        update_price::handler(ctx)
    }

    pub fn un_listing(ctx: Context<UnListing>) -> Result<()> {
        un_listing::handler(ctx)
    }
}
