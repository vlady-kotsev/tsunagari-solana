use std::collections::HashSet;

use anchor_lang::{
    prelude::*,
    solana_program::{keccak, secp256k1_recover::secp256k1_recover},
};

use crate::error::BridgeError;

fn validate_signature(
    members_keyes: &Vec<[u8; 20]>,
    message: &[u8],
    signature_bytes: &[u8],
) -> Result<()> {
    let message_hash = {
        let mut hasher = keccak::Hasher::default();
        hasher.hash(message);
        hasher.result()
    };

    let recovered_pubkey =
        secp256k1_recover(&message_hash.0, signature_bytes[64], &signature_bytes[..64])
            .map_err(|_| ProgramError::InvalidArgument)?;

    let mut hasher = keccak::Hasher::default();
    hasher.hash(&recovered_pubkey.0);
    let address: [u8; 20] = (&hasher.result().0[12..])
        .try_into()
        .map_err(|_| BridgeError::MemberKeyRecoveryError)?;

    require!(members_keyes.contains(&address), BridgeError::InvalidSigner);

    Ok(())
}

fn validate_against_threshold(threshold: u8, signatures: &Vec<&[u8]>) -> Result<()> {
    require!(
        signatures.len() >= threshold as usize,
        BridgeError::NotEnoughSignatures
    );
    Ok(())
}
fn validate_against_duplicates(signatures: &Vec<&[u8]>) -> Result<()> {
    let mut unique_signatures: HashSet<&[u8]> = HashSet::new();
    for &sig in signatures {
        unique_signatures.insert(sig);
    }

    require!(
        signatures.len() == unique_signatures.len(),
        BridgeError::NotEnoughSignatures
    );
    Ok(())
}
pub fn verify_signatures(
    members_keyes: &Vec<[u8; 20]>,
    threshold: u8,
    message: &[u8],
    signatures: Vec<&[u8]>,
) -> Result<()> {
    // check if signatures meet the threshold
    validate_against_threshold(threshold, &signatures)?;

    // check if signatures are unique
    validate_against_duplicates(&signatures)?;

    // check if signatures are correct
    validate_signature(members_keyes, message, signatures[0])?;

    Ok(())
}
