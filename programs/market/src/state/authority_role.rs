use anchor_lang::prelude::*;

use crate::AuthRole;

#[account]
pub struct AuthorityRole {
    pub bump: u8,                 //1
    pub status: bool,             //1
    pub authorities: Vec<Pubkey>, //4 + 32*5 =
    pub role: AuthRole,           //1
}
impl AuthorityRole {
    pub fn initialize(
        &mut self,
        authorities: &Vec<Pubkey>,
        bump: u8,
        role: AuthRole,
    ) -> Result<()> {
        self.set_authorities(authorities)?;
        self.bump = bump;
        self.status = true;
        self.role = role;
        Ok(())
    }

    pub fn set_authorities(&mut self, authorities: &Vec<Pubkey>) -> Result<()> {
        self.authorities = vec![];

        for authority in authorities.iter() {
            self.add_authority(authority)?;
        }

        Ok(())
    }

    pub fn add_authority(&mut self, authority: &Pubkey) -> Result<()> {
        if !self.is_authority(authority) {
            self.authorities.push(*authority);
        }

        Ok(())
    }

    pub fn is_authority(&self, authority: &Pubkey) -> bool {
        for authority_check in self.authorities.iter() {
            if *authority == *authority_check {
                return true;
            }
        }

        false
    }

    pub fn set_status_account(&mut self, status: bool) {
        self.status = status;
    }

    pub fn set_role(&mut self, role: AuthRole) {
        self.role = role;
    }
}
