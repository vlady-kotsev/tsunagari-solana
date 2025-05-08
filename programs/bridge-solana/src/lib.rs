pub mod constants;
pub mod ecdsa_util;
pub mod error;
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

    pub use instructions::*;

    pub fn initialize(mut ctx: Context<Initialize>, params: InitializeParams) -> Result<()> {
        instructions::initialize(&mut ctx, &params)
    }

    pub fn set_member(mut ctx: Context<SetMember>, params: SetMemberParams) -> Result<()> {
        instructions::set_member(&mut ctx, &params)
    }

    pub fn set_threshold(mut ctx: Context<SetThreshold>, params: SetThresholdParams) -> Result<()> {
        instructions::set_threshold(&mut ctx, params)
    }

    pub fn add_supported_token(
        mut ctx: Context<AddSupportedToken>,
        params: AddSupportedTokenParams,
    ) -> Result<()> {
        instructions::add_supported_token(&mut ctx, &params)
    }

    pub fn remove_supported_token(
        ctx: Context<RemoveSupportedToken>,
        params: RemoveSupportedTokenParams,
    ) -> Result<()> {
        instructions::remove_supported_token(&ctx, &params)
    }

    pub fn mint_wrapped(ctx: Context<MintWrapped>, params: MintWrappedParams) -> Result<()> {
        instructions::mint_wrapped(&ctx, &params)
    }
    // burnWrapped

    // lockNative

    //unlockNative
}
