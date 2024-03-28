use anchor_lang::prelude::*;

#[derive(PartialEq, Eq, AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum MarketStatus {
    Waiting,
    Private,
    Public,
    Close,
}
