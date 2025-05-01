import { BankrunProvider } from "anchor-bankrun";
import { LAMPORTS_PER_SOL, Keypair, SystemProgram } from "@solana/web3.js";
import { Program, Wallet } from "@coral-xyz/anchor";
import { BridgeSolana } from "../target/types/bridge_solana";
import { randomBytes } from "crypto";
import BRIDGE_IDL from "../target/idl/bridge_solana.json";
import { contextPromise } from "./context";

import { ProgramTestContext } from "solana-bankrun";
import { loadKeypair, signMessage } from "./utils";
import pdaDeriver from "./pda-deriver";
import { getPrivateKey } from "./env";

describe("bridge_solana_tests", () => {
  let bridgeProgram: Program<BridgeSolana>;
  let authority: Keypair;
  let context: ProgramTestContext;
  let provider: BankrunProvider;

  beforeAll(async () => {
    context = await contextPromise;
    authority = await loadKeypair();
    provider = new BankrunProvider(context, new Wallet(authority));
    bridgeProgram = new Program<BridgeSolana>(BRIDGE_IDL as any, provider);
  });

  test("initialize", async () => {
    const hexStringMember1 = "4747b7f5c40599E1C5CF5a72C535D953B64916b6";
    // Convert to byte array
    const bytesMember1 = new Uint8Array(
      hexStringMember1.match(/.{1,2}/g)!.map((byte) => parseInt(byte, 16))
    );
    const ethAddressMember1 = Array.from(bytesMember1.slice(0, 20));

    const hexStringMember2 = "604B11e1F6b2ffD330dE68a65cA26bBd9958A985";
    // Convert to byte array
    const bytesMember2 = new Uint8Array(
      hexStringMember2.match(/.{1,2}/g)!.map((byte) => parseInt(byte, 16))
    );
    const ethAddressMember2 = Array.from(bytesMember2.slice(0, 20));

    const bridgeConfigPDA = pdaDeriver.bridge_config();
    await bridgeProgram.methods
      .initialize({
        threshold: 1,
        members: [ethAddressMember1, ethAddressMember2],
      })
      .accounts({
        payer: authority.publicKey,
        //@ts-ignore
        bridgeConfig: bridgeConfigPDA,
        systemProgram: SystemProgram.programId,
      })
      .signers([authority])
      .rpc();

    const bridgeConfig = await bridgeProgram.account.bridgeConfig.fetch(
      bridgeConfigPDA
    );

    expect(bridgeConfig.threshold).toBe(1);
  });

  test("set_threshold", async () => {
    const bridgeConfigPDA = pdaDeriver.bridge_config();
    const privateKey1 = getPrivateKey(1);
    const privateKey2 = getPrivateKey(2);

    const message = randomBytes(32);
    const signature1 = await signMessage(message, privateKey1);
    const signature2 = await signMessage(message, privateKey2);

    await bridgeProgram.methods
      .setThreshold({
        threshold: 2,
        message: message,
        signatures: [signature1, signature2],
      })
      .accounts({
        payer: authority.publicKey,
        //@ts-ignore
        bridgeConfig: bridgeConfigPDA,
      })
      .signers([authority])
      .rpc();

    const bridgeConfig = await bridgeProgram.account.bridgeConfig.fetch(
      bridgeConfigPDA
    );

    expect(bridgeConfig.threshold).toBe(2);
  });
});
