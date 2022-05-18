use crate::model::event::Event;
use crate::model::Todo;

pub mod card;
pub mod table;

pub use card::Card;
pub use table::TableFormatter;

/// Formatter is used to format items in a user-friendly way.
pub trait Formatter {
    /// Format a slice of todos nicely.
    fn todos(&self, todos: &[Todo]) -> String;
    /// Format a single Todo.
    fn todo(&self, todo: &Todo) -> String;
    /// Format a slice of events.
    fn events(&self, events: &[Event]) -> String;
}
