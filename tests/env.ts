import * as dotenv from "dotenv";

dotenv.config();

export function getPrivateKey(): string {
  const privateKey = process.env.ETH_TEST_PRIVATE_KEY;
  if (!privateKey) {
    throw new Error("ETH_TEST_PRIVATE_KEY not found in environment variables");
  }
  return privateKey.startsWith("0x") ? privateKey.slice(2) : privateKey;
}
