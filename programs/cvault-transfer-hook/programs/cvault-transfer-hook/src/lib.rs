use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

/// cVAULT-TRADE Transfer Hook Program
/// 
/// This program enforces compliance on every transfer of cVAULT-TRADE tokens.
/// It validates:
/// 1. KYC/AML compliance via Chainlink ACE (or mock oracle)
/// 2. Whitelisting status
/// 3. Travel Rule metadata

declare_id!("Cvau1tT3xGK9XQDqVjG1qGjvMaVQDqVjG1qGjvMaVQD");

pub const WHITELIST_SEED: &[u8] = b"whitelist";
pub const COMPLIANCE_SEED: &[u8] = b"compliance";

/// Whitelist entry - tracks compliant wallet addresses
#[account]
pub struct WhitelistEntry {
    pub wallet: Pubkey,
    pub kyc_compliant: bool,
    pub aml_clear: bool,
    pub travel_rule_compliant: bool,
    pub added_at: i64,
    pub expiry: i64,
}

/// Compliance config - stores compliance oracle and settings
#[account]
pub struct ComplianceConfig {
    pub authority: Pubkey,
    pub compliance_oracle: Pubkey,
    pub min_kyc_level: u8,
    pub allow_transfers: bool,
}

/// Initialize compliance configuration
#[derive(Accounts)]
pub struct InitializeCompliance<'info> {
    #[account(init, payer = payer, space = 8 + 64, seeds = [COMPLIANCE_SEED], bump)]
    pub config: Account<'info, ComplianceConfig>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// Add a wallet to the whitelist
#[derive(Accounts)]
pub struct AddToWhitelist<'info> {
    #[account(mut, seeds = [COMPLIANCE_SEED], bump)]
    pub config: Account<'info, ComplianceConfig>,
    #[account(init, payer = payer, space = 8 + 80, seeds = [WHITELIST_SEED, wallet.key().as_ref()], bump)]
    pub entry: Account<'info, WhitelistEntry>,
    pub wallet: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct AddWhitelistParams {
    pub kyc_compliant: bool,
    pub aml_clear: bool,
    pub travel_rule_compliant: bool,
    pub expiry_days: i64,
}

/// Remove a wallet from the whitelist
#[derive(Accounts)]
pub struct RemoveFromWhitelist<'info> {
    #[account(mut, seeds = [COMPLIANCE_SEED], bump)]
    pub config: Account<'info, ComplianceConfig>,
    #[account(mut, seeds = [WHITELIST_SEED, wallet.key().as_ref()], bump)]
    pub entry: Account<'info, WhitelistEntry>,
    pub wallet: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
}

/// Transfer hook - called on every token transfer
/// This is the entry point for Token-2022 transfer hook extension
#[derive(Accounts)]
pub struct TransferHook<'info> {
    /// The token mint (cVAULT-TRADE)
    pub mint: UncheckedAccount<'info>,
    /// Source token account
    #[account(token::mint = mint, token::authority = authority)]
    pub source: Account<'info, TokenAccount>,
    /// Destination token account
    #[account(token::mint = mint, token::authority = authority)]
    pub destination: Account<'info, TokenAccount>,
    /// Authority that signed the transfer (could be source owner or delegate)
    pub authority: Signer<'info>,
    /// Compliance configuration account
    #[account(seeds = [COMPLIANCE_SEED], bump)]
    pub config: Account<'info, ComplianceConfig>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct TransferHookParams {
    pub amount: u64,
}

#[error_code]
pub enum TransferHookError {
    #[msg("Wallet not whitelisted")]
    NotWhitelisted,
    #[msg("KYC requirement not completed")]
    KycNotCompleted,
    #[msg("AML check failed")]
    AmlCheckFailed,
    #[msg("Travel Rule not satisfied")]
    TravelRuleNotSatisfied,
    #[msg("Whitelist entry has expired")]
    EntryExpired,
    #[msg("Transfers are currently disabled")]
    TransferDisabled,
}

/// Initialize compliance config
pub fn initialize_compliance(ctx: Context<InitializeCompliance>, params: InitializeComplianceParams) -> Result<()> {
    ctx.accounts.config.authority = params.authority;
    ctx.accounts.config.compliance_oracle = params.compliance_oracle;
    ctx.accounts.config.min_kyc_level = 1;
    ctx.accounts.config.allow_transfers = true;
    msg!("Compliance config initialized");
    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeComplianceParams {
    pub authority: Pubkey,
    pub compliance_oracle: Pubkey,
}

/// Add wallet to whitelist
pub fn add_to_whitelist(ctx: Context<AddToWhitelist>, params: AddWhitelistParams) -> Result<()> {
    let clock = Clock::get()?;
    ctx.accounts.entry.wallet = ctx.accounts.wallet.key();
    ctx.accounts.entry.kyc_compliant = params.kyc_compliant;
    ctx.accounts.entry.aml_clear = params.aml_clear;
    ctx.accounts.entry.travel_rule_compliant = params.travel_rule_compliant;
    ctx.accounts.entry.added_at = clock.unix_timestamp;
    ctx.accounts.entry.expiry = clock.unix_timestamp + (params.expiry_days * 86400);
    msg!("Added {} to whitelist", ctx.accounts.entry.wallet);
    Ok(())
}

/// Remove wallet from whitelist
pub fn remove_from_whitelist(ctx: Context<RemoveFromWhitelist>) -> Result<()> {
    ctx.accounts.entry.close(ctx.accounts.authority.to_account_info())?;
    msg!("Removed {} from whitelist", ctx.accounts.entry.wallet);
    Ok(())
}

/// Execute transfer hook - validates compliance on every transfer
pub fn execute_transfer_hook(ctx: Context<TransferHook>, _params: TransferHookParams) -> Result<()> {
    require!(ctx.accounts.config.allow_transfers, TransferHookError::TransferDisabled);
    
    let source_address = ctx.accounts.source.owner;
    let dest_address = ctx.accounts.destination.owner;
    
    // Get current time for expiry check
    let clock = Clock::get()?;
    
    // Skip compliance check for minting (source is mint) or burning (destination is mint/zero)
    let is_mint = source_address == ctx.accounts.mint.key();
    let is_burn = dest_address == ctx.accounts.mint.key() || dest_address == Pubkey::default();
    
    // Get the program ID for PDA derivation
    let program_id = ctx.accounts.config.to_account_info().owner;
    
    // Derive whitelist PDA for destination
    let (dest_whitelist_key, _) = Pubkey::find_program_address(
        &[WHITELIST_SEED, dest_address.as_ref()],
        program_id,
    );
    
    // Validate destination compliance (always required except for burn)
    if !is_burn {
        if let Ok(entry) = Account::<WhitelistEntry>::try_from(&dest_whitelist_key.to_account_info()) {
            require!(entry.kyc_compliant, TransferHookError::KycNotCompleted);
            require!(entry.aml_clear, TransferHookError::AmlCheckFailed);
            require!(entry.travel_rule_compliant, TransferHookError::TravelRuleNotSatisfied);
            require!(entry.expiry > clock.unix_timestamp, TransferHookError::EntryExpired);
            msg!("Destination {} is compliant", dest_address);
        } else {
            // No whitelist entry found - reject transfer to non-whitelisted destination
            return err!(TransferHookError::NotWhitelisted);
        }
    }
    
    // For source, we allow transfers if whitelisted OR if it's a mint operation
    if !is_mint {
        let (source_whitelist_key, _) = Pubkey::find_program_address(
            &[WHITELIST_SEED, source_address.as_ref()],
            program_id,
        );
        if let Ok(entry) = Account::<WhitelistEntry>::try_from(&source_whitelist_key.to_account_info()) {
            require!(entry.kyc_compliant, TransferHookError::KycNotCompleted);
            require!(entry.aml_clear, TransferHookError::AmlCheckFailed);
            require!(entry.expiry > clock.unix_timestamp, TransferHookError::EntryExpired);
            msg!("Source {} is compliant", source_address);
        }
        // Note: We don't reject if source is not whitelisted - they are sending tokens they already have
        // The key compliance point is destination must be whitelisted to receive
    }
    
    msg!("Transfer compliance check passed: {} -> {}", source_address, dest_address);
    
    Ok(())
}

/// Update compliance settings
#[derive(Accounts)]
pub struct UpdateCompliance<'info> {
    #[account(mut, seeds = [COMPLIANCE_SEED], bump)]
    pub config: Account<'info, ComplianceConfig>,
    pub authority: Signer<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateComplianceParams {
    pub allow_transfers: Option<bool>,
    pub min_kyc_level: Option<u8>,
    pub compliance_oracle: Option<Pubkey>,
}

pub fn update_compliance(ctx: Context<UpdateCompliance>, params: UpdateComplianceParams) -> Result<()> {
    if let Some(enabled) = params.allow_transfers {
        ctx.accounts.config.allow_transfers = enabled;
    }
    if let Some(level) = params.min_kyc_level {
        ctx.accounts.config.min_kyc_level = level;
    }
    if let Some(oracle) = params.compliance_oracle {
        ctx.accounts.config.compliance_oracle = oracle;
    }
    msg!("Updated compliance config");
    Ok(())
}