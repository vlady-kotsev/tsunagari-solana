use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

use crate::{events::TokensLocked, BridgeConfig, TokenDetails, SPL_VAULT_SEED};

#[derive(Accounts)]
#[instruction(params: LockParams)]
pub struct Lock<'info> {
    pub payer: Signer<'info>,
    #[account(
        seeds = [TokenDetails::SEED, params.token_mint.as_ref()],
        bump
    )]
    pub token_details: Account<'info, TokenDetails>,
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
        constraint = from.mint == mint.key(),
        constraint = from.owner == payer.key()
    )]
    pub from: Account<'info, TokenAccount>,
    #[account(mut,
        constraint = to.mint == mint.key(),
        constraint = to.owner == spl_vault.key()
    )]
    pub to: Account<'info, TokenAccount>,
    #[account(
        seeds = [BridgeConfig::SEED],
        bump = bridge_config.bump)]
    pub bridge_config: Account<'info, BridgeConfig>,
    pub token_program: Program<'info, Token>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct LockParams {
    pub token_mint: Pubkey,
    pub amount: u64,
    pub destination_chain: u64,
}

pub fn lock(ctx: &Context<Lock>, params: &LockParams) -> Result<()> {
    let cpi_accounts = Transfer {
        from: ctx.accounts.from.to_account_info(),
        to: ctx.accounts.to.to_account_info(),
        authority: ctx.accounts.payer.to_account_info(),
    };

    let amount = params.amount;

    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);

    transfer(cpi_ctx, amount)?;

    let fee = ctx.accounts.bridge_config.fee as u64;
    let amout_after_fee = amount - (amount * fee) / 100;

    emit!(TokensLocked {
        amount: amout_after_fee,
        locked_token_mint: params.token_mint,
        destination_chain: params.destination_chain
    });

    Ok(())
}
