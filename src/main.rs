mod database;
mod model;
use database::{DbAddress, create_and_connect, create_schema};


#[tokio::main]
async fn main() -> Result<(), sqlx::Error>{
    env_logger::init();
    let db = create_and_connect(DbAddress::Path("db.sqlite".into())).await?;
    create_schema(&db).await?;
    // sqlx::query("CREATE TABLE IF NOT EXISTS tasks (id TEXT PRIMARY KEY, name TEXT NOT NULL)").execute(&db).await?;
    
    sqlx::query("INSERT INTO tasks (id, name) VALUES (?, ?)").bind(1).bind("Hell world").execute(&db).await?;
    Ok(())
}
