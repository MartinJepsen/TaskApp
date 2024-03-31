use crate::database::Database;
use log::warn;
use serde::{Deserialize, Serialize};
use sqlx::{
    types::chrono::{DateTime, Utc},
    FromRow,
};

/// Task model.
#[derive(Debug, Default, FromRow, Clone, PartialEq, Serialize)]
pub struct Task {
    pub id: i64,
    pub name: String,
    pub status: TaskStatus,
    pub creation_time: DateTime<Utc>,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "task_status_enum")]
#[sqlx(rename_all = "lowercase")]
pub enum TaskStatus {
    #[default]
    Open,
    Closed,
}

/// Patch type for creating or updating a task.
#[derive(Debug, Default, Clone, Deserialize)]
pub struct TaskPatch {
    pub name: Option<String>,
    pub status: Option<TaskStatus>,
}

/// Task model access controller.
pub struct TaskMac;

impl TaskMac {
    const TABLE_NAME: &'static str = "tasks";
    const COLUMNS: &'static [&'static str] = &["id", "name", "status", "creation_time"];
    const INSERT_SQL: &'static str = r#"INSERT INTO tasks (
        name, status, creation_time
    ) VALUES (
        ?,
        ?,
        strftime('%s', ?)
    ) RETURNING id, name, status, creation_time"#;
    const GET_SQL: &'static str = r"SELECT id, name, status, creation_time FROM tasks WHERE id = ?";
    const DELETE_SQL: &'static str = "DELETE FROM tasks WHERE id = ?";
    const LIST_SQL: &'static str = "SELECT id, name, status, creation_time FROM tasks";

    /// Insert a new task into the database.
    pub async fn insert(db: &Database, data: TaskPatch) -> Result<Task, crate::Error> {
        // let query = format!(
        //     "INSERT INTO {0} (name, status, creation_time) VALUES (?, ?, strftime('%s', ?)) RETURNING {1}",
        //     Self::TABLE_NAME,
        //     Self::COLUMNS.join(", ")
        // );

        // let task_name = &data.name.unwrap_or_else(||{ warn!("Got empty task name. Defaulting to \"untitled\"."); "untitled".to_string()});
        let task_status = &data.status.unwrap_or(TaskStatus::Open);

        let response = sqlx::query_as::<_, Task>(Self::INSERT_SQL)
            .bind(&data.name)
            .bind(&task_status)
            .bind(Utc::now().naive_utc());

        let task = response.fetch_one(db).await?;
        Ok(task)
    }

    /// Get a task from the database by id.
    pub async fn get(db: &Database, id: i64) -> Result<Task, crate::Error> {
        let response = sqlx::query_as::<_, Task>(Self::GET_SQL).bind(id);
        let task = response.fetch_one(db).await?;
        Ok(task)
    }

    /// Update a task in the database.
    pub async fn update(db: &Database, id: i64, data: TaskPatch) -> Result<Task, crate::Error> {
        let mut query = format!("UPDATE {0} SET ", Self::TABLE_NAME);
        let mut set_statements = Vec::new();

        // Get fields to update
        if data.name.is_some() {
            set_statements.push("name = ?");
        }
        if data.status.is_some() {
            set_statements.push("status = ?");
        }

        // Early return if nothing to update
        if set_statements.is_empty() {
            warn!("No fields to update for task with id {}", id);
            return TaskMac::get(db, id).await;
        }

        // Add SET clause
        query.push_str(&set_statements.join(", "));
        // Add WHERE clause
        query.push_str(&format!(
            " WHERE id = ? RETURNING {0}",
            Self::COLUMNS.join(", ")
        ));

        let mut response = sqlx::query_as::<_, Task>(&query);

        // Add bindings
        if let Some(task_name) = &data.name {
            response = response.bind(task_name);
        }
        if let Some(task_status) = &data.status {
            response = response.bind(task_status);
        }
        response = response.bind(id);

        let task = response.fetch_one(db).await?;
        Ok(task)
    }

    /// Delete a task from the database.
    pub async fn delete(db: &Database, id: i64) -> Result<(), crate::Error> {
        sqlx::query(Self::DELETE_SQL).bind(id).execute(db).await?;
        Ok(())
    }

    /// List all tasks from the database.
    pub async fn list(db: &Database) -> Result<Vec<Task>, crate::Error> {
        let response = sqlx::query_as::<_, Task>(Self::LIST_SQL);
        let tasks = response.fetch_all(db).await?;
        Ok(tasks)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::database::{create_and_connect, create_schema, DbAddress};
    use crate::model::task::TaskStatus;

    /// Test insertion of a new task
    #[tokio::test]
    async fn test_insert() -> Result<(), crate::Error> {
        let db = create_and_connect(DbAddress::Memory).await?;
        create_schema(&db).await?;

        let task_fixture = TaskPatch {
            name: Some("Hello world".to_string()),
            status: None,
        };

        let task = TaskMac::insert(&db, task_fixture).await?;
        println!("{:?}", task);
        assert_eq!(task.name, "Hello world");
        assert_eq!(task.id, 1);
        Ok(())
    }

    /// Test retreival of a task by id
    #[tokio::test]
    async fn test_get() -> Result<(), crate::Error> {
        // # Setup
        let db = create_and_connect(DbAddress::Memory).await?;
        create_schema(&db).await?;

        // # Fixture
        let task_fixture = TaskPatch {
            name: Some("Hello world".to_string()),
            status: Some(TaskStatus::Open),
        };

        // # Action
        let inserted_task = TaskMac::insert(&db, task_fixture).await?;

        // # Check
        let retreived_task = TaskMac::get(&db, inserted_task.id).await?;
        assert_eq!(inserted_task, retreived_task);
        Ok(())
    }

    /// Test update of a task name by id.
    #[tokio::test]
    async fn test_update_name() -> Result<(), crate::Error> {
        // # Setup
        let db = create_and_connect(DbAddress::Memory).await?;
        create_schema(&db).await?;

        // # Fixture
        let task_fixture = TaskPatch {
            name: Some("Hello world".to_string()),
            status: Some(TaskStatus::Open),
        };
        let inserted_task = TaskMac::insert(&db, task_fixture).await?;

        // # Action
        let updated_task = TaskMac::update(
            &db,
            inserted_task.id,
            TaskPatch {
                name: Some("Updated".to_string()),
                status: None,
            },
        )
        .await?;

        // # Check
        assert_eq!(updated_task.name, "Updated");
        assert_eq!(inserted_task.id, updated_task.id);
        assert_eq!(inserted_task.status, updated_task.status);
        Ok(())
    }

    /// Test update of a task where nothing has changed. Should return the same task.
    #[tokio::test]
    async fn test_update_none() -> Result<(), crate::Error> {
        // # Setup
        let db = create_and_connect(DbAddress::Memory).await?;
        create_schema(&db).await?;

        // # Fixture
        let task_fixture = TaskPatch {
            name: Some("Hello world".to_string()),
            status: Some(TaskStatus::Open),
        };
        let inserted_task = TaskMac::insert(&db, task_fixture).await?;

        // # Action
        let updated_task = TaskMac::update(
            &db,
            inserted_task.id,
            TaskPatch {
                name: None,
                status: None,
            },
        )
        .await?;

        // # Check
        assert_eq!(updated_task, inserted_task);
        Ok(())
    }

    /// Test update of a task status.
    #[tokio::test]
    async fn test_update_status() -> Result<(), crate::Error> {
        // # Setup
        let db = create_and_connect(DbAddress::Memory).await?;
        create_schema(&db).await?;

        // # Fixture
        let task_fixture = TaskPatch {
            name: Some("Hello world".to_string()),
            status: Some(TaskStatus::Open),
        };
        let inserted_task = TaskMac::insert(&db, task_fixture).await?;

        // # Action
        let updated_task = TaskMac::update(
            &db,
            inserted_task.id,
            TaskPatch {
                name: None,
                status: Some(TaskStatus::Closed),
            },
        )
        .await?;

        // # Check
        assert_eq!(updated_task.name, "Hello world");
        assert_eq!(inserted_task.id, updated_task.id);
        assert_eq!(TaskStatus::Closed, updated_task.status);
        Ok(())
    }

    /// Test listing all tasks.
    #[tokio::test]
    async fn test_list() -> Result<(), crate::Error> {
        // # Setup
        let db = create_and_connect(DbAddress::Memory).await?;
        create_schema(&db).await?;

        // # Fixture
        let task_fixture = vec![
            TaskPatch {
                name: Some("One".to_string()),
                status: Some(TaskStatus::Open),
            },
            TaskPatch {
                name: Some("Two".to_string()),
                status: Some(TaskStatus::Closed),
            },
        ];

        // # Action
        let mut inserted_tasks: Vec<Task> = Vec::new();
        for task in task_fixture {
            inserted_tasks.push(TaskMac::insert(&db, task).await?);
        }

        // # Check
        let tasks = TaskMac::list(&db).await?;
        assert_eq!(tasks, inserted_tasks);
        Ok(())
    }
}
