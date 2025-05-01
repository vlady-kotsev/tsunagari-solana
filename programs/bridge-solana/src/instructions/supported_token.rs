use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::SPL_VAULT_SEED;

#[derive(Accounts)]
#[instruction(params: AddSupportedTokenParams)]
pub struct AddSupportedToken<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        constraint = token_mint.key() ==  params.token_mint
    )]
    pub token_mint: Account<'info, Mint>,
    /// CHECK: SPL vault
    #[account(
        seeds = [SPL_VAULT_SEED],
        bump
    )]
    pub spl_vault: AccountInfo<'info>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = token_mint,
        associated_token::authority = spl_vault
    )]
    pub bridge_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct AddSupportedTokenParams {
    token_mint: Pubkey,
}

pub fn add_supported_token(
    ctx: &Context<AddSupportedToken>,
    params: AddSupportedTokenParams,
) -> Result<()> {
    Ok(())
}
