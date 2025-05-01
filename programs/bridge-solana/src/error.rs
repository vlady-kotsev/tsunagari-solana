use anchor_lang::prelude::*;

#[error_code]
pub enum BridgeError {
    #[msg("Invalid receiver")]
    InvalidReceiver,
    #[msg("Invalid threshold")]
    InvalidThreshold,
    #[msg("Invalid members count")]
    InvalidMembersCount,
    #[msg("Bridge already initialized")]
    AlreadyInitialized,
    #[msg("Maximum members reached")]
    MaximumMembers,
    #[msg("Members cannot be below threshold")]
    BelowThreshold,
    #[msg("Invalid member key")]
    InvalidMemberKey,
    #[msg("Member key already added")]
    MemberKeyAlreadyAdded,
    #[msg("Member key not found")]
    MemberKeyNotFound,
    #[msg("Member key recovery error")]
    MemberKeyRecoveryError,
    #[msg("Invalid signer")]
    InvalidSigner,
    #[msg("Not enough signatures")]
    NotEnoughSignatures,
    #[msg("Signatures not unique")]
    NotUniqueSignatures,
}
