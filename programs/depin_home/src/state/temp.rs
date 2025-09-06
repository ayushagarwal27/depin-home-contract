use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Temp {
    pub value: u32,
    pub updated_at: i64,
    pub bump: u8,
}
