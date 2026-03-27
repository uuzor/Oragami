/**
 * Fireblocks Configuration
 * 
 * Configuration for Fireblocks institutional custody integration
 */

import type { FireblocksConfig } from './types';

/**
 * Get Fireblocks configuration from environment variables
 */
export function getFireblocksConfig(): FireblocksConfig {
  const apiKey = process.env.NEXT_PUBLIC_FIREBLOCKS_API_KEY;
  const apiSecret = process.env.FIREBLOCKS_API_SECRET; // Server-side only
  const vaultAccountId = process.env.NEXT_PUBLIC_FIREBLOCKS_VAULT_ID;
  const sandbox = process.env.NEXT_PUBLIC_FIREBLOCKS_SANDBOX === 'true';

  if (!apiKey || !vaultAccountId) {
    throw new Error(
      'Fireblocks configuration missing. Please set NEXT_PUBLIC_FIREBLOCKS_API_KEY and NEXT_PUBLIC_FIREBLOCKS_VAULT_ID environment variables.'
    );
  }

  return {
    apiKey,
    apiSecret: apiSecret || '',
    vaultAccountId,
    sandbox,
    baseUrl: sandbox
      ? 'https://sandbox-api.fireblocks.io'
      : 'https://api.fireblocks.io',
  };
}

/**
 * Check if Fireblocks is configured
 */
export function isFireblocksConfigured(): boolean {
  try {
    const apiKey = process.env.NEXT_PUBLIC_FIREBLOCKS_API_KEY;
    const vaultAccountId = process.env.NEXT_PUBLIC_FIREBLOCKS_VAULT_ID;
    return !!(apiKey && vaultAccountId);
  } catch {
    return false;
  }
}

/**
 * Fireblocks asset ID for Solana
 */
export const FIREBLOCKS_SOLANA_ASSET_ID = 'SOL';

/**
 * Fireblocks asset ID for USDC on Solana
 */
export const FIREBLOCKS_USDC_ASSET_ID = 'USDC_SOL';

/**
 * Default vault account name
 */
export const DEFAULT_VAULT_NAME = 'Oragami Institutional Vault';

/**
 * Fireblocks sandbox mode
 */
export const FIREBLOCKS_SANDBOX_MODE = process.env.NEXT_PUBLIC_FIREBLOCKS_SANDBOX === 'true';
