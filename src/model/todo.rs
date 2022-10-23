use super::{Link, Prio, Status, CSV, ID};
use chrono::{DateTime, Local};
use std::cmp::Ordering;

/// Todo is the central model for this projet and represents
/// a unit of work that has a status (current state), priority, etc.
#[derive(Clone, Debug)]
pub struct Todo {
    /// ID of this todo.
    pub id: ID,
    /// A datetime string when this todo was created.
    pub created: DateTime<Local>,
    /// Current status.
    pub status: Status,
    /// The priority of this todo.
    pub prio: Prio,
    // Subject is a short summary of this todo.
    pub subject: String,
    /// Description can contain more details about this todo.
    pub description: String,
    /// The context, if any, that this todo belongs to.
    /// Typical values are `work` and `home`, or perhaps a project.
    pub context: Option<String>,
    /// Linked todos.
    pub links: CSV<Link>,
    /// Tags can include certain attributes for a todo.
    pub tags: CSV<String>,
}

impl Todo {
    // Creates a new Todo from the parameters.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: ID,
        created: DateTime<Local>,
        status: Status,
        prio: Prio,
        subject: String,
        description: String,
        tags: CSV<String>,
        context: Option<String>,
        links: CSV<Link>,
    ) -> Self {
        Self {
            id,
            created,
            status,
            prio,
            subject,
            description,
            tags,
            context,
            links,
        }
    }

    /// Returns true if the status of this todo is done.
    pub fn is_done(&self) -> bool {
        matches!(self.status, Status::Done)
    }
}

impl PartialEq for Todo {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Todo {}

impl PartialOrd for Todo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Ordering of todos is based on fields in order:
///     prio > created
///
/// Note that Ordering::Less means it ends up before other
/// values when sorting.
impl Ord for Todo {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.prio.cmp(&other.prio) {
            Ordering::Equal => self.created.cmp(&other.created),
            o => o,
        }
    }
}
