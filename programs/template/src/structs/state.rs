use anchor_lang::prelude::*;

#[account(zero_copy)]
#[derive(PartialEq, Default, Debug)]
pub struct State {
    pub admin: Pubkey,
    pub nonce: u8,
    pub authority: Pubkey,
    pub bump: u8,
}
