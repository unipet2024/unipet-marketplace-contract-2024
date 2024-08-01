pub mod constants;
pub mod error;
pub mod events;
pub mod instructions;
pub mod state;
pub mod types;

use anchor_lang::prelude::*;

pub use constants::*;
pub use error::*;
pub use events::*;
pub use instructions::*;
pub use state::*;
pub use types::*;

declare_id!("BHefxRSoYeaBqhHh4SHK8R6HcSzvVNfrVjK347cdGVbB");

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

    pub fn set_authority(
        ctx: Context<AdminInstruction>,
        role: AuthRole,
        operators: Vec<Pubkey>,
    ) -> Result<()> {
        admin_instruction::set_authority_handler(ctx, role, operators)
    }

    pub fn set_status(ctx: Context<AdminInstruction>, status: MarketStatus) -> Result<()> {
        admin_instruction::set_status_handler(ctx, status)
    }

    pub fn listing(ctx: Context<Listing>, listing_params: Vec<ListingParam>) -> Result<()> {
        Listing::listing_handler(ctx, listing_params)
    }

    pub fn listing_by_operator(
        ctx: Context<ListingByOperator>,
        listing_params: Vec<MintListingParam>,
    ) -> Result<()> {
        ListingByOperator::listing_by_operator_handler(ctx, listing_params)
    }

    pub fn buy_with_spl(ctx: Context<BuyWithSPL>) -> Result<()> {
        buy_with_spl::buy_with_spl_hanlder(ctx)
    }

    pub fn buy_with_sol(ctx: Context<BuyWithSOL>) -> Result<()> {
        buy_with_sol::buy_with_sol_hanlder(ctx)
    }

    pub fn update_price(ctx: Context<UpdatePrice>, listing_params: Vec<MintListingParam>,) -> Result<()> {
        UpdatePrice::update_price_handler(ctx, listing_params)
    }

    pub fn un_listing(ctx: Context<UnListing>) -> Result<()> {
        UnListing::unlisting_handler(ctx)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        withdraw::withdraw_handler(ctx, amount)
    }
}
