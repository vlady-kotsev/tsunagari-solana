#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

use crate::{
    error::BridgeError,
    instructions::utils::{mark_used_signatures, validate_signature_accounts, validate_signatures},
    BridgeConfig,
};

#[derive(Accounts)]
pub struct SetThreshold<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut,
    seeds = [BridgeConfig::SEED],
    bump = bridge_config.bump)]
    pub bridge_config: Account<'info, BridgeConfig>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct SetThresholdParams {
    pub threshold: u8,
    pub message: Vec<u8>,
    pub signatures: Vec<Vec<u8>>,
}

pub fn set_threshold<'info>(
    ctx: &mut Context<'_,'_,'_,'info,SetThreshold<'info>>,
    params: SetThresholdParams,
) -> Result<()> {
    let bridge_config = &mut ctx.accounts.bridge_config;
    let members = &bridge_config.members;
    let threshold = bridge_config.threshold;
    let signature_accounts = ctx.remaining_accounts.to_vec();
    // Verify signatures
    validate_signatures(threshold, members, &params.message, &params.signatures)?;

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

    require!(
        params.threshold > 0 && params.threshold <= members.len() as u8,
        BridgeError::InvalidThreshold
    );
    bridge_config.threshold = params.threshold;

    Ok(())
}
