use anchor_lang::prelude::*;

use crate::MarketStatus;

// total 230
#[account]
pub struct Market {
    pub admin: Pubkey,          //32
    pub operator: Pubkey,       //32
    pub market_storage: Pubkey, //32
    // pub vault: Pubkey,           //32
    pub duration: i64,           //8
    pub currencies: Vec<Pubkey>, // Max 5 => 4+ 32*5=164
    pub commission: u64,         //8
    pub status: MarketStatus,    //
    // pub public: bool,            //1
    pub bump: u8, //1
}

impl Market {
    pub fn init(
        &mut self,
        admin: Pubkey,
        operator: Pubkey,
        market_storage: Pubkey,
        bump: u8,
        duration: i64,
        currencies: &Vec<Pubkey>,
        commission: u64,
    ) -> Result<()> {
        self.admin = admin;
        self.operator = operator;
        self.market_storage = market_storage;
        self.duration = duration;
        self.commission = commission;
        // self.vault = vault;
        self.bump = bump;
        self.status = MarketStatus::Waiting;
        // self.public = false;

        self.set_currencies(&currencies)?;

        Ok(())
    }

    pub fn set_currencies(&mut self, currencies: &Vec<Pubkey>) -> Result<()> {
        self.currencies = vec![];
        for (_, token) in currencies.iter().enumerate() {
            self.add_currency(*token);
        }
        Ok(())
    }

    fn add_currency(&mut self, token: Pubkey) {
        self.currencies.push(token)
    }

    pub fn check_currency_support(&self, token: &Pubkey) -> bool {
        if self.currencies.contains(token) {
            msg!("Found currency");
            return true;
        } else {
            msg!("Not found currency");
            return false;
        }
    }

    pub fn set_status(&mut self, status: MarketStatus) {
        self.status = status;
    }

    // pub fn set_public(&mut self, public: bool) {
    //     self.public = public;
    // }
}
