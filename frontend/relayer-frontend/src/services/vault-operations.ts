/**
 * Vault Operation Service
 * Integrates compliance checking with vault operations (deposit/redeem)
 */

import { checkWalletRisk } from './risk-check';
import { API_BASE_URL } from '@/lib/constants';

export interface VaultOperationRequest {
  operation: 'deposit' | 'redeem';
  amount: number;
  tokenMint?: string;
  fromAddress: string;
  toAddress: string;
  nonce: string;
}

export interface VaultOperationResponse {
  success: boolean;
  transaction?: string;
  signature?: string;
  error?: string;
}

/**
 * Pre-flight compliance check before any vault operation
 * This must pass before the transaction is signed and submitted
 */
export async function checkVaultCompliance(
  walletAddress: string
): Promise<{ compliant: boolean; reason?: string }> {
  try {
    const result = await checkWalletRisk(walletAddress);
    
    if (result.status === 'blocked') {
      return { 
        compliant: false, 
        reason: result.blocklistReason || 'Wallet failed compliance check' 
      };
    }
    
    if (result.riskScore && result.riskScore > 70) {
      return { 
        compliant: false, 
        reason: `High risk score: ${result.riskScore}` 
      };
    }
    
    return { compliant: true };
  } catch (error) {
    return { 
      compliant: false, 
      reason: error instanceof Error ? error.message : 'Unknown error' 
    };
  }
}

/**
 * Submit vault operation after compliance check passes
 * This would call the backend which then submits to the Solana program
 */
export async function submitVaultOperation(
  request: VaultOperationRequest,
  signedMessage: string
): Promise<VaultOperationResponse> {
  const response = await fetch(`${API_BASE_URL}/vault-operation`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      ...request,
      signature: signedMessage,
    }),
  });

  if (!response.ok) {
    const error = await response.json().catch(() => ({ error: 'Unknown error' }));
    return { 
      success: false, 
      error: error.error?.message || 'Operation failed' 
    };
  }

  return response.json();
}

/**
 * Generate a unique nonce for replay protection
 */
export function generateNonce(): string {
  return `${Date.now()}-${Math.random().toString(36).substring(2, 11)}`;
}

/**
 * Format vault operation message for signing
 * Format: {from}:{to}:{amount}:{token}:{nonce}
 */
export function formatOperationMessage(
  fromAddress: string,
  toAddress: string,
  amount: number,
  tokenMint: string = 'SOL',
  nonce: string
): string {
  return `${fromAddress}:${toAddress}:${amount}:${tokenMint}:${nonce}`;
}