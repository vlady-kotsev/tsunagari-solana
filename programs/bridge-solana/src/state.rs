use anchor_lang::prelude::*;

use crate::{MAX_MEMBERS, MAX_TOKEN_SYMBOL_LENGTH};

#[account]
#[derive(InitSpace)]
pub struct BridgeConfig {
    #[max_len(MAX_MEMBERS)]
    pub members: Vec<[u8; 20]>,
    pub threshold: u8,
    pub fee: u8,
    pub is_initialized: bool,
    pub bump: u8,
}

impl BridgeConfig {
    pub const SEED: &'static [u8; 10] = b"BridgeConf";
    pub const SIZE: usize = BridgeConfig::INIT_SPACE + 8;
}

#[account]
#[derive(InitSpace)]
pub struct TokenDetails {
    #[max_len(MAX_TOKEN_SYMBOL_LENGTH)]
    pub symbol: String,
    pub decimals: u8,
    pub mint: Pubkey,
    pub min_amount: u64,
}

impl TokenDetails {
    pub const SEED: &'static [u8; 10] = b"TokDetails";
    pub const SIZE: usize = TokenDetails::INIT_SPACE + 8;
}

#[account]
#[derive(InitSpace)]
pub struct UsedSignature {}

impl UsedSignature {
    pub const SEED: &'static [u8; 4] = b"sign";
    pub const SIZE: usize = UsedSignature::INIT_SPACE + 8;
}
