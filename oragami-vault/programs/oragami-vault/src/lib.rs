use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount, Token};

declare_id!("G3bkZfZRsyzRjeZdmBRwkYDksw1RtDHfM11oAMTNmUth");

pub const CVAULT_MINT_SEED: &[u8] = b"cvault_mint";
pub const VAULT_STATE_SEED: &[u8] = b"vault_state";
pub const VAULT_TOKEN_SEED: &[u8] = b"vault_token_account";

#[account]
pub struct VaultState {
    pub bump: u8,
    pub cvault_mint: Pubkey,
    pub cvault_trade_mint: Pubkey,
    pub vault_token_account: Pubkey,
    pub treasury: Pubkey,
    pub authority: Pubkey,
    pub min_deposit: u64,
    pub max_deposit: u64,
    pub usx_allocation_bps: u16,
    pub paused: bool,
    pub total_deposits: u64,
    pub total_supply: u64,
    pub last_yield_claim: i64,
    pub secondary_market_enabled: bool,
}

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(init, payer = payer, space = 8 + 181, seeds = [VAULT_STATE_SEED], bump)]
    pub vault_state: Account<'info, VaultState>,
    #[account(init, payer = payer, mint::decimals = 6, mint::authority = vault_state, seeds = [CVAULT_MINT_SEED], bump)]
    pub cvault_mint: Account<'info, Mint>,
    #[account(init, payer = payer, token::mint = cvault_mint, token::authority = vault_state, seeds = [VAULT_TOKEN_SEED], bump)]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)] pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut, seeds = [VAULT_STATE_SEED], bump = vault_state.bump)]
    pub vault_state: Account<'info, VaultState>,
    #[account(mut, address = vault_state.cvault_mint)]
    pub cvault_mint: Account<'info, Mint>,
    #[account(mut, seeds = [VAULT_TOKEN_SEED], bump)]
    pub vault_token_account: Account<'info, TokenAccount>,
    #[account(mut, address = vault_state.treasury)]
    pub treasury: Account<'info, TokenAccount>,
    #[account(mut, owner = payer.key(), token::mint = deposit_token_mint, token::authority = payer)]
    pub payer_token_account: Account<'info, TokenAccount>,
    pub deposit_token_mint: Account<'info, Mint>,
    #[account(mut)] pub payer: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Redeem<'info> {
    #[account(mut, seeds = [VAULT_STATE_SEED], bump = vault_state.bump)]
    pub vault_state: Account<'info, VaultState>,
    #[account(mut, address = vault_state.cvault_mint)]
    pub cvault_mint: Account<'info, Mint>,
    #[account(mut, seeds = [VAULT_TOKEN_SEED], bump)]
    pub vault_token_account: Account<'info, TokenAccount>,
    #[account(mut, address = vault_state.treasury)]
    pub treasury: Account<'info, TokenAccount>,
    #[account(mut, owner = redeemer.key(), token::mint = cvault_mint, token::authority = redeemer)]
    pub redeemer_token_account: Account<'info, TokenAccount>,
    #[account(mut)] pub redeemer: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct SetPause<'info> {
    #[account(mut, seeds = [VAULT_STATE_SEED], bump = vault_state.bump)]
    pub vault_state: Account<'info, VaultState>,
    #[account(address = vault_state.authority)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(mut, seeds = [VAULT_STATE_SEED], bump = vault_state.bump)]
    pub vault_state: Account<'info, VaultState>,
    #[account(address = vault_state.authority)]
    pub authority: Signer<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeVaultParams {
    pub treasury: Pubkey,
    pub authority: Pubkey,
    pub min_deposit: u64,
    pub max_deposit: u64,
    pub usx_allocation_bps: u16,
    pub cvault_trade_mint: Pubkey,
    pub secondary_market_enabled: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct DepositParams { pub amount: u64, pub nonce: String }
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RedeemParams { pub cvault_amount: u64, pub nonce: String }
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SetPauseParams { pub paused: bool }
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateConfigParams {
    pub min_deposit: Option<u64>,
    pub max_deposit: Option<u64>,
    pub usx_allocation_bps: Option<u16>,
    pub secondary_market_enabled: Option<bool>,
}

pub fn initialize_vault(ctx: Context<InitializeVault>, params: InitializeVaultParams) -> Result<()> {
    let vs = &mut ctx.accounts.vault_state;
    vs.bump = ctx.bumps.vault_state;
    vs.cvault_mint = ctx.accounts.cvault_mint.key();
    vs.cvault_trade_mint = params.cvault_trade_mint;
    vs.vault_token_account = ctx.accounts.token_account.key();
    vs.treasury = params.treasury;
    vs.authority = params.authority;
    vs.min_deposit = params.min_deposit;
    vs.max_deposit = params.max_deposit;
    vs.usx_allocation_bps = params.usx_allocation_bps;
    vs.paused = false;
    vs.total_deposits = 0;
    vs.total_supply = 0;
    vs.last_yield_claim = Clock::get()?.unix_timestamp;
    vs.secondary_market_enabled = params.secondary_market_enabled;
    msg!("Vault initialized: {}", vs.key());
    Ok(())
}

pub fn deposit(ctx: Context<Deposit>, params: DepositParams) -> Result<()> {
    require!(!ctx.accounts.vault_state.paused, ErrorCode::VaultPaused);
    require!(params.amount >= ctx.accounts.vault_state.min_deposit, ErrorCode::DepositTooSmall);
    require!(params.amount <= ctx.accounts.vault_state.max_deposit, ErrorCode::DepositTooLarge);
    
    let token_program = ctx.accounts.token_program.to_account_info();
    let payer_info = ctx.accounts.payer.to_account_info();
    let vault_state_key = ctx.accounts.vault_state.key();
    
    let cpi_ctx = CpiContext::new(token_program.clone(), token::Transfer {
        from: ctx.accounts.payer_token_account.to_account_info(),
        to: ctx.accounts.vault_token_account.to_account_info(),
        authority: payer_info.clone(),
    });
    token::transfer(cpi_ctx, params.amount)?;
    
    let seeds = &[VAULT_STATE_SEED, &[ctx.accounts.vault_state.bump]];
    let signer = &[seeds.as_ref()];
    let mint_cpi = CpiContext::new_with_signer(token_program.clone(), token::MintTo {
        mint: ctx.accounts.cvault_mint.to_account_info(),
        to: ctx.accounts.payer_token_account.to_account_info(),
        authority: ctx.accounts.vault_state.to_account_info(),
    }, signer);
    token::mint_to(mint_cpi, params.amount)?;
    
    ctx.accounts.vault_state.total_deposits = ctx.accounts.vault_state.total_deposits.checked_add(params.amount).ok_or(ErrorCode::Overflow)?;
    ctx.accounts.vault_state.total_supply = ctx.accounts.vault_state.total_supply.checked_add(params.amount).ok_or(ErrorCode::Overflow)?;
    msg!("Deposited {}", params.amount);
    Ok(())
}

pub fn redeem(ctx: Context<Redeem>, params: RedeemParams) -> Result<()> {
    require!(!ctx.accounts.vault_state.paused, ErrorCode::VaultPaused);
    require!(ctx.accounts.vault_token_account.amount >= params.cvault_amount, ErrorCode::InsufficientVaultFunds);
    
    let token_program = ctx.accounts.token_program.to_account_info();
    let redeemer_info = ctx.accounts.redeemer.to_account_info();
    
    let burn_cpi = CpiContext::new(token_program.clone(), token::Burn {
        mint: ctx.accounts.cvault_mint.to_account_info(),
        from: ctx.accounts.redeemer_token_account.to_account_info(),
        authority: redeemer_info.clone(),
    });
    token::burn(burn_cpi, params.cvault_amount)?;
    
    let seeds = &[VAULT_STATE_SEED, &[ctx.accounts.vault_state.bump]];
    let signer = &[seeds.as_ref()];
    let transfer_cpi = CpiContext::new_with_signer(token_program.clone(), token::Transfer {
        from: ctx.accounts.vault_token_account.to_account_info(),
        to: ctx.accounts.treasury.to_account_info(),
        authority: ctx.accounts.vault_state.to_account_info(),
    }, signer);
    token::transfer(transfer_cpi, params.cvault_amount)?;
    
    ctx.accounts.vault_state.total_deposits = ctx.accounts.vault_state.total_deposits.checked_sub(params.cvault_amount).ok_or(ErrorCode::Overflow)?;
    ctx.accounts.vault_state.total_supply = ctx.accounts.vault_state.total_supply.checked_sub(params.cvault_amount).ok_or(ErrorCode::Overflow)?;
    msg!("Redeemed {}", params.cvault_amount);
    Ok(())
}

pub fn set_pause(ctx: Context<SetPause>, params: SetPauseParams) -> Result<()> {
    ctx.accounts.vault_state.paused = params.paused;
    msg!("Paused: {}", params.paused);
    Ok(())
}

pub fn update_config(ctx: Context<UpdateConfig>, params: UpdateConfigParams) -> Result<()> {
    let vs = &mut ctx.accounts.vault_state;
    if let Some(m) = params.min_deposit { vs.min_deposit = m; }
    if let Some(m) = params.max_deposit { vs.max_deposit = m; }
    if let Some(b) = params.usx_allocation_bps { require!(b <= 10000, ErrorCode::InvalidAllocation); vs.usx_allocation_bps = b; }
    if let Some(enabled) = params.secondary_market_enabled { vs.secondary_market_enabled = enabled; }
    msg!("Config updated");
    Ok(())
}

#[error_code]
pub enum ErrorCode {
    #[msg("Vault paused")] VaultPaused,
    #[msg("Deposit too small")] DepositTooSmall,
    #[msg("Deposit too large")] DepositTooLarge,
    #[msg("Insufficient funds")] InsufficientVaultFunds,
    #[msg("Overflow")] Overflow,
    #[msg("Invalid allocation")] InvalidAllocation,
    #[msg("Secondary market not enabled")] SecondaryMarketDisabled,
    #[msg("Invalid recipient")] InvalidRecipient,
    #[msg("Compliance check failed")] ComplianceCheckFailed,
}

/// Convert cVAULT (non-tradable) to cVAULT-TRADE (tradable)
/// This enables secondary market participation while maintaining 1:1 backing
#[derive(Accounts)]
pub struct ConvertToTradeable<'info> {
    #[account(mut)]
    pub vault_state: Account<'info, VaultState>,
    #[account(mut)]
    pub cvault_mint: Account<'info, Mint>,
    #[account(mut)]
    pub cvault_trade_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_cvault_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_cvault_trade_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ConvertToTradeableParams {
    pub amount: u64,
}

/// Redeem cVAULT-TRADE back to underlying assets (cVAULT or direct RWA)
#[derive(Accounts)]
pub struct RedeemTradeable<'info> {
    #[account(mut)]
    pub vault_state: Account<'info, VaultState>,
    #[account(mut)]
    pub cvault_trade_mint: Account<'info, Mint>,
    #[account(mut)]
    pub cvault_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_cvault_trade_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_cvault_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_destination: Account<'info, TokenAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RedeemTradeableParams {
    pub amount: u64,
    pub redeem_to_cvault: bool, // true = cVAULT, false = underlying RWAs
}

pub fn convert_to_tradeable(ctx: Context<ConvertToTradeable>, params: ConvertToTradeableParams) -> Result<()> {
    require!(!ctx.accounts.vault_state.paused, ErrorCode::VaultPaused);
    
    // Burn cVAULT from user account
    let token_program = ctx.accounts.token_program.to_account_info();
    let authority_info = ctx.accounts.authority.to_account_info();
    
    let burn_cpi = CpiContext::new(token_program.clone(), token::Burn {
        mint: ctx.accounts.cvault_mint.to_account_info(),
        from: ctx.accounts.user_cvault_account.to_account_info(),
        authority: authority_info.clone(),
    });
    token::burn(burn_cpi, params.amount)?;
    
    // Mint cVAULT-TRADE to user account (1:1 conversion)
    let seeds = &[VAULT_STATE_SEED, &[ctx.accounts.vault_state.bump]];
    let signer = &[seeds.as_ref()];
    
    let mint_cpi = CpiContext::new_with_signer(token_program.clone(), token::MintTo {
        mint: ctx.accounts.cvault_trade_mint.to_account_info(),
        to: ctx.accounts.user_cvault_trade_account.to_account_info(),
        authority: ctx.accounts.vault_state.to_account_info(),
    }, signer);
    token::mint_to(mint_cpi, params.amount)?;
    
    msg!("Converted {} cVAULT to cVAULT-TRADE", params.amount);
    Ok(())
}

pub fn redeem_tradeable(ctx: Context<RedeemTradeable>, params: RedeemTradeableParams) -> Result<()> {
    require!(!ctx.accounts.vault_state.paused, ErrorCode::VaultPaused);
    
    let token_program = ctx.accounts.token_program.to_account_info();
    let authority_info = ctx.accounts.authority.to_account_info();
    
    // Burn cVAULT-TRADE from user account
    let burn_cpi = CpiContext::new(token_program.clone(), token::Burn {
        mint: ctx.accounts.cvault_trade_mint.to_account_info(),
        from: ctx.accounts.user_cvault_trade_account.to_account_info(),
        authority: authority_info.clone(),
    });
    token::burn(burn_cpi, params.amount)?;
    
    if params.redeem_to_cvault {
        // Mint cVAULT back to user
        let seeds = &[VAULT_STATE_SEED, &[ctx.accounts.vault_state.bump]];
        let signer = &[seeds.as_ref()];
        
        let mint_cpi = CpiContext::new_with_signer(token_program.clone(), token::MintTo {
            mint: ctx.accounts.cvault_mint.to_account_info(),
            to: ctx.accounts.user_cvault_account.to_account_info(),
            authority: ctx.accounts.vault_state.to_account_info(),
        }, signer);
        token::mint_to(mint_cpi, params.amount)?;
    } else {
        // Transfer underlying RWAs to user (simplified - in production would handle basket)
        let seeds = &[VAULT_STATE_SEED, &[ctx.accounts.vault_state.bump]];
        let signer = &[seeds.as_ref()];
        
        let transfer_cpi = CpiContext::new_with_signer(token_program.clone(), token::Transfer {
            from: ctx.accounts.vault_state.to_account_info(),
            to: ctx.accounts.user_destination.to_account_info(),
            authority: ctx.accounts.vault_state.to_account_info(),
        }, signer);
        token::transfer(transfer_cpi, params.amount)?;
    }
    
    msg!("Redeemed {} cVAULT-TRADE", params.amount);
    Ok(())

}
/// Claim yield accumulated in USX from Solstice vault
/// This instruction allows the vault to claim earned yield and distribute to vault share holders
#[derive(Accounts)]
pub struct ClaimYield<'info> {
    #[account(mut, seeds = [VAULT_STATE_SEED], bump = vault_state.bump)]
    pub vault_state: Account<'info, VaultState>,
    #[account(mut)]
    pub vault_usx_account: Account<'info, TokenAccount>,
    #[account(address = vault_state.authority)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ClaimYieldParams {
    pub amount: u64,  // Amount of USX yield to claim
}

pub fn claim_yield(ctx: Context<ClaimYield>, params: ClaimYieldParams) -> Result<()> {
    require!(!ctx.accounts.vault_state.paused, ErrorCode::VaultPaused);
    
    let token_program = ctx.accounts.token_program.to_account_info();
    let seeds = &[VAULT_STATE_SEED, &[ctx.accounts.vault_state.bump]];
    let signer = &[seeds.as_ref()];
    
    // Transfer claimed yield to vault's treasury for distribution
    // In production: this would go to a yield distribution account
    let transfer_cpi = CpiContext::new_with_signer(token_program.clone(), token::Transfer {
        from: ctx.accounts.vault_usx_account.to_account_info(),
        to: ctx.accounts.vault_state.to_account_info(),
        authority: ctx.accounts.vault_state.to_account_info(),
    }, signer);
    
    // Note: This assumes the vault has authority over the USX account
    // In production, this would be a CPI to Solstice's yield vault
    
    ctx.accounts.vault_state.last_yield_claim = Clock::get()?.unix_timestamp;
    msg!("Claimed {} USX yield", params.amount);
    Ok(())
}

/// Sync yield state - updates the vault's yield tracking
/// Can be called periodically to update yield information
#[derive(Accounts)]
pub struct SyncYield<'info> {
    #[account(mut, seeds = [VAULT_STATE_SEED], bump = vault_state.bump)]
    pub vault_state: Account<'info, VaultState>,
    #[account(address = vault_state.authority)]
    pub authority: Signer<'info>,
}

pub fn sync_yield(ctx: Context<SyncYield>) -> Result<()> {
    let clock = Clock::get()?;
    ctx.accounts.vault_state.last_yield_claim = clock.unix_timestamp;
    msg!("Yield state synced at {}", clock.unix_timestamp);
    Ok(())
}
