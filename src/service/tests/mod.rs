use super::*;
use crate::error::Result;
use crate::model::{Prio, Status, Todo, ID};
use crate::repository::Repository;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::str::FromStr;

mod add;
mod context;
mod filter;
mod list;
mod remove;
mod update;

struct Fixture {
    service: Service,
    todo_new: Todo,
    todo_done: Todo,
    todo_started: Todo,
    test_context_name: String,
}

impl Fixture {
    // Setup an in-memory sqlite database and run migrations.
    async fn setup() -> Result<Self> {
        let connection_options =
            SqliteConnectOptions::from_str("sqlite::memory:")?.read_only(false);
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(connection_options)
            .await?;
        sqlx::migrate!().run(&pool).await.expect("run migrations");

        let test_context_name = String::from("test");

        let service = Service::new(Repository::new(pool));
        let (todo_new, todo_done, todo_started, _) = tokio::join!(
            service.add_todo(
                Status::New,
                Prio::Normal,
                "New subject".to_string(),
                "Description.".to_string(),
                CSV::new(vec!["new".to_string()]),
            ),
            service.add_todo(
                Status::Done,
                Prio::Normal,
                "Done subject".to_string(),
                "Description.".to_string(),
                CSV::new(vec!["done".to_string()]),
            ),
            service.add_todo(
                Status::Started,
                Prio::Normal,
                "Started something".to_string(),
                "Description".to_string(),
                CSV::new(vec!["done".to_string()]),
            ),
            service.add_context(&test_context_name),
        );

        Ok(Self {
            service,
            test_context_name,
            todo_new: todo_new?,
            todo_done: todo_done?,
            todo_started: todo_started?,
        })
    }

    async fn todo_exists(&self, id: &ID) -> Result<bool> {
        match self.service.repo.get_todo(id).await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    async fn create_todo(&self) -> Result<Todo> {
        let todo = self
            .service
            .add_todo(
                Status::New,
                Prio::Normal,
                "Subject".to_string(),
                "Description".to_string(),
                CSV::default(),
            )
            .await?;
        Ok(todo)
    }

    async fn todo_count(&self) -> Result<usize> {
        Ok(self.service.list_todos(None).await?.len())
    }
}
