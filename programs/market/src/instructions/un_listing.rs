use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UnListing {}

pub fn handler(ctx: Context<UnListing>) -> Result<()> {
    Ok(())
}
