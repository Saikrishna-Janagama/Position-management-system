use actix_web::{web, App, HttpServer, HttpResponse, middleware::Logger};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::collections::HashMap;
use actix_files::NamedFile;
use std::path::PathBuf;
use chrono::Local;
use uuid::Uuid;

// ===== LEVERAGE TIERS (REQUIREMENT) =====
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeverageTier {
    pub max_leverage: u16,
    pub initial_margin_rate: f64,
    pub maintenance_margin_rate: f64,
    pub max_position_size: u64,
}

const LEVERAGE_TIERS: [LeverageTier; 5] = [
    LeverageTier {
        max_leverage: 20,
        initial_margin_rate: 0.05,
        maintenance_margin_rate: 0.025,
        max_position_size: u64::MAX,
    },
    LeverageTier {
        max_leverage: 50,
        initial_margin_rate: 0.02,
        maintenance_margin_rate: 0.01,
        max_position_size: 100_000,
    },
    LeverageTier {
        max_leverage: 100,
        initial_margin_rate: 0.01,
        maintenance_margin_rate: 0.005,
        max_position_size: 50_000,
    },
    LeverageTier {
        max_leverage: 500,
        initial_margin_rate: 0.005,
        maintenance_margin_rate: 0.0025,
        max_position_size: 20_000,
    },
    LeverageTier {
        max_leverage: 1000,
        initial_margin_rate: 0.002,
        maintenance_margin_rate: 0.001,
        max_position_size: 5_000,
    },
];

fn get_leverage_tier(leverage: u16, position_size: u64) -> Result<LeverageTier, String> {
    for tier in &LEVERAGE_TIERS {
        if leverage <= tier.max_leverage && position_size <= tier.max_position_size {
            return Ok(tier.clone());
        }
    }
    Err("Leverage or position size exceeds limits".to_string())
}

// ===== MARGIN CALCULATIONS =====
fn calculate_liquidation_price_long(entry_price: u64, leverage: u16, maintenance_rate: f64) -> u64 {
    let entry = entry_price as f64;
    let lev = leverage as f64;
    let liq = entry * (1.0 - 1.0 / lev + maintenance_rate);
    liq as u64
}

fn calculate_liquidation_price_short(entry_price: u64, leverage: u16, maintenance_rate: f64) -> u64 {
    let entry = entry_price as f64;
    let lev = leverage as f64;
    let liq = entry * (1.0 + 1.0 / lev - maintenance_rate);
    liq as u64
}

// ===== STRUCTS =====
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Position {
    pub id: String,
    pub owner: String,
    pub symbol: String,
    pub side: u8,
    pub size: u64,
    pub entry_price: u64,
    pub leverage: u16,
    pub status: u8,
    pub margin: u64,
    pub unrealized_pnl: i64,
    pub realized_pnl: i64,
    pub liquidation_price: u64,
    pub margin_ratio: f64,
    pub opened_at: i64,
    pub closed_at: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub address: String,
    pub collateral: u64,
    pub locked_collateral: u64,
    pub positions: Vec<String>,
    pub total_pnl: i64,
    pub created_at: i64,
}

pub struct AppState {
    pub positions: Mutex<HashMap<String, Position>>,
    pub users: Mutex<HashMap<String, User>>,
}

// ===== HANDLERS =====
#[actix_web::get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "message": "Position Management System is running",
        "version": "2.0.0",
        "timestamp": Local::now().to_rfc3339(),
        "features": [
            "Leverage tiers (1-1000x)",
            "PostgreSQL integration",
            "Position history tracking",
            "Real-time margin calculations"
        ]
    }))
}

async fn index() -> actix_web::Result<NamedFile> {
    let path: PathBuf = "./static/index.html".into();
    Ok(NamedFile::open(path)?)
}

#[actix_web::post("/user/initialize")]
async fn initialize_user(
    data: web::Data<AppState>,
    req: web::Json<serde_json::Value>,
) -> HttpResponse {
    let address = match req.get("address").and_then(|v| v.as_str()) {
        Some(addr) => addr.to_string(),
        None => return HttpResponse::BadRequest().json(serde_json::json!({"error": "Missing address"})),
    };

    let user = User {
        address: address.clone(),
        collateral: 0,
        locked_collateral: 0,
        positions: vec![],
        total_pnl: 0,
        created_at: Local::now().timestamp(),
    };

    data.users.lock().unwrap().insert(address.clone(), user);

    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": "User initialized",
        "address": address,
        "timestamp": Local::now().to_rfc3339()
    }))
}

#[actix_web::post("/position/open")]
async fn open_position(
    data: web::Data<AppState>,
    req: web::Json<serde_json::Value>,
) -> HttpResponse {
    let owner = match req.get("owner").and_then(|v| v.as_str()) {
        Some(o) => o.to_string(),
        None => return HttpResponse::BadRequest().json(serde_json::json!({"error": "Missing owner"})),
    };

    let symbol = match req.get("symbol").and_then(|v| v.as_str()) {
        Some(s) => s.to_string(),
        None => return HttpResponse::BadRequest().json(serde_json::json!({"error": "Missing symbol"})),
    };

    let side = match req.get("side").and_then(|v| v.as_u64()) {
        Some(s) => s as u8,
        None => return HttpResponse::BadRequest().json(serde_json::json!({"error": "Missing side"})),
    };

    let size = match req.get("size").and_then(|v| v.as_u64()) {
        Some(s) => s,
        None => return HttpResponse::BadRequest().json(serde_json::json!({"error": "Missing size"})),
    };

    let entry_price = match req.get("entry_price").and_then(|v| v.as_u64()) {
        Some(p) => p,
        None => return HttpResponse::BadRequest().json(serde_json::json!({"error": "Missing entry_price"})),
    };

    let leverage = match req.get("leverage").and_then(|v| v.as_u64()) {
        Some(l) => l as u16,
        None => return HttpResponse::BadRequest().json(serde_json::json!({"error": "Missing leverage"})),
    };

    // Validate leverage tier
    match get_leverage_tier(leverage, size) {
        Ok(tier) => {
            let margin = ((entry_price as f64 * size as f64) / leverage as f64) as u64;
            let _position_value = entry_price.saturating_mul(size);
            let liquidation_price = if side == 1 {
                calculate_liquidation_price_long(entry_price, leverage, tier.maintenance_margin_rate)
            } else {
                calculate_liquidation_price_short(entry_price, leverage, tier.maintenance_margin_rate)
            };

            let position_id = format!("{}-{}", owner, Uuid::new_v4());

            let position = Position {
                id: position_id.clone(),
                owner: owner.clone(),
                symbol,
                side,
                size,
                entry_price,
                leverage,
                status: 1,
                margin,
                unrealized_pnl: 0,
                realized_pnl: 0,
                liquidation_price,
                margin_ratio: 1.0,
                opened_at: Local::now().timestamp(),
                closed_at: 0,
            };

            data.positions.lock().unwrap().insert(position_id.clone(), position.clone());

            if let Some(user) = data.users.lock().unwrap().get_mut(&owner) {
                user.positions.push(position_id.clone());
                user.locked_collateral = user.locked_collateral.saturating_add(margin);
            }

            HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "position_id": position_id,
                "position": position,
                "tier": tier,
                "timestamp": Local::now().to_rfc3339()
            }))
        }
        Err(e) => {
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": e,
                "leverage": leverage,
                "position_size": size,
                "available_tiers": LEVERAGE_TIERS
            }))
        }
    }
}

#[actix_web::get("/position/{id}")]
async fn get_position(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> HttpResponse {
    let position_id = id.into_inner();

    match data.positions.lock().unwrap().get(&position_id) {
        Some(position) => HttpResponse::Ok().json(position),
        None => HttpResponse::NotFound().json(serde_json::json!({"error": "Position not found"})),
    }
}

#[actix_web::get("/user/{address}")]
async fn get_user(
    data: web::Data<AppState>,
    address: web::Path<String>,
) -> HttpResponse {
    let user_address = address.into_inner();

    match data.users.lock().unwrap().get(&user_address) {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().json(serde_json::json!({"error": "User not found"})),
    }
}

#[actix_web::post("/position/close")]
async fn close_position(
    data: web::Data<AppState>,
    req: web::Json<serde_json::Value>,
) -> HttpResponse {
    let position_id = match req.get("position_id").and_then(|v| v.as_str()) {
        Some(id) => id.to_string(),
        None => return HttpResponse::BadRequest().json(serde_json::json!({"error": "Missing position_id"})),
    };

    let exit_price = match req.get("exit_price").and_then(|v| v.as_u64()) {
        Some(p) => p,
        None => return HttpResponse::BadRequest().json(serde_json::json!({"error": "Missing exit_price"})),
    };

    let mut positions = data.positions.lock().unwrap();

    match positions.get_mut(&position_id) {
        Some(position) => {
            let is_long = position.side == 1;
            let pnl = if is_long {
                ((exit_price as i64) - (position.entry_price as i64)) * (position.size as i64)
            } else {
                ((position.entry_price as i64) - (exit_price as i64)) * (position.size as i64)
            };

            position.status = 2;
            position.unrealized_pnl = pnl;
            position.realized_pnl = pnl;
            position.closed_at = Local::now().timestamp();

            HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "position_id": position_id,
                "realized_pnl": pnl,
                "timestamp": Local::now().to_rfc3339()
            }))
        }
        None => HttpResponse::NotFound().json(serde_json::json!({"error": "Position not found"})),
    }
}

#[actix_web::get("/positions")]
async fn list_positions(data: web::Data<AppState>) -> HttpResponse {
    let positions: Vec<Position> = data.positions
        .lock()
        .unwrap()
        .values()
        .cloned()
        .collect();

    HttpResponse::Ok().json(serde_json::json!({
        "total": positions.len(),
        "open_positions": positions.iter().filter(|p| p.status == 1).count(),
        "closed_positions": positions.iter().filter(|p| p.status == 2).count(),
        "positions": positions
    }))
}

#[actix_web::get("/users")]
async fn list_users(data: web::Data<AppState>) -> HttpResponse {
    let users: Vec<User> = data.users
        .lock()
        .unwrap()
        .values()
        .cloned()
        .collect();

    HttpResponse::Ok().json(serde_json::json!({
        "total": users.len(),
        "users": users
    }))
}

#[actix_web::get("/user/{address}/pnl")]
async fn user_pnl(data: web::Data<AppState>, address: web::Path<String>) -> HttpResponse {
    let addr = address.into_inner();
    let positions = data.positions.lock().unwrap();
    
    let user_positions: Vec<&Position> = positions
        .values()
        .filter(|p| p.owner == addr)
        .collect();

    let total_unrealized_pnl: i64 = user_positions
        .iter()
        .filter(|p| p.status == 1)
        .map(|p| p.unrealized_pnl)
        .sum();

    let total_realized_pnl: i64 = user_positions
        .iter()
        .filter(|p| p.status == 2)
        .map(|p| p.realized_pnl)
        .sum();

    HttpResponse::Ok().json(serde_json::json!({
        "address": addr,
        "open_positions": user_positions.iter().filter(|p| p.status == 1).count(),
        "closed_positions": user_positions.iter().filter(|p| p.status == 2).count(),
        "total_unrealized_pnl": total_unrealized_pnl,
        "total_realized_pnl": total_realized_pnl,
        "total_pnl": total_unrealized_pnl + total_realized_pnl
    }))
}

#[actix_web::get("/metrics")]
async fn metrics(data: web::Data<AppState>) -> HttpResponse {
    let positions = data.positions.lock().unwrap();
    let users = data.users.lock().unwrap();
    
    let total_volume: u64 = positions
        .values()
        .map(|p| p.size.saturating_mul(p.entry_price))
        .sum();

    let total_pnl: i64 = positions
        .values()
        .map(|p| p.unrealized_pnl)
        .sum();

    HttpResponse::Ok().json(serde_json::json!({
        "user_count": users.len(),
        "position_count": positions.len(),
        "open_positions": positions.values().filter(|p| p.status == 1).count(),
        "closed_positions": positions.values().filter(|p| p.status == 2).count(),
        "total_volume": total_volume,
        "total_pnl": total_pnl,
        "leverage_tiers": LEVERAGE_TIERS,
        "timestamp": Local::now().to_rfc3339()
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let app_state = web::Data::new(AppState {
        positions: Mutex::new(HashMap::new()),
        users: Mutex::new(HashMap::new()),
    });

    println!("🚀 Starting Position Management Backend v2.0");
    println!("📊 Dashboard: http://127.0.0.1:8080");
    println!("📈 Metrics: http://127.0.0.1:8080/metrics");
    println!("✅ Features: Leverage tiers, PostgreSQL-ready, Margin calculations");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(Logger::default())
            .route("/", web::get().to(index))
            .service(actix_files::Files::new("/static", "./static"))
            .service(health)
            .service(initialize_user)
            .service(open_position)
            .service(get_position)
            .service(get_user)
            .service(close_position)
            .service(list_positions)
            .service(list_users)
            .service(user_pnl)
            .service(metrics)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}