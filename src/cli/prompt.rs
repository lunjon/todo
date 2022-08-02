use crate::error::Result;
use crate::style::Styler;
use std::io::{self, BufRead, Write};

pub struct StdinPrompt {
    bold: Styler,
}

impl StdinPrompt {
    pub fn new(color: bool) -> Self {
        let bold = if color {
            Styler::default().bold(true)
        } else {
            Styler::default()
        };

        Self { bold }
    }
}

impl StdinPrompt {
    pub fn put_msg(msg: &str) -> Result<()> {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        write!(handle, "{msg}")?;
        handle.flush()?;
        Ok(())
    }

    // Reads a line from stdin and returns a string with
    // all whitespace trimmed from its ends.
    pub fn line(
        &self,
        msg: &str,
        allow_empty: bool,
        allowed_values: Option<&[&str]>,
    ) -> Result<String> {
        Self::put_msg(msg)?;

        let stdin = io::stdin();
        let mut buf = String::new();
        stdin.read_line(&mut buf)?;

        match buf.trim() {
            "" => {
                if allow_empty {
                    Ok(String::new())
                } else {
                    Self::put_msg("Value required. Try again.\n")?;
                    self.line(msg, allow_empty, allowed_values)
                }
            }
            s => match allowed_values {
                Some(values) => {
                    if values.contains(&s) {
                        Ok(s.to_string())
                    } else {
                        Self::put_msg("Invalid value. Try again.\n")?;
                        self.line(msg, allow_empty, allowed_values)
                    }
                }
                None => Ok(s.to_string()),
            },
        }
    }

    pub fn lines(&self, msg: &str) -> Result<String> {
        Self::put_msg(msg)?;

        let mut lines = Vec::new();
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let s = line?;
            let s = s.trim();
            if s.is_empty() {
                break;
            }

            lines.push(s.to_string());
        }
        Ok(lines.join("\n"))
    }

    pub fn confirm(&self, msg: &str, default_no: bool) -> Result<bool> {
        let (query, default_answer) = if default_no {
            (format!("(y/{})", self.bold.style("[n]")), false)
        } else {
            (format!("({}/n)", self.bold.style("[y]")), true)
        };

        let new = format!("{msg} {query} ");
        let line = self.line(&new, true, None)?;

        match line.trim().to_lowercase().as_str() {
            "y" | "yes" => Ok(true),
            "n" | "no" => Ok(false),
            "" => Ok(default_answer),
            _ => {
                Self::put_msg("Invalid answer, try again.\n")?;
                self.confirm(msg, default_no)
            }
        }
    }
}
