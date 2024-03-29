mod database;
use database::{connect, DbAddress};

use std::path::PathBuf;

#[tokio::main]
async fn main() {
    env_logger::init();
    let db = connect(DbAddress::Path(Box::new(PathBuf::from("db.sqlite"))))
        .await
        .unwrap();
}
