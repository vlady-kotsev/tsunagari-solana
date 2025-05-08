use anchor_lang::prelude::*;
use anchor_spl::token::{burn, Burn, Mint, Token, TokenAccount};

use crate::{events::TokensBurned, BridgeConfig, TokenDetails};

#[derive(Accounts)]
#[instruction(params: BurnWrappedParams)]
pub struct BurnWrapped<'info> {
    pub payer: Signer<'info>,
    #[account(
        mut,
        constraint = mint.key() == params.wrapped_token_mint
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        mut,
        constraint = from.mint == mint.key(),
        constraint = from.owner == payer.key()
    )]
    pub from: Account<'info, TokenAccount>,
    #[account(
        seeds = [TokenDetails::SEED, params.wrapped_token_mint.as_ref()],
        bump
    )]
    pub token_details: Account<'info, TokenDetails>,
    #[account(
        seeds = [BridgeConfig::SEED],
        bump = bridge_config.bump)]
    pub bridge_config: Account<'info, BridgeConfig>,
    pub token_program: Program<'info, Token>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct BurnWrappedParams {
    pub amount: u64,
    pub wrapped_token_mint: Pubkey,
    pub destination_chain: u64,
}

pub fn burn_wrapped(ctx: &Context<BurnWrapped>, params: &BurnWrappedParams) -> Result<()> {
    let cpi_accounts = Burn {
        mint: ctx.accounts.mint.to_account_info(),
        from: ctx.accounts.from.to_account_info(),
        authority: ctx.accounts.payer.to_account_info(),
    };
    let amount = params.amount;

    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);

    burn(cpi_ctx, amount)?;
    
    let fee = ctx.accounts.bridge_config.fee as u64;
    let amout_after_fee = amount - (amount * fee) / 100;

    emit!(TokensBurned {
        amount: amout_after_fee,
        burned_token_mint: params.wrapped_token_mint,
        destination_chain: params.destination_chain
    });

    Ok(())
}
