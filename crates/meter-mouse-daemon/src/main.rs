use axum::{extract::State, http::StatusCode, routing::{get, post}, Json, Router};
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, path::PathBuf, sync::{Arc, Mutex}};
use tower_http::cors::CorsLayer;

#[derive(Clone)]
struct AppState { db: Arc<Mutex<Connection>> }

#[derive(Debug, Deserialize)]
struct RecordUsage {
    app: String,
    provider: String,
    model: String,
    input_tokens: Option<u64>,
    output_tokens: Option<u64>,
    latency_ms: Option<u64>,
    status: String,
    usage_source: String,
}

#[derive(Debug, Serialize)]
struct UsageReceipt { id: i64, recorded_at: DateTime<Utc>, #[serde(flatten)] event: RecordUsage }

#[derive(Debug, Serialize)]
struct GardenSummary { requests: i64, input_tokens: i64, output_tokens: i64, providers: i64, agents: i64, unknown_values: i64, prompt_bodies_stored: bool }

fn init_db(path: PathBuf) -> rusqlite::Result<Connection> {
    let db = Connection::open(path)?;
    db.execute_batch("PRAGMA journal_mode = WAL; CREATE TABLE IF NOT EXISTS usage_events (id INTEGER PRIMARY KEY AUTOINCREMENT, recorded_at TEXT NOT NULL, app TEXT NOT NULL, provider TEXT NOT NULL, model TEXT NOT NULL, input_tokens INTEGER, output_tokens INTEGER, latency_ms INTEGER, status TEXT NOT NULL, usage_source TEXT NOT NULL, content_recorded INTEGER NOT NULL DEFAULT 0); CREATE INDEX IF NOT EXISTS idx_usage_events_provider ON usage_events(provider); CREATE INDEX IF NOT EXISTS idx_usage_events_recorded_at ON usage_events(recorded_at);")?;
    Ok(db)
}

async fn health() -> Json<serde_json::Value> { Json(serde_json::json!({"ok": true, "service": "meter-mouse-daemon", "privacy": {"content_recorded": false}})) }

async fn record_usage(State(state): State<AppState>, Json(event): Json<RecordUsage>) -> Result<(StatusCode, Json<UsageReceipt>), (StatusCode, String)> {
    let now = Utc::now();
    let db = state.db.lock().map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "database lock failed".into()))?;
    db.execute("INSERT INTO usage_events (recorded_at, app, provider, model, input_tokens, output_tokens, latency_ms, status, usage_source, content_recorded) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, 0)", params![now, event.app, event.provider, event.model, event.input_tokens, event.output_tokens, event.latency_ms, event.status, event.usage_source]).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let id = db.last_insert_rowid();
    Ok((StatusCode::CREATED, Json(UsageReceipt { id, recorded_at: now, event })))
}

async fn summary(State(state): State<AppState>) -> Result<Json<GardenSummary>, (StatusCode, String)> {
    let db = state.db.lock().map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "database lock failed".into()))?;
    let (requests, input_tokens, output_tokens, providers, agents): (i64, i64, i64, i64, i64) = db.query_row("SELECT COUNT(*), COALESCE(SUM(input_tokens), 0), COALESCE(SUM(output_tokens), 0), COUNT(DISTINCT provider), COUNT(DISTINCT app) FROM usage_events", [], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?))).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let unknown_values: i64 = db.query_row("SELECT COUNT(*) FROM usage_events WHERE usage_source = 'unknown'", [], |row| row.get(0)).unwrap_or(0);
    Ok(Json(GardenSummary { requests, input_tokens, output_tokens, providers, agents, unknown_values, prompt_bodies_stored: false }))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data_dir = std::env::var_os("METER_MOUSE_DATA").map(PathBuf::from).unwrap_or_else(|| PathBuf::from(".meter-mouse"));
    std::fs::create_dir_all(&data_dir)?;
    let db = init_db(data_dir.join("garden.sqlite3"))?;
    let state = AppState { db: Arc::new(Mutex::new(db)) };
    let app = Router::new().route("/api/health", get(health)).route("/api/usage", post(record_usage)).route("/api/garden/summary", get(summary)).with_state(state).layer(CorsLayer::very_permissive());
    let addr: SocketAddr = "127.0.0.1:8787".parse()?;
    println!("MeterMouseGarden local daemon listening on http://{addr}");
    println!("Data is stored in {} and prompt bodies are never recorded.", data_dir.display());
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
