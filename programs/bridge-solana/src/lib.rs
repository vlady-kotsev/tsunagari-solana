pub mod constants;
pub mod ecdsa_util;
pub mod error;
pub mod events;
pub mod instructions;
pub mod state;
use anchor_lang::prelude::*;
use instructions::*;

pub use constants::*;
pub use state::*;

declare_id!("NfuWnZr8HR4mxULPG61Nh7zSbdinwGtNQGVoeuxM5Jf");

#[program]
pub mod bridge_solana {

    use super::*;

    pub fn initialize(mut ctx: Context<Initialize>, params: InitializeParams) -> Result<()> {
        instructions::initialize(&mut ctx, &params)
    }

    pub fn set_member<'info>(
        mut ctx: Context<'_, '_, '_, 'info, SetMember<'info>>,
        params: SetMemberParams,
    ) -> Result<()> {
        instructions::set_member(&mut ctx, &params)
    }

    pub fn set_fee<'info>(
        mut ctx: Context<'_, '_, '_, 'info, SetFee<'info>>,
        params: SetFeeParams,
    ) -> Result<()> {
        instructions::set_fee(&mut ctx, &params)
    }

    pub fn set_threshold<'info>(
        mut ctx: Context<'_, '_, '_, 'info, SetThreshold<'info>>,
        params: SetThresholdParams,
    ) -> Result<()> {
        instructions::set_threshold(&mut ctx, params)
    }

    pub fn add_supported_token<'info>(
        mut ctx: Context<'_, '_, '_, 'info, AddSupportedToken<'info>>,
        params: AddSupportedTokenParams,
    ) -> Result<()> {
        instructions::add_supported_token(&mut ctx, &params)
    }

    pub fn remove_supported_token<'info>(
        ctx: Context<'_, '_, '_, 'info, RemoveSupportedToken<'info>>,
        params: RemoveSupportedTokenParams,
    ) -> Result<()> {
        instructions::remove_supported_token(&ctx, &params)
    }

    pub fn mint_wrapped<'info>(
        ctx: Context<'_, '_, '_, 'info, MintWrapped<'info>>,
        params: MintWrappedParams,
    ) -> Result<()> {
        instructions::mint_wrapped(&ctx, &params)
    }

    pub fn burn_wrapped(ctx: Context<BurnWrapped>, params: BurnWrappedParams) -> Result<()> {
        instructions::burn_wrapped(&ctx, &params)
    }

    pub fn lock(ctx: Context<Lock>, params: LockParams) -> Result<()> {
        instructions::lock(&ctx, &params)
    }

    pub fn unlock<'info>(
        ctx: Context<'_, '_, '_, 'info, Unlock<'info>>,
        params: UnlockParams,
    ) -> Result<()> {
        instructions::unlock(&ctx, &params)
    }

    pub fn create_wrapped<'info>(
        ctx: Context<'_, '_, '_, 'info, CreateWrapped<'info>>,
        params: CreateWrappedParams,
    ) -> Result<()> {
        instructions::create_wrapped(&ctx, &params)
    }
}
