use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub admin: Pubkey,
    pub rewards_bump: u8,
    pub reward_amount_temp: u8,
    pub reward_amount_noise: u8,
    pub bump: u8,
}
