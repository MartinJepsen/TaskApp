use crate::database::Database;
use crate::model::task::{TaskMac, TaskPatch};

use super::json_response;
use serde_json::json;
use std::convert::Infallible;
use std::sync::Arc;
use warp::reply::Json;
use warp::Filter;

pub fn task_rest_filters(
    base_path: &'static str,
    database: Arc<Database>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let task_path = warp::path(base_path).and(warp::path("tasks")); // /api/tasks
    let common = with_db(database.clone());

    // List tasks (GET /api/tasks/)
    let list = task_path
        .and(warp::get())
        .and(warp::path::end())
        .and(common.clone())
        .and_then(task_list);

    // Get task (GET /api/tasks/:id)
    let get = task_path
        .and(warp::get())
        .and(common.clone())
        .and(warp::path::param())
        .and_then(task_get);

    // Create task (POST /api/tasks with body TaskPatch)
    let insert = task_path
        .and(warp::post())
        .and(common.clone())
        .and(warp::body::json())
        .and_then(task_insert);

    // Update task (PATCH /api/tasks with body TaskPatch and id)
    let update = task_path
        .and(warp::patch())
        .and(common.clone())
        .and(warp::path::param())
        .and(warp::body::json())
        .and_then(task_update);

    // Delete task (DELETE /api/tasks/:id)
    let delete = task_path
        .and(warp::delete())
        .and(common.clone())
        .and(warp::path::param())
        .and_then(task_delete);

    list.or(get).or(insert).or(update).or(delete)
}

/// List all tasks.
async fn task_list(database: Arc<Database>) -> Result<Json, warp::Rejection> {
    // FIXME: use error handling
    let tasks = TaskMac::list(&database).await.unwrap();
    json_response(tasks)
}

/// Get a task by id.
async fn task_get(database: Arc<Database>, id: i64) -> Result<Json, warp::Rejection> {
    let task = TaskMac::get(&database, id).await?;
    json_response(task)
}

/// Insert a new task.
async fn task_insert(database: Arc<Database>, data: TaskPatch) -> Result<Json, warp::Rejection> {
    let task = TaskMac::insert(&database, data).await?;
    json_response(task)
}

/// Delete a task by id.
async fn task_delete(database: Arc<Database>, id: i64) -> Result<Json, warp::Rejection> {
    TaskMac::delete(&database, id).await?;
    json_response(json!({}))
}

/// Update a task by id.
async fn task_update(
    database: Arc<Database>,
    id: i64,
    data: TaskPatch,
) -> Result<Json, warp::Rejection> {
    let task = TaskMac::update(&database, id, data).await?;
    json_response(task)
}

/// Extract the database from the request.
pub fn with_db(
    database: Arc<Database>,
) -> impl Filter<Extract = (Arc<Database>,), Error = Infallible> + Clone {
    warp::any().map(move || database.clone())
}

#[cfg(test)]
mod test {
    use crate::database::{create_and_connect, DbAddress};
    use crate::model::task::{TaskMac, TaskPatch, TaskStatus};
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

    #[tokio::test]
    async fn test_task_get() -> Result<()> {
        // # Setup
        let database = Arc::new(create_and_connect(DbAddress::Memory).await.unwrap());
        TaskMac::insert(
            &database,
            TaskPatch {
                name: Some("Hello world".to_string()),
                status: Some(TaskStatus::Open),
            },
        )
        .await
        .unwrap();

        let filters =
            task_rest_filters("api", database.clone()).recover(super::super::handle_rejection);

        let response = warp::test::request()
            .method("GET")
            .path("/api/tasks/1")
            .reply(&filters)
            .await;

        println!("{:#?}", response.body());
        assert_eq!(response.status(), StatusCode::OK);

        Ok(())
    }
}
