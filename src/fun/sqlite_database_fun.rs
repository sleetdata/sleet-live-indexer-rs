use anyhow::{Context, Result};
use rusqlite::{Connection, params};
use std::path::Path;
// ===========================================
const DEFAULT_DB_PATH: &str = "./temp/indexer.db";
// ===========================================

pub fn init_database_fun(db_path: Option<&str>) -> Result<Connection> {
    let path = db_path.unwrap_or(DEFAULT_DB_PATH);
    
    // Create parent directories if needed
    if let Some(parent) = Path::new(path).parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
    }
    
    let conn = Connection::open(path)
        .with_context(|| format!("Failed to open database: {}", path))?;
    
    // Create tables
    conn.execute(
        "CREATE TABLE IF NOT EXISTS transactions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            tx_hash TEXT UNIQUE NOT NULL,
            block_height INTEGER NOT NULL,
            shard_id INTEGER NOT NULL,
            signer_id TEXT NOT NULL,
            receiver_id TEXT NOT NULL,
            action_type TEXT NOT NULL,
            method_name TEXT,
            deposit TEXT,
            gas INTEGER,
            logs TEXT,
            timestamp TEXT NOT NULL
        )",
        [],
    )?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS deleted_accounts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            tx_hash TEXT UNIQUE NOT NULL,
            block_height INTEGER NOT NULL,
            shard_id INTEGER NOT NULL,
            deleted_account_id TEXT NOT NULL,
            beneficiary_id TEXT NOT NULL,
            signer_id TEXT NOT NULL,
            deposit TEXT,
            timestamp TEXT NOT NULL
        )",
        [],
    )?;
    
    // Create indexes for faster queries
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_tx_hash ON transactions(tx_hash)",
        [],
    )?;
    
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_signer_id ON transactions(signer_id)",
        [],
    )?;
    
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_receiver_id ON transactions(receiver_id)",
        [],
    )?;
    
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_block_height ON transactions(block_height)",
        [],
    )?;
    
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_deleted_account_id ON deleted_accounts(deleted_account_id)",
        [],
    )?;
    
    Ok(conn)
}
// ===========================================

pub fn save_transaction_fun(
    conn: &Connection,
    tx_hash: &str,
    block_height: u64,
    shard_id: u64,
    signer_id: &str,
    receiver_id: &str,
    action_type: &str,
    method_name: Option<&str>,
    deposit: Option<&str>,
    gas: Option<u64>,
    logs: &[String],
) -> Result<u64> {
    let timestamp = chrono::Utc::now().to_rfc3339();
    let logs_json = serde_json::to_string(logs).unwrap_or_default();
    
    conn.execute(
        "INSERT OR IGNORE INTO transactions 
         (tx_hash, block_height, shard_id, signer_id, receiver_id, action_type, method_name, deposit, gas, logs, timestamp)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        params![
            tx_hash,
            block_height,
            shard_id,
            signer_id,
            receiver_id,
            action_type,
            method_name,
            deposit,
            gas,
            logs_json,
            timestamp
        ],
    )?;
    
    // Get the row id
    let mut stmt = conn.prepare("SELECT id FROM transactions WHERE tx_hash = ?1")?;
    let id: u64 = stmt.query_row(params![tx_hash], |row| row.get(0))?;
    
    Ok(id)
}
// ===========================================

pub fn save_deleted_account_fun(
    conn: &Connection,
    tx_hash: &str,
    block_height: u64,
    shard_id: u64,
    deleted_account_id: &str,
    beneficiary_id: &str,
    signer_id: &str,
    deposit: Option<&str>,
) -> Result<u64> {
    let timestamp = chrono::Utc::now().to_rfc3339();
    
    conn.execute(
        "INSERT OR IGNORE INTO deleted_accounts 
         (tx_hash, block_height, shard_id, deleted_account_id, beneficiary_id, signer_id, deposit, timestamp)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            tx_hash,
            block_height,
            shard_id,
            deleted_account_id,
            beneficiary_id,
            signer_id,
            deposit,
            timestamp
        ],
    )?;
    
    // Get the row id
    let mut stmt = conn.prepare("SELECT id FROM deleted_accounts WHERE tx_hash = ?1")?;
    let id: u64 = stmt.query_row(params![tx_hash], |row| row.get(0))?;
    
    Ok(id)
}
// ===========================================

pub fn get_deleted_accounts_fun(conn: &Connection, limit: u32) -> Result<Vec<DeletedAccountRecord>> {
    let mut stmt = conn.prepare(
        "SELECT * FROM deleted_accounts ORDER BY block_height DESC LIMIT ?1"
    )?;
    
    let accounts = stmt.query_map(params![limit], |row| {
        Ok(DeletedAccountRecord {
            id: row.get(0)?,
            tx_hash: row.get(1)?,
            block_height: row.get(2)?,
            shard_id: row.get(3)?,
            deleted_account_id: row.get(4)?,
            beneficiary_id: row.get(5)?,
            signer_id: row.get(6)?,
            deposit: row.get(7)?,
            timestamp: row.get(8)?,
        })
    })?;
    
    let mut result = Vec::new();
    for account in accounts {
        result.push(account?);
    }
    
    Ok(result)
}
// ===========================================

pub struct DeletedAccountRecord {
    pub id: u64,
    pub tx_hash: String,
    pub block_height: u64,
    pub shard_id: u64,
    pub deleted_account_id: String,
    pub beneficiary_id: String,
    pub signer_id: String,
    pub deposit: Option<String>,
    pub timestamp: String,
}
// ===========================================
