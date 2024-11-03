use anchor_lang::predule::*;

#[account]
pub struct StakeConfig{
    pub points_per_user: u8,
    pub max_staked: u8,
    pub freeze_perios: u32,
    pub rewards_bump: u8,
}

impl Space for UserAccount{
    const INIT_SPACE: usize = 8 + 1 + 1 + 4 + 1 + 1;
}