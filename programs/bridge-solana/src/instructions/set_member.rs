use anchor_lang::prelude::*;

use crate::{error::BridgeError, BridgeConfig, MAX_MEMBERS};

use super::utils::{mark_used_signatures, validate_signature_accounts, validate_signatures};

#[derive(Accounts)]
pub struct SetMember<'info> {
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
pub struct SetMemberParams {
    pub member_key: [u8; 20],
    pub action: bool,
    pub message: Vec<u8>,
    pub signatures: Vec<Vec<u8>>,
}

pub fn set_member<'info>(
    ctx: &mut Context<'_, '_, '_, 'info, SetMember<'info>>,
    params: &SetMemberParams,
) -> Result<()> {
    let bridge_config = &mut ctx.accounts.bridge_config;

    // Verify signatures
    validate_signatures(
        bridge_config.threshold,
        &bridge_config.members,
        &params.message,
        &params.signatures,
    )?;

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
