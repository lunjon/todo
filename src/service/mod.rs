use crate::err;
use crate::error::Result;
use crate::model::{Action, Event, Kind, Prio, Status, Tags, Todo, ID};
use crate::repository::Repository;
use chrono::Local;

pub mod filter;
pub use filter::*;

pub struct Service {
    repo: Repository,
}

impl Service {
    pub fn new(todos: Repository) -> Self {
        Self { repo: todos }
    }
}

// Impl block for todos.
impl Service {
    /// Gets a todo by ID.
    pub async fn get_todo(&self, id: &ID) -> Result<Todo> {
        self.repo.get_todo(id).await
    }

    /// Lists available todos.
    pub async fn list_todos(&self, filter: Option<Filter>) -> Result<Vec<Todo>> {
        let todos = self.repo.get_todos().await?;
        let mut todos = match filter {
            Some(filter) => {
                let context = self.get_context().await?;
                filter.apply(todos, context)
            }
            None => todos,
        };

        todos.sort();
        Ok(todos)
    }

    /// Adds a new todo from the parameter and returns the created todo including an ID.
    pub async fn add_todo(
        &self,
        status: Status,
        prio: Prio,
        title: String,
        description: String,
        tags: Tags,
    ) -> Result<Todo> {
        let context = self.get_context().await?;

        let now = Local::now();
        let tmp = Todo::new(
            ID::new(0),
            now,
            status,
            prio,
            title,
            description,
            tags,
            context,
        );
        let todo = self.repo.add_todo(tmp).await?;

        self.add_todo_event(todo.clone()).await?;
        log::info!("Added todo: {:?}", todo);
        Ok(todo)
    }

    pub async fn remove_todo(&self, id: &ID) -> Result<()> {
        let todo = self.repo.remove_todo(id).await?;
        self.remove_todo_event(todo).await?;
        log::info!("Removed todo with ID {}", id);
        Ok(())
    }

    pub async fn update_todo(
        &self,
        id: &ID,
        title: Option<String>,
        status: Option<Status>,
        prio: Option<Prio>,
        description: Option<String>,
        context: Option<String>,
    ) -> Result<Todo> {
        let mut after = self.repo.get_todo(id).await?;
        let before = after.clone();

        if let Some(s) = title {
            after.title = s;
            log::info!("Service.update_todo: updating title");
        }
        if let Some(s) = status {
            after.status = s;
            log::info!("Service.update_todo: updating status");
        }
        if let Some(s) = prio {
            after.prio = s;
            log::info!("Service.update_todo: updating prio");
        }
        if let Some(s) = description {
            after.description = s;
            log::info!("Service.update_todo: updating description");
        }
        if let Some(s) = context {
            after.context = Some(s);
            log::info!("Service.update_todo: updating context");
        }

        self.repo.replace_todo(&after).await?;
        self.update_todo_event(before, after.clone()).await?;
        log::info!("Updated todo with ID {}", id);
        Ok(after)
    }
}

// Impl block for events.
impl Service {
    fn create_event(action: Action, kind: Kind) -> Event {
        Event::new(ID::new(0), action, kind, Local::now().timestamp())
    }

    pub async fn list_events(&self) -> Result<Vec<Event>> {
        self.repo.get_all_events().await
    }

    async fn add_todo_event(&self, todo: Todo) -> Result<()> {
        let event = Self::create_event(Action::Add, Kind::AddTodo(todo));
        self.repo.add_event(event).await?;
        log::info!("Registered event for: add todo");
        Ok(())
    }

    async fn update_todo_event(&self, before: Todo, after: Todo) -> Result<()> {
        let event = Self::create_event(Action::Update, Kind::UpdateTodo { before, after });
        self.repo.add_event(event).await?;
        log::info!("Registered event for: update todo");
        Ok(())
    }

    async fn remove_todo_event(&self, todo: Todo) -> Result<()> {
        let event = Self::create_event(Action::Remove, Kind::RemoveTodo(todo));
        self.repo.add_event(event).await?;
        log::info!("Registered event for: remove todo");
        Ok(())
    }

    async fn add_context_event(&self, context: &str) -> Result<()> {
        let event = Self::create_event(Action::Add, Kind::AddContext(context.to_string()));
        self.repo.add_event(event).await?;
        log::info!("Registered event for: add context");
        Ok(())
    }

    async fn set_context_event(&self, before: &str, after: &str) -> Result<()> {
        let event = Self::create_event(
            Action::Update,
            Kind::SetContext {
                before: before.to_string(),
                after: after.to_string(),
            },
        );
        self.repo.add_event(event).await?;
        log::info!("Registered event for: set context");
        Ok(())
    }

    async fn remove_context_event(&self, context: &str, todos: Vec<Todo>) -> Result<()> {
        let event = Self::create_event(
            Action::Remove,
            Kind::RemoveContext(context.to_string(), todos),
        );
        self.repo.add_event(event).await?;
        log::info!("Registered event for: remove context");
        Ok(())
    }
}

// Context.
impl Service {
    // TODO: validate context string. It should be a short, single word string
    // with only characters from [a-z].
    pub async fn add_context(&self, context: &str) -> Result<()> {
        let context = self.validate_context_name(context)?;
        let contexts = self.list_contexts().await?;
        if contexts.iter().any(|name| name == &context) {
            return err!("context name already exists: {}", context);
        }

        self.repo.add_context(&context).await?;
        self.add_context_event(&context).await?;
        log::info!("Added new context: {context}");
        Ok(())
    }

    pub async fn set_context(&self, context: &str) -> Result<()> {
        let context = self.validate_context_name(context)?;

        let contexts = self.list_contexts().await?;
        if !contexts.iter().any(|name| name == &context) {
            return err!("context name not found: {}", context);
        }

        let current = self
            .repo
            .get_context()
            .await?
            .unwrap_or_else(|| String::from(""));
        if current == context {
            log::info!("Context already set to {context}");
            return Ok(());
        }

        self.repo.set_context(&context).await?;
        self.set_context_event(&current, &context).await?;
        log::info!("Changed context to: {context}");
        Ok(())
    }

    pub async fn unset_context(&self) -> Result<()> {
        self.repo.unset_context().await?;
        log::info!("Current context was unset");
        Ok(())
    }

    pub async fn get_context(&self) -> Result<Option<String>> {
        self.repo.get_context().await
    }

    pub async fn list_contexts(&self) -> Result<Vec<String>> {
        let contexts = self.repo.get_contexts().await?;
        log::info!("Listed {} context(s)", contexts.len());
        Ok(contexts)
    }

    pub async fn remove_context(&self, context: &str, cascade: bool) -> Result<()> {
        let current = self.get_context().await?;
        let todos = self.list_todos(Some(Filter::default())).await?;
        if let Some(ctx) = current {
            if ctx == context {
                log::info!("Removing current context");
                self.unset_context().await?;
            }
        }

        log::info!(
            "Found {} TODOs linked to context {} being removed",
            todos.len(),
            context
        );

        let removed = if cascade {
            log::info!(
                "Removing {} TODOs due to cascading remove of context {}",
                todos.len(),
                context
            );
            todos
        } else {
            log::info!(
                "Replacing {} TODOs that was linked to context being removed",
                todos.len()
            );
            for mut todo in todos {
                todo.context = None;
                self.repo.replace_todo(&todo).await?;
            }
            Vec::new()
        };

        // Remove context.
        self.repo.remove_context(context).await?;
        self.remove_context_event(context, removed).await?;
        log::info!("Removed context: {context}");
        Ok(())
    }

    fn validate_context_name(&self, context: &str) -> Result<String> {
        let s = context.trim();
        if s.is_empty() {
            err!("invalid context name: {}", context)
        } else if s.len() < 2 {
            err!("invalid context name: length less than 2")
        } else if s.len() > 10 {
            err!("invalid context name: length greater than 10")
        } else {
            Ok(s.to_string())
        }
    }
}

#[cfg(test)]
mod tests;
