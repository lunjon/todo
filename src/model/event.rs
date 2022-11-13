use super::{Todo, ID};
use crate::err;
use crate::error::Error;
use crate::style::{Color, StyleDisplay, Styler};
use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum Action {
    Add,
    Update,
    Remove,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Action::Add => write!(f, "add"),
            Action::Update => write!(f, "update"),
            Action::Remove => write!(f, "remove"),
        }
    }
}

impl StyleDisplay for Action {
    fn styler(&self) -> crate::style::Styler {
        let styler = Styler::default();
        match self {
            Action::Add => styler.fg(Color::Green),
            Action::Update => styler.fg(Color::Cyan),
            Action::Remove => styler.fg(Color::Red),
        }
    }
}

impl TryFrom<String> for Action {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "add" => Ok(Self::Add),
            "update" => Ok(Self::Update),
            "remove" => Ok(Self::Remove),
            _ => err!("invalid action: {}", value),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub enum Kind {
    AddTodo(Todo),
    UpdateTodo { before: Todo, after: Todo },
    RemoveTodo(Todo),
    AddContext(String),
    RemoveContext(String, Vec<Todo>),
    SetContext { before: String, after: String },
}

use Kind::*;

impl Kind {
    pub fn type_str(&self) -> String {
        match self {
            AddTodo(_) => "add_todo".to_string(),
            UpdateTodo {
                before: _,
                after: _,
            } => "update_todo".to_string(),
            RemoveTodo(_) => "remove_todo".to_string(),
            AddContext(_) => "add_context".to_string(),
            RemoveContext(_, _) => "remove_context".to_string(),
            SetContext {
                before: _,
                after: _,
            } => "set_context".to_string(),
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Kind::AddTodo(todo) => write!(f, "Added todo with ID {}", todo.id),
            Kind::UpdateTodo { before, after } => {
                let mut changes: Vec<String> = Vec::new();
                if before.status != after.status {
                    changes.push(format!("status: '{}' -> '{}'", before.status, after.status))
                }
                if before.prio != after.prio {
                    changes.push(format!("prio: '{}' -> '{}'", before.prio, after.prio))
                }
                if before.subject != after.subject {
                    changes.push(format!(
                        "subject: '{}' -> '{}'",
                        before.subject, after.subject
                    ))
                }
                if before.description != after.description {
                    changes.push("description: <...>".to_string());
                }
                if before.context != after.context {
                    if before.context.is_some() && after.context.is_some() {
                        changes.push(format!(
                            "context: '{}' -> '{}'",
                            before.context.as_ref().unwrap(),
                            after.context.as_ref().unwrap()
                        ));
                    } else if before.context.is_some() {
                        changes.push(format!(
                            "context: removed from '{}'",
                            before.context.as_ref().unwrap()
                        ));
                    } else if after.context.is_some() {
                        changes.push(format!(
                            "context: set to '{}'",
                            after.context.as_ref().unwrap()
                        ));
                    }
                }

                if changes.is_empty() {
                    write!(f, "Updated todo: no changes")
                } else {
                    write!(f, "Updated todo: {}", changes.join(", "))
                }
            }
            Kind::RemoveTodo(todo) => write!(f, "Removed todo with ID {}", todo.id),
            Kind::AddContext(context) => write!(f, "Added new context with name '{}'", context),
            Kind::RemoveContext(context, todos) => write!(
                f,
                "Removed context with name '{}' and {} associated todos",
                context,
                todos.len()
            ),
            Kind::SetContext { before, after } => {
                if before.is_empty() {
                    write!(f, "Context set to '{}'", after)
                } else if after.is_empty() {
                    write!(f, "Unset context from '{}'", before)
                } else {
                    write!(f, "Changed context from '{}' to '{}'", before, after)
                }
            }
        }
    }
}

impl StyleDisplay for Kind {
    fn styler(&self) -> Styler {
        Styler::default()
    }
}

pub struct Event {
    pub id: ID,
    pub action: Action,
    pub kind: Kind,
    pub timestamp: i64,
}

impl Event {
    pub fn new(id: ID, action: Action, kind: Kind, timestamp: i64) -> Self {
        Self {
            id,
            action,
            kind,
            timestamp,
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -- {}", self.id, self.kind)
    }
}
