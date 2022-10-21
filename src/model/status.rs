use crate::err;
use crate::error::Error;
use crate::style::{Color, StyleDisplay, Styler};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Status represents the state of a Todo.
#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
pub enum Status {
    /// A newly created todo.
    New,
    /// Todo is in progress.
    Started,
    /// The todo is completed.
    Done,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::New => write!(f, "new"),
            Status::Started => write!(f, "started"),
            Status::Done => write!(f, "done"),
        }
    }
}

impl StyleDisplay for Status {
    fn styler(&self) -> Styler {
        let styler = Styler::default();
        match self {
            Status::New => styler.fg(Color::Cyan),
            Status::Started => styler.fg(Color::Blue),
            Status::Done => styler.fg(Color::Green),
        }
    }
}

impl TryFrom<&str> for Status {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "new" => Ok(Status::New),
            "started" | "in-progress" => Ok(Status::Started),
            "done" => Ok(Status::Done),
            _ => err!("unknown status: {}", value),
        }
    }
}

impl TryFrom<String> for Status {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from() {
        let cases = vec![
            ("new", Status::New),
            ("started", Status::Started),
            ("done", Status::Done),
        ];
        for (s, expected) in cases {
            let actual = Status::try_from(s).unwrap();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn try_from_invalid() {
        let r = Status::try_from("unknown");
        assert!(r.is_err());
    }
}
