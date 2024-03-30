mod database;
mod model;
mod web;
use database::{DbAddress, create_and_connect, create_schema};
use web::serve;
use std::sync::Arc;


#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    #[error("Server failed to start. Root directory {0} does not exist.")]
    RootNotFound(String),
}

const PORT: u16 = 8080;
const ROOT_DIR : &'static str = "frontend/dist";

#[tokio::main]
async fn main() -> Result<(), Error>{
    env_logger::init();
    let db = create_and_connect(DbAddress::Path("db.sqlite".into())).await?;

    serve(ROOT_DIR, PORT, Arc::new(db)).await?;
    // create_schema(&db).await?;
    Ok(())
}
