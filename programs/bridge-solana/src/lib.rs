pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use state::*;

declare_id!("74EDYiVSPy4S3SHVzsxW3iW4ryVFzUXxs52CTejyuUSq");

#[program]
pub mod bridge_solana {
    use super::*;
    pub use instructions::*;

    pub fn initialize(mut ctx: Context<Initialize>, params: InitializeParams) -> Result<()> {
        instructions::initialize(&mut ctx, &params)
    }

    pub fn set_member(ctx: Context<SetMember>, params: SetMemberParams) -> Result<()> {
        instructions::set_member(&ctx, &params)
    }

    pub fn set_threshold(ctx: Context<SetThreshold>, params: SetThresholdParams) -> Result<()> {
        instructions::set_threshold(&ctx, params)
    }

    
}
