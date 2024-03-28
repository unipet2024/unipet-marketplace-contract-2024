use anchor_lang::prelude::*;

use crate::AuthRole;

#[account]
pub struct AuthorityRole {
    pub bump: u8,          //1
    pub status: bool,      //1
    pub authority: Pubkey, //32
    pub role: AuthRole,    //1+1
}

impl AuthorityRole {
    pub fn initialize(&mut self, authority: Pubkey, bump: u8, role: AuthRole) -> Result<()> {
        self.authority = authority;
        self.bump = bump;
        self.role = role;

        self.status = true;
        Ok(())
    }

    pub fn set_authority(&mut self, authority: Pubkey) {
        self.authority = authority;
    }
}
