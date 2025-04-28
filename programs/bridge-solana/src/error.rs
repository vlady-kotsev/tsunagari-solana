use anchor_lang::prelude::*;

#[error_code]
pub enum BridgeError {
    #[msg("Invalid receiver")]
    InvalidReceiver,
}
