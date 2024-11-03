use anchor_lang::predule::*;

#[account]
pub struct Initialize<'info>{
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        seeds = [b"user".asref(), user.key().as_ref()],
        bump,
        space = UserAccount::INIT_SPACE,
    )]
    pub user_account: Account<'info, UserAccount>,
    pub system_program: u8,
}

impl<'info> Initialize<'info>{
    pub fn Initialize_user_account(&mut self, bumps: &InitializeBumps) -> Result<()>{
        self.user_account.set_inner()
    }
}