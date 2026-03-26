//! SIX Financial Data API routes for yield backing.
//! 
//! Provides endpoints for:
//! - Forex rates (EUR/USD, CHF/USD, etc.)
//! - Precious metals (Gold, Silver, Platinum)
//! - Equities (NYSE, NASDAQ, Copenhagen)
//! - Vault NAV calculations

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::infra::six::{SixApiClient, SixConfig, instruments};

/// Request for forex rate
#[derive(Debug, Deserialize)]
pub struct ForexRateRequest {
    pub base: String,
    pub quote: String,
}

/// Request for precious metal price
#[derive(Debug, Deserialize)]
pub struct PreciousMetalRequest {
    pub metal: String,
}

/// Request for equity price
#[derive(Debug, Deserialize)]
pub struct EquityPriceRequest {
    pub valor: String,
    pub market_bc: String,
}

/// Request for vault NAV calculation
#[derive(Debug, Deserialize)]
pub struct VaultNavRequest {
    pub holdings: Vec<(String, f64)>, // (instrument_id, quantity)
}

/// Response for market data
#[derive(Debug, Serialize)]
pub struct MarketDataResponse {
    pub success: bool,
    pub data: Option<serde_json::Value>,
    pub error: Option<String>,
}

/// GET /api/six/forex/{base}/{quote}
/// Fetch forex rate for a currency pair
pub async fn get_forex_rate(
    path: web::Path<(String, String)>,
    six_client: web::Data<SixApiClient>,
) -> impl Responder {
    let (base, quote) = path.into_inner();
    
    match six_client.get_forex_rate(&base, &quote).await {
        Ok(rate) => HttpResponse::Ok().json(MarketDataResponse {
            success: true,
            data: Some(serde_json::to_value(rate).unwrap()),
            error: None,
        }),
        Err(e) => HttpResponse::InternalServerError().json(MarketDataResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

/// GET /api/six/metal/{metal}
/// Fetch precious metal price
pub async fn get_precious_metal_price(
    path: web::Path<String>,
    six_client: web::Data<SixApiClient>,
) -> impl Responder {
    let metal = path.into_inner();
    
    match six_client.get_precious_metal_price(&metal).await {
        Ok(price) => HttpResponse::Ok().json(MarketDataResponse {
            success: true,
            data: Some(serde_json::to_value(price).unwrap()),
            error: None,
        }),
        Err(e) => HttpResponse::InternalServerError().json(MarketDataResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

/// GET /api/six/equity/{valor}/{market_bc}
/// Fetch equity price
pub async fn get_equity_price(
    path: web::Path<(String, String)>,
    six_client: web::Data<SixApiClient>,
) -> impl Responder {
    let (valor, market_bc) = path.into_inner();
    
    match six_client.get_equity_price(&valor, &market_bc).await {
        Ok(data) => HttpResponse::Ok().json(MarketDataResponse {
            success: true,
            data: Some(serde_json::to_value(data).unwrap()),
            error: None,
        }),
        Err(e) => HttpResponse::InternalServerError().json(MarketDataResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

/// POST /api/six/nav
/// Calculate vault NAV using SIX market data
pub async fn calculate_vault_nav(
    body: web::Json<VaultNavRequest>,
    six_client: web::Data<SixApiClient>,
) -> impl Responder {
    match six_client.calculate_vault_nav(&body.holdings).await {
        Ok(nav) => HttpResponse::Ok().json(MarketDataResponse {
            success: true,
            data: Some(serde_json::json!({ "nav": nav })),
            error: None,
        }),
        Err(e) => HttpResponse::InternalServerError().json(MarketDataResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

/// GET /api/six/health
/// Health check for SIX API connectivity
pub async fn health_check(
    six_client: web::Data<SixApiClient>,
) -> impl Responder {
    let healthy = six_client.health_check().await;
    
    if healthy {
        HttpResponse::Ok().json(MarketDataResponse {
            success: true,
            data: Some(serde_json::json!({ "status": "healthy" })),
            error: None,
        })
    } else {
        HttpResponse::ServiceUnavailable().json(MarketDataResponse {
            success: false,
            data: None,
            error: Some("SIX API is not reachable".to_string()),
        })
    }
}

/// GET /api/six/instruments
/// List available SIX instruments for the hackathon
pub async fn list_instruments() -> impl Responder {
    let instruments = serde_json::json!({
        "nyse": {
            "market_bc": "65",
            "instruments": {
                "coca_cola": instruments::COCA_COLA,
                "blackrock": instruments::BLACKROCK,
                "mt_bank": instruments::MT_BANK,
                "oracle": instruments::ORACLE,
            }
        },
        "forex": {
            "market_bc": "149",
            "instruments": {
                "eur_usd": instruments::EUR_USD,
                "chf_usd": instruments::CHF_USD,
                "chf_eur": instruments::CHF_EUR,
                "gbp_usd": instruments::GBP_USD,
            }
        },
        "nasdaq": {
            "market_bc": "67",
            "instruments": {
                "apple": instruments::APPLE,
                "microsoft": instruments::MICROSOFT,
                "walmart": instruments::WALMART,
                "intel": instruments::INTEL,
            }
        },
        "nasdaq_copenhagen": {
            "market_bc": "12",
            "instruments": {
                "novo_nord": instruments::NOVO_NORD,
                "danske_bank": instruments::DANSKE_BANK,
                "carlsberg": instruments::CARLSBERG,
                "nordea": instruments::NORDEA,
            }
        }
    });
    
    HttpResponse::Ok().json(MarketDataResponse {
        success: true,
        data: Some(instruments),
        error: None,
    })
}

/// Configure SIX API routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/six")
            .route("/health", web::get().to(health_check))
            .route("/instruments", web::get().to(list_instruments))
            .route("/forex/{base}/{quote}", web::get().to(get_forex_rate))
            .route("/metal/{metal}", web::get().to(get_precious_metal_price))
            .route("/equity/{valor}/{market_bc}", web::get().to(get_equity_price))
            .route("/nav", web::post().to(calculate_vault_nav)),
    );
}
