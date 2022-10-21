use crate::err;
use crate::error::{map_sqlx_error, Error, Result};
use crate::model::{Action, Event, Prio, Status, Tags, Todo, ID};
use chrono::{DateTime, Local};
use sqlx::sqlite::{SqlitePool, SqliteRow};
use sqlx::Row;

pub struct Repository {
    pool: SqlitePool,
}

// For todos.
impl Repository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get_todo(&self, id: &ID) -> Result<Todo> {
        let result = sqlx::query("SELECT * FROM todos WHERE id = $1")
            .bind(id.to_string())
            .map(map_todo)
            .fetch_one(&self.pool)
            .await;

        match result {
            Ok(todo) => Ok(todo),
            Err(err) => match map_sqlx_error(err) {
                Error::NotFound(_) => Err(Error::NotFound(Some(id.to_string()))),
                error => Err(error),
            },
        }
    }

    pub async fn get_todos(&self) -> Result<Vec<Todo>> {
        let result = sqlx::query("SELECT * FROM todos")
            .map(map_todo)
            .fetch_all(&self.pool)
            .await;
        match result {
            Ok(todos) => Ok(todos),
            Err(err) => err!(err),
        }
    }

    pub async fn add_todo(&self, todo: Todo) -> Result<Todo> {
        let todo = sqlx::query(
            "INSERT INTO todos (created, title, status, prio, description, tags, context) VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, created, status, prio, title, description, tags, context",
        )
        .bind(todo.created.format("%Y-%m-%d %H:%M:%S %z").to_string())
        .bind(todo.title)
        .bind(todo.status.to_string())
        .bind(todo.prio.to_string())
        .bind(todo.description)
        .bind(todo.tags.to_string())
        .bind(todo.context)
        .map(map_todo)
        .fetch_one(&self.pool)
        .await?;

        Ok(todo)
    }

    pub async fn replace_todo(&self, todo: &Todo) -> Result<()> {
        sqlx::query(
            "REPLACE INTO todos (id, created, status, prio, title, description, tags, context)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        )
        .bind(todo.id.to_string())
        .bind(todo.created.format("%Y-%m-%d %H:%M:%S %z").to_string())
        .bind(todo.status.to_string())
        .bind(todo.prio.to_string())
        .bind(&todo.title)
        .bind(&todo.description)
        .bind(todo.tags.to_string())
        .bind(&todo.context)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn remove_todo(&self, id: &ID) -> Result<Todo> {
        let todo = sqlx::query(
            "DELETE FROM todos WHERE id = $1
            RETURNING id, created, status, prio, title, description, tags, context",
        )
        .bind(id.to_string())
        .map(map_todo)
        .fetch_one(&self.pool)
        .await?;
        Ok(todo)
    }
}

// For events.
impl Repository {
    pub async fn get_all_events(&self) -> Result<Vec<Event>> {
        let events = sqlx::query("SELECT * FROM events")
            .map(map_event)
            .fetch_all(&self.pool)
            .await?;
        Ok(events)
    }

    pub async fn add_event(&self, event: Event) -> Result<Event> {
        let json = serde_json::to_string(&event.kind)?;
        let id = sqlx::query(
            "INSERT INTO events (action, json, ts) VALUES ($1, $2, $3)
            RETURNING id",
        )
        .bind(event.action.to_string())
        .bind(json)
        .bind(event.timestamp)
        .map(|row| ID::new(row.get("id")))
        .fetch_one(&self.pool)
        .await?;

        let event = Event::new(id, event.action, event.kind, event.timestamp);
        Ok(event)
    }
}

// For contexts.
impl Repository {
    // Gets current context, if any.
    pub async fn get_context(&self) -> Result<Option<String>> {
        let context = sqlx::query("SELECT value FROM context")
            .map(|row: SqliteRow| {
                let context: Option<String> = row.get("value");
                context
            })
            .fetch_one(&self.pool)
            .await?;
        Ok(context)
    }

    // Gets all context values.
    pub async fn get_contexts(&self) -> Result<Vec<String>> {
        let contexts = sqlx::query("SELECT name FROM contexts")
            .map(|row: SqliteRow| {
                let name: String = row.get("name");
                name
            })
            .fetch_all(&self.pool)
            .await?;
        Ok(contexts)
    }

    // Sets the current context.
    pub async fn set_context(&self, context: &str) -> Result<()> {
        sqlx::query("UPDATE context SET value = $1 WHERE id = 1")
            .bind(context)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    // Unsets the current context.
    pub async fn unset_context(&self) -> Result<()> {
        sqlx::query("UPDATE context SET value = NULL WHERE id = 1")
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    // Adds a new context name.
    pub async fn add_context(&self, context: &str) -> Result<()> {
        sqlx::query("INSERT INTO contexts (name) VALUES ($1)")
            .bind(context)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    // Removes a context ny name.
    pub async fn remove_context(&self, context: &str) -> Result<()> {
        sqlx::query("DELETE FROM contexts WHERE name = $1")
            .bind(context)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

fn map_todo(row: SqliteRow) -> Todo {
    let created: String = row.get("created");
    let created: DateTime<Local> = created.parse().unwrap();

    let tags: String = row.get("tags");
    let status: String = row.get("status");
    let prio: String = row.get("prio");
    let prio = Prio::try_from(prio).unwrap();
    let context: Option<String> = row.get("context");

    Todo::new(
        ID::new(row.get("id")),
        created,
        Status::try_from(status).unwrap(),
        prio,
        row.get("title"),
        row.get("description"),
        Tags::try_from(tags).unwrap(),
        context,
    )
}

fn map_event(row: SqliteRow) -> Event {
    let id = ID::new(row.get("id"));

    let action: String = row.get("action");
    let action = Action::try_from(action).unwrap();

    let json: String = row.get("json");
    let kind = serde_json::from_str(&json).expect("deserialize event");

    let timestamp = row.get("ts");
    Event::new(id, action, kind, timestamp)
}
