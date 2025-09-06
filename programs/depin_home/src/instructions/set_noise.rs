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
        seeds = [b"temp", user.key().as_ref()],
        bump,
    )]
    pub noise: Account<'info, Noise>,

    #[account(     
        seeds = [b"user".as_ref(), user.key().as_ref()],
        bump = user_config.bump
    )]
    pub user_config: Account<'info, UserConfig>,

    pub system_program: Program<'info, System>,
}

impl<'info> SetNoise<'info> {
    pub fn set_noise(&mut self, value: u32) -> Result<()> {
        let noise = &mut self.noise;
        noise.value = value;
        noise.updated_at = Clock::get()?.unix_timestamp;

         // Update user noise points
        self.user_config.noise_data_points += 1;
        Ok(())
    }
}
