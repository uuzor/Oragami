# Project Cleanup Recommendations

## Oragami (CommoVault) - Space Optimization Analysis

**Date:** March 27, 2026  
**Purpose:** Identify unnecessary files to delete and free up space

---

## Executive Summary

After analyzing the Oragami project structure, I've identified several categories of files that can be deleted or consolidated to free up significant disk space. The total potential savings is approximately **1.3 MB** of text files, plus additional space from binary files.

---

## 1. Duplicate/Redundant Research Documents

### Files to Consolidate

| File | Size | Issue |
|------|------|-------|
| `FIREBLOCKS_SOLANA_RESEARCH.md` | 27,683 chars | Overlaps with comprehensive research |
| `FIREBLOCKS_COMPREHENSIVE_RESEARCH.md` | 32,093 chars | Contains similar content |

**Recommendation:**  
Consolidate both documents into a single comprehensive Fireblocks research document. The combined document should include:
- Platform overview
- Solana capabilities
- Policy engine details
- Travel Rule compliance
- Integration architecture
- Implementation roadmap

**Space Saved:** ~27,683 chars (27 KB)

---

## 2. Large Lock Files

Lock files are auto-generated and can be regenerated. They take up significant space.

### Files to Delete

| File | Size | Location |
|------|------|----------|
| `frontend/mosaic/pnpm-lock.yaml` | 452,316 chars | Frontend Mosaic |
| `frontend/relayer-frontend/package-lock.json` | 279,691 chars | Relayer Frontend |
| `frontend/relayer-frontend/pnpm-lock.yaml` | 175,422 chars | Relayer Frontend |
| `backend/compliance-relayer/Cargo.lock` | 215,583 chars | Backend |
| `oragami-vault/Cargo.lock` | 74,943 chars | Vault Program |
| `oragami-vault/yarn.lock` | 52,187 chars | Vault Program |
| `programs/cvault-transfer-hook/programs/cvault-transfer-hook/Cargo.lock` | 67,268 chars | Transfer Hook |

**Total Space:** ~1,317,410 chars (1.3 MB)

**Recommendation:**  
Add these to `.gitignore` and delete from repository. They will be regenerated when running `npm install`, `pnpm install`, `cargo build`, or `yarn install`.

**Note:** Keep `Cargo.lock` for Rust projects if you want reproducible builds, but the npm/pnpm/yarn lock files can safely be deleted.

---

## 3. Duplicate Lock Files

### Issue
`frontend/relayer-frontend/` has both `package-lock.json` and `pnpm-lock.yaml`, which is redundant.

**Recommendation:**  
Choose one package manager (pnpm is recommended for this project) and delete the other lock file.

**Space Saved:** ~279,691 chars (280 KB)

---

## 4. Redundant Documentation Files

### Files to Review/Delete

| File | Size | Issue |
|------|------|-------|
| `backend/compliance-relayer/CONTRIBUTING.md` | 12,252 chars | May be redundant with other docs |
| `backend/compliance-relayer/README.md` | 8,868 chars | May be redundant with other docs |
| `frontend/mosaic/CLAUDE.md` | 3,535 chars | AI assistant instructions, not needed |
| `frontend/mosaic/CONTRIBUTING.md` | 2,246 chars | May be redundant |
| `frontend/mosaic/README.md` | 7,700 chars | May be redundant |
| `frontend/relayer-frontend/README.md` | 7,891 chars | May be redundant |

**Total Space:** ~42,592 chars (43 KB)

**Recommendation:**  
- Keep only essential documentation
- Consolidate similar docs into single files
- Delete AI-specific files like `CLAUDE.md`

---

## 5. Test Files (Optional for Hackathon)

### Directories to Consider Removing

| Directory | Contents | Issue |
|-----------|----------|-------|
| `backend/compliance-relayer/tests/` | 5 test files | May not be needed for hackathon demo |
| `frontend/mosaic/packages/sdk/src/__tests__/` | Large test directory | Extensive tests |
| `frontend/mosaic/packages/sdk/src/__mocks__/` | Mock files | Test infrastructure |

**Recommendation:**  
For hackathon purposes, these test files are not essential. They can be deleted to save space and will be recreated when needed for production.

**Note:** Keep if you plan to run tests during development.

---

## 6. CLI Package (Optional)

### Directory to Consider Removing

| Directory | Contents | Issue |
|-----------|----------|-------|
| `frontend/mosaic/packages/cli/` | CLI tool | May not be needed for hackathon demo |

**Recommendation:**  
The CLI tool is useful for production but not essential for the hackathon demo. Consider removing if space is critical.

---

## 7. Six Data Access Files (Sensitive)

### Files to Review

| File | Size | Issue |
|------|------|-------|
| `six-data-access/CH56655-api2026hack22/certificate.p12` | 7,212 chars | Sensitive credential |
| `six-data-access/CH56655-api2026hack22/CSR.pem` | 1,322 chars | Sensitive credential |
| `six-data-access/CH56655-api2026hack22/password.txt` | 16 chars | Sensitive credential |
| `six-data-access/CH56655-api2026hack22/private-key.pem` | 2,524 chars | Sensitive credential |
| `six-data-access/CH56655-api2026hack22/signed-certificate.pem` | 7,370 chars | Sensitive credential |
| `six-data-access/Cross Currency and Precious Metals Identifiers.xlsx` | 51,150 chars | Data file |

**Total Space:** ~69,594 chars (70 KB)

**Recommendation:**  
These files contain sensitive credentials and should NOT be in the repository. They should be:
1. Added to `.gitignore`
2. Deleted from the repository
3. Stored securely outside the repository (e.g., in a secrets manager)

**Security Risk:** HIGH - These files contain private keys and passwords.

---

## 8. Configuration Files (Review)

### Files to Review

| File | Size | Issue |
|------|------|-------|
| `frontend/mosaic/.editorconfig` | 246 chars | May not be needed |
| `frontend/mosaic/.npmrc` | 54 chars | May not be needed |
| `frontend/mosaic/.nvmrc` | 8 chars | May not be needed |
| `frontend/mosaic/.prettierignore` | 159 chars | May not be needed |
| `frontend/mosaic/.prettierrc.cjs` | 72 chars | May not be needed |
| `backend/compliance-relayer/.cargo/` | Directory | Build configuration |

**Recommendation:**  
These are small files and can be kept for development consistency. Low priority for deletion.

---

## Summary of Deletion Recommendations

### High Priority (Delete Immediately)

| Category | Files | Space Saved |
|----------|-------|-------------|
| **Duplicate Research Docs** | 1 file | ~27 KB |
| **Lock Files** | 7 files | ~1.3 MB |
| **Sensitive Credentials** | 6 files | ~70 KB |
| **Duplicate Lock Files** | 1 file | ~280 KB |

**Total High Priority:** ~1.7 MB

### Medium Priority (Review & Delete)

| Category | Files | Space Saved |
|----------|-------|-------------|
| **Redundant Documentation** | 6 files | ~43 KB |
| **Test Files** | 3 directories | Variable |
| **CLI Package** | 1 directory | Variable |

**Total Medium Priority:** ~50+ KB

### Low Priority (Optional)

| Category | Files | Space Saved |
|----------|-------|-------------|
| **Config Files** | 6 files | ~0.6 KB |

---

## Recommended Action Plan

### Step 1: Immediate Security Fix (HIGH PRIORITY)
```bash
# Delete sensitive credentials from repository
rm -rf six-data-access/

# Add to .gitignore
echo "six-data-access/" >> .gitignore
```

### Step 2: Consolidate Research Documents
```bash
# Keep FIREBLOCKS_COMPREHENSIVE_RESEARCH.md as the main document
# Delete the redundant one
rm FIREBLOCKS_SOLANA_RESEARCH.md
```

### Step 3: Remove Lock Files
```bash
# Remove large lock files
rm frontend/mosaic/pnpm-lock.yaml
rm frontend/relayer-frontend/package-lock.json
rm frontend/relayer-frontend/pnpm-lock.yaml
rm oragami-vault/yarn.lock

# Add to .gitignore
echo "pnpm-lock.yaml" >> .gitignore
echo "package-lock.json" >> .gitignore
echo "yarn.lock" >> .gitignore
```

### Step 4: Remove Redundant Documentation
```bash
# Remove AI-specific files
rm frontend/mosaic/CLAUDE.md

# Review and remove redundant docs
rm backend/compliance-relayer/CONTRIBUTING.md
rm frontend/mosaic/CONTRIBUTING.md
```

### Step 5: Optional - Remove Test Files
```bash
# Only if space is critical and tests aren't needed for hackathon
rm -rf backend/compliance-relayer/tests/
rm -rf frontend/mosaic/packages/sdk/src/__tests__/
rm -rf frontend/mosaic/packages/sdk/src/__mocks__/
```

---

## Updated .gitignore Recommendations

Add these entries to your `.gitignore` file:

```gitignore
# Lock files (regenerated on install)
pnpm-lock.yaml
package-lock.json
yarn.lock
Cargo.lock

# Sensitive credentials
six-data-access/
*.p12
*.pem
password.txt

# AI assistant files
CLAUDE.md

# Test files (optional)
**/tests/
**/__tests__/
**/__mocks__/
```

---

## Space Savings Summary

| Category | Space Saved |
|----------|-------------|
| Duplicate Research Docs | 27 KB |
| Lock Files | 1.3 MB |
| Sensitive Credentials | 70 KB |
| Redundant Documentation | 43 KB |
| **Total** | **~1.44 MB** |

---

## Important Notes

1. **Lock Files:** While deleting lock files saves space, they ensure reproducible builds. Consider keeping `Cargo.lock` for Rust projects if you need deterministic builds.

2. **Test Files:** Only delete test files if you're certain they won't be needed during the hackathon. Tests are valuable for development.

3. **Sensitive Credentials:** The `six-data-access/` directory contains private keys and passwords. This is a **CRITICAL SECURITY ISSUE** and should be addressed immediately.

4. **Documentation:** Keep at least one comprehensive README per major component. Don't delete all documentation.

5. **CLI Package:** The CLI tool is useful for production but not essential for the hackathon demo.

---

## Post-Cleanup Verification

After cleanup, verify the project still works:

```bash
# Backend
cd backend/compliance-relayer
cargo build

# Frontend
cd frontend/mosaic
pnpm install
pnpm build

# Programs
cd programs/cvault-transfer-hook
anchor build
```

---

*Generated for Oragami (CommoVault) - StableHacks 2026*  
*Date: March 27, 2026*
