use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)] //does not contemplate the 8 bytes discriminator
pub struct Escrow{
    pub sees: u64,
    pub maker: Pubkey,
    pub taker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub receive: u64,
    pub bump: u8, //to save compute units
}