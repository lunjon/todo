use core::fmt;
use crossterm::style::Stylize;

/// Trait for displaying with styling.
pub trait StyleDisplay: fmt::Display {
    fn style(&self) -> String;
}

pub enum Color {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
}

// TODO: add builder

#[derive(Default)]
pub struct Styler {
    fg: Option<Color>,
    bold: bool,
    underline: bool,
}

impl Styler {
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    pub fn bold(mut self, yes: bool) -> Self {
        self.bold = yes;
        self
    }

    pub fn underline(mut self, yes: bool) -> Self {
        self.underline = yes;
        self
    }

    pub fn style(&self, s: &str) -> String {
        if !(self.bold || self.underline) && self.fg.is_none() {
            return s.to_string();
        }

        let c = if let Some(fg) = &self.fg {
            match fg {
                Color::Red => s.red(),
                Color::Green => s.green(),
                Color::Yellow => s.yellow(),
                Color::Blue => s.blue(),
                Color::Magenta => s.magenta(),
                Color::Cyan => s.cyan(),
            }
        } else {
            s.white()
        };

        let c = if self.bold { c.bold() } else { c };
        let c = if self.underline { c.underlined() } else { c };
        format!("{c}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let st = Styler::default();
        let s = st.style("default");
        assert_eq!(s, "default");
    }

    #[test]
    fn test_bold() {
        let st = Styler::default().bold(true);
        let s = st.style("bold");
        assert!(s.len() > 4);
    }

    #[test]
    fn test_underline() {
        let st = Styler::default().underline(true);
        let s = st.style("underline");
        assert!(s.len() > 9);
    }

    #[test]
    fn test_bold_underline() {
        let st = Styler::default().bold(true).underline(true);
        let s = st.style("combo");
        assert!(s.len() > 5);
    }

    #[test]
    fn test_fg() {
        let st = Styler::default().fg(Color::Red);
        let s = st.style("red");
        assert!(s.len() > 3);
    }

    #[test]
    fn test_bold_fg() {
        let st = Styler::default().bold(true).fg(Color::Red);
        let s = st.style("red");
        assert!(s.len() > 3);
    }
}
