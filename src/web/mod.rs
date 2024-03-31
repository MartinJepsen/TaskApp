use crate::database::Database;
use std::path::Path;
use std::str::from_utf8;
use std::sync::Arc;
use log::info;
use serde_json::Value;
use warp::hyper::{Response, body::Bytes};
use warp::Filter;
use crate::Error;

// use std::io::Result;

mod task;


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


fn extract_body_data<D>(response: Response<Bytes>) -> std::io::Result<D>
where
    for <'de> D: serde::de::Deserialize<'de>,
    D: serde::de::DeserializeOwned,
{
    let body = from_utf8(response.body()).expect("Could not parse response body as UTF8.");
    let mut body: Value =
        serde_json::from_str(body).expect(&format!("Cannot parse JSON response body: {}", body));
    let data = body["data"].take();
    let data: D = serde_json::from_value(data)?;

    Ok(data)
}


/// A custom error for Warp-related stuff.
#[derive(Debug)]
pub struct WebError {
    typ: &'static str,
    message: String,
}

impl warp::reject::Reject for WebError {}

impl WebError {
    pub fn rejection(typ: &'static str, message: String) -> warp::Rejection {
        warp::reject::custom(WebError { typ, message })
    }
}

impl From<crate::Error> for WebError {
    fn from(e: crate::Error) -> Self {
        WebError {
            typ: "internal",
            message: e.to_string(),
        }
    }
}