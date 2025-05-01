import * as dotenv from "dotenv";

dotenv.config();

export function getPrivateKey(id: number): string {
  const privateKey = process.env[`ETH_TEST_PRIVATE_KEY_${id}`];
  if (!privateKey) {
    throw new Error(
      `ETH_TEST_PRIVATE_KEY_${id} not found in environment variables`
    );
  }
  return privateKey.startsWith("0x") ? privateKey.slice(2) : privateKey;
}
