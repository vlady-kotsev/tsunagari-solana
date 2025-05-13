use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

use crate::{
    instructions::utils::{mark_used_signatures, validate_signature_accounts, validate_signatures},
    BridgeConfig, SPL_VAULT_SEED,
};

#[derive(Accounts)]
#[instruction(params: CreateWrappedParams)]
pub struct CreateWrapped<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut,
        seeds = [BridgeConfig::SEED],
        bump = bridge_config.bump)
    ]
    pub bridge_config: Account<'info, BridgeConfig>,
    /// CHECK: SPL vault
    #[account(
            seeds = [SPL_VAULT_SEED],
            bump
        )]
    pub spl_vault: AccountInfo<'info>,
    #[account(
        init,
        payer = payer,
        mint::decimals = params.decimals,
        mint::authority = spl_vault,
        mint::freeze_authority = spl_vault,
    )]
    pub mint: Account<'info, Mint>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct CreateWrappedParams {
    decimals: u8,
    pub message: Vec<u8>,
    pub signatures: Vec<Vec<u8>>,
}

pub fn create_wrapped<'info>(
    ctx: &Context<'_, '_, '_, 'info, CreateWrapped<'info>>,
    params: &CreateWrappedParams,
) -> Result<()> {
    let mint = &ctx.accounts.mint;
    let bridge_config = &ctx.accounts.bridge_config;
    let members = &bridge_config.members;
    let threshold = bridge_config.threshold;

    validate_signatures(threshold, members, &params.message, &params.signatures)?;

    let signature_accounts = ctx.remaining_accounts.to_vec();
    let signature_accounts_bumps = validate_signature_accounts(
        &signature_accounts,
        &params.signatures,
        ctx.program_id,
        ctx.accounts.system_program.key,
    )?;

    mark_used_signatures(
        &params.signatures,
        &ctx.accounts.payer,
        ctx.program_id,
        &ctx.accounts.system_program,
        signature_accounts,
        signature_accounts_bumps,
    )?;

    msg!("New token: {}", mint.key());
    Ok(())
}
