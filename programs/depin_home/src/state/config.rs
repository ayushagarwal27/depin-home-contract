use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub admin: Pubkey,
    pub rewards_bump: u8,
    pub reward_amount_temp: u16,
    pub reward_amount_noise: u16,
    pub bump: u8,
}
