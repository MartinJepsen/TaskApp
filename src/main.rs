mod database;
mod model;
use database::{DbAddress, create_and_connect, create_schema};


#[tokio::main]
async fn main() -> Result<(), sqlx::Error>{
    env_logger::init();
    let db = create_and_connect(DbAddress::Path("db.sqlite".into())).await?;
    create_schema(&db).await?;
    Ok(())
}
