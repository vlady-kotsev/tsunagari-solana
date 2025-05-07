use anchor_lang::prelude::*;

#[constant]
pub const SPL_VAULT_SEED: &[u8] = b"splv";

#[constant]
pub const MAX_MEMBERS: u8 = 10;

#[constant]
pub const MAX_TOKEN_SYMBOL_LENGTH: u8 = 10;