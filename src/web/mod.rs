use crate::database::Database;
use std::path::Path;
use std::sync::Arc;
use log::info;
use warp::Filter;
use crate::Error;


pub async fn serve(root_dir: &str, port: u16, database: Arc<Database>) -> Result<(), Error> {
    if !Path::new(root_dir).exists() {
        return Err(Error::RootNotFound("Root directory does not exist.".to_owned()));
    }
    
    // Static site
    let content = warp::fs::dir(root_dir.to_string());
    let index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(format!("{}/index.html", root_dir)));
    let static_site = content.or(index);

    let routes = static_site;

    info!("Starting server http://127.0.0.1:{} from {}", port, root_dir);
    warp::serve(routes).run(([127, 0, 0, 1], port)).await;


    Ok(())
}

