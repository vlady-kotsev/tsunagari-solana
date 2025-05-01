import { PublicKey } from "@solana/web3.js";
import { BRIDGE_CONFIG_SEED } from "./consts";

const BRIDGE_PROGRAM_ID = new PublicKey(
  "NfuWnZr8HR4mxULPG61Nh7zSbdinwGtNQGVoeuxM5Jf"
);
class PDADeriver {
  constructor(public readonly program: PublicKey) {}

  bridge_config(): PublicKey {
    return PublicKey.findProgramAddressSync(
      [Buffer.from(BRIDGE_CONFIG_SEED)],
      this.program
    )[0];
  }
}
export default new PDADeriver(BRIDGE_PROGRAM_ID);
