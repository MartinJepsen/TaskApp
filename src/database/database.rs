use log::{debug, info, warn};
use sqlx::migrate::MigrateDatabase;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::path::{Path};

use std::time::Duration;

pub type Database = Pool<Sqlite>;


/// Address to the database.
pub enum DbAddress {
    /// Address is a path.
    Path(String),
    /// Address is in-memory.
    Memory,
}

impl DbAddress {

    pub async fn to_sqlite_string(&self) -> String {
        match self {
            DbAddress::Path(path) => {
                format!("sqlite://{}", path)
            }
            DbAddress::Memory => "sqlite::memory:".to_string(),
        }
    }
}

/// Create a new database or connect to an existing one.
pub async fn create_and_connect(address: DbAddress) -> Result<Database, sqlx::Error> {
    
    let address_str = address.to_sqlite_string().await;
    match &address {
        DbAddress::Path(path) => {
            if !Sqlite::database_exists(&path).await.unwrap_or(false) {
                info!("Creating database at {}", &address_str);
                match Sqlite::create_database(&address_str).await {
                    Ok(_) => info!("Database created at {}", &address_str),
                    Err(e) => {
                        panic!("Failed to create database at {}: {}", &address_str, e);
                    }
                }
            } else {
                info!("Database already exists at {}", &address_str);
            }
        },
        DbAddress::Memory => ()
    };

    connect(address).await
}

/// Connect to the database.
pub async fn connect(address: DbAddress) -> Result<Database, sqlx::Error> {
    let conn_str = address.to_sqlite_string().await;
    info!("Connecting to {}", conn_str);
    SqlitePoolOptions::new()
        .max_connections(1)
        .idle_timeout(Duration::from_secs(300))
        .acquire_timeout(Duration::from_secs(5))
        .connect(&conn_str)
        .await
}

pub async fn create_schema(db: &Database) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tasks (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL
        )
        "#,
    )
    .execute(db)
    .await?;
    Ok(())
}



#[cfg(test)]
mod tests {
    use super::{connect, DbAddress, create_and_connect};

    #[tokio::test]
    async fn connect_to_path() -> Result<(), sqlx::Error> {
        let _ = connect(DbAddress::Path("db.sqlite".into())).await?;
        Ok(())
    }

    #[tokio::test]
    async fn connect_to_memory() -> Result<(), sqlx::Error> {
        let _ = connect(DbAddress::Memory).await?;
        Ok(())
    }

    #[tokio::test]
    async fn new_db_from_path() {
        let _ = create_and_connect(DbAddress::Path("db.sqlite".into())).await;
    }
}