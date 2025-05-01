use anchor_lang::prelude::*;

use crate::{error::BridgeError, BridgeConfig, MAX_MEMBERS};

#[derive(Accounts)]
pub struct SetMember<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut,
        seeds = [BridgeConfig::SEED],
        bump = bridge_config.bump)
    ]
    pub bridge_config: Account<'info, BridgeConfig>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct SetMemberParams {
    pub member_key: [u8; 20],
    pub action: bool,
}

pub fn set_member(ctx: &mut Context<SetMember>, params: &SetMemberParams) -> Result<()> {
    let bridge_config = &mut ctx.accounts.bridge_config;
    // add member
    if params.action {
        require!(
            bridge_config.members.len() < MAX_MEMBERS as usize,
            BridgeError::MaximumMembers
        );
        require!(
            params.member_key != [0u8; 20],
            BridgeError::InvalidMemberKey
        );
        require!(
            !bridge_config.members.contains(&params.member_key),
            BridgeError::MemberKeyAlreadyAdded
        );

        bridge_config.members.push(params.member_key);
    } else {
        // remove member
        require!(
            bridge_config.members.len() as u8 - 1 >= bridge_config.threshold,
            BridgeError::BelowThreshold
        );

        match bridge_config
            .members
            .iter()
            .position(|x| *x == params.member_key)
        {
            Some(pos) => {
                bridge_config.members.remove(pos);
            }
            None => {
                return Err(BridgeError::MemberKeyNotFound.into());
            }
        }
    }
    Ok(())
}
