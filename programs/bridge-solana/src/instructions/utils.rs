use anchor_lang::{
    prelude::*,
    solana_program::{keccak::hash, program::invoke_signed, system_instruction},
};

use crate::{ecdsa_util::verify_signatures, error::BridgeError, UsedSignature};

pub fn validate_signatures(
    threshold: u8,
    members: &[[u8; 20]],
    message: &Vec<u8>,
    signatures: &[Vec<u8>],
) -> Result<()> {
    let message = message.as_slice();
    let signatures = signatures
        .iter()
        .map(|signature| signature.as_slice())
        .collect::<Vec<&[u8]>>();

    verify_signatures(members, threshold, message, &signatures)?;

    Ok(())
}

fn check_signagture_used(
    signature_accounts: &Vec<AccountInfo>,
    system_program_id: &Pubkey,
) -> Result<()> {
    for signature in signature_accounts {
        require_keys_eq!(
            *signature.owner,
            *system_program_id,
            BridgeError::SignatureAlreadyUsed
        );
    }
    Ok(())
}

pub fn validate_signature_accounts(
    signature_accounts: &Vec<AccountInfo>,
    signatures: &[Vec<u8>],
    program_id: &Pubkey,
    system_program_id: &Pubkey,
) -> Result<Vec<u8>> {
    require!(
        signature_accounts.len() == signatures.len(),
        BridgeError::InvalidSignatureAccounts
    );

    let mut signature_accounts_bumps: Vec<u8> = Vec::new();
    for (index, sign_acc) in signature_accounts.iter().enumerate() {
        let signature_hash = hash(signatures[index].as_slice());
        let (expected_signature_key, bump) = Pubkey::find_program_address(
            &[UsedSignature::SEED, &signature_hash.to_bytes()],
            program_id,
        );

        require_keys_eq!(
            sign_acc.key(),
            expected_signature_key,
            BridgeError::InvalidSignatureAccounts
        );
        signature_accounts_bumps.push(bump);
    }

    check_signagture_used(signature_accounts, system_program_id)?;
    Ok(signature_accounts_bumps)
}

pub fn mark_used_signatures<'info>(
    signatures: &[Vec<u8>],
    payer: &Signer<'info>,
    program_id: &Pubkey,
    system_program: &Program<'info, System>,
    used_signatures: Vec<AccountInfo<'info>>,
    used_signature_bumps: Vec<u8>,
) -> Result<()> {
    for (index, signature) in signatures.iter().enumerate() {
        let signature_hash = hash(signature);
        let (pda, _) = Pubkey::find_program_address(
            &[UsedSignature::SEED, &signature_hash.to_bytes()],
            program_id,
        );

        let rent = Rent::get()?;

        let create_ix = system_instruction::create_account(
            &payer.key(),
            &pda,
            rent.minimum_balance(UsedSignature::SIZE),
            UsedSignature::SIZE as u64,
            program_id,
        );

        let signer_seeds: &[&[&[u8]]] = &[&[
            UsedSignature::SEED,
            &signature_hash.to_bytes(),
            &[used_signature_bumps[index]],
        ]];

        invoke_signed(
            &create_ix,
            &[
                payer.to_account_info(),
                used_signatures[index].to_account_info(),
                system_program.to_account_info(),
            ],
            signer_seeds,
        )?;
    }
    Ok(())
}
