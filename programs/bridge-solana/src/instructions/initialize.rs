use anchor_lang::prelude::*;

use crate::{error::BridgeError, BridgeConfig};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = BridgeConfig::SIZE,
        seeds = [BridgeConfig::SEED],
        bump)]
    pub bridge_config: Account<'info, BridgeConfig>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct InitializeParams {
    pub members: Vec<[u8; 20]>,
    pub threshold: u8,
}

pub fn initialize(ctx: &mut Context<Initialize>, params: &InitializeParams) -> Result<()> {
    let bridge_config = &mut ctx.accounts.bridge_config;
    require!(
        !bridge_config.is_initialized,
        BridgeError::AlreadyInitialized
    );
    require!(params.members.len() > 0, BridgeError::InvalidMembersCount);
    require!(
        params.threshold > 0 && params.threshold <= params.members.len() as u8,
        BridgeError::InvalidThreshold
    );

    bridge_config.bump = ctx.bumps.bridge_config;
    bridge_config.threshold = params.threshold;
    for member in &params.members {
        bridge_config.members.push(*member);
    }
    bridge_config.is_initialized = true;

    Ok(())
}
