use crate::database::Database;
use crate::model::TaskMac;

use std::sync::Arc;
use std::convert::Infallible;
use serde_json::json;
use warp::reply::Json;
use warp::Filter;

pub fn task_rest_filters (
    base_path: &'static str,
    database: Arc<Database>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let task_path = warp::path(base_path).and(warp::path("tasks"));  // /api/tasks
    let common = with_db(database.clone());

    // List tasks (GET /api/tasks)
    let list = task_path
        .and(warp::get())
        .and(warp::path::end())
        .and(common.clone())
        .and_then(task_list);

    list
}

async fn task_list(database: Arc<Database>) -> Result<Json, warp::Rejection> {
    // FIXME: use error handling
    let tasks = TaskMac::list(&database).await.unwrap();

    let response = json!({"data": tasks});
    Ok(warp::reply::json(&response))
}


pub fn with_db(database: Arc<Database>) -> impl Filter<Extract = (Arc<Database>,), Error = Infallible> + Clone {
    warp::any().map(move || database.clone())
}


#[cfg(test)]
mod test {
    use crate::database::{create_and_connect, DbAddress};
    use std::io::Result;

    use super::*;
    use warp::http::StatusCode;

    #[tokio::test]
    async fn test_task_list() -> Result<()> {
        let database = Arc::new(create_and_connect(DbAddress::Memory).await.unwrap());
        let filters = task_rest_filters("api", database.clone());

        let resp = warp::test::request()
            .method("GET")
            .path("/api/tasks")
            .reply(&filters)
            .await;

        assert_eq!(resp.status(), StatusCode::OK);

        Ok(())
    }
}