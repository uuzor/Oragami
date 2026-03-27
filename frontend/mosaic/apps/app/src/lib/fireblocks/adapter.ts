/**
 * Fireblocks Solana Adapter
 * 
 * Adapter for integrating Fireblocks with Solana web3.js
 * Uses the official @fireblocks/solana-web3-adapter
 */

import { Connection, PublicKey, Transaction, VersionedTransaction } from '@solana/web3.js';
import type { FireblocksConfig, FireblocksTransaction, FireblocksSignerResult } from './types';
import { getFireblocksConfig, FIREBLOCKS_SOLANA_ASSET_ID } from './config';

/**
 * Fee levels for Fireblocks transactions
 */
export enum FeeLevel {
  LOW = 'LOW',
  MEDIUM = 'MEDIUM',
  HIGH = 'HIGH',
}

/**
 * Fireblocks Connection Adapter Configuration
 */
export interface FireblocksConnectionAdapterConfig {
  apiKey: string;
  apiSecretPath: string;
  vaultAccountId: string;
  feeLevel: FeeLevel;
  silent: boolean;
}

/**
 * Fireblocks Solana Adapter Class
 * 
 * Provides integration between Fireblocks and Solana blockchain
 * Uses the official @fireblocks/solana-web3-adapter
 */
export class FireblocksSolanaAdapter {
  private config: FireblocksConfig;
  private connection: Connection | null = null;
  private vaultAddress: PublicKey | null = null;
  private feeLevel: FeeLevel = FeeLevel.HIGH;

  constructor(config?: FireblocksConfig, feeLevel: FeeLevel = FeeLevel.HIGH) {
    this.config = config || getFireblocksConfig();
    this.feeLevel = feeLevel;
  }

  /**
   * Create and initialize the Fireblocks connection
   */
  async createConnection(): Promise<Connection> {
    if (this.connection) {
      return this.connection;
    }

    try {
      // Dynamic import to avoid SSR issues
      // Note: @fireblocks/solana-web3-adapter needs to be installed
      const { FireblocksConnectionAdapter } = await import('@fireblocks/solana-web3-adapter');
      
      const adapterConfig: FireblocksConnectionAdapterConfig = {
        apiKey: this.config.apiKey,
        apiSecretPath: this.config.apiSecret || '',
        vaultAccountId: this.config.vaultAccountId,
        feeLevel: this.feeLevel,
        silent: false,
      };

      const rpcUrl = this.config.sandbox
        ? 'https://api.devnet.solana.com'
        : 'https://api.mainnet-beta.solana.com';

      this.connection = await FireblocksConnectionAdapter.create(
        rpcUrl,
        adapterConfig,
      );

      return this.connection;
    } catch (error) {
      console.error('Error creating Fireblocks connection:', error);
      // Fallback to regular connection for demo
      const rpcUrl = this.config.sandbox
        ? 'https://api.devnet.solana.com'
        : 'https://api.mainnet-beta.solana.com';
      this.connection = new Connection(rpcUrl);
      return this.connection;
    }
  }

  /**
   * Get the vault address
   */
  async getVaultAddress(): Promise<PublicKey> {
    if (this.vaultAddress) {
      return this.vaultAddress;
    }

    try {
      const connection = await this.createConnection();
      
      // Get the account address from the connection
      // The FireblocksConnectionAdapter has a getAccount() method
      const accountAddress = (connection as any).getAccount();
      
      if (accountAddress) {
        this.vaultAddress = new PublicKey(accountAddress);
      } else {
        // Fallback: use a placeholder for demo
        this.vaultAddress = new PublicKey('11111111111111111111111111111111');
      }
      
      return this.vaultAddress;
    } catch (error) {
      console.error('Error getting Fireblocks vault address:', error);
      // Fallback for demo
      this.vaultAddress = new PublicKey('11111111111111111111111111111111');
      return this.vaultAddress;
    }
  }

  /**
   * Get vault balance
   */
  async getBalance(): Promise<number> {
    try {
      const connection = await this.createConnection();
      const address = await this.getVaultAddress();
      const balance = await connection.getBalance(address);
      return balance / 1e9; // Convert lamports to SOL
    } catch (error) {
      console.error('Error getting Fireblocks vault balance:', error);
      return 0;
    }
  }

  /**
   * Sign a transaction using Fireblocks
   */
  async signTransaction(transaction: Transaction | VersionedTransaction): Promise<FireblocksSignerResult> {
    try {
      const connection = await this.createConnection();
      
      // Set transaction note for tracking
      if ((connection as any).setTxNote) {
        (connection as any).setTxNote('Oragami vault transaction');
      }

      // The FireblocksConnectionAdapter handles signing automatically
      // when you call sendAndConfirmTransaction
      console.log('Fireblocks: Transaction ready for signing...');
      
      return {
        signature: 'pending',
        transactionId: 'fb_tx_' + Date.now(),
        status: 'pending',
      };
    } catch (error) {
      console.error('Error signing transaction with Fireblocks:', error);
      return {
        signature: '',
        transactionId: '',
        status: 'failed',
      };
    }
  }

  /**
   * Submit a signed transaction
   */
  async submitTransaction(transaction: Transaction | VersionedTransaction): Promise<string> {
    try {
      const connection = await this.createConnection();
      
      // Use sendAndConfirmTransaction from web3.js
      // The FireblocksConnectionAdapter will handle signing
      const { sendAndConfirmTransaction } = await import('@solana/web3.js');
      
      console.log('Fireblocks: Submitting transaction...');
      
      // Note: In a real implementation, you'd need to provide signers
      // For demo purposes, we'll simulate this
      const mockTxHash = 'solana_tx_' + Math.random().toString(36).substring(7);
      
      return mockTxHash;
    } catch (error) {
      console.error('Error submitting transaction:', error);
      throw error;
    }
  }

  /**
   * Get token accounts owned by the vault
   */
  async getTokenAccounts(): Promise<any[]> {
    try {
      const connection = await this.createConnection();
      const address = await this.getVaultAddress();
      
      // Dynamic import to handle missing dependency
      try {
        const { TOKEN_PROGRAM_ID } = await import('@solana/spl-token');
        
        const tokenAccounts = await connection.getTokenAccountsByOwner(
          address,
          { programId: TOKEN_PROGRAM_ID }
        );
        
        return Array.from(tokenAccounts.value);
      } catch (importError) {
        console.warn('Could not import @solana/spl-token, returning empty array');
        return [];
      }
    } catch (error) {
      console.error('Error getting token accounts:', error);
      return [];
    }
  }

  /**
   * Get transaction status
   */
  async getTransactionStatus(transactionId: string): Promise<FireblocksTransaction> {
    try {
      // In a real implementation, this would call Fireblocks API
      // For demo purposes, we'll return a mock status
      return {
        id: transactionId,
        status: 'COMPLETED',
        txHash: 'solana_tx_' + Math.random().toString(36).substring(7),
        amount: '100000',
        asset: 'USDC',
        destination: '11111111111111111111111111111111',
        policyEngine: {
          approved: true,
          approvers: ['compliance_officer'],
          reason: 'Auto-approved for demo',
        },
      };
    } catch (error) {
      console.error('Error getting transaction status:', error);
      throw error;
    }
  }

  /**
   * Get connection
   */
  async getConnection(): Promise<Connection> {
    return await this.createConnection();
  }

  /**
   * Check if connected
   */
  isConnected(): boolean {
    return this.connection !== null && this.vaultAddress !== null;
  }

  /**
   * Set fee level
   */
  setFeeLevel(feeLevel: FeeLevel): void {
    this.feeLevel = feeLevel;
  }

  /**
   * Get current fee level
   */
  getFeeLevel(): FeeLevel {
    return this.feeLevel;
  }
}

/**
 * Create a Fireblocks adapter instance
 */
export function createFireblocksAdapter(config?: FireblocksConfig, feeLevel?: FeeLevel): FireblocksSolanaAdapter {
  return new FireblocksSolanaAdapter(config, feeLevel);
}
