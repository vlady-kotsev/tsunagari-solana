use anchor_lang::prelude::*;

use crate::{ecdsa_util::verify_signatures, error::BridgeError, BridgeConfig};

#[derive(Accounts)]
pub struct SetThreshold<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut,
    seeds = [BridgeConfig::SEED],
    bump = bridge_config.bump)]
    pub bridge_config: Account<'info, BridgeConfig>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct SetThresholdParams {
    pub threshold: u8,
    pub message: Vec<u8>,
    pub signatures: Vec<Vec<u8>>,
}

pub fn set_threshold<'info>(
    ctx: &mut Context<SetThreshold>,
    params: SetThresholdParams,
) -> Result<()> {
    let bridge_config = &mut ctx.accounts.bridge_config;
    let members = &bridge_config.members;
    let threshold = &bridge_config.threshold;

    let message = params.message.as_slice();
    let signatures = params
        .signatures
        .iter()
        .map(|signature| signature.as_slice())
        .collect::<Vec<&[u8]>>();

    verify_signatures(members, *threshold, message, signatures)?;

    require!(
        params.threshold > 0 && params.threshold <= members.len() as u8,
        BridgeError::InvalidThreshold
    );
    bridge_config.threshold = params.threshold;

    Ok(())
}
