use anchor_lang::prelude::*;

use crate::BridgeConfig;

#[derive(Accounts)]
pub struct SetMember<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub bridge_config: Account<'info, BridgeConfig>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct SetMemberParams {
    pub member_key: Pubkey,
    pub action: bool,
}

pub fn set_member(ctx: &Context<SetMember>, params: &SetMemberParams) -> Result<()> {
    Ok(())
}
