use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserConfig {
    pub points: u32,
    pub temp_data_points: u64,
    pub noise_data_points: u64,
    pub bump: u8,
}
