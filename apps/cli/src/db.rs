use std::fs;
use std::path::PathBuf;
use rusqlite::Connection;

pub fn set_database_path(custom_path: &str) {
    let config_path = get_config_path();
    fs::write(config_path, custom_path).expect("Failed to store DB path");
}

pub fn get_database_path() -> PathBuf {
    let config_path = get_config_path();

    if let Ok(db_path) = fs::read_to_string(config_path) {
        return PathBuf::from(db_path.trim());
    }

    PathBuf::from("/tmp/tracking.db")
}

/// Get the config path where the DB path is stored
fn get_config_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    PathBuf::from(format!("{}/.timestack_config", home))
}

/// Open the database connection
pub fn open_database() -> Connection {
    let db_path = get_database_path();
    Connection::open(db_path).expect("Failed to open database")
}