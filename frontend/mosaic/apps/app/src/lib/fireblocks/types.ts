/**
 * Fireblocks Integration Types
 * 
 * Types for Fireblocks institutional custody integration
 */

export interface FireblocksConfig {
  apiKey: string;
  apiSecret: string;
  vaultAccountId: string;
  sandbox: boolean;
  baseUrl?: string;
}

export interface FireblocksWallet {
  id: string;
  name: string;
  address: string;
  vaultId: string;
  type: 'fireblocks';
  icon: string;
}

export interface FireblocksConnectionState {
  connected: boolean;
  connecting: boolean;
  wallet: FireblocksWallet | null;
  error: string | null;
}

export interface FireblocksTransaction {
  id: string;
  status: 'PENDING' | 'SUBMITTED' | 'COMPLETED' | 'FAILED';
  txHash?: string;
  amount: string;
  asset: string;
  destination: string;
  policyEngine?: PolicyEngineResult;
}

export interface PolicyEngineResult {
  approved: boolean;
  approvers: string[];
  reason?: string;
}

export interface FireblocksSignerResult {
  signature: string;
  transactionId: string;
  status: 'signed' | 'pending' | 'failed';
}

export type FireblocksEventType = 
  | 'connected'
  | 'disconnected'
  | 'transaction_created'
  | 'transaction_completed'
  | 'transaction_failed'
  | 'policy_approval_required';

export interface FireblocksEvent {
  type: FireblocksEventType;
  data?: unknown;
  timestamp: number;
}
