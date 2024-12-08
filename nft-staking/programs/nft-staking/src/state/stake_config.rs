use anchor_lang::prelude::*;
//one for the platform
#[account]
pub struct StakeConfig{
    pub points_per_user: u8,
    pub max_staked: u8,
    pub freeze_period: u32,
    pub rewards_bump: u8,
    pub bump: u8,
}

impl Space for StakeConfig{
    const INIT_SPACE: usize = 8 + 1 + 1 + 4 + 1 + 1;
}