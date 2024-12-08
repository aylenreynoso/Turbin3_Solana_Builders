use anchor_lang::prelude::*;

mod state;
mod instructions;
mod errors;

pub use instructions::*;
declare_id!("H5MZBwEc4eGKANMHkBu66ccuYHvvJT79YZVcMVduhgat");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfig>, points_per_stake: u8, max_stake: u8, freeze_period: u32) -> Result<()> {
        ctx.accounts.initialize_config(points_per_stake, max_stake, freeze_period, &ctx.bumps)
    }

    pub fn initialize_user(ctx: Context<InitializeUser>) {
        ctx.accounts.initialize_user(&ctx.bumps)
    }

    pub fn stake(ctx: Context<Stake>) {
        ctx.accounts.stake()
    }

    pub fn unstake(ctx: Context<Unstake>) {
        ctx.accounts.unstake()
    }

    pub fn claim(ctx: Context<Claim>) {
        ctx.accounts.claim()
    }
}

