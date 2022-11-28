use crate::error::Result;
use crate::model::{Prio, Status, Todo, CSV};
use crate::service::Changeset;
use crate::{err, util};
use inquire::{Confirm, Select, Text};
use serde::{Deserialize, Serialize};
use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
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
}

pub struct EditorBuilder {
    content: Vec<String>,
}

impl EditorBuilder {
    pub fn new() -> Self {
        Self {
            content: Vec::new(),
        }
    }

    pub fn push(&mut self, s: String) {
        self.content.push(s);
    }

    pub fn build(&self, file_ext: Option<&str>) -> Result<Editor> {
        let ext = match file_ext {
            Some(e) => e,
            None => "",
        };

        let path = env::temp_dir().join(format!("todo-{}{}", util::random_string(8), ext));
        let editor = if let Some(e) = util::try_get_env("EDITOR") {
            e
        } else if let Some(e) = util::try_get_env("VISUAL") {
            e
        } else {
            String::from("nano")
        };

        log::info!("Using editor: {}", &editor);

        if !self.content.is_empty() {
            let content = self.content.join("\r\n");
            util::write_file(&path, &content)?;
        }

        Ok(Editor::new(path, editor))
    }
}

pub struct Editor {
    path: PathBuf,
    editor: String,
}

impl Editor {
    fn new(path: PathBuf, editor: String) -> Self {
        Self { path, editor }
    }

    pub fn empty() -> Self {
        EditorBuilder::new().build(None).unwrap()
    }

    pub fn todo(todo: &Todo) -> Result<Changeset> {
        let text = toml::to_string_pretty(&EditTodo::from(todo))?;

        let mut builder = EditorBuilder::new();
        for line in text.lines() {
            builder.push(line.to_string());
        }

        let edited = builder.build(Some(".toml"))?.edit()?;
        let updated: EditTodo = toml::from_str(&edited)?;

        let prio = Prio::try_from(updated.prio)?;
        let tags = CSV::try_from(updated.tags.join(","))?;

        let cs = Changeset::default()
            .with_subject(updated.subject)
            .with_status(updated.status)
            .with_prio(prio)
            .with_description(updated.description)
            .with_tags(tags);

        Ok(cs)
    }

    pub fn edit(&self) -> Result<String> {
        let mut cmd = Command::new(&self.editor);
        cmd.arg(self.path.to_str().unwrap());

        match cmd.status() {
            Ok(_) => {
                let s = util::read_file(&self.path)?;
                Ok(s)
            }
            Err(err) => err!(err),
        }
    }
}

#[derive(Serialize, Deserialize)]
/// Used to edit a todo from an editor.
struct EditTodo {
    subject: String,
    status: Status,
    prio: String,
    tags: Vec<String>,
    description: String,
}

impl From<&Todo> for EditTodo {
    fn from(todo: &Todo) -> Self {
        Self {
            subject: todo.subject.to_string(),
            status: todo.status.clone(),
            prio: todo.prio.to_string(),
            tags: todo.tags.display_values(),
            description: todo.description.to_string(),
        }
    }
}
