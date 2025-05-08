use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

use crate::{BridgeConfig, SPL_VAULT_SEED};

use super::utils::validate_signatures;

#[derive(Accounts)]
#[instruction(params: UnlockParams)]
pub struct Unlock<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        constraint = mint.key() == params.token_mint
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        seeds = [SPL_VAULT_SEED],
        bump
    )]
    pub spl_vault: SystemAccount<'info>,
    #[account(mut,
        seeds = [BridgeConfig::SEED],
        bump = bridge_config.bump)
    ]
    pub bridge_config: Account<'info, BridgeConfig>,
    #[account(mut,
        constraint = from.mint == mint.key(),
        constraint = from.owner == spl_vault.key()
    )]
    pub from: Account<'info, TokenAccount>,
    #[account(mut,
        constraint = to.mint == mint.key(),
        constraint = to.owner == payer.key()
    )]
    pub to: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct UnlockParams {
    pub token_mint: Pubkey,
    pub amount: u64,
    pub message: Vec<u8>,
    pub signatures: Vec<Vec<u8>>,
}

pub fn unlock(ctx: &Context<Unlock>, params: &UnlockParams) -> Result<()> {
    let bridge_config = &ctx.accounts.bridge_config;
    let threshold = bridge_config.threshold;
    let members = &bridge_config.members;

    validate_signatures(threshold, members, &params.message, &params.signatures)?;

    let cpi_accounts = Transfer {
        from: ctx.accounts.from.to_account_info(),
        to: ctx.accounts.to.to_account_info(),
        authority: ctx.accounts.spl_vault.to_account_info(),
    };

    let amount = params.amount;
    let signer_seeds: &[&[&[u8]]] = &[&[SPL_VAULT_SEED, &[ctx.bumps.spl_vault]]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer_seeds,
    );

    transfer(cpi_ctx, amount)?;
    Ok(())
}
