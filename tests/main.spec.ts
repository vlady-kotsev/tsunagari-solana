import { BankrunProvider } from "anchor-bankrun";
import {
  LAMPORTS_PER_SOL,
  Keypair,
  SystemProgram,
  PublicKey,
} from "@solana/web3.js";
import {
  createMint,
  createAssociatedTokenAccount,
  mintTo,
  getAccount,
  //@ts-ignore
} from "spl-token-bankrun";
import { BN, Program, Wallet } from "@coral-xyz/anchor";
import { BridgeSolana } from "../target/types/bridge_solana";
import { randomBytes } from "crypto";
import BRIDGE_IDL from "../target/idl/bridge_solana.json";
import { contextPromise } from "./context";

import { ProgramTestContext } from "solana-bankrun";
import { loadKeypair, signMessage } from "./utils";
import pdaDeriver from "./pda-deriver";
import { getPrivateKey } from "./env";
import {
  ACCOUNT_SIZE,
  AccountLayout,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddress,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";

const hexStringMember1 = "4747b7f5c40599E1C5CF5a72C535D953B64916b6";
const hexStringMember2 = "604B11e1F6b2ffD330dE68a65cA26bBd9958A985";

describe("bridge_solana_tests", () => {
  let bridgeProgram: Program<BridgeSolana>;
  let authority: Keypair;
  let context: ProgramTestContext;
  let provider: BankrunProvider;
  let nativeTokenMint: PublicKey;
  let wrappedTokenMint: PublicKey;

  beforeAll(async () => {
    context = await contextPromise;
    authority = await loadKeypair();
    provider = new BankrunProvider(context, new Wallet(authority));
    bridgeProgram = new Program<BridgeSolana>(BRIDGE_IDL as any, provider);
  });

  test("initialize", async () => {
    // Convert to byte array
    const bytesMember1 = new Uint8Array(
      hexStringMember1.match(/.{1,2}/g)!.map((byte) => parseInt(byte, 16))
    );
    const ethAddressMember1 = Array.from(bytesMember1.slice(0, 20));

    // Convert to byte array
    const bytesMember2 = new Uint8Array(
      hexStringMember2.match(/.{1,2}/g)!.map((byte) => parseInt(byte, 16))
    );
    const ethAddressMember2 = Array.from(bytesMember2.slice(0, 20));

    const bridgeConfigPDA = pdaDeriver.bridgeConfig();
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
    const bridgeConfigPDA = pdaDeriver.bridgeConfig();
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

    let bridgeConfig = await bridgeProgram.account.bridgeConfig.fetch(
      bridgeConfigPDA
    );

    expect(bridgeConfig.threshold).toBe(2);

    await bridgeProgram.methods
      .setThreshold({
        threshold: 1,
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

    bridgeConfig = await bridgeProgram.account.bridgeConfig.fetch(
      bridgeConfigPDA
    );

    expect(bridgeConfig.threshold).toBe(1);
  });

  test("set_member", async () => {
    const bridgeConfigPDA = pdaDeriver.bridgeConfig();
    const privateKey1 = getPrivateKey(1);
    const privateKey2 = getPrivateKey(2);

    const message = randomBytes(32);
    const signature1 = await signMessage(message, privateKey1);
    const signature2 = await signMessage(message, privateKey2);

    const bytesMember2 = new Uint8Array(
      hexStringMember2.match(/.{1,2}/g)!.map((byte) => parseInt(byte, 16))
    );

    await bridgeProgram.methods
      .setMember({
        memberKey: Array.from(bytesMember2),
        action: false,
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

    expect(bridgeConfig.members.length).toBe(1);
  });

  test("add_supported_tokens", async () => {
    nativeTokenMint = await createMint(
      context.banksClient,
      authority,
      authority.publicKey,
      null,
      3
    );

    const splVaultPDA = pdaDeriver.splVault();

    const splVaultAtaPDA = await getAssociatedTokenAddress(
      nativeTokenMint,
      splVaultPDA,
      true
    );

    const tokenDetailsPDA = pdaDeriver.tokenDetails(nativeTokenMint);
    const bridgeConfigPDA = pdaDeriver.bridgeConfig();

    const privateKey1 = getPrivateKey(1);
    const message = randomBytes(32);
    const signature1 = await signMessage(message, privateKey1);

    await bridgeProgram.methods
      .addSupportedToken({
        tokenMint: nativeTokenMint,
        symbol: "TokenA",
        minAmount: new BN(1),
        message: message,
        signatures: [signature1],
      })
      .accounts({
        payer: authority.publicKey,
        tokenMint: nativeTokenMint,
        //@ts-ignore
        splVault: splVaultPDA,
        bridgeAta: splVaultAtaPDA,
        tokenDetails: tokenDetailsPDA,
        bridgeConfig: bridgeConfigPDA,
        tokenProgram: TOKEN_PROGRAM_ID,
        associcatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SYSTEM_PROGRAM_ID,
      })

      .signers([authority])
      .rpc();

    const tokenDetails = await bridgeProgram.account.tokenDetails.fetch(
      tokenDetailsPDA
    );

    expect(tokenDetails.decimals).toBe(3);
  });

  test("mint", async () => {
    const privateKey1 = getPrivateKey(1);
    const message = randomBytes(32);
    const signature1 = await signMessage(message, privateKey1);

    const splVaultPDA = pdaDeriver.splVault();

    wrappedTokenMint = await createMint(
      context.banksClient,
      authority,
      splVaultPDA,
      null,
      3
    );

    const receiverATA = await createAssociatedTokenAccount(
      context.banksClient,
      authority,
      wrappedTokenMint,
      authority.publicKey
    );

    const bridgeConfigPDA = pdaDeriver.bridgeConfig();

    await bridgeProgram.methods
      .mintWrapped({
        amount: new BN(1000), // 1 token with 3 decimals
        to: authority.publicKey,
        wrappedTokenAddress: wrappedTokenMint,
        message,
        signatures: [signature1],
      })
      .accounts({
        payer: authority.publicKey,
        receiver: authority.publicKey,
        mint: wrappedTokenMint,
        //@ts-ignore
        receiverAta: receiverATA,
        splVault: splVaultPDA,
        bridgeConfig: bridgeConfigPDA,
        tokenProgram: TOKEN_PROGRAM_ID,
        associcatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SYSTEM_PROGRAM_ID,
      })
      .signers([authority])
      .rpc();

    const ataAccount = await context.banksClient.getAccount(receiverATA);
    expect(ataAccount).not.toBeNull();

    expect(Number(AccountLayout.decode(ataAccount!.data).amount)).toBe(1000);
  });

  test("burn", async () => {
    const receiverATA = await getAssociatedTokenAddress(
      wrappedTokenMint,
      authority.publicKey
    );

    await bridgeProgram.methods
      .burnWrapped({
        amount: new BN(500), // 0.5 token with 3 decimals
        wrappedTokenAddress: wrappedTokenMint,
      })
      .accounts({
        payer: authority.publicKey,
        mint: wrappedTokenMint,
        from: receiverATA,
        //@ts-ignore
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([authority])
      .rpc();

    const ataAccount = await context.banksClient.getAccount(receiverATA);
    expect(ataAccount).not.toBeNull();

    expect(Number(AccountLayout.decode(ataAccount!.data).amount)).toBe(500);
  });

  test("lock", async () => {
    const tokenDetailsPDA = pdaDeriver.tokenDetails(nativeTokenMint);
    const vaultPDA = pdaDeriver.splVault();

    const userAta = await createAssociatedTokenAccount(
      context.banksClient,
      authority,
      nativeTokenMint,
      authority.publicKey
    );

    await mintTo(
      context.banksClient,
      authority,
      nativeTokenMint,
      userAta,
      authority,
      1 * 10 ** 3
    );

    const vaultAtaPDA = await getAssociatedTokenAddress(
      nativeTokenMint,
      vaultPDA,
      true
    );

    await bridgeProgram.methods
      .lock({
        tokenMint: nativeTokenMint,
        amount: new BN(500),
      })
      .accounts({
        payer: authority.publicKey,
        //@ts-ignore
        tokenDetails: tokenDetailsPDA,
        mint: nativeTokenMint,
        splVault: vaultPDA,
        from: userAta,
        to: vaultAtaPDA,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([authority])
      .rpc();

    const userAtaAccount = await context.banksClient.getAccount(userAta);
    expect(Number(AccountLayout.decode(userAtaAccount!.data).amount)).toBe(500);

    const vaultAtaAccount = await context.banksClient.getAccount(vaultAtaPDA);
    expect(Number(AccountLayout.decode(vaultAtaAccount!.data).amount)).toBe(
      500
    );
  });
});
