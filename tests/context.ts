import { startAnchor } from "solana-bankrun";
import { PublicKey } from "@solana/web3.js";
import { web3 } from "@coral-xyz/anchor";

export const BRIDGE_PROGRAM_ID = new PublicKey(
  "NfuWnZr8HR4mxULPG61Nh7zSbdinwGtNQGVoeuxM5Jf"
);

export const contextPromise = startAnchor(
  ".",
  [
    {
      name: "bridge_solana",
      programId: BRIDGE_PROGRAM_ID,
    },
  ],
  [
    {
      address: new PublicKey("5sKBF5yF6hQXZExkUKHBSnW8UgiXwpvijDNakWPm4u15"),
      info: {
        data: Buffer.from([]),
        owner: web3.SystemProgram.programId,
        executable: false,
        lamports: 100_000_000_000, // 100 sol
      },
    },
  ]
);
