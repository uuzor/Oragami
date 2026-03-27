/**
 * Fireblocks Connect Button
 * 
 * Button component for connecting Fireblocks institutional custody wallet
 */

'use client';

import { Button } from '@/components/ui/button';
import { useFireblocks } from '../hooks/use-fireblocks';
import { Shield, Loader2, AlertCircle } from 'lucide-react';
import { cn } from '@/lib/utils';

interface FireblocksConnectButtonProps {
  className?: string;
  variant?: 'default' | 'outline' | 'secondary' | 'ghost' | 'link' | 'destructive';
  size?: 'default' | 'sm' | 'lg' | 'icon';
}

export function FireblocksConnectButton({
  className,
  variant = 'outline',
  size = 'sm',
}: FireblocksConnectButtonProps) {
  const { connected, connecting, error, connect, disconnect, isConfigured, wallet } = useFireblocks();

  // If not configured, show disabled state
  if (!isConfigured) {
    return (
      <Button
        variant="outline"
        size={size}
        disabled
        className={cn('opacity-50 cursor-not-allowed', className)}
        title="Fireblocks is not configured. Set NEXT_PUBLIC_FIREBLOCKS_API_KEY and NEXT_PUBLIC_FIREBLOCKS_VAULT_ID environment variables."
      >
        <Shield className="mr-2 h-4 w-4" />
        Fireblocks (Not Configured)
      </Button>
    );
  }

  // If connecting, show loading state
  if (connecting) {
    return (
      <Button variant={variant} size={size} disabled className={className}>
        <Loader2 className="mr-2 h-4 w-4 animate-spin" />
        Connecting...
      </Button>
    );
  }

  // If connected, show connected state
  if (connected && wallet) {
    const shortAddress = `${wallet.address.slice(0, 4)}...${wallet.address.slice(-4)}`;
    
    return (
      <Button
        variant="default"
        size={size}
        onClick={disconnect}
        className={cn('bg-gradient-to-r from-blue-600 to-purple-600 hover:from-blue-700 hover:to-purple-700', className)}
      >
        <Shield className="mr-2 h-4 w-4" />
        <span className="text-xs">{shortAddress}</span>
      </Button>
    );
  }

  // If there's an error, show error state
  if (error) {
    return (
      <Button
        variant="destructive"
        size={size}
        onClick={connect}
        className={className}
      >
        <AlertCircle className="mr-2 h-4 w-4" />
        Retry Fireblocks
      </Button>
    );
  }

  // Default: show connect button
  return (
    <Button
      variant={variant}
      size={size}
      onClick={connect}
      className={cn(
        'border-2 border-blue-500/30 hover:border-blue-500/50 hover:bg-blue-500/10',
        className
      )}
    >
      <Shield className="mr-2 h-4 w-4 text-blue-500" />
      Connect Fireblocks
    </Button>
  );
}
