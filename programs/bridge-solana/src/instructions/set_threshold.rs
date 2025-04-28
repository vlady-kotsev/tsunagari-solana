use anchor_lang::prelude::*;

use crate::BridgeConfig;

#[derive(Accounts)]
pub struct SetThreshold<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub bridge_config: Account<'info, BridgeConfig>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct SetThresholdParams {
    pub threshold: u8,
}

pub fn set_threshold(ctx: &Context<SetThreshold>, params: SetThresholdParams) -> Result<()> {
    Ok(())
}
