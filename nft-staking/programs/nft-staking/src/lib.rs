use anchor_lang::prelude::*;

declare_id!("H5MZBwEc4eGKANMHkBu66ccuYHvvJT79YZVcMVduhgat");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
