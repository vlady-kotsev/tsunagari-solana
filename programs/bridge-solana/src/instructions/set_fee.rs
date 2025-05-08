use anchor_lang::prelude::*;

use crate::{error::BridgeError, BridgeConfig};

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
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct SetFeeParams {
    pub fee: u8,
    pub message: Vec<u8>,
    pub signatures: Vec<Vec<u8>>,
}

pub fn set_fee(ctx: &mut Context<SetFee>, params: &SetFeeParams) -> Result<()> {
    let bridge_config = &mut ctx.accounts.bridge_config;

    // Verify signatures
    validate_signatures(
        bridge_config.threshold,
        &bridge_config.members,
        &params.message,
        &params.signatures,
    )?;

    require!(params.fee <= 100, BridgeError::InvalidFee);
    bridge_config.fee = params.fee;
    Ok(())
}
