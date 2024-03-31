use crate::database::Database;
use crate::Error;
use log::{error, info};
use serde::Serialize;
use serde_json::{json, Value};
use std::convert::Infallible;
use std::path::Path;
use std::str::from_utf8;
use std::sync::Arc;
use warp::hyper::{body::Bytes, Response};
use warp::reply::Json;
use warp::Filter;

// use std::io::Result;

mod task;

pub async fn serve(root_dir: &str, port: u16, database: Arc<Database>) -> Result<(), Error> {
    if !Path::new(root_dir).exists() {
        return Err(Error::RootNotFound(
            "Root directory does not exist.".to_owned(),
        ));
    }

    // Static site
    let content = warp::fs::dir(root_dir.to_string());
    let index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(format!("{}/index.html", root_dir)));
    let static_site = content.or(index);

    let api = task::task_rest_filters("api", database);

    // Combine routes
    let routes = api.or(static_site).recover(handle_rejection);

    info!(
        "Starting server http://127.0.0.1:{} from {}",
        port, root_dir
    );
    warp::serve(routes).run(([127, 0, 0, 1], port)).await;

    Ok(())
}

fn extract_body_data<D>(response: Response<Bytes>) -> std::io::Result<D>
where
    for<'de> D: serde::de::Deserialize<'de>,
    D: serde::de::DeserializeOwned,
{
    let body = from_utf8(response.body()).expect("Could not parse response body as UTF8.");
    let mut body: Value =
        serde_json::from_str(body).expect(&format!("Cannot parse JSON response body: {}", body));
    let data = body["data"].take();
    let data: D = serde_json::from_value(data)?;

    Ok(data)
}

// # Error handling

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

impl From<crate::Error> for warp::Rejection {
    fn from(other: crate::Error) -> Self {
        WebError::rejection("crate::error", format!("{}", other))
    }
}

async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, Infallible> {
    error!("Error: {:?}", err);

    // TODO: logging API?

    let message = match err.find::<WebError>() {
        Some(err) => err.typ.to_string(),
        None => "Unknown error".to_string(),
    };

    let result = json!({"errorMessage": message});
    let result = warp::reply::json(&result);

    Ok(warp::reply::with_status(
        result,
        warp::http::StatusCode::BAD_REQUEST,
    ))
}

// # API Response helpers

fn json_response<D: Serialize>(data: D) -> Result<Json, warp::Rejection> {
    let response = json!({ "data": data});
    Ok(warp::reply::json(&response))
}
