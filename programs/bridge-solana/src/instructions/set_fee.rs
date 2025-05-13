#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

use crate::{
    error::BridgeError,
    instructions::utils::{mark_used_signatures, validate_signature_accounts},
    BridgeConfig,
};

use super::utils::validate_signatures;

#[derive(Accounts)]
pub struct SetFee<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut,
        seeds = [BridgeConfig::SEED],
        bump = bridge_config.bump)
    ]
    pub bridge_config: Account<'info, BridgeConfig>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct SetFeeParams {
    pub fee: u8,
    pub message: Vec<u8>,
    pub signatures: Vec<Vec<u8>>,
}

pub fn set_fee<'info>(
    ctx: &mut Context<'_, '_, '_, 'info, SetFee<'info>>,
    params: &SetFeeParams,
) -> Result<()> {
    let bridge_config = &mut ctx.accounts.bridge_config;
    let signature_accounts = ctx.remaining_accounts.to_vec();

    // Verify signatures
    validate_signatures(
        bridge_config.threshold,
        &bridge_config.members,
        &params.message,
        &params.signatures,
    )?;

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

    require!(params.fee <= 100, BridgeError::InvalidFee);
    bridge_config.fee = params.fee;
    Ok(())
}
