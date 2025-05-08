use anchor_lang::prelude::*;

use crate::ecdsa_util::verify_signatures;

pub fn validate_signatures(
    threshold: u8,
    members: &Vec<[u8; 20]>,
    message: &Vec<u8>,
    signatures: &Vec<Vec<u8>>,
) -> Result<()> {
    let message = message.as_slice();
    let signatures = signatures
        .iter()
        .map(|signature| signature.as_slice())
        .collect::<Vec<&[u8]>>();

    verify_signatures(members, threshold, message, signatures)?;
    Ok(())
}
