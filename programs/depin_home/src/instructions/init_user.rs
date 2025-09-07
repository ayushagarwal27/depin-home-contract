use anchor_lang::prelude::*;

use crate::state::UserConfig;

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        seeds = [b"user", user.key().as_ref()],
        space = 8 + UserConfig::INIT_SPACE,
        bump
    )]
    pub user_config: Box<Account<'info, UserConfig>>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeUser<'info> {
    pub fn initialize_user(&mut self, bumps: &InitializeUserBumps) -> Result<()> {
        self.user_config.set_inner(UserConfig {
            points: 0,
            temp_data_points: 0,
            temp_data_count: 0,
            noise_data_points: 0,
            noise_data_count: 0,
            bump: bumps.user_config,
        });
        Ok(())
    }
}
