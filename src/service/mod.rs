use crate::err;
use crate::error::Result;
use crate::model::{Link, Prio, Status, Todo, CSV, ID};
use crate::repository::Repository;
use chrono::Local;

pub mod changeset;
pub mod filter;
pub use filter::*;

pub use self::changeset::Changeset;

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
        subject: String,
        description: String,
        tags: CSV<String>,
    ) -> Result<Todo> {
        let context = self.get_context().await?;

        let now = Local::now();
        let tmp = Todo::new(
            ID::new(0),
            now,
            status,
            prio,
            subject,
            description,
            tags,
            context,
            CSV::empty(),
        );
        let todo = self.repo.add_todo(tmp).await?;

        log::info!("Added todo: {:?}", todo);
        Ok(todo)
    }

    pub async fn remove_todo(&self, id: &ID) -> Result<()> {
        let _todo = self.repo.remove_todo(id).await?;
        log::info!("Removed todo with ID {}", id);
        Ok(())
    }

    pub async fn update_todo(&self, id: &ID, changeset: Changeset) -> Result<Todo> {
        let mut after = self.repo.get_todo(id).await?;
        changeset.apply(&mut after);

        self.repo.replace_todo(&after).await?;
        log::info!("Updated todo with ID {}", id);
        Ok(after)
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

        if cascade {
            log::info!(
                "Removing {} TODOs due to cascading remove of context {}",
                todos.len(),
                context
            );
        } else {
            log::info!(
                "Replacing {} TODOs that was linked to context being removed",
                todos.len()
            );
            for mut todo in todos {
                todo.context = None;
                self.repo.replace_todo(&todo).await?;
            }
        };

        // Remove context.
        self.repo.remove_context(context).await?;
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

// Links.
impl Service {
    pub async fn link(&self, id: ID, link: Link) -> Result<()> {
        if link.id() == id {
            return err!("cannot link to self");
        }

        let mut todo = self.get_todo(&id).await?;
        if todo.links.has_any(&[link]) {
            log::info!("Todo with ID {} already has link {}", id, link);
            return Ok(());
        }

        // Check that this doesn't have the other link with same ID.
        // That is, do not allow circular referencing.
        if let Some(other_link) = link.bi_directional(id) {
            if todo.links.has_any(&[other_link]) {
                return err!("cannot add circular references");
            }
        }

        // Check that the ID referenced in the link doesn't already have it,
        // and that it doesn't create a circular reference.
        let mut other = self.get_todo(&link.id()).await?;
        let circular = link.with_id(id);
        if other.links.has_any(&[circular]) {
            log::info!("Tried to add link that already exist on other part");
            return err!("link already exist on other part");
        }

        todo.links.push(link);
        let changeset = Changeset::default().with_links(todo.links);
        let changeset = match link {
            Link::BlockedBy(_) => changeset.with_status(Status::Blocked),
            _ => changeset,
        };
        self.update_todo(&id, changeset).await?;

        if let Some(other_link) = link.bi_directional(id) {
            log::info!("Adding counter link: {}", other_link);
            other.links.push(other_link);

            let changeset = Changeset::default().with_links(other.links);
            let changeset = match other_link {
                Link::BlockedBy(_) => changeset.with_status(Status::Blocked),
                _ => changeset,
            };

            self.update_todo(&other.id, changeset).await?;
        }

        log::info!("Added link {} to {}", link, id);
        Ok(())
    }

    pub async fn unlink(&self, _id: ID, _link: Link) -> Result<()> {
        todo!()
    }
}

#[cfg(test)]
mod tests;
