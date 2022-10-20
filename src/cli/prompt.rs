use crate::error::Result;
use inquire::{Confirm, Select, Text};
use std::io::{self, BufRead, Write};

#[derive(Default)]
pub struct StdinPrompt {}

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
        let text = match allowed_values {
            Some(allowed) => {
                let text = Select::new(msg, allowed.to_vec())
                    .with_vim_mode(true)
                    .prompt()?;
                text.to_string()
            }
            None => Text::new(msg).prompt()?,
        };

        match text.trim() {
            "" => {
                if allow_empty {
                    Ok(String::new())
                } else {
                    Self::put_msg("Value required. Try again.\n")?;
                    self.line(msg, allow_empty, allowed_values)
                }
            }
            s => Ok(s.to_string()),
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
        let ok = Confirm::new(msg).with_default(!default_no).prompt()?;
        Ok(ok)
    }
}
