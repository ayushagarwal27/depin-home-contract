use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Noise {
    pub value: u32,
    pub updated_at: i64,
    pub bump: u8,
}
