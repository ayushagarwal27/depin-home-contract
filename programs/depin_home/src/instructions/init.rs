use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::state::Config;

#[derive(Accounts)]
#[instruction(name:String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        seeds = [b"config"],
        space = 8 + Config::INIT_SPACE,
        bump,
    )]
    pub config: Account<'info, Config>,

    #[account(
        init,
        payer = admin,
        seeds = [b"rewards"],
        bump,
        mint::decimals = 6,
        mint::authority = config,
    )]
    pub rewards_mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(
        &mut self,
        reward_amount_temp: u16,
        reward_amount_noise: u16,
        bumps: &InitializeBumps,
    ) -> Result<()> {
        // Initialize marketplace
        self.config.set_inner(Config {
            admin: self.admin.key(),
            rewards_bump: bumps.rewards_mint,
            reward_amount_temp,
            reward_amount_noise,
            bump: bumps.config,
        });
        Ok(())
    }
}
