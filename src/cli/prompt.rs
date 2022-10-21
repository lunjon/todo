use crate::error::{Error, Result};
use crate::util;
use inquire::{Confirm, Select, Text};
use std::env;
use std::io::{self, Write};
use std::process::Command;

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

    pub fn select(&self, msg: &str, values: Vec<String>) -> Result<String> {
        let option = Select::new(msg, values).with_vim_mode(true).prompt()?;
        Ok(option)
    }

    // Reads a line from stdin and returns a string with
    // all whitespace trimmed from its ends.
    pub fn line(&self, msg: &str, allow_empty: bool) -> Result<String> {
        let text = Text::new(msg).prompt()?;
        match text.trim() {
            "" => {
                if allow_empty {
                    Ok(String::new())
                } else {
                    Self::put_msg("Value required. Try again.\n")?;
                    self.line(msg, allow_empty)
                }
            }
            s => Ok(s.to_string()),
        }
    }

    pub fn confirm(&self, msg: &str, default_no: bool) -> Result<bool> {
        let ok = Confirm::new(msg).with_default(!default_no).prompt()?;
        Ok(ok)
    }

    pub fn editor(&self) -> Result<String> {
        let editor: String;
        let editor_env = util::try_get_env("EDITOR");
        let visual_env = util::try_get_env("VISUAL");
        if let Some(e) = editor_env {
            editor = e;
        } else if let Some(e) = visual_env {
            editor = e;
        } else {
            editor = String::from("nano");
        }

        log::info!("Using editor: {}", &editor);

        let tmp_file = env::temp_dir().join(format!("todo-{}", util::random_string(8)));
        let mut cmd = Command::new(editor);
        cmd.arg(tmp_file.to_str().unwrap());

        match cmd.status() {
            Ok(_) => {
                let s = util::read_file(&tmp_file)?;
                Ok(s)
            }
            Err(err) => Err(Error::IOError(err.to_string())),
        }
    }
}
