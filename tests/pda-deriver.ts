import { PublicKey } from "@solana/web3.js";
import { keccak_256 } from "@noble/hashes/sha3";
import {
  BRIDGE_CONFIG_SEED,
  SPL_VAULT_SEED,
  TOKEN_DETAILS_SEED,
  USED_SIGNATURE_SEED,
} from "./consts";

const BRIDGE_PROGRAM_ID = new PublicKey(
  "NfuWnZr8HR4mxULPG61Nh7zSbdinwGtNQGVoeuxM5Jf"
);
class PDADeriver {
  constructor(public readonly program: PublicKey) {}

  bridgeConfig(): PublicKey {
    return PublicKey.findProgramAddressSync(
      [Buffer.from(BRIDGE_CONFIG_SEED)],
      this.program
    )[0];
  }

  splVault(): PublicKey {
    return PublicKey.findProgramAddressSync(
      [Buffer.from(SPL_VAULT_SEED)],
      this.program
    )[0];
  }

  tokenDetails(tokenMint: PublicKey): PublicKey {
    return PublicKey.findProgramAddressSync(
      [Buffer.from(TOKEN_DETAILS_SEED), tokenMint.toBuffer()],
      this.program
    )[0];
  }

  usedSignature(signature: Buffer): PublicKey {
    const signatureHash = keccak_256(signature);
    return PublicKey.findProgramAddressSync(
      [Buffer.from(USED_SIGNATURE_SEED), signatureHash],
      this.program
    )[0];
  }
}
export default new PDADeriver(BRIDGE_PROGRAM_ID);
