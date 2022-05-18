use core::fmt;

/// Trait for displaying with styling.
pub trait StyleDisplay: fmt::Display {
    fn style(&self) -> String;
}

/// Based on:
/// https://stackoverflow.com/questions/4842424/list-of-ansi-color-escape-sequences

pub enum Color {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
}

impl Color {
    fn rgb(&self) -> &'static str {
        match self {
            Color::Red => "31",
            Color::Green => "32",
            Color::Yellow => "33",
            Color::Blue => "34",
            Color::Magenta => "35",
            Color::Cyan => "36",
        }
    }
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
        let mut codes: Vec<&str> = Vec::new();

        if let Some(fg) = &self.fg {
            codes.push("38"); // Set foreground color
            codes.push("2");
            let c = fg.rgb();
            codes.push(c);
        }

        if self.bold {
            codes.push("1");
        }

        if self.underline {
            codes.push("4");
        }

        if codes.is_empty() {
            return s.to_string();
        }

        let values = codes.join(";");
        format!("\x1b[{}m{}\x1b[0m", values, s)
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
