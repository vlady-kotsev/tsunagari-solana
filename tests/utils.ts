import { Keypair } from "@solana/web3.js";
import { secp256k1 } from "@noble/curves/secp256k1";
import * as fs from "fs";
import { ethers } from "ethers";

export const loadKeypair = async (
  keyPath: string = "./tests/wallet.json"
): Promise<Keypair> => {
  if (!fs.existsSync(keyPath)) {
    throw new Error("Wallet file does not exist");
  }

  const secretKeyArray = JSON.parse(fs.readFileSync(keyPath, "utf-8"));
  const secretKey = new Uint8Array(secretKeyArray);

  return Keypair.fromSecretKey(secretKey);
};

export async function signMessage(message: Uint8Array, privateKey: string) {
  const messageHash = ethers.keccak256(message).slice(2);
  const signature = secp256k1.sign(messageHash, privateKey);
  const signatureBytes = Buffer.concat([
    signature.toCompactRawBytes(),
    Buffer.from([signature.recovery]),
  ]);

  return signatureBytes;
}
