use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdatePrice {}

pub fn handler(ctx: Context<UpdatePrice>) -> Result<()> {
    Ok(())
}
