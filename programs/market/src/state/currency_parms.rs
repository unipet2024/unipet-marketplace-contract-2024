use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct CurrencyParams {
    pub currency: Vec<Pubkey>,
}
