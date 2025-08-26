#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::fs;
use tauri::{AppHandle};

#[derive(Serialize, Deserialize)]
struct User {
    id: i64,
    email: String,
    password: String,
}

fn init_db(_app_handle: &AppHandle) -> Result<Connection, String> {
    // Use current working directory instead of system app data dir
    let project_dir = std::env::current_dir()
        .map_err(|e| format!("Failed to get current dir: {}", e))?;

    let db_path = project_dir.join("app.db");

    println!("📂 Database path: {:?}", db_path);

    fs::create_dir_all(&project_dir)
        .map_err(|e| format!("Failed to create project directory: {}", e))?;

    let conn = Connection::open(db_path)
        .map_err(|e| format!("Failed to open database: {}", e))?;

    // ⚡ Always drop and recreate users table
    conn.execute("DROP TABLE IF EXISTS users", [])
        .map_err(|e| format!("Failed to drop old users table: {}", e))?;

    conn.execute(
        "CREATE TABLE users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            email TEXT UNIQUE NOT NULL,
            password TEXT NOT NULL
        )",
        [],
    )
    .map_err(|e| format!("Failed to create table: {}", e))?;

    Ok(conn)
}

#[tauri::command]
async fn signup(app_handle: AppHandle, email: String, password: String) -> Result<User, String> {
    let conn = init_db(&app_handle)?;

    conn.execute(
        "INSERT INTO users (email, password) VALUES (?1, ?2)",
        params![email, password],
    )
    .map_err(|e| format!("Failed to insert user: {}", e))?;

    let id = conn.last_insert_rowid();
    Ok(User { id, email, password })
}

#[tauri::command]
async fn login(app_handle: AppHandle, email: String, password: String) -> Result<User, String> {
    let conn = init_db(&app_handle)?;

    let mut stmt = conn
        .prepare("SELECT id, email, password FROM users WHERE email = ?1 AND password = ?2")
        .map_err(|e| format!("Database error: {}", e))?;

    let user_result: Result<(i64, String, String), rusqlite::Error> = stmt
        .query_row(params![email, password], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        });

    match user_result {
        Ok((id, user_email, user_password)) => Ok(User {
            id,
            email: user_email,
            password: user_password,
        }),
        Err(rusqlite::Error::QueryReturnedNoRows) => Err("Invalid email or password".into()),
        Err(e) => Err(format!("Database error: {}", e)),
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![signup, login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
