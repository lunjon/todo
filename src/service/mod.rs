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
        let todo = self.repo.get_todo(id).await?;
        if changeset.is_empty() {
            return Ok(todo);
        }

        // Handle transitions
        let mut load = false;
        if let Some(status) = &changeset.status {
            match status {
                Status::Started => {
                    load = true;
                    for link in todo.links.values() {
                        if let Link::BlockedBy(blocker) = link {
                            self.unlink_block(*blocker, *id).await?;
                        }
                    }
                }
                Status::Done => {
                    load = true;
                    for link in todo.links.values() {
                        if let Link::Blocks(blocked) = link {
                            self.unlink_block(*id, *blocked).await?;
                        }
                    }
                }
                _ => {}
            }
        }

        // If any links where changed we must reload the todo after.
        let mut todo = if load { self.get_todo(id).await? } else { todo };

        changeset.apply(&mut todo);

        self.repo.replace_todo(&todo).await?;
        log::info!("Updated todo with ID {}", id);

        Ok(todo)
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
    pub async fn link(&self, id: ID, link: Link) -> Result<Todo> {
        if link.id() == id {
            return err!("cannot link to self");
        }

        match link {
            Link::Blocks(blocked_id) => self.link_block(id, blocked_id).await,
            Link::BlockedBy(blocker_id) => self.link_block(blocker_id, id).await,
            link => self.link_uni(id, link).await,
        }
    }

    pub async fn unlink(&self, id: ID, link: Link) -> Result<Todo> {
        match link {
            Link::Blocks(blocked) => self.unlink_block(id, blocked).await,
            Link::BlockedBy(blocker) => self.unlink_block(blocker, id).await,
            link => self.unlink_uni(id, link).await,
        }
    }

    pub async fn unlink_block(&self, blocker: ID, blocked: ID) -> Result<Todo> {
        let blocks_link = Link::Blocks(blocked);
        let blocked_by_link = Link::BlockedBy(blocker);

        let mut blocker = self.get_todo(&blocker).await?;
        if !blocker.links.contains(&blocks_link) {
            return Ok(blocker);
        }

        blocker.links = blocker.links.remove(&blocks_link);
        self.repo.replace_todo(&blocker).await?;

        let mut blocked = self.get_todo(&blocked).await?;
        blocked.links = blocked.links.remove(&blocked_by_link);

        if !blocked
            .links
            .values()
            .iter()
            .any(|link| link.is_blocked_by())
        {
            blocked.status = Status::New;
        } else {
        }

        self.repo.replace_todo(&blocked).await?;
        Ok(blocker)
    }
    async fn link_block(&self, blocker: ID, blocked: ID) -> Result<Todo> {
        let blocks_link = Link::Blocks(blocked);
        let blocked_by_link = Link::BlockedBy(blocker);

        let mut blocker = self.get_todo(&blocker).await?;
        if blocker.links.contains(&blocks_link) {
            log::info!(
                "Todo with ID {} already has link {}",
                blocker.id,
                blocks_link
            );
            return Ok(blocker);
        }

        let circular = Link::BlockedBy(blocked);
        if blocker.links.contains(&circular) {
            return err!("circular link not allowed");
        }

        blocker.links.push_not_exists(blocks_link);
        self.repo.replace_todo(&blocker).await?;

        let mut blocked = self.get_todo(&blocked).await?;
        blocked.links.push_not_exists(blocked_by_link);
        blocked.status = Status::Blocked;
        self.repo.replace_todo(&blocked).await?;

        log::info!("Added link: {} blocks {}", blocker.id, blocked.id);
        Ok(blocker)
    }

    /// Adds a unilateral link to todo with `id`.
    async fn link_uni(&self, id: ID, link: Link) -> Result<Todo> {
        let mut todo = self.get_todo(&id).await?;
        if todo.links.contains(&link) {
            return Ok(todo);
        }

        todo.links.push_not_exists(link);
        self.repo.replace_todo(&todo).await?;
        Ok(todo)
    }

    async fn unlink_uni(&self, id: ID, link: Link) -> Result<Todo> {
        let mut todo = self.get_todo(&id).await?;
        todo.links = todo.links.remove(&link);
        self.repo.replace_todo(&todo).await?;
        Ok(todo)
    }
}

#[cfg(test)]
mod tests;
