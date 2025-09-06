use anchor_lang::prelude::*;

declare_id!("qmMusrZUZS5QfYMKWF7sCwNNXjxB73eoWknuGTwqZFU");

mod instructions;
mod state;

use instructions::*;

#[program]
pub mod depin_home {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        reward_amount_temp: u16,
        reward_amount_noise: u16,
    ) -> Result<()> {
        ctx.accounts
            .initialize(reward_amount_temp, reward_amount_noise, &ctx.bumps)
    }
    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        ctx.accounts.initialize_user(&ctx.bumps)
    }

    pub fn set_temp(ctx: Context<SetTemp>, value: u32) -> Result<()> {
        ctx.accounts.set_temp(value)
    }
    pub fn set_noise(ctx: Context<SetNoise>, value: u32) -> Result<()> {
        ctx.accounts.set_noise(value)
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        ctx.accounts.claim()
    }
}
