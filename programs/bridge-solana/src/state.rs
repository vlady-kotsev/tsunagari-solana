use anchor_lang::prelude::*;

use crate::MAX_MEMBERS;

#[account]
#[derive(InitSpace)]
pub struct BridgeConfig {
    #[max_len(MAX_MEMBERS)]
    pub members: Vec<[u8; 20]>,
    pub threshold: u8,
    pub is_initialized: bool,
    pub bump: u8,
}

impl BridgeConfig {
    pub const SEED: &'static [u8; 10] = b"BridgeConf";

    pub const SIZE: usize = BridgeConfig::INIT_SPACE + 8;
}
