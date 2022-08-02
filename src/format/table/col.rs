use super::{pad, Align};
use crate::style::{Color, Styler};
use crate::util::word_chunks;

// Col represents a column in a table that spans
// one or more lines, depending on `width`.
pub struct Col {
    width: usize,
    lines: Vec<String>,
}

impl Col {
    pub fn new(width: usize, content: String, align: Align) -> Self {
        let lines = word_chunks(&content, width)
            .iter()
            .map(|c| pad(c, width, &align))
            .collect();
        Self { width, lines }
    }

    // Returns the number of lines this column spans.
    pub fn height(&self) -> usize {
        self.lines.len()
    }

    // nth returns the `n`th line of this column if available.
    // If number of lines is less than `n` it returns an empty
    // string with length `self.width`.
    pub fn nth(&self, n: usize) -> String {
        match self.lines.get(n) {
            Some(line) => line.clone(),
            None => " ".repeat(self.width),
        }
    }
}

/// Allows for custom styling/coloring of columns based on column index.
pub trait ColFormatter {
    /// Style `s` given the column `index`.
    fn format(&self, index: usize, s: &str) -> String;
}

/// DefaultColFormatter doesn't apply any style/color.
#[derive(Default)]
pub struct DefaultColFormatter;

impl ColFormatter for DefaultColFormatter {
    fn format(&self, _: usize, s: &str) -> String {
        s.to_string()
    }
}

/// Used to format table header.
pub struct HeaderColFormatter {
    bold: Styler,
}

impl Default for HeaderColFormatter {
    fn default() -> Self {
        Self {
            bold: Styler::default().bold(true),
        }
    }
}

impl ColFormatter for HeaderColFormatter {
    fn format(&self, _: usize, s: &str) -> String {
        self.bold.style(s)
    }
}

pub struct TodoColFormatter {
    bold: Styler,
    blue: Styler,
    green: Styler,
}

impl Default for TodoColFormatter {
    fn default() -> Self {
        Self {
            bold: Styler::default().bold(true),
            blue: Styler::default().bold(true).fg(Color::Blue),
            green: Styler::default().bold(true).fg(Color::Green),
        }
    }
}

impl ColFormatter for TodoColFormatter {
    fn format(&self, index: usize, s: &str) -> String {
        match index {
            // Status
            1 => {
                if s.contains("done") {
                    self.green.style(s)
                } else if s.contains("start") {
                    self.blue.style(s)
                } else {
                    self.bold.style(s)
                }
            }
            // ID, title, description, tags
            _ => s.to_string(),
        }
    }
}

pub struct EventColFormatter {
    green: Styler,
    blue: Styler,
    red: Styler,
}

impl Default for EventColFormatter {
    fn default() -> Self {
        Self {
            red: Styler::default().bold(true).fg(Color::Red),
            green: Styler::default().bold(true).fg(Color::Green),
            blue: Styler::default().bold(true).fg(Color::Blue),
        }
    }
}

impl ColFormatter for EventColFormatter {
    fn format(&self, index: usize, s: &str) -> String {
        match index {
            // Action
            1 => {
                if s.contains("add") {
                    self.green.style(s)
                } else if s.contains("update") {
                    self.blue.style(s)
                } else if s.contains("remove") {
                    self.red.style(s)
                } else {
                    s.to_string()
                }
            }
            // ID, timestamp, details
            _ => s.to_string(),
        }
    }
}
