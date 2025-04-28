use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::error::BridgeError;

#[derive(Accounts)]
#[instruction(params: MintWrappedParams)]
pub struct MintWrapped<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: The receiver account
    #[account(
        constraint = receiver.key() == params.to @ BridgeError::InvalidReceiver
    )]
    pub receiver: AccountInfo<'info>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = receiver,
    )]
    pub associated_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct MintWrappedParams {
    amount: u64,
    to: Pubkey,
    wrapped_token_address: Pubkey,
    message: Vec<u8>,
    signatures: Vec<Vec<u8>>,
}
