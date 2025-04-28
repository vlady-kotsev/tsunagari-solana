use anchor_lang::prelude::*;

use crate::BridgeConfig;

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
    pub members: Vec<Vec<u8>>,
    pub threshold: u8,
}

pub fn initialize(ctx: &mut Context<Initialize>, params: &InitializeParams) -> Result<()> {
    Ok(())
}
