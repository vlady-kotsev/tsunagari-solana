use anchor_lang::prelude::*;
use anchor_spl::token::{burn, Burn, Mint, Token, TokenAccount};

#[derive(Accounts)]
#[instruction(params: BurnWrappedParams)]
pub struct BurnWrapped<'info> {
    pub payer: Signer<'info>,
    #[account(
        mut,
        constraint = mint.key() == params.wrapped_token_address
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        mut,
        constraint = from.mint == mint.key(),
        constraint = from.owner == payer.key()
    )]
    pub from: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct BurnWrappedParams {
    amount: u64,
    wrapped_token_address: Pubkey,
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

    Ok(())
}
