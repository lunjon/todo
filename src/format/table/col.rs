use super::{pad, Align};
use crate::style::{StyleDisplay, Styler};
use crate::util::word_chunks;

// Col represents a column in a table that spans
// one or more lines, depending on `width`.
pub struct Col {
    width: usize,
    lines: Vec<String>,
    styler: Styler,
}

impl Col {
    pub fn new<D: StyleDisplay>(width: usize, content: &D, align: Align) -> Self {
        let raw = content.to_string();
        let lines = word_chunks(&raw, width)
            .iter()
            .map(|c| pad(c, width, &align))
            .collect();

        Self {
            width,
            lines,
            styler: content.styler(),
        }
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
            Some(line) => self.styler.style(line),
            None => " ".repeat(self.width),
        }
    }
}
