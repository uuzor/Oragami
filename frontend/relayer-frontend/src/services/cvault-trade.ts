/**
 * cVAULT-TRADE Service
 * 
 * Secondary market tradeability for CommoVault vault shares
 * Uses Token-2022 Transfer Hook for compliance enforcement
 */

import { API_BASE_URL } from '@/lib/constants';

export interface TradeableTokenConfig {
  name: string;
  symbol: string;
  decimals: number;
  mintAuthority: string;
  transferHookProgramId: string;
}

export interface WhitelistCheckResult {
  whitelisted: boolean;
  kycCompliant: boolean;
  amlClear: boolean;
  travelRuleCompliant: boolean;
  reason?: string;
}

/**
 * Check if a wallet is whitelisted for trading cVAULT-TRADE
 * This integrates with the compliance relayer for full KYC/KYT/AML checks
 */
export async function checkTradingEligibility(
  walletAddress: string
): Promise<WhitelistCheckResult> {
  try {
    // Call the risk check endpoint which performs full compliance screening
    const response = await fetch(`${API_BASE_URL}/risk-check`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ address: walletAddress }),
    });

    if (!response.ok) {
      return {
        whitelisted: false,
        kycCompliant: false,
        amlClear: false,
        travelRuleCompliant: false,
        reason: 'Compliance check service unavailable',
      };
    }

    const result = await response.json();
    
    // Map the risk check result to whitelist status
    const isCompliant = result.status !== 'blocked' && (result.riskScore ?? 0) <= 70;
    
    return {
      whitelisted: isCompliant,
      kycCompliant: isCompliant,
      amlClear: isCompliant,
      travelRuleCompliant: isCompliant,
      reason: result.blocklistReason || (isCompliant ? undefined : 'High risk score'),
    };
  } catch (error) {
    return {
      whitelisted: false,
      kycCompliant: false,
      amlClear: false,
      travelRuleCompliant: false,
      reason: error instanceof Error ? error.message : 'Unknown error',
    };
  }
}

/**
 * Submit a transfer request for cVAULT-TRADE
 * This goes through the compliance relayer which enforces transfer hook logic
 */
export async function submitTradeableTransfer(
  fromAddress: string,
  toAddress: string,
  amount: number,
  tokenMint: string
): Promise<{ success: boolean; signature?: string; error?: string }> {
  try {
    const response = await fetch(`${API_BASE_URL}/transfer`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        fromAddress,
        toAddress,
        amount,
        tokenMint,
        // Add compliance metadata for Travel Rule
        transferMetadata: {
          purpose: 'secondary_market_trade',
          source: 'cvault_trade',
        },
      }),
    });

    if (!response.ok) {
      const error = await response.json().catch(() => ({ error: 'Transfer failed' }));
      return {
        success: false,
        error: error.error?.message || 'Transfer failed',
      };
    }

    const result = await response.json();
    return {
      success: true,
      signature: result.signature,
    };
  } catch (error) {
    return {
      success: false,
      error: error instanceof Error ? error.message : 'Unknown error',
    };
  }
}

/**
 * Convert cVAULT (non-tradable) to cVAULT-TRADE (tradable)
 * This is the opt-in mechanism for secondary market participation
 */
export interface ConversionRequest {
  cvaultAmount: number;
  userWallet: string;
}

export interface ConversionResult {
  success: boolean;
  cvaultTradeAmount?: number;
  signature?: string;
  error?: string;
}

export async function convertToTradeable(
  request: ConversionRequest
): Promise<ConversionResult> {
  // In production, this would call the vault program
  // For demo purposes, we'll simulate the conversion
  try {
    const response = await fetch(`${API_BASE_URL}/vault/convert-to-tradeable`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(request),
    });

    if (!response.ok) {
      const error = await response.json().catch(() => ({ error: 'Conversion failed' }));
      return {
        success: false,
        error: error.error?.message || 'Conversion failed',
      };
    }

    const result = await response.json();
    return {
      success: true,
      cvaultTradeAmount: result.cvaultTradeAmount,
      signature: result.signature,
    };
  } catch (error) {
    return {
      success: false,
      error: error instanceof Error ? error.message : 'Unknown error',
    };
  }
}

/**
 * Redeem cVAULT-TRADE back to cVAULT or underlying assets
 */
export interface RedemptionRequest {
  cvaultTradeAmount: number;
  destinationAddress: string;
  redeemToCvault: boolean; // true = cVAULT, false = underlying RWAs
}

export interface RedemptionResult {
  success: boolean;
  redeemedAmount?: number;
  signature?: string;
  error?: string;
}

export async function redeemTradeableTokens(
  request: RedemptionRequest
): Promise<RedemptionResult> {
  try {
    const response = await fetch(`${API_BASE_URL}/vault/redeem-tradeable`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(request),
    });

    if (!response.ok) {
      const error = await response.json().catch(() => ({ error: 'Redemption failed' }));
      return {
        success: false,
        error: error.error?.message || 'Redemption failed',
      };
    }

    const result = await response.json();
    return {
      success: true,
      redeemedAmount: result.redeemedAmount,
      signature: result.signature,
    };
  } catch (error) {
    return {
      success: false,
      error: error instanceof Error ? error.message : 'Unknown error',
    };
  }
}

/**
 * Get the current market price of cVAULT-TRADE
 * This would use Chainlink price feeds in production
 */
export async function getTradeableTokenPrice(): Promise<{ price: number; timestamp: number }> {
  // In production, this would query Chainlink Data Feeds
  // For demo, return a mock price
  return {
    price: 1.0, // Should be backed 1:1 with cVAULT
    timestamp: Date.now(),
  };
}

/**
 * Check if secondary market trading is enabled for the vault
 */
export async function isSecondaryMarketEnabled(): Promise<boolean> {
  try {
    const response = await fetch(`${API_BASE_URL}/vault/config`);
    if (!response.ok) return false;
    
    const config = await response.json();
    return config.secondaryMarketEnabled ?? false;
  } catch {
    return false;
  }
}