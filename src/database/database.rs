use log::{debug, info, warn};
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::path::{Path};

use std::time::Duration;

pub type Database = Pool<Sqlite>;


/// Address to the database.
pub enum DbAddress {
    /// Address is a path.
    Path(Box<dyn AsRef<Path>>),
    /// Address is a URL.
    Url(String),
    /// Address is in-memory.
    Memory,
}

/// Connect to the database.
pub async fn connect(address: DbAddress) -> Result<Database, sqlx::Error> {
    let conn_str = match address {
        DbAddress::Path(path) => {
            format!("sqlite:{}", path.as_ref().as_ref().to_string_lossy())
        }
        DbAddress::Url(url) => format!("sqlite:{}", url),
        DbAddress::Memory => "sqlite::memory:".to_string(),
    };
    info!("Connecting to {}", conn_str);
    SqlitePoolOptions::new()
        .max_connections(1)
        .idle_timeout(Duration::from_secs(300))
        .acquire_timeout(Duration::from_secs(5))
        .connect(&conn_str)
        .await
}
