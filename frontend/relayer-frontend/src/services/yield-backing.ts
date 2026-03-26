/**
 * Yield Backing Service
 * 
 * Manages yield allocation, distribution, and tracking
 * for CommoVault vault shares.
 */

import { PublicKey, Connection } from '@solana/web3.js';

export interface YieldDistributionConfig {
  /** How much yield goes to vault share holders vs vault treasury */
  holderShareBps: number;  // e.g., 8000 = 80% to holders, 20% to treasury
  /** Minimum yield threshold before distribution */
  minDistributionThreshold: number;
  /** Distribution frequency in seconds */
  distributionFrequencySeconds: number;
}

export interface VaultYieldStats {
  /** Total TVL in the vault */
  totalValueLocked: number;
  /** Current APY from USX */
  currentApy: number;
  /** Daily yield earnings */
  dailyYield: number;
  /** Monthly projected yield */
  monthlyProjectedYield: number;
  /** Yield already distributed */
  distributedYield: number;
  /** Pending yield to be distributed */
  pendingYield: number;
}

/**
 * Calculate yield distribution for vault share holders
 * 
 * @param totalYield - Total yield to distribute
 * @param holderShareBps - Percentage for holders (in basis points)
 * @returns { holderAmount, treasuryAmount }
 */
export function calculateYieldDistribution(
  totalYield: number,
  holderShareBps: number
): { holderAmount: number; treasuryAmount: number } {
  const holderAmount = Math.floor((totalYield * holderShareBps) / 10000);
  const treasuryAmount = totalYield - holderAmount;
  
  return { holderAmount, treasuryAmount };
}

/**
 * Get current yield statistics for the vault
 * This aggregates data from multiple sources
 */
export async function getVaultYieldStats(
  connection: Connection,
  vaultAddress: PublicKey,
  totalSupply: number,
  usxPrice: number
): Promise<VaultYieldStats> {
  // Get current APY from USX (approximately 5% in 2026)
  const currentApy = 0.05;
  
  // Calculate yields based on TVL
  const dailyYield = (totalSupply * currentApy) / 365;
  const monthlyProjectedYield = dailyYield * 30;
  
  return {
    totalValueLocked: totalSupply,
    currentApy: currentApy * 100,
    dailyYield: Math.floor(dailyYield * usxPrice),
    monthlyProjectedYield: Math.floor(monthlyProjectedYield * usxPrice),
    distributedYield: 0, // Would query from vault state
    pendingYield: 0, // Would query from vault state
  };
}

/**
 * Calculate yield per cVAULT share
 * 
 * @param totalYield - Total yield to distribute
 * @param totalSupply - Total cVAULT supply
 * @returns Yield per 1 cVAULT token
 */
export function calculateYieldPerShare(
  totalYield: number,
  totalSupply: number
): number {
  if (totalSupply === 0) return 0;
  return totalYield / totalSupply;
}

/**
 * Determine if a yield distribution should occur
 * based on config and pending yield
 */
export function shouldDistributeYield(
  pendingYield: number,
  minThreshold: number,
  timeSinceLastDistribution: number,
  frequencySeconds: number
): boolean {
  // Check if pending yield exceeds threshold
  if (pendingYield < minThreshold) {
    return false;
  }
  
  // Check if enough time has passed
  if (timeSinceLastDistribution < frequencySeconds) {
    return false;
  }
  
  return true;
}

/**
 * Format yield data for display in the UI
 */
export interface FormattedYieldData {
  apyDisplay: string;
  dailyYieldDisplay: string;
  monthlyYieldDisplay: string;
  yearlyYieldDisplay: string;
  totalValueLockedDisplay: string;
}

export function formatYieldData(stats: VaultYieldStats): FormattedYieldData {
  return {
    apyDisplay: `${stats.currentApy.toFixed(2)}%`,
    dailyYieldDisplay: `$${stats.dailyYield.toLocaleString()}`,
    monthlyYieldDisplay: `$${stats.monthlyProjectedYield.toLocaleString()}`,
    yearlyYieldDisplay: `$${(stats.monthlyProjectedYield * 12).toLocaleString()}`,
    totalValueLockedDisplay: `$${stats.totalValueLocked.toLocaleString()}`,
  };
}

/**
 * Calculate projected returns for a given deposit amount
 */
export interface ProjectedReturns {
  dailyYield: number;
  monthlyYield: number;
  yearlyYield: number;
  effectiveApy: number;
}

export function calculateProjectedReturns(
  depositAmount: number,
  apy: number
): ProjectedReturns {
  const dailyYield = (depositAmount * apy) / 365;
  const monthlyYield = dailyYield * 30;
  const yearlyYield = dailyYield * 365;
  
  return {
    dailyYield: Math.floor(dailyYield),
    monthlyYield: Math.floor(monthlyYield),
    yearlyYield: Math.floor(yearlyYield),
    effectiveApy: apy * 100,
  };
}

/**
 * Get yield history for a vault
 * This would query historical data in production
 */
export interface YieldHistoryEntry {
  date: number;
  amount: number;
  type: 'claim' | 'distribution';
}

export async function getYieldHistory(
  connection: Connection,
  vaultAddress: PublicKey,
  limit: number = 30
): Promise<YieldHistoryEntry[]> {
  // In production, this would query the blockchain for yield events
  // For demo, return mock data
  const mockHistory: YieldHistoryEntry[] = [];
  const now = Date.now();
  
  for (let i = 0; i < limit; i++) {
    mockHistory.push({
      date: now - (i * 86400000), // each day
      amount: Math.floor(Math.random() * 500) + 100,
      type: i % 7 === 0 ? 'distribution' : 'claim',
    });
  }
  
  return mockHistory;
}

/**
 * Default yield distribution configuration
 */
export const DEFAULT_YIELD_CONFIG: YieldDistributionConfig = {
  holderShareBps: 8000, // 80% to holders
  minDistributionThreshold: 100, // $100 minimum
  distributionFrequencySeconds: 86400, // Daily
};

/**
 * Calculate the value of cVAULT including accumulated yield
 */
export interface CvaultValuation {
  /** Base value (1:1 with deposited assets) */
  baseValue: number;
  /** Accumulated yield */
  accumulatedYield: number;
  /** Total value */
  totalValue: number;
  /** Yield APY */
  yieldApy: number;
}

export function calculateCvaultValuation(
  cvaultAmount: number,
  pendingYield: number,
  apy: number
): CvaultValuation {
  return {
    baseValue: cvaultAmount,
    accumulatedYield: pendingYield,
    totalValue: cvaultAmount + pendingYield,
    yieldApy: apy * 100,
  };
}