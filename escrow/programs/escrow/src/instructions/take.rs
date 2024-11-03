use anchor_lang::prelude::*;
use anchor_spl::token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked, CloseAccount, close_account};

use crate::state::Escrow;

#[derive(Accounts)] 
pub struct Deposit<'info>{
    #[account(mut)]
    pub taker: Signer<'info>,
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = maker,
    )]
    pub maker_ata_b: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,// init if needed
        associated_token::mint = mint_b,
        associated_token::authority = taker,
    )]
    pub taker_ata_b: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,

}

impl<'info> Deposit<'info>{
    pub fn deposit(&mut self) -> Result<()>{
        let amount = self.escrow.receive; //???
        let cpi_program= self.token_program.to_account_info();

        let cpi_accounts =  TransferChecked{
            from: self.taker_ata_b.to_account_info(),
            to: self.maker_ata_b.to_account_info(),
            mint: self.mint_b.to_account_info(),
            authority: self.taker.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer_checked(cpi_ctx, amount , self.mint_b.decimals)?;
        Ok(())
    }
}

#[derive(Accounts)] 
pub struct Withdraw<'info>{
    #[account(mut)]
    pub maker: Signer<'info>, //system acc why? use for deriving escrow
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(
        //what happerns if the taker doesnt have a ATA for mint A yet
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
    )]
    pub taker_ata_a: InterfaceAccount<'info, TokenAccount>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    #[account(
        mut
        close = maker,
        has_one = maker,
        has_one = mint_a, //check
        has_one = maker,
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        mut
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,

}

impl<'info> Withdraw<'info>{
    pub fn withdraw(&mut self) -> Result<()>{
        
        let cpi_program= self.token_program.to_account_info();

        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"escrow",
            self.maker.to_account_info().key.as_ref(), //makers is marked as systemaccount
            &self.escrow.seed.to_le_bytes()[..],
            &[self.escrow.bump]
            ]];
        
        //withdraw
        let cpi_accounts =  TransferChecked{
            from: self.vault.to_account_info(),
            to: self.taker_ata_a.to_account_info(),
            mint: self.mint_a.to_account_info(),
            authority: self.escrow.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds);
        transfer_checked(cpi_ctx, self.vault.amount , self.mint_a.decimals)?;

        //close
        let cpi_accounts =  CloseAccount{
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.escrow.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds);
        close_account(cpi_ctx)?;
        Ok(())
    }
}