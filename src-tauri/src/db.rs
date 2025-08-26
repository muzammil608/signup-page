use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use tauri::AppHandle;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub password_hash: String,
}

/// Initialize the SQLite database inside the Tauri app data directory
pub fn init_db(app_handle: &AppHandle) -> Result<Connection> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .expect("Failed to get app data dir");

    if let Err(e) = fs::create_dir_all(&app_dir) {
        eprintln!("Failed to create app dir: {}", e);
    }

    let db_path = app_dir.join("app.db");
    let conn = Connection::open(db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            email TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL
        )",
        [],
    )?;

    Ok(conn)
}

/// Create a new user with hashed password
pub fn create_user(conn: &Connection, email: &str, password: &str) -> Result<()> {
    let password_hash = hash_password(password);
    conn.execute(
        "INSERT INTO users (email, password_hash) VALUES (?1, ?2)",
        params![email, password_hash],
    )?;
    Ok(())
}

/// Verify user login credentials
pub fn verify_user(conn: &Connection, email: &str, password: &str) -> Result<Option<User>> {
    let mut stmt = conn.prepare("SELECT id, email, password_hash FROM users WHERE email = ?1")?;
    let user_iter = stmt.query_map(params![email], |row| {
        Ok(User {
            id: row.get(0)?,
            email: row.get(1)?,
            password_hash: row.get(2)?,
        })
    })?;

    for user in user_iter {
        let user = user?;
        if verify_password(password, &user.password_hash) {
            return Ok(Some(user));
        }
    }

    Ok(None)
}

/// Hash password using SHA256
fn hash_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Verify input password against stored hash
fn verify_password(password: &str, hash: &str) -> bool {
    hash_password(password) == hash
}
