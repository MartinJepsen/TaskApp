use sqlx::{FromRow, types::chrono::{NaiveDateTime, DateTime, Utc}};
use crate::database::{Database};


/// Task model.
#[derive(Debug, Default, FromRow, Clone, PartialEq)]
pub struct Task {
    pub id: i64,
    pub name: String,
    pub creation_time: NaiveDateTime
}

/// Patch type for creating or updating a task.
#[derive(Debug, Default, Clone)]
pub struct TaskPatch {
    pub name: String,
}


/// Task model access controller.
pub struct TaskMac;

impl TaskMac {
    /// Insert a new task into the database.
    pub async fn insert(db: &Database, data: TaskPatch) -> Result<Task, sqlx::Error> {
        let query = "INSERT INTO tasks (name, creation_time) VALUES (?, strftime('%s', ?)) RETURNING id, name, creation_time";
        let response = sqlx::query_as::<_, Task>(query)
            .bind(&data.name)
            .bind(Utc::now().naive_utc());
            
        let task = response.fetch_one(db).await?;
        Ok(task)
    }

    /// Get a task from the database by id.
    pub async fn get(db: &Database, id: i64) -> Result<Task, sqlx::Error> {
        let query = "SELECT id, name, creation_time FROM tasks WHERE id = ?";
        let response = sqlx::query_as::<_, Task>(query)
            .bind(id);
        let task = response.fetch_one(db).await?;
        Ok(task)
    }

    /// Update a task in the database.
    pub async fn update(db: &Database, id: i64, data: TaskPatch) -> Result<Task, sqlx::Error> {
        let query = "UPDATE tasks SET name = ? WHERE id = ? RETURNING id, name";
        let response = sqlx::query_as::<_, Task>(query)
            .bind(data.name)
            .bind(id);
        let task = response.fetch_one(db).await?;
        Ok(task)
    }

    /// Delete a task from the database.
    pub async fn delete(db: &Database, id: i64) -> Result<(), sqlx::Error> {
        let query = "DELETE FROM tasks WHERE id = ?";
        sqlx::query(query).bind(id).execute(db).await?;
        Ok(())
    }

    /// List all tasks from the database.	
    pub async fn list(db: &Database) -> Result<Vec<Task>, sqlx::Error> {
        let query = "SELECT id, name, creation_time FROM tasks";
        let response = sqlx::query_as::<_, Task>(query);
        let tasks = response.fetch_all(db).await?;
        Ok(tasks)
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::database::{DbAddress, create_and_connect, create_schema};
    
    #[tokio::test]
    async fn test_insert() -> Result<(), sqlx::Error> {
        let db = create_and_connect(DbAddress::Memory).await?;
        create_schema(&db).await?;

        let task_fixture = TaskPatch { name: "Hello world".to_string() };

        let task = TaskMac::insert(&db, task_fixture).await?;
        println!("{:?}", task);
        assert_eq!(task.name, "Hello world");
        assert_eq!(task.id, 1);
        Ok(())
    }


    #[tokio::test]
    async fn test_get() -> Result<(), sqlx::Error> {
        // # Fixture
        let db = create_and_connect(DbAddress::Memory).await?;
        create_schema(&db).await?;
        let task_fixture = TaskPatch { name: "Hello world".to_string() };

        // # Action
        let inserted_task = TaskMac::insert(&db, task_fixture).await?;

        // # Check
        let retreived_task = TaskMac::get(&db, inserted_task.id).await?;
        assert_eq!(inserted_task, retreived_task);
        Ok(())
    }

    #[tokio::test]
    async fn test_list() -> Result<(), sqlx::Error> {
        let db = create_and_connect(DbAddress::Memory).await?;
        create_schema(&db).await?;

        let task_fixture = vec![TaskPatch { name: "One".to_string() }, TaskPatch { name: "Two".to_string() }];
        let mut inserted_tasks: Vec<Task> = Vec::new();
        for task in task_fixture {
            inserted_tasks.push(TaskMac::insert(&db, task).await?);
        }

        let tasks = TaskMac::list(&db).await?;
        assert_eq!(tasks, inserted_tasks);
        Ok(())
    }
}