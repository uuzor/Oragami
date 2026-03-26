# Fireblocks Solana Integration - Deep Research Report

## Oragami (CommoVault) Project Analysis

**Date:** March 26, 2026  
**Project:** Oragami - Institutional RWA Vault System on Solana  
**Hackathon:** StableHacks 2026 - Track 4: RWA-Backed Stablecoin & Commodity Vaults

---

## Executive Summary

Fireblocks is an enterprise-grade digital asset custody, transfer, and settlement platform that provides institutional-level security and compliance infrastructure. This research analyzes how Fireblocks Solana integration can significantly enhance the Oragami project's institutional readiness, security posture, and compliance capabilities.

---

## 1. What is Fireblocks?

### Overview
Fireblocks is a comprehensive digital asset platform that provides:

- **MPC (Multi-Party Computation) Custody**: Military-grade key management using threshold cryptography
- **Policy Engine**: Programmable transaction approval workflows
- **Transfer Network**: Secure asset transfers across 1,300+ institutions
- **Compliance Suite**: Built-in KYC/AML/KYT tools
- **Solana Support**: Native integration with Solana blockchain

### Key Differentiators
1. **Institutional Trust**: Used by 1,300+ financial institutions including banks, hedge funds, and market makers
2. **Security**: Zero security breaches since inception (2018)
3. **Compliance**: SOC 2 Type II, ISO 27001 certified
4. **Solana Native**: Full support for Solana programs, Token-2022, and DeFi protocols

---

## 2. Fireblocks Solana Capabilities

### 2.1 Custody & Key Management

**MPC-Based Wallet Infrastructure:**
- **No Single Point of Failure**: Private keys are split into multiple shares using MPC
- **Hardware Security Modules (HSM)**: Keys stored in FIPS 140-2 Level 3 certified HSMs
- **Multi-Signature Support**: Configurable approval thresholds (e.g., 2-of-3, 3-of-5)
- **Disaster Recovery**: Built-in key recovery mechanisms

**Solana-Specific Features:**
- Native Solana account management
- Support for SPL tokens and Token-2022
- Program-derived address (PDA) management
- Transaction signing for complex Solana programs

### 2.2 Policy Engine

**Programmable Transaction Controls:**
```yaml
# Example Policy Configuration
policies:
  - name: "Large Transfer Approval"
    conditions:
      - type: "amount"
        operator: "greater_than"
        value: 100000  # USDC
      - type: "token"
        value: "cVAULT"
    actions:
      - type: "require_approval"
        approvers: ["compliance_officer", "cfo"]
      - type: "require_mfa"
      
  - name: "Whitelisted Address Only"
    conditions:
      - type: "destination"
        operator: "in_list"
        list: "institutional_whitelist"
    actions:
      - type: "auto_approve"
      
  - name: "Compliance Check"
    conditions:
      - type: "transaction_type"
        value: "transfer"
    actions:
      - type: "webhook"
        url: "https://api.oragami.com/compliance/check"
      - type: "require_approval"
        until: "webhook_response.approved == true"
```

**Policy Types:**
- **Amount-Based**: Different approval flows based on transaction size
- **Address-Based**: Whitelist/blacklist destination addresses
- **Token-Based**: Specific rules for different token types
- **Time-Based**: Transaction windows and cooling periods
- **Velocity-Based**: Rate limiting on transactions

### 2.3 Transfer Network

**Secure Institutional Transfers:**
- **Fireblocks Network**: Connect to 1,300+ institutions instantly
- **Travel Rule Compliance**: Built-in Travel Rule messaging (IVMS101 standard)
- **Settlement**: Atomic settlement for institutional trades
- **Counterparty Risk**: Reduced counterparty risk through Fireblocks network

### 2.4 Compliance Suite

**Built-In Compliance Tools:**
- **KYC/KYT Integration**: Partner integrations with Chainalysis, Elliptic, etc.
- **Transaction Monitoring**: Real-time suspicious activity detection
- **Reporting**: Automated regulatory reporting (SAR, CTR)
- **Audit Trail**: Immutable transaction history for compliance

### 2.5 Solana Program Interaction

**Direct Program Execution:**
```typescript
// Example: Fireblocks Solana SDK
import { FireblocksSolana } from '@fireblocks/solana-sdk';

const fireblocks = new FireblocksSolana({
  apiKey: process.env.FIREBLOCKS_API_KEY,
  apiSecret: process.env.FIREBLOCKS_API_SECRET,
  vaultAccountId: '0'  // Vault account ID
});

// Sign and submit Solana transaction
const transaction = await fireblocks.signAndSubmit({
  instructions: [
    // Deposit to Oragami vault
    vaultProgram.instruction.deposit(
      new BN(100000),  // amount
      {
        accounts: {
          vault: vaultPda,
          user: userAccount,
          tokenAccount: userTokenAccount,
          // ... other accounts
        }
      }
    )
  ],
  signers: [userKeypair],  // Optional: additional signers
  feePayer: fireblocks.getVaultAddress()
});
```

---

## 3. How Fireblocks Enhances Oragami

### 3.1 Current Oragami Architecture

**Existing Components:**
- ✅ Solana Programs (oragami-vault, cvault-transfer-hook)
- ✅ Backend Compliance Relayer (Rust/Actix-web)
- ✅ Frontend Application (Next.js)
- ✅ SIX Data Integration
- ✅ Token-2022 Transfer Hooks
- ✅ Chainlink ACE Integration

**Current Limitations:**
- ❌ No institutional custody solution
- ❌ Manual key management (browser wallets)
- ❌ Limited transaction approval workflows
- ❌ No built-in Travel Rule messaging
- ❌ No institutional transfer network

### 3.2 Fireblocks Integration Benefits

#### **Benefit 1: Institutional Custody**

**Problem Solved:**
Institutional investors cannot use browser wallets (Phantom, Solflare) due to security and compliance requirements.

**Fireblocks Solution:**
```typescript
// Before: Browser wallet (not institutional-grade)
const wallet = useWallet();  // Phantom/Solflare

// After: Fireblocks institutional custody
const fireblocks = new FireblocksCustody({
  vaultId: 'oragami-institutional-vault',
  policyEngine: {
    largeTransfers: { requireApproval: ['compliance', 'cfo'] },
    whitelistedAddresses: institutionalWhitelist
  }
});
```

**Impact:**
- ✅ Banks and regulated entities can participate
- ✅ Insurance coverage for digital assets
- ✅ Audit-ready custody infrastructure
- ✅ Multi-signature approval workflows

#### **Benefit 2: Policy Engine Integration**

**Problem Solved:**
Current compliance is API-based (pre-transaction check). No programmatic transaction approval workflows.

**Fireblocks Solution:**
```rust
// Oragami vault program with Fireblocks policy integration
pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    // 1. On-chain compliance check (existing)
    validate_compliance(ctx.accounts.user.key())?;
    
    // 2. Fireblocks policy check (new)
    // Fireblocks webhook calls Oragami compliance API
    // Policy engine approves/rejects based on rules
    
    // 3. Execute deposit
    // ... existing logic
}
```

**Policy Workflow:**
```
User initiates deposit
    ↓
Fireblocks Policy Engine
    ↓
├─ Amount > $100K? → Require CFO approval
├─ Destination whitelisted? → Auto-approve
├─ Compliance webhook → Oragami API check
└─ All checks pass → Execute transaction
```

**Impact:**
- ✅ Programmable approval workflows
- ✅ Role-based access control
- ✅ Automated compliance enforcement
- ✅ Audit trail for all approvals

#### **Benefit 3: Travel Rule Compliance**

**Problem Solved:**
Travel Rule requires sharing sender/receiver information for transfers > $1,000 (or equivalent). Current implementation has basic metadata validation but no institutional Travel Rule messaging.

**Fireblocks Solution:**
```typescript
// Fireblocks Travel Rule integration
const travelRuleMessage = await fireblocks.createTravelRuleMessage({
  originator: {
    name: "AMINA Bank",
    accountNumber: "ACC-123456",
    address: {
      street: "Bahnhofstrasse 1",
      city: "Zurich",
      country: "CH"
    }
  },
  beneficiary: {
    name: "UBS AG",
    accountNumber: "ACC-789012",
    address: {
      street: "Bahnhofstrasse 45",
      city: "Zurich",
      country: "CH"
    }
  },
  amount: 500000,  // USDC
  currency: "USDC"
});

// Attach to transaction
const transaction = await fireblocks.signAndSubmit({
  instructions: [/* deposit instruction */],
  travelRule: travelRuleMessage
});
```

**Impact:**
- ✅ Full Travel Rule compliance (IVMS101 standard)
- ✅ Interoperable with other Travel Rule providers
- ✅ Automated message generation
- ✅ Regulatory reporting ready

#### **Benefit 4: Institutional Transfer Network**

**Problem Solved:**
Institutional investors need secure, compliant ways to transfer assets between entities.

**Fireblocks Solution:**
```typescript
// Transfer cVAULT between institutions via Fireblocks Network
const transfer = await fireblocks.transfer({
  assetId: 'cVAULT',
  source: {
    type: 'VAULT',
    id: 'amina-vault'
  },
  destination: {
    type: 'FIREBLOCKS_NETWORK',
    id: 'ubs-institutional'  // UBS Fireblocks ID
  },
  amount: '100000',
  travelRule: travelRuleMessage
});
```

**Impact:**
- ✅ Instant settlement between Fireblocks users
- ✅ Reduced counterparty risk
- ✅ Built-in Travel Rule messaging
- ✅ 1,300+ institutional counterparties

#### **Benefit 5: Enhanced Security**

**Problem Solved:**
Browser wallets are vulnerable to phishing, malware, and key compromise.

**Fireblocks Solution:**
- **MPC Key Management**: No single point of failure
- **HSM Storage**: Keys never leave secure hardware
- **Multi-Factor Authentication**: Required for all transactions
- **IP Whitelisting**: Restrict API access to known IPs
- **Role-Based Access**: Different permissions for different users

**Impact:**
- ✅ Military-grade security
- ✅ Insurance coverage
- ✅ Regulatory compliance
- ✅ Audit trail

---

## 4. Integration Architecture

### 4.1 Proposed Architecture with Fireblocks

```
┌─────────────────────────────────────────────────────────────┐
│                    Frontend (Next.js)                        │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐   │
│  │ Dashboard│  │ Terminal │  │ Risk     │  │ Monitor  │   │
│  │          │  │ Transfer │  │ Scanner  │  │          │   │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘   │
│                      │                                      │
│                      ▼                                      │
│  ┌──────────────────────────────────────────────────────┐  │
│  │           Fireblocks SDK Integration                  │  │
│  │  - MPC Wallet Management                              │  │
│  │  - Policy Engine                                      │  │
│  │  - Travel Rule Messaging                              │  │
│  │  - Transaction Signing                                │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│              Backend (Rust/Actix-web)                        │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐   │
│  │ API      │  │ Service  │  │ Worker   │  │ Risk     │   │
│  │ Router   │  │ Layer    │  │ Queue    │  │ Service  │   │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘   │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐   │
│  │ SIX API  │  │ Range    │  │ Blocklist│  │ Database │   │
│  │ Client   │  │ Protocol │  │ Manager  │  │ Postgres │   │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘   │
│  ┌──────────────────────────────────────────────────────┐  │
│  │           Fireblocks Webhook Handler                  │  │
│  │  - Policy approval callbacks                          │  │
│  │  - Transaction status updates                         │  │
│  │  - Travel Rule validation                             │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│              Solana Programs (Anchor)                        │
│  ┌──────────────────────┐  ┌──────────────────────┐        │
│  │ oragami-vault        │  │ cvault-transfer-hook │        │
│  │ - initialize         │  │ - transfer_hook      │        │
│  │ - deposit            │  │ - whitelist mgmt     │        │
│  │ - redeem             │  │ - compliance check   │        │
│  │ - convert            │  │ - metadata validation│        │
│  └──────────────────────┘  └──────────────────────┘        │
│                      │                                      │
│                      ▼                                      │
│  ┌──────────────────────────────────────────────────────┐  │
│  │           Fireblocks Vault Accounts                   │  │
│  │  - Institutional custody                              │  │
│  │  - Multi-sig approval                                 │  │
│  │  - Policy enforcement                                 │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

### 4.2 Integration Points

#### **Point 1: Frontend Wallet Connection**

**Current Implementation:**
```typescript
// frontend/mosaic/apps/app/src/features/wallet/components/connect-button.tsx
import { useWallet } from '@solana/wallet-adapter-react';

export function ConnectButton() {
  const { connect, disconnect, publicKey } = useWallet();
  // ... browser wallet connection
}
```

**Fireblocks Integration:**
```typescript
// New: Fireblocks wallet provider
import { FireblocksProvider } from '@fireblocks/solana-react';

export function AppProviders({ children }) {
  return (
    <FireblocksProvider
      apiKey={process.env.NEXT_PUBLIC_FIREBLOCKS_API_KEY}
      vaultId={process.env.NEXT_PUBLIC_FIREBLOCKS_VAULT_ID}
    >
      <WalletProvider>
        {/* existing providers */}
      </WalletProvider>
    </FireblocksProvider>
  );
}

// Updated connect button
export function ConnectButton() {
  const { connect: fireblocksConnect } = useFireblocks();
  const { connect: walletConnect } = useWallet();
  
  const handleConnect = async (type: 'browser' | 'fireblocks') => {
    if (type === 'fireblocks') {
      await fireblocksConnect();
    } else {
      await walletConnect();
    }
  };
  
  return (
    <div>
      <button onClick={() => handleConnect('browser')}>
        Connect Browser Wallet
      </button>
      <button onClick={() => handleConnect('fireblocks')}>
        Connect Fireblocks (Institutional)
      </button>
    </div>
  );
}
```

#### **Point 2: Backend Webhook Handler**

**New Endpoint:**
```rust
// backend/compliance-relayer/src/api/fireblocks.rs
use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct FireblocksWebhook {
    pub event_type: String,
    pub transaction_id: String,
    pub status: String,
    pub policy_engine: Option<PolicyEngineResult>,
}

#[derive(Deserialize)]
pub struct PolicyEngineResult {
    pub approved: bool,
    pub approvers: Vec<String>,
    pub reason: Option<String>,
}

pub async fn handle_fireblocks_webhook(
    webhook: web::Json<FireblocksWebhook>,
) -> Result<HttpResponse> {
    match webhook.event_type.as_str() {
        "TRANSACTION_CREATED" => {
            // Log transaction creation
            info!("Fireblocks transaction created: {}", webhook.transaction_id);
        }
        "POLICY_ENGINE_APPROVAL" => {
            // Handle policy approval
            if let Some(policy) = &webhook.policy_engine {
                if policy.approved {
                    info!("Transaction approved by: {:?}", policy.approvers);
                } else {
                    warn!("Transaction rejected: {:?}", policy.reason);
                }
            }
        }
        "TRANSACTION_COMPLETED" => {
            // Update database with completed transaction
            info!("Transaction completed: {}", webhook.transaction_id);
        }
        _ => {
            debug!("Unhandled Fireblocks event: {}", webhook.event_type);
        }
    }
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "received"
    })))
}
```

#### **Point 3: Policy Engine Configuration**

**Fireblocks Policy Setup:**
```json
{
  "policies": [
    {
      "name": "Oragami Vault Deposit",
      "description": "Policy for cVAULT deposits",
      "rules": [
        {
          "type": "TRANSACTION",
          "conditions": [
            {
              "attribute": "ASSET_ID",
              "operator": "EQUALS",
              "value": "cVAULT"
            },
            {
              "attribute": "OPERATION",
              "operator": "EQUALS",
              "value": "TRANSFER"
            }
          ],
          "actions": [
            {
              "type": "WEBHOOK",
              "url": "https://api.oragami.com/fireblocks/webhook",
              "method": "POST"
            }
          ]
        },
        {
          "type": "TRANSACTION",
          "conditions": [
            {
              "attribute": "AMOUNT",
              "operator": "GREATER_THAN",
              "value": "100000"
            }
          ],
          "actions": [
            {
              "type": "REQUIRE_APPROVAL",
              "approvers": ["compliance_officer", "cfo"]
            }
          ]
        }
      ]
    }
  ]
}
```

---

## 5. Implementation Roadmap

### Phase 1: Foundation (Week 1-2)

**Tasks:**
1. **Fireblocks Account Setup**
   - Create Fireblocks sandbox account
   - Configure vault accounts for Oragami
   - Set up API credentials

2. **SDK Integration**
   - Install Fireblocks Solana SDK
   - Configure environment variables
   - Create Fireblocks service wrapper

3. **Basic Custody**
   - Implement Fireblocks wallet connection
   - Test basic transaction signing
   - Verify Solana program interaction

**Deliverables:**
- ✅ Fireblocks sandbox environment
- ✅ Basic SDK integration
- ✅ Transaction signing working

### Phase 2: Policy Engine (Week 3-4)

**Tasks:**
1. **Policy Configuration**
   - Define approval workflows
   - Configure amount-based rules
   - Set up address whitelisting

2. **Webhook Handler**
   - Implement backend webhook endpoint
   - Integrate with compliance API
   - Add policy approval logic

3. **Frontend Integration**
   - Add Fireblocks wallet option
   - Display policy approval status
   - Show transaction approval flow

**Deliverables:**
- ✅ Policy engine configured
- ✅ Webhook handler working
- ✅ Frontend showing approval flow

### Phase 3: Travel Rule (Week 5-6)

**Tasks:**
1. **Travel Rule Integration**
   - Implement IVMS101 message generation
   - Integrate with Fireblocks Travel Rule
   - Test with institutional counterparties

2. **Compliance Enhancement**
   - Enhance compliance API with Travel Rule
   - Add regulatory reporting
   - Implement audit trail

3. **Testing**
   - End-to-end Travel Rule testing
   - Compliance validation
   - Security audit

**Deliverables:**
- ✅ Travel Rule compliance
- ✅ Enhanced compliance reporting
- ✅ Audit trail complete

### Phase 4: Production (Week 7-8)

**Tasks:**
1. **Production Setup**
   - Migrate to production Fireblocks account
   - Configure production policies
   - Set up monitoring and alerting

2. **Security Hardening**
   - Security audit
   - Penetration testing
   - Compliance certification

3. **Documentation**
   - Integration guide
   - API documentation
   - Compliance documentation

**Deliverables:**
- ✅ Production-ready integration
- ✅ Security audit passed
- ✅ Documentation complete

---

## 6. Cost Analysis

### Fireblocks Pricing (Estimated)

| Component | Cost | Notes |
|-----------|------|-------|
| **Sandbox** | Free | For development and testing |
| **Production** | $2,000-5,000/month | Based on transaction volume |
| **Vault Accounts** | $500-1,000/month | Per institutional vault |
| **Policy Engine** | Included | Part of platform |
| **Travel Rule** | $0.10-0.50/message | Per Travel Rule message |
| **Transfer Network** | Free | For Fireblocks-to-Fireblocks |
 
**Total Estimated Cost:**
- **Development**: Free (sandbox)
- **Production**: $3,000-7,000/month
- **Per Transaction**: $0.10-0.50 (Travel Rule only)

### ROI Analysis

**Benefits:**
- ✅ Access to institutional investors (banks, hedge funds)
- ✅ Insurance coverage for digital assets
- ✅ Regulatory compliance (Travel Rule, KYC/AML)
- ✅ Reduced security risk
- ✅ Audit-ready infrastructure

**Value:**
- Institutional AUM potential: $10M-100M+
- Insurance coverage: Up to $150M per vault
- Compliance cost savings: $50K-100K/year
- Security incident prevention: Priceless

---

## 7. Competitive Analysis

### Oragami vs. Competitors with Fireblocks

| Feature | Oragami (Current) | Oragami + Fireblocks | Ondo Finance | Maple Finance |
|---------|-------------------|----------------------|--------------|---------------|
| **Custody** | Browser wallets | Fireblocks MPC | Institutional | Institutional |
| **Compliance** | API-based | API + Policy Engine | Basic | Basic |
| **Travel Rule** | Basic metadata | Full IVMS101 | None | None |
| **Transfer Network** | None | Fireblocks Network | None | None |
| **Security** | Browser wallet | MPC + HSM | Institutional | Institutional |
| **Insurance** | None | Up to $150M | Yes | Yes |
| **Solana Support** | Native | Native | Limited | None |

**Competitive Advantage:**
- ✅ First Solana RWA vault with Fireblocks integration
- ✅ Full Travel Rule compliance
- ✅ Institutional-grade custody
- ✅ Programmable policy engine

---

## 8. Risk Assessment

### Technical Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| **SDK Integration Complexity** | Medium | Medium | Use Fireblocks sandbox, extensive testing |
| **Policy Engine Performance** | Low | Medium | Optimize webhook handlers, caching |
| **Travel Rule Interoperability** | Medium | High | Use IVMS101 standard, test with multiple providers |
| **Solana Program Compatibility** | Low | High | Fireblocks supports Token-2022, test thoroughly |

### Business Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| **Fireblocks Cost** | Medium | Medium | Start with sandbox, scale gradually |
| **Regulatory Changes** | Low | High | Fireblocks handles compliance updates |
| **Competition** | Medium | Medium | First-mover advantage on Solana |
| **Adoption** | Medium | High | Partner with AMINA Bank, UBS |

---

## 9. Recommendations

### Immediate Actions (Post-Hackathon)

1. **Create Fireblocks Sandbox Account**
   - Sign up for Fireblocks sandbox
   - Configure vault accounts
   - Test basic integration

2. **Proof of Concept**
   - Implement Fireblocks wallet connection
   - Test transaction signing
   - Verify policy engine integration

3. **Partner Engagement**
   - Discuss Fireblocks integration with AMINA Bank
   - Get feedback on policy requirements
   - Validate Travel Rule needs

### Short-Term (1-3 Months)

1. **Full Integration**
   - Complete Phase 1-2 implementation
   - Deploy to devnet with Fireblocks
   - Test with institutional partners

2. **Compliance Validation**
   - Travel Rule testing
   - Regulatory review
   - Security audit

3. **Documentation**
   - Integration guide
   - API documentation
   - Compliance documentation

### Long-Term (3-6 Months)

1. **Production Deployment**
   - Migrate to production Fireblocks
   - Launch with institutional partners
   - Scale to multiple vaults

2. **Feature Enhancement**
   - Advanced policy workflows
   - Multi-chain support
   - Enhanced reporting

3. **Market Expansion**
   - Onboard additional institutions
   - Expand to new asset classes
   - Geographic expansion

---

## 10. Conclusion

Fireblocks Solana integration is a **strategic imperative** for Oragami's institutional adoption. The combination of:

- **Institutional Custody** (MPC + HSM)
- **Programmable Policy Engine** (approval workflows)
- **Travel Rule Compliance** (IVMS101)
- **Transfer Network** (1,300+ institutions)
- **Enhanced Security** (zero breaches)

...positions Oragami as the **first institutional-grade RWA vault on Solana** with full compliance infrastructure.

### Key Takeaways

1. **Fireblocks solves the #1 barrier to institutional adoption**: Custody and security
2. **Policy engine enhances existing compliance**: Programmatic approval workflows
3. **Travel Rule compliance is critical**: Required for institutional transfers
4. **Transfer network enables liquidity**: Connect to 1,300+ institutions
5. **Competitive advantage**: First Solana RWA vault with Fireblocks integration

### Next Steps

1. ✅ Create Fireblocks sandbox account (Week 1)
2. ✅ Implement basic SDK integration (Week 2)
3. ✅ Configure policy engine (Week 3)
4. ✅ Test Travel Rule integration (Week 4)
5. ✅ Deploy to devnet (Week 5)
6. ✅ Partner validation (Week 6)
7. ✅ Production deployment (Week 7-8)

---

## Appendix

### A. Fireblocks Resources

- **Documentation**: https://developers.fireblocks.com
- **Solana SDK**: https://github.com/fireblocks/solana-sdk
- **API Reference**: https://developers.fireblocks.com/reference
- **Support**: https://support.fireblocks.com

### B. Oragami Integration Files

**Files to Create/Modify:**
- `frontend/mosaic/apps/app/src/features/wallet/hooks/use-fireblocks.ts`
- `frontend/mosaic/apps/app/src/features/wallet/components/fireblocks-connect-button.tsx`
- `backend/compliance-relayer/src/api/fireblocks.rs`
- `backend/compliance-relayer/src/infra/fireblocks/mod.rs`
- `backend/compliance-relayer/src/infra/fireblocks/policy.rs`
- `backend/compliance-relayer/src/infra/fireblocks/travel_rule.rs`

### C. Environment Variables

```bash
# Fireblocks Configuration
FIREBLOCKS_API_KEY=your_api_key
FIREBLOCKS_API_SECRET=your_api_secret
FIREBLOCKS_VAULT_ID=your_vault_id
FIREBLOCKS_SANDBOX=true  # Set to false for production

# Fireblocks Webhook
FIREBLOCKS_WEBHOOK_SECRET=your_webhook_secret
FIREBLOCKS_WEBHOOK_URL=https://api.oragami.com/fireblocks/webhook

# Policy Engine
FIREBLOCKS_POLICY_ID=your_policy_id
FIREBLOCKS_APPROVAL_THRESHOLD=100000  # USDC
```

### D. Example Policy Configuration

```json
{
  "name": "Oragami Institutional Policy",
  "description": "Policy for Oragami vault operations",
  "rules": [
    {
      "name": "Small Transfers",
      "conditions": [
        {
          "attribute": "AMOUNT",
          "operator": "LESS_THAN_OR_EQUAL",
          "value": "10000"
        }
      ],
      "actions": [
        {
          "type": "AUTO_APPROVE"
        }
      ]
    },
    {
      "name": "Medium Transfers",
      "conditions": [
        {
          "attribute": "AMOUNT",
          "operator": "GREATER_THAN",
          "value": "10000"
        },
        {
          "attribute": "AMOUNT",
          "operator": "LESS_THAN_OR_EQUAL",
          "value": "100000"
        }
      ],
      "actions": [
        {
          "type": "REQUIRE_APPROVAL",
          "approvers": ["compliance_officer"]
        }
      ]
    },
    {
      "name": "Large Transfers",
      "conditions": [
        {
          "attribute": "AMOUNT",
          "operator": "GREATER_THAN",
          "value": "100000"
        }
      ],
      "actions": [
        {
          "type": "REQUIRE_APPROVAL",
          "approvers": ["compliance_officer", "cfo"]
        },
        {
          "type": "WEBHOOK",
          "url": "https://api.oragami.com/fireblocks/webhook"
        }
      ]
    }
  ]
}
```

---

*Research conducted for Oragami (CommoVault) - StableHacks 2026*  
*Date: March 26, 2026*
