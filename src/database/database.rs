use log::info;
use sqlx::migrate::MigrateDatabase;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

use std::fs::File;
use std::path::Path;
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
    /// Convert the address to a string.
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
pub async fn create_and_connect(address: DbAddress) -> Result<Database, crate::Error> {
    let address_str = address.to_sqlite_string().await;
    match &address {
        DbAddress::Path(path) => {
            if !Sqlite::database_exists(&path).await.unwrap_or(false) {
                info!("Creating database at {}", &address_str);
                match Path::new(&path).exists() {
                    false => {
                        File::create(&path).expect("Failed to create file database.");
                    }
                    true => info!(
                        "Database file already exists at {}. Connecting.",
                        &address_str
                    ),
                };
                match Sqlite::create_database(&address_str).await {
                    Ok(_) => info!("Database created at {}", &address_str),
                    Err(e) => {
                        panic!("Failed to create database at {}: {}", &address_str, e);
                    }
                }
            } else {
                info!("Database already exists at {}", &address_str);
            }
        }
        DbAddress::Memory => (),
    };

    let pool = connect(address).await?;
    create_schema(&pool).await?;
    Ok(pool)
}

/// Connect to the database.
pub async fn connect(address: DbAddress) -> Result<Database, crate::Error> {
    let conn_str = address.to_sqlite_string().await;
    info!("Connecting to {}", conn_str);
    let options = SqlitePoolOptions::new()
        .max_connections(1)
        .idle_timeout(Duration::from_secs(300))
        .acquire_timeout(Duration::from_secs(5))
        .connect(&conn_str)
        .await?;
    Ok(options)
}

/// Create the database schema
pub async fn create_schema(db: &Database) -> Result<(), crate::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER NOT NULL PRIMARY KEY ,
            name TEXT NOT NULL,
            status VARCHAR(5) NOT NULL DEFAULT 'open',
            creation_time INTEGER NOT NULL
        );
        "#,
    )
    .execute(db)
    .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{connect, create_and_connect, create_schema, DbAddress};
    use sqlx::Row;

    #[tokio::test]
    async fn connect_to_path() -> Result<(), crate::Error> {
        let _ = connect(DbAddress::Path("test.sqlite".into())).await?;
        Ok(())
    }

    #[tokio::test]
    async fn connect_to_memory() -> Result<(), crate::Error> {
        let _ = connect(DbAddress::Memory).await?;
        Ok(())
    }

    #[tokio::test]
    async fn new_db_from_path() {
        let _ = create_and_connect(DbAddress::Path("test.sqlite".into())).await;
    }

    #[tokio::test]
    async fn test_schema() -> Result<(), crate::Error> {
        // # Fixture
        let db = create_and_connect(DbAddress::Memory).await?;
        let _ = create_schema(&db).await;

        // # Get schema
        let rows = sqlx::query("PRAGMA table_info(tasks)")
            .fetch_all(&db)
            .await?;

        let mut schema: Vec<(String, String, bool, bool)> = Vec::new();
        for row in rows {
            let colname = row.try_get::<String, _>("name").unwrap();
            let dtype = row.try_get::<String, _>("type").unwrap();
            let notnull = row.try_get::<bool, _>("notnull").unwrap();
            let pk = row.try_get::<bool, _>("pk").unwrap();
            schema.push((colname, dtype, notnull, pk))
        }

        // Check schema
        assert_eq!(
            schema,
            vec![
                ("id".to_string(), "INTEGER".to_string(), true, true),
                ("name".to_string(), "TEXT".to_string(), true, false),
                ("status".to_string(), "VARCHAR(5)".to_string(), true, false),
                (
                    "creation_time".to_string(),
                    "INTEGER".to_string(),
                    true,
                    false
                ),
            ]
        );
        Ok(())
    }
}
