use anchor_lang::prelude::*;

declare_id!("4VWHjbnK6eKdTn8HNEdwGF3Qn9ZXA9eeahzyxMpZgLfE");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
