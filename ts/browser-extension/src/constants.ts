/* eslint-disable @typescript-eslint/no-non-null-assertion */

export const CONSTANTS = {
  BALANCE_REFETCH_INTERVAL: 5000,
  GAS_PRICE_REFETCH_INTERVAL: 10000,
  CHAIN_ID: parseInt(process.env.PLASMO_PUBLIC_CHAIN_ID!),
  RPC_HTTP_ENDPOINT: process.env.PLASMO_PUBLIC_RPC_URL!,
  SHIELDER_CONTRACT_ADDRESS: process.env
    .PLASMO_PUBLIC_SHIELDER_CONTRACT_ADDRESS! as `0x${string}`,
  RELAYER_URL: process.env.PLASMO_PUBLIC_RELAYER_URL!,
  RELAYER_ADDRESS: process.env.PLASMO_PUBLIC_RELAYER_ADDRESS! as `0x${string}`,
  PUBLIC_SEND_ACTION_GAS_LIMIT: 30_000n,
};