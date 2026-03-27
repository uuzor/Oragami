# Fireblocks Frontend Implementation Plan

## Oragami (CommoVault) - Competitive Differentiation

**Date:** March 27, 2026  
**Goal:** Implement Fireblocks institutional custody in frontend for demo video  
**Purpose:** Differentiate from competitors with institutional-grade security

---

## Executive Summary

Implement Fireblocks institutional custody integration in the Oragami frontend to demonstrate enterprise-grade security and compliance features. This will be a key differentiator for the StableHacks 2026 demo, showing that Oragami is the first Solana RWA vault with institutional custody support.

---

## Implementation Strategy

### Phase 1: Core Integration (Demo-Ready)

**Goal:** Basic Fireblocks wallet connection and transaction signing for demo video

**Components:**
1. Fireblocks SDK installation
2. Fireblocks wallet provider
3. Fireblocks connection button
4. Transaction signing integration

### Phase 2: Enhanced Features (Post-Hackathon)

**Goal:** Full policy engine and Travel Rule integration

**Components:**
1. Policy engine configuration
2. Travel Rule messaging
3. Webhook handlers
4. Advanced approval workflows

---

## Technical Architecture

### Current Architecture
```
Frontend (Next.js)
├── Wallet Provider (@solana/connector)
├── Connect Button (browser wallets)
└── Transaction Signing (browser wallets)
```

### New Architecture with Fireblocks
```
Frontend (Next.js)
├── Wallet Provider (@solana/connector)
│   ├── Browser Wallets (Phantom, Solflare)
│   └── Fireblocks Wallet (NEW)
├── Connect Button
│   ├── Browser Wallet Option
│   └── Fireblocks Option (NEW)
└── Transaction Signing
    ├── Browser Wallet Signing
    └── Fireblocks Signing (NEW)
```

---

## Implementation Steps

### Step 1: Install Fireblocks SDK

**Dependencies to Add:**
```json
{
  "dependencies": {
    "@fireblocks/solana-web3-adapter": "^1.0.0",
    "@fireblocks/ts-sdk": "^1.0.0"
  }
}
```

**File:** `frontend/mosaic/apps/app/package.json`

### Step 2: Create Fireblocks Wallet Provider

**New File:** `frontend/mosaic/apps/app/src/features/wallet/hooks/use-fireblocks.ts`

**Purpose:** Manage Fireblocks connection state and authentication

**Features:**
- API key management
- Vault account configuration
- Connection state management
- Authentication handling

### Step 3: Create Fireblocks Connection Button

**New File:** `frontend/mosaic/apps/app/src/features/wallet/components/fireblocks-connect-button.tsx`

**Purpose:** UI for connecting Fireblocks wallet

**Features:**
- Fireblocks branding
- Connection status display
- Error handling
- Loading states

### Step 4: Update Connect Button

**Modify:** `frontend/mosaic/apps/app/src/features/wallet/components/connect-button.tsx`

**Changes:**
- Add Fireblocks connection option
- Show both browser wallet and Fireblocks options
- Handle Fireblocks connection state

### Step 5: Create Fireblocks Transaction Signer

**New File:** `frontend/mosaic/apps/app/src/features/wallet/hooks/use-fireblocks-signer.ts`

**Purpose:** Sign transactions using Fireblocks

**Features:**
- Transaction preparation
- Fireblocks signing
- Transaction submission
- Error handling

### Step 6: Update Providers

**Modify:** `frontend/mosaic/apps/app/src/app/providers.tsx`

**Changes:**
- Add Fireblocks provider
- Configure Fireblocks API credentials
- Integrate with existing wallet system

### Step 7: Environment Configuration

**Modify:** `frontend/mosaic/apps/app/.env.example`

**Add:**
```bash
# Fireblocks Configuration
NEXT_PUBLIC_FIREBLOCKS_API_KEY=your_api_key
NEXT_PUBLIC_FIREBLOCKS_VAULT_ID=your_vault_id
NEXT_PUBLIC_FIREBLOCKS_SANDBOX=true
```

---

## File Structure

```
frontend/mosaic/apps/app/src/
├── features/
│   └── wallet/
│       ├── components/
│       │   ├── connect-button.tsx (MODIFY)
│       │   ├── fireblocks-connect-button.tsx (NEW)
│       │   ├── wallet-modal.tsx (MODIFY)
│       │   └── wallet-dropdown-content.tsx (MODIFY)
│       └── hooks/
│           ├── use-fireblocks.ts (NEW)
│           ├── use-fireblocks-signer.ts (NEW)
│           └── use-connector-signer.ts (MODIFY)
├── app/
│   └── providers.tsx (MODIFY)
└── lib/
    └── fireblocks/
        ├── config.ts (NEW)
        ├── adapter.ts (NEW)
        └── types.ts (NEW)
```

---

## Key Features for Demo

### 1. Dual Wallet Connection
- Show both browser wallet and Fireblocks options
- Clear visual distinction between wallet types
- Seamless switching between wallets

### 2. Institutional Branding
- Fireblocks logo and branding
- "Institutional Custody" badge
- Security indicators (MPC, HSM)

### 3. Transaction Signing
- Fireblocks transaction signing flow
- Policy engine approval display (simulated)
- Transaction status tracking

### 4. Security Features Display
- MPC key management indicator
- Multi-signature approval display
- Insurance coverage badge

---

## Demo Video Script

### Opening
"Oragami is the first Solana RWA vault with institutional custody support through Fireblocks integration."

### Wallet Connection
"Users can connect either a browser wallet for retail access or Fireblocks for institutional custody."

### Transaction Flow
"When an institution initiates a deposit, Fireblocks policy engine enforces approval workflows."

### Security Features
"Fireblocks provides MPC key management, multi-signature approvals, and up to $150M insurance coverage."

### Competitive Advantage
"This institutional-grade security differentiates Oragami from competitors like Ondo Finance and Maple Finance."

---

## Implementation Priority

### Must-Have (Demo Video)
1. ✅ Fireblocks wallet connection button
2. ✅ Basic transaction signing
3. ✅ Visual distinction from browser wallets
4. ✅ Security feature indicators

### Nice-to-Have (Demo Video)
1. ⚠️ Policy engine approval simulation
2. ⚠️ Travel Rule message display
3. ⚠️ Transaction status tracking

### Post-Hackathon
1. ❌ Full policy engine integration
2. ❌ Real Travel Rule messaging
3. ❌ Webhook handlers

---

## Testing Strategy

### Unit Tests
- Fireblocks hook tests
- Connection state tests
- Transaction signing tests

### Integration Tests
- Wallet connection flow
- Transaction submission
- Error handling

### Demo Tests
- End-to-end wallet connection
- Transaction signing flow
- UI/UX validation

---

## Risk Assessment

### Technical Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| SDK Integration Issues | Medium | High | Use sandbox, extensive testing |
| Transaction Signing Errors | Low | High | Fallback to browser wallet |
| UI/UX Complexity | Low | Medium | Keep simple for demo |

### Demo Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Fireblocks API Down | Low | High | Use sandbox, have backup |
| Connection Failures | Medium | Medium | Show error states clearly |
| Slow Transaction Signing | Medium | Low | Show loading states |

---

## Success Criteria

### Demo Video
- ✅ Fireblocks wallet connects successfully
- ✅ Transaction signing works
- ✅ Visual distinction from browser wallets
- ✅ Security features displayed clearly

### Competitive Differentiation
- ✅ First Solana RWA vault with Fireblocks
- ✅ Institutional-grade custody
- ✅ Policy engine integration (simulated)
- ✅ Travel Rule compliance (simulated)

---

## Timeline

### Day 1 (Today)
- Install Fireblocks SDK
- Create Fireblocks wallet provider
- Create Fireblocks connection button

### Day 2
- Implement transaction signing
- Update connect button
- Update providers

### Day 3
- Add environment configuration
- Test integration
- Prepare for demo video

---

## Next Steps

1. **Immediate:** Install Fireblocks SDK dependencies
2. **Create:** Fireblocks wallet provider hook
3. **Implement:** Fireblocks connection button
4. **Integrate:** Transaction signing
5. **Test:** End-to-end flow
6. **Record:** Demo video

---

*Implementation Plan for Oragami (CommoVault) - StableHacks 2026*  
*Date: March 27, 2026*
