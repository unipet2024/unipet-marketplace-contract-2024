use anchor_lang::prelude::*;

#[constant]
pub const ADMIN_ROLE: &[u8] = b"ADMIN_ROLE";
pub const OPERATOR_ROLE: &[u8] = b"OPERATOR_ROLE";
pub const TOKEN_LIST: &[u8] = b"TOKEN_LIST";
pub const MARKET_ACCOUNT: &[u8] = b"MARKET_ACCOUNT";
pub const MARKET_STORAGE_ACCOUNT: &[u8] = b"MARKET_STORAGE_ACCOUNT";
pub const LISTING_ACCOUNT: &[u8] = b"LISTING_ACCOUNT";


pub const MAX_LISTING: u8 = 5;