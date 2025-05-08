use anchor_lang::prelude::*;

#[event]
pub struct TokensLocked {
    pub amount: u64,
    pub locked_token_mint: Pubkey,
    pub destination_chain: u64,
}

#[event]
pub struct TokensBurned {
    pub amount: u64,
    pub burned_token_mint: Pubkey,
    pub destination_chain: u64,
}
