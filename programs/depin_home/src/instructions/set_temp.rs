use anchor_lang::prelude::*;

use crate::state::{Temp, UserConfig};

#[derive(Accounts)]
pub struct SetTemp<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + Temp::INIT_SPACE,
        seeds = [b"temp", user.key().as_ref()],
        bump,
    )]
    pub temp: Box<Account<'info, Temp>>,

    #[account(     
        mut,
        seeds = [b"user".as_ref(), user.key().as_ref()],
        bump = user_config.bump
    )]
    pub user_config: Box<Account<'info, UserConfig>>,

    pub system_program: Program<'info, System>,
}

impl<'info> SetTemp<'info> {
    pub fn set_temp(&mut self, value: u32) -> Result<()> {
        let temp = &mut self.temp;
        temp.value = value;
        temp.updated_at = Clock::get()?.unix_timestamp;

        // Update user temp points
        self.user_config.temp_data_points += 1;
        Ok(())
    }
}
