mod database;
mod model;
mod web;
use database::{create_and_connect, DbAddress};
use log::{info, warn};
use std::sync::Arc;
use web::serve;
use tracing_subscriber::EnvFilter;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    #[error("Server failed to start. Root directory {0} does not exist.")]
    RootNotFound(String),
}

const PORT: u16 = 8080;
const ROOT_DIR: &str = "frontend/dist";

#[tokio::main]
async fn main() -> Result<(), Error> {
    let formatter = tracing_subscriber::fmt().pretty().with_env_filter(EnvFilter::from_default_env()).finish();
    match tracing::subscriber::set_global_default(formatter)
    {
        Ok(_) => info!("Tracing initialized."),
        Err(reason) => warn!("Failed to initialize tracing: {}", reason),
    };


    let db = create_and_connect(DbAddress::Memory).await?;

    // Insert some mock data
    model::task::TaskMac::insert(&db, model::task::TaskPatch {
        name: Some("Mock 1".into()),
        status: Some(model::task::TaskStatus::Open)
    }).await?;
    model::task::TaskMac::insert(&db, model::task::TaskPatch {
        name: Some("Mock 2".into()),
        status: Some(model::task::TaskStatus::Closed)
    }).await?;

    serve(ROOT_DIR, PORT, Arc::new(db)).await?;
    Ok(())
}
