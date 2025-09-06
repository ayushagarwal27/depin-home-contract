use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{mint_to, Mint, MintTo, Token, TokenAccount};

use crate::state::{Config, UserConfig};

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
         mut,
         seeds = [b"user".as_ref(), user.key().as_ref()],
         bump = user_config.bump
    )]
    pub user_config: Account<'info, UserConfig>,

    #[account(
        seeds = [b"config"],
        bump = config_account.bump
    )]
    pub config_account: Account<'info, Config>,

    #[account(
        mut,
        seeds = [b"rewards".as_ref()],
        bump = config_account.rewards_bump,
    )]
    pub rewards_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = rewards_mint,
        associated_token::authority = user,
    )]
    pub rewards_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Claim<'info> {
    pub fn claim(&mut self) -> Result<()> {
        // Mint Tokens to User Rewards ATA
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: self.rewards_mint.to_account_info(),
            authority: self.config_account.to_account_info(),
            to: self.rewards_ata.to_account_info(),
        };

        let seeds = &[b"config".as_ref()];
        let signer_seeds = &[&seeds[..]];

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        let total_points = self.user_config.noise_data_points + self.user_config.temp_data_points;

        mint_to(
            cpi_context,
            total_points as u64 * 10_u64.pow(self.rewards_mint.decimals as u32),
        )?;

        // Make user points to zero
        self.user_config.noise_data_points = 0;
        self.user_config.temp_data_points = 0;

        Ok(())
    }
}
