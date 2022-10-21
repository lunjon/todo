use crate::model::Todo;
use crate::style::{Color, StyleDisplay, Styler};
use crate::util::word_chunks;

const INDENT: usize = 13;

/// Card is used to get a user-friendly view of a todo.
pub struct Card {
    color: bool,
    heading: Styler,
    bold_white: Styler,
    blue: Styler,
}

impl Card {
    pub fn new(color: bool) -> Self {
        let heading: Styler;
        let bold_white: Styler;
        let blue: Styler;

        if color {
            heading = Styler::default().underline(true).bold(true);
            bold_white = Styler::default().bold(true);
            blue = Styler::default().bold(true).fg(Color::Blue);
        } else {
            heading = Styler::default();
            bold_white = Styler::default();
            blue = Styler::default();
        }

        Self {
            color,
            heading,
            bold_white,
            blue,
        }
    }

    /// Format `todo` into a detailed string.
    pub fn format(&self, todo: &Todo) -> String {
        let mut lines: Vec<String> = Vec::new();

        let title = self.heading.style(&todo.title);
        lines.push(title);
        lines.push("".to_string());

        let (status, prio) = if self.color {
            (todo.status.style(), todo.prio.style())
        } else {
            (todo.status.to_string(), todo.status.to_string())
        };

        let status = format!("{}:      {}", self.bold_white.style("Status"), status);
        lines.push(status);

        let prio = format!("{}:    {}", self.bold_white.style("Priority"), prio);
        lines.push(prio);

        if let Some(c) = &todo.context {
            let context = format!(
                "{}:     {}",
                self.bold_white.style("Context"),
                self.blue.style(c)
            );
            lines.push(context);
        }

        let tags = todo.tags.values();
        if !tags.is_empty() {
            let tags = format!(
                "{}:        {}",
                self.bold_white.style("Tags"),
                tags.join(", ")
            );
            lines.push(tags);
        }

        if let Some(s) = self.format_description(&todo.description) {
            lines.push("".to_string());
            for l in s {
                lines.push(l);
            }
        }

        lines.join("\n")
    }

    fn format_description(&self, desc: &str) -> Option<Vec<String>> {
        if desc.is_empty() {
            return None;
        }

        let field = "Description";
        let prefix = " ".repeat(INDENT);
        let chunks = word_chunks(desc, 100);
        let mut lines = Vec::new();
        if let Some(s) = chunks.first() {
            lines.push(format!("{}: {}", self.bold_white.style(field), s));
        }

        for line in chunks.iter().skip(1) {
            lines.push(format!("{prefix}{line}"));
        }

        Some(lines)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Prio, Status, Tags, Todo, ID};
    use chrono::Local;

    #[test]
    fn test_format() {
        let todo = Todo::new(
            ID::new(1),
            Local::now(),
            Status::New,
            Prio::Normal,
            "".to_string(),
            "".to_string(),
            Tags::default(),
            None,
        );

        let card = Card::new(true);
        let s = card.format(&todo);
        assert!(!s.is_empty());
    }
}