/**
 * Fireblocks Wallet Hook
 * 
 * React hook for managing Fireblocks wallet connection and state
 */

'use client';

import { useState, useCallback, useEffect } from 'react';
import type { FireblocksWallet, FireblocksConnectionState } from '@/lib/fireblocks/types';
import { isFireblocksConfigured, getFireblocksConfig } from '@/lib/fireblocks/config';
import { createFireblocksAdapter } from '@/lib/fireblocks/adapter';

/**
 * Hook for managing Fireblocks wallet connection
 */
export function useFireblocks() {
  const [connectionState, setConnectionState] = useState<FireblocksConnectionState>({
    connected: false,
    connecting: false,
    wallet: null,
    error: null,
  });

  const [adapter, setAdapter] = useState<ReturnType<typeof createFireblocksAdapter> | null>(null);

  /**
   * Connect to Fireblocks wallet
   */
  const connect = useCallback(async () => {
    if (!isFireblocksConfigured()) {
      setConnectionState(prev => ({
        ...prev,
        error: 'Fireblocks is not configured. Please set NEXT_PUBLIC_FIREBLOCKS_API_KEY and NEXT_PUBLIC_FIREBLOCKS_VAULT_ID environment variables.',
      }));
      return;
    }

    setConnectionState(prev => ({ ...prev, connecting: true, error: null }));

    try {
      const config = getFireblocksConfig();
      const fireblocksAdapter = createFireblocksAdapter(config);
      
      // Get vault address
      const vaultAddress = await fireblocksAdapter.getVaultAddress();
      
      // Create wallet object
      const wallet: FireblocksWallet = {
        id: config.vaultAccountId,
        name: 'Oragami Institutional Vault',
        address: vaultAddress.toBase58(),
        vaultId: config.vaultAccountId,
        type: 'fireblocks',
        icon: '/fireblocks-icon.svg', // You'll need to add this icon
      };

      setAdapter(fireblocksAdapter);
      setConnectionState({
        connected: true,
        connecting: false,
        wallet,
        error: null,
      });

      console.log('Fireblocks wallet connected:', wallet.address);
    } catch (error) {
      console.error('Error connecting to Fireblocks:', error);
      setConnectionState({
        connected: false,
        connecting: false,
        wallet: null,
        error: error instanceof Error ? error.message : 'Failed to connect to Fireblocks',
      });
    }
  }, []);

  /**
   * Disconnect from Fireblocks wallet
   */
  const disconnect = useCallback(() => {
    setAdapter(null);
    setConnectionState({
      connected: false,
      connecting: false,
      wallet: null,
      error: null,
    });
    console.log('Fireblocks wallet disconnected');
  }, []);

  /**
   * Get vault balance
   */
  const getBalance = useCallback(async () => {
    if (!adapter || !connectionState.connected) {
      return 0;
    }

    try {
      return await adapter.getBalance();
    } catch (error) {
      console.error('Error getting Fireblocks balance:', error);
      return 0;
    }
  }, [adapter, connectionState.connected]);

  /**
   * Sign a transaction
   */
  const signTransaction = useCallback(async (transaction: Parameters<NonNullable<typeof adapter>['signTransaction']>[0]) => {
    if (!adapter || !connectionState.connected) {
      throw new Error('Fireblocks wallet not connected');
    }

    return await adapter.signTransaction(transaction);
  }, [adapter, connectionState.connected]);

  /**
   * Check if Fireblocks is configured
   */
  const isConfigured = isFireblocksConfigured();

  return {
    // Connection state
    connected: connectionState.connected,
    connecting: connectionState.connecting,
    wallet: connectionState.wallet,
    error: connectionState.error,
    
    // Configuration
    isConfigured,
    
    // Actions
    connect,
    disconnect,
    getBalance,
    signTransaction,
    
    // Adapter
    adapter,
  };
}
