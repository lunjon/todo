use crate::err;
use crate::error::Error;
use crate::style::{Color, StyleDisplay, Styler};
use serde::de::Visitor;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;

/// Status represents the state of a Todo.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Status {
    /// A newly created todo.
    New,
    /// Todo is in progress.
    Started,
    /// The todo is completed.
    Done,
    /// This is blocked by another todo.
    Blocked,
}

impl Serialize for Status {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct StatusVisitor;

impl<'de> Visitor<'de> for StatusVisitor {
    type Value = Status;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match Status::try_from(v) {
            Ok(s) => Ok(s),
            Err(_) => Err(serde::de::Error::custom("invalid status")),
        }
    }
}

impl<'de> Deserialize<'de> for Status {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(StatusVisitor)
    }
}

use Status::*;

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            New => write!(f, "new"),
            Started => write!(f, "started"),
            Done => write!(f, "done"),
            Blocked => write!(f, "blocked"),
        }
    }
}

impl StyleDisplay for Status {
    fn styler(&self) -> Styler {
        let styler = Styler::default();
        match self {
            New => styler.fg(Color::Cyan),
            Started => styler.fg(Color::Blue),
            Done => styler.fg(Color::Green),
            Blocked => styler.fg(Color::Red),
        }
    }
}

impl TryFrom<&str> for Status {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().trim() {
            "new" => Ok(New),
            "started" | "in-progress" => Ok(Started),
            "done" => Ok(Done),
            "blocked" => Ok(Blocked),
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

impl PartialOrd for Status {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// started > new > blocked > done
///
/// Note that Ordering::Less means it ends up before other
/// values when sorting.
impl Ord for Status {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Started => match other {
                Started => Ordering::Equal,
                _ => Ordering::Less,
            },
            New => match other {
                Started => Ordering::Greater,
                New => Ordering::Equal,
                _ => Ordering::Less,
            },
            Blocked => match other {
                Blocked => Ordering::Equal,
                Done => Ordering::Less,
                _ => Ordering::Greater,
            },
            Done => match other {
                Done => Ordering::Equal,
                _ => Ordering::Less,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from() {
        let cases = vec![
            ("new", New),
            ("started", Started),
            ("done", Done),
            ("blocked", Blocked),
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
