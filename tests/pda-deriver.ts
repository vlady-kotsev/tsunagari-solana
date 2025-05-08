import { PublicKey } from "@solana/web3.js";
import { BRIDGE_CONFIG_SEED, SPL_VAULT_SEED, TOKEN_DETAILS } from "./consts";

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
      [Buffer.from(TOKEN_DETAILS), tokenMint.toBuffer()],
      this.program
    )[0];
  }
}
export default new PDADeriver(BRIDGE_PROGRAM_ID);
