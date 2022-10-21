use core::fmt;

use super::Formatter;
use crate::{
    model::{Event, Todo},
    style::{StyleDisplay, Styler},
};
use chrono::NaiveDateTime;

mod col;
#[cfg(test)]
mod tests;

use col::Col;

const ID_COL_WIDTH: usize = 3;
const PRIO_COL_WIDTH: usize = 10;
const STATUS_COL_WIDTH: usize = 8;
const CTX_COL_WIDTH: usize = 12;
const TITLE_COL_WIDTH: usize = 50;
const ACTION_COL_WIDTH: usize = 8;
const TIMESTAMP_COL_WIDTH: usize = 20;
const DETAILS_COL_WIDTH: usize = 50;

/// Specifies how to align the content.
pub enum Align {
    Left,
    Center,
}

// Pad `s` to fill out the given `size`.
fn pad(s: &str, size: usize, align: &Align) -> String {
    let d = size - s.len();
    if d == 0 {
        return s.to_string();
    }

    match align {
        Align::Left => format!("{}{}", s, " ".repeat(d)),
        Align::Center => {
            let n = d / 2;
            let (left, right) = if n == 0 {
                (d, 0)
            } else if n * 2 + s.len() > size {
                (n - 1, n)
            } else if n * 2 + s.len() < size {
                (n, n + 1)
            } else {
                (n, n)
            };
            format!("{}{}{}", " ".repeat(left), s, " ".repeat(right))
        }
    }
}

// Truncate `s` to a maximum size of `size`.
fn truncate(s: &str, size: usize) -> String {
    if s.len() < size {
        s.to_string()
    } else {
        let s = s[0..size - 4].to_string();
        format!("{s} ...")
    }
}

/// Formats a list of todos into a table.
pub struct TableFormatter {}

impl TableFormatter {
    pub fn new(_color: bool) -> Self {
        Self {}
    }

    fn todo_table_header(&self) -> String {
        let id = Header::from(" ID");
        let prio = Header::from("Priority");
        let status = Header::from("Status");
        let ctx = Header::from("Context");
        let title = Header::from("Title");

        let header = vec![
            Col::new(ID_COL_WIDTH, &id, Align::Left),
            Col::new(PRIO_COL_WIDTH, &prio, Align::Left),
            Col::new(STATUS_COL_WIDTH, &status, Align::Left),
            Col::new(CTX_COL_WIDTH, &ctx, Align::Left),
            Col::new(TITLE_COL_WIDTH, &title, Align::Left),
        ];

        format_row(&header)
    }

    fn event_table_header(&self) -> String {
        let id = Header::from(" ID");
        let action = Header::from("Action");
        let timestamp = Header::from("Timestamp");
        let details = Header::from("Details");

        let header = vec![
            Col::new(ID_COL_WIDTH, &id, Align::Left),
            Col::new(ACTION_COL_WIDTH, &action, Align::Center),
            Col::new(TIMESTAMP_COL_WIDTH, &timestamp, Align::Left),
            Col::new(DETAILS_COL_WIDTH, &details, Align::Left),
        ];

        format_row(&header)
    }

    // ID | Prio | Status | Context | Title
    fn map_todo(todo: &Todo) -> Vec<Col> {
        let id = Col::new(ID_COL_WIDTH, &format!(" {}", todo.id), Align::Left);
        let prio = Col::new(PRIO_COL_WIDTH, &todo.prio, Align::Left);
        let status = Col::new(STATUS_COL_WIDTH, &todo.status, Align::Left);
        let title = Col::new(TITLE_COL_WIDTH, &todo.title, Align::Left);
        let context = match &todo.context {
            Some(cx) => Col::new(CTX_COL_WIDTH, &truncate(&cx, CTX_COL_WIDTH), Align::Left),
            None => Col::new(CTX_COL_WIDTH, &"".to_string(), Align::Left),
        };
        vec![id, prio, status, context, title]
    }

    // ID  | Action  | Timestamp | Details
    fn map_event(event: &Event) -> Vec<Col> {
        let id = Col::new(ID_COL_WIDTH, &format!(" {}", event.id), Align::Left);
        let action = Col::new(ACTION_COL_WIDTH, &event.action, Align::Center);
        let datetime = NaiveDateTime::from_timestamp(event.timestamp, 0);
        let datetime = Col::new(TIMESTAMP_COL_WIDTH, &datetime.to_string(), Align::Left);
        let details = Col::new(DETAILS_COL_WIDTH, &event.kind, Align::Left);
        vec![id, action, datetime, details]
    }
}

impl Formatter for TableFormatter {
    fn todos(&self, todos: &[Todo]) -> String {
        let table = todos
            .iter()
            .map(Self::map_todo)
            .map(|cols| format_row(&cols))
            .collect::<Vec<String>>()
            .join("\n");

        format!("{}\n{}", self.todo_table_header(), table)
    }

    fn todo(&self, todo: &Todo) -> String {
        self.todos(&[todo.clone()])
    }

    fn events(&self, events: &[Event]) -> String {
        let table = events
            .iter()
            .map(Self::map_event)
            .map(|cols| format_row(&cols))
            .collect::<Vec<String>>()
            .join("\n");

        format!("{}\n{}", self.event_table_header(), table)
    }
}

pub fn format_row(cols: &[Col]) -> String {
    let height = cols.iter().map(|col| col.height()).max().unwrap_or(0);

    let mut rows: Vec<String> = Vec::new();
    for row in 0..height {
        let mut curr: Vec<String> = Vec::new();
        for col in cols {
            let s = col.nth(row);
            curr.push(s);
        }

        rows.push(curr.join(" "));
    }

    rows.join("\n")
}

struct Header(String);

impl From<&str> for Header {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl StyleDisplay for Header {
    fn styler(&self) -> crate::style::Styler {
        Styler::default().bold(true)
    }
}
