use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

use crate::{error::BridgeError, BridgeConfig, SPL_VAULT_SEED};

#[derive(Accounts)]
#[instruction(params: MintWrappedParams)]
pub struct MintWrapped<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: The receiver account
    #[account(
        constraint = receiver.key() == params.to @ BridgeError::InvalidReceiver
    )]
    pub receiver: AccountInfo<'info>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = receiver,
    )]
    pub receiver_ata: Account<'info, TokenAccount>,
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
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct MintWrappedParams {
    amount: u64,
    to: Pubkey,
    wrapped_token_address: Pubkey,
    message: Vec<u8>,
    signatures: Vec<Vec<u8>>,
}

pub fn mint_wrapped(ctx: &Context<MintWrapped>, params: &MintWrappedParams) -> Result<()> {
    let amount = params.amount;
    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.receiver_ata.to_account_info(),
        authority: ctx.accounts.spl_vault.to_account_info(),
    };
    let signer_seeds: &[&[&[u8]]] = &[&[SPL_VAULT_SEED, &[ctx.bumps.spl_vault]]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer_seeds,
    );
    mint_to(cpi_ctx, amount)?;

    Ok(())
}
