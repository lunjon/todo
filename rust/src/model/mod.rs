use serde::{Deserialize, Serialize};
use std::fmt;

pub mod event;
pub mod prio;
pub mod status;
pub mod tags;
pub mod todo;

pub use self::todo::*;
pub use event::*;
pub use prio::*;
pub use status::*;
pub use tags::*;

/// An identifier for Todos for simple referencing.
#[derive(Clone, Debug, Deserialize, Eq, Serialize)]
pub struct ID(u16);

impl ID {
    pub fn new(id: u16) -> Self {
        Self(id)
    }
}

impl PartialEq for ID {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl fmt::Display for ID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
