use anchor_lang::prelude::*;
//pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;
declare_id!("B8cn7tHRFQQosRMfKvepLbnUvtLK4vcnenZfcBJ5b7yt");

pub use instructions::*;
pub use state::*;

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
