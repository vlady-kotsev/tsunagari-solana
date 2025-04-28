use anchor_lang::prelude::*;

use crate::MAX_MEMBERS;

#[account]
#[derive(InitSpace)]
pub struct BridgeConfig {
    #[max_len(MAX_MEMBERS)]
    pub members: Vec<Pubkey>,
    pub threshold: u8,
    pub bum: u8,
}

impl BridgeConfig {
    pub const SEED: &[u8; 10] = b"BridgeConf";

    pub const SIZE: usize = BridgeConfig::INIT_SPACE + 8;
}
