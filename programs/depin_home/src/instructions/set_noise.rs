use anchor_lang::prelude::*;

use crate::state::{Noise, UserConfig};

#[derive(Accounts)]
pub struct SetNoise<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + Noise::INIT_SPACE,
        seeds = [b"noise", user.key().as_ref(), &user_config.noise_data_count.to_le_bytes()],
        bump,
    )]
    pub noise: Box<Account<'info, Noise>>,

    #[account(     
        mut,
        seeds = [b"user".as_ref(), user.key().as_ref()],
        bump = user_config.bump
    )]
    pub user_config: Box<Account<'info, UserConfig>>,

    pub system_program: Program<'info, System>,
}

impl<'info> SetNoise<'info> {
    pub fn set_noise(&mut self, value: u32) -> Result<()> {
        let noise = &mut self.noise;
        noise.value = value;
        noise.updated_at = Clock::get()?.unix_timestamp;

         // Update user noise points
        self.user_config.noise_data_points += 1;
        self.user_config.noise_data_count += 1;
        Ok(())
    }
}
