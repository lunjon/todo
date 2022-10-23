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
    svc: Service,
    // Todos added for testing: todo_<status>
    todo_new: Todo,
    todo_done: Todo,
    todo_started: Todo,
    todo_blocked: Todo, // Blocked by started
    /// Name of an added context
    ctx: String,
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

        // Setup initial data
        let (todo_new, todo_done, todo_started, todo_blocked, _) = tokio::join!(
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
            service.add_todo(
                Status::Blocked,
                Prio::Normal,
                "Blocked by started".to_string(),
                "Just blocked".to_string(),
                CSV::default(),
            ),
            service.add_context(&test_context_name),
        );

        let todo_new = todo_new?;
        let todo_done = todo_done?;
        let todo_started = todo_started?;
        let todo_blocked = todo_blocked?;

        // Add link to blocked
        let link = Link::Blocks(todo_blocked.id);
        service.link(todo_started.id, link).await?;
        let todo_blocked = service.get_todo(&todo_blocked.id).await?;

        Ok(Self {
            svc: service,
            ctx: test_context_name,
            todo_new,
            todo_done,
            todo_started,
            todo_blocked,
        })
    }

    async fn todo_exists(&self, id: &ID) -> Result<bool> {
        match self.svc.repo.get_todo(id).await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    async fn create_todo(&self) -> Result<Todo> {
        let todo = self
            .svc
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
        Ok(self.svc.list_todos(None).await?.len())
    }
}
