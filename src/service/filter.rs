use crate::model::{Status, Todo};

/// Used to filter based on status.
pub enum StatusFilter {
    /// Include any status.
    Any,
    /// Include all statuses != done.
    Relevant,
    /// Only the given status.
    Status(Status),
}

/// Used specifically to filter based on context.
pub enum ContextFilter {
    /// Context doesn't matter.
    Any,
    /// Only todos that doesn't have any context.
    None,
    /// Use the current context.
    Current,
    // Context with a given name.
    Name(String),
}

/// Filter is used when listing todos, e.g. by status.
/// The default filter includes only statuses != done
/// and that has the current (in any) context.
pub struct Filter {
    /// Include only todos with this status.
    /// None means all statuses.
    status: StatusFilter,
    /// Include only todos with the given context.
    context: ContextFilter,
    /// Todos with at least one of the tags.
    tags: Option<Vec<String>>,
}

impl Default for Filter {
    fn default() -> Self {
        Self {
            status: StatusFilter::Relevant,
            context: ContextFilter::Current,
            tags: None,
        }
    }
}

impl Filter {
    pub fn status(mut self, f: StatusFilter) -> Self {
        self.status = f;
        self
    }

    pub fn context(mut self, f: ContextFilter) -> Self {
        self.context = f;
        self
    }

    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.tags = Some(tags);
        self
    }

    pub fn apply(&self, todos: Vec<Todo>, current_context: Option<String>) -> Vec<Todo> {
        todos
            .into_iter()
            .filter(|todo| match &self.status {
                StatusFilter::Any => true,
                StatusFilter::Relevant => todo.status != Status::Done,
                StatusFilter::Status(status) => &todo.status == status,
            })
            .filter(|todo| match &self.context {
                ContextFilter::Any => true,
                ContextFilter::None => todo.context.is_none(),
                ContextFilter::Current => match &current_context {
                    Some(curr) => match &todo.context {
                        Some(ctx) => ctx == curr,
                        None => false,
                    },
                    None => true,
                },
                ContextFilter::Name(ctx) => match &todo.context {
                    Some(name) => ctx == name,
                    None => false,
                },
            })
            .filter(|todo| match &self.tags {
                Some(tags) => todo.tags.has_any(tags),
                None => true,
            })
            .collect()
    }
}
