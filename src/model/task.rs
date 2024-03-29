use sqlx::{FromRow};
use crate::database::{Database};


/// Task model.
#[derive(Debug, Default, FromRow, Clone)]
pub struct Task {
    pub id: i64,
    pub name: String,
}

/// Patch type for creating or updating a task.
#[derive(Debug, Default, Clone)]
pub struct TaskPatch {
    pub name: String,
}


/// Task model access controller
pub struct TaskMac;

impl TaskMac {
    pub async fn insert(db: &Database, data: TaskPatch) -> Result<Task, sqlx::Error> {
        let query = "INSERT INTO tasks (name) VALUES (?) RETURNING id, name";
        let response = sqlx::query_as::<_, Task>(query)
            .bind(data.name);
        let task = response.fetch_one(db).await?;
        Ok(task)
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
        assert_eq!(task.name, "Hello world");
        Ok(())
    }
}