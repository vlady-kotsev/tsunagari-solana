use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::{
    ecdsa_util::verify_signatures, error::BridgeError, instructions::utils::validate_signatures,
    BridgeConfig, TokenDetails, MAX_TOKEN_SYMBOL_LENGTH, SPL_VAULT_SEED,
};

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
    #[account(
        init,
        payer = payer,
        space = TokenDetails::SIZE,
        seeds = [TokenDetails::SEED, params.token_mint.as_ref()],
        bump
    )]
    pub token_details: Account<'info, TokenDetails>,
    #[account(mut,
        seeds = [BridgeConfig::SEED],
        bump = bridge_config.bump)
    ]
    pub bridge_config: Account<'info, BridgeConfig>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct AddSupportedTokenParams {
    pub token_mint: Pubkey,
    pub symbol: String,
    pub min_amount: u64,
    pub message: Vec<u8>,
    pub signatures: Vec<Vec<u8>>,
}

pub fn add_supported_token(
    ctx: &mut Context<AddSupportedToken>,
    params: &AddSupportedTokenParams,
) -> Result<()> {
    let bridge_config = &ctx.accounts.bridge_config;
    let members = &bridge_config.members;
    let threshold = bridge_config.threshold;

    validate_signatures(threshold, members, &params.message, &params.signatures)?;

    require!(
        params.symbol.len() <= MAX_TOKEN_SYMBOL_LENGTH as usize,
        BridgeError::InvalidTokenSymbolLength
    );

    let token_details = &mut ctx.accounts.token_details;
    let mint = &ctx.accounts.token_mint;

    token_details.decimals = mint.decimals;
    token_details.mint = mint.key();
    token_details.symbol = params.symbol.clone();
    token_details.min_amount = params.min_amount;

    Ok(())
}

#[derive(Accounts)]
#[instruction(params: AddSupportedTokenParams)]
pub struct RemoveSupportedToken<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        constraint = token_mint.key() ==  params.token_mint
    )]
    pub token_mint: Account<'info, Mint>,
    #[account(
        mut,
        close = payer,
        seeds = [TokenDetails::SEED, params.token_mint.as_ref()],
        bump
    )]
    pub token_details: Account<'info, TokenDetails>,
    #[account(mut,
        seeds = [BridgeConfig::SEED],
        bump = bridge_config.bump)
    ]
    pub bridge_config: Account<'info, BridgeConfig>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct RemoveSupportedTokenParams {
    token_mint: Pubkey,
    pub message: Vec<u8>,
    pub signatures: Vec<Vec<u8>>,
}

pub fn remove_supported_token(
    ctx: &Context<RemoveSupportedToken>,
    params: &RemoveSupportedTokenParams,
) -> Result<()> {
    let bridge_config = &ctx.accounts.bridge_config;
    let members = &bridge_config.members;
    let threshold = bridge_config.threshold;
    let message = params.message.as_slice();
    let signatures = params
        .signatures
        .iter()
        .map(|signature| signature.as_slice())
        .collect::<Vec<&[u8]>>();

    verify_signatures(members, threshold, message, signatures)?;

    Ok(())
}
