use anchor_lang::prelude::*;

#[error_code]
pub enum MarketErrors {
    #[msg("Item not found")]
    ItemNotFound,

    #[msg("Item still lock")]
    ItemStillLock,

    #[msg("Listing already")]
    ListingAlready,

    #[msg("Market not open")]
    MatketNotOpen,

    #[msg("Currency not support")]
    CurrencyNotSupport,

    // #[msg("Market status invalid")]
    // MatketStatusInvalid,
    #[msg("Admin account invalid")]
    AdminAccountInvalid,

    #[msg("Operator account invalid")]
    OperatorAccountInvalid,

    #[msg("Only admin")]
    OnlyAdmin,

    #[msg("Only Operator")]
    OnlyOperator,

    #[msg("Operator not change")]
    OperatorNotChange,

    #[msg("Input invalid")]
    InputInvalid,

    #[msg("Insufficient amount")]
    InsufficientAmount,

    #[msg("Only owner")]
    OnlyOwner,
}

impl From<MarketErrors> for ProgramError {
    fn from(e: MarketErrors) -> Self {
        ProgramError::Custom(e as u32)
    }
}
