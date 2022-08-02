use self::col::ColFormatter;

use super::Formatter;
use crate::model::{Event, Todo};
use chrono::NaiveDateTime;

mod col;
#[cfg(test)]
mod tests;

use col::{Col, EventColFormatter, HeaderColFormatter, TodoColFormatter};

const ID_COL_WIDTH: usize = 3;
const STATUS_COL_WIDTH: usize = 8;
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
#[allow(dead_code)]
fn truncate(s: &str, size: usize) -> String {
    if s.len() < size {
        s.to_string()
    } else {
        let s = s[0..size - 4].to_string();
        format!("{s} ...")
    }
}

/// Formats a list of todos into a table.
pub struct TableFormatter {
    header_formatter: HeaderColFormatter,
    todo_formatter: TodoColFormatter,
    event_formatter: EventColFormatter,
}

impl TableFormatter {
    pub fn new(_color: bool) -> Self {
        let header_formatter = HeaderColFormatter::default();
        let todo_formatter = TodoColFormatter::default();
        let event_formatter = EventColFormatter::default();

        Self {
            todo_formatter,
            event_formatter,
            header_formatter,
        }
    }

    fn todo_table_header(&self) -> String {
        let header = vec![
            Col::new(ID_COL_WIDTH, " ID".to_string(), Align::Left),
            Col::new(STATUS_COL_WIDTH, "Status".to_string(), Align::Left),
            Col::new(TITLE_COL_WIDTH, "Title".to_string(), Align::Left),
        ];

        format_cols(&header, &self.header_formatter)
    }

    fn event_table_header(&self) -> String {
        let header = vec![
            Col::new(ID_COL_WIDTH, " ID".to_string(), Align::Left),
            Col::new(ACTION_COL_WIDTH, "Action".to_string(), Align::Center),
            Col::new(TIMESTAMP_COL_WIDTH, "Timestamp".to_string(), Align::Left),
            Col::new(DETAILS_COL_WIDTH, "Details".to_string(), Align::Left),
        ];

        format_cols(&header, &self.header_formatter)
    }

    // ID  | Status | Title
    fn map_todo(todo: &Todo) -> Vec<Col> {
        let id = Col::new(ID_COL_WIDTH, format!(" {}", todo.id), Align::Left);
        let title = Col::new(TITLE_COL_WIDTH, todo.title.to_string(), Align::Left);
        let status = Col::new(STATUS_COL_WIDTH, todo.status.to_string(), Align::Left);
        vec![id, status, title]
    }

    // ID  | Action  | Timestamp | Details
    fn map_event(event: &Event) -> Vec<Col> {
        let id = Col::new(ID_COL_WIDTH, format!(" {}", event.id), Align::Left);
        let action = Col::new(ACTION_COL_WIDTH, event.action.to_string(), Align::Center);

        let datetime = NaiveDateTime::from_timestamp(event.timestamp, 0);
        let datetime = Col::new(TIMESTAMP_COL_WIDTH, datetime.to_string(), Align::Left);

        let details = Col::new(DETAILS_COL_WIDTH, event.kind.to_string(), Align::Left);

        vec![id, action, datetime, details]
    }
}

impl Formatter for TableFormatter {
    fn todos(&self, todos: &[Todo]) -> String {
        let table = todos
            .iter()
            .map(Self::map_todo)
            .map(|cols| format_cols(&cols, &self.todo_formatter))
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
            .map(|cols| format_cols(&cols, &self.event_formatter))
            .collect::<Vec<String>>()
            .join("\n");

        format!("{}\n{}", self.event_table_header(), table)
    }
}

pub fn format_cols<F: ColFormatter>(cols: &[Col], f: &F) -> String {
    let height = cols.iter().map(|col| col.height()).max().unwrap_or(0);

    let mut rows: Vec<String> = Vec::new();
    for row in 0..height {
        let mut curr: Vec<String> = Vec::new();
        for (i, col) in cols.iter().enumerate() {
            let s = col.nth(row);
            let s = f.format(i, &s);
            curr.push(s);
        }

        rows.push(curr.join(" "));
    }

    rows.join("\n")
}
