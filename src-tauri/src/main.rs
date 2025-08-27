#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rusqlite::{params, Connection};
use serde::Serialize;
use tauri::{AppHandle};
use std::path::PathBuf;

#[derive(Serialize)]
struct User {
    id: i64,
    email: String,
    password: String,
}


fn get_db_path(_app_handle: &AppHandle) -> PathBuf {
    let folder = PathBuf::from("src-tauri"); // force inside src-tauri
    std::fs::create_dir_all(&folder).expect("⚠️ Failed to create src-tauri folder");
    folder.join("app.db")
}

/// ✅ Initialize DB
fn init_db(app_handle: &AppHandle) -> Result<Connection, String> {
    let db_path = get_db_path(app_handle);
    println!("📂 Database path: {:?}", db_path);

    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            email TEXT UNIQUE NOT NULL,
            password TEXT NOT NULL
        )",
        [],
    )
    .map_err(|e| e.to_string())?;

    Ok(conn)
}

#[tauri::command]
fn signup(app_handle: AppHandle, email: String, password: String) -> Result<String, String> {
    let conn = init_db(&app_handle)?;
    conn.execute(
        "INSERT INTO users (email, password) VALUES (?1, ?2)",
        params![email, password],
    )
    .map_err(|e| e.to_string())?;
    Ok("Signup successful".into())
}

#[tauri::command]
fn login(app_handle: AppHandle, email: String, password: String) -> Result<User, String> {
    let conn = init_db(&app_handle)?;

    let mut stmt = conn
        .prepare("SELECT id, email, password FROM users WHERE email = ?1")
        .map_err(|e| e.to_string())?;

    let user_result = stmt.query_row(params![email], |row| {
        Ok(User {
            id: row.get(0)?,
            email: row.get(1)?,
            password: row.get(2)?,
        })
    });

    match user_result {
        Ok(user) => {
            if user.password == password {
                Ok(user)
            } else {
                Err("Invalid password".into())
            }
        }
        Err(_) => Err("User not found".into()),
    }
}

/// ✅ New command: Fetch all users
#[tauri::command]
fn get_all_users(app_handle: AppHandle) -> Result<Vec<User>, String> {
    let conn = init_db(&app_handle)?;

    let mut stmt = conn
        .prepare("SELECT id, email, password FROM users")
        .map_err(|e| e.to_string())?;

    let users_iter = stmt
        .query_map([], |row| {
            Ok(User {
                id: row.get(0)?,
                email: row.get(1)?,
                password: row.get(2)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut users = Vec::new();
    for user in users_iter {
        users.push(user.map_err(|e| e.to_string())?);
    }

    Ok(users)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![signup, login, get_all_users])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
