use crate::error::Error;
use crate::style::{Color, StyleDisplay, Styler};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;

/// Used to set a priority on a todo.
#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
pub enum Prio {
    Low,
    Normal,
    High,
    Critical,
}

impl PartialOrd for Prio {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Note that Ordering::Less means it ends up before other
/// values when sorting.
impl Ord for Prio {
    fn cmp(&self, other: &Prio) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }

        match self {
            Prio::Low => Ordering::Greater,
            Prio::Normal => match other {
                Prio::Low => Ordering::Less,
                _ => Ordering::Greater,
            },
            Prio::High => match other {
                Prio::Critical => Ordering::Greater,
                _ => Ordering::Less,
            },
            Prio::Critical => Ordering::Less,
        }
    }
}

impl fmt::Display for Prio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Prio::Low => write!(f, "low"),
            Prio::Normal => write!(f, "normal"),
            Prio::High => write!(f, "high"),
            Prio::Critical => write!(f, "critical"),
        }
    }
}

impl StyleDisplay for Prio {
    fn style(&self) -> String {
        let styler = Styler::default().bold(true);
        let styler = match self {
            Prio::Low => styler.fg(Color::Blue),
            Prio::Normal => styler,
            Prio::High => styler.fg(Color::Yellow),
            Prio::Critical => styler.fg(Color::Red),
        };
        styler.style(&self.to_string())
    }
}

impl TryFrom<&str> for Prio {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "low" => Ok(Prio::Low),
            "normal" => Ok(Prio::Normal),
            "high" => Ok(Prio::High),
            "critical" => Ok(Prio::Critical),
            _ => Err(Error::ArgError(format!("invalid prio value: {value}"))),
        }
    }
}

impl TryFrom<String> for Prio {
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
            ("low", Prio::Low),
            ("normal", Prio::Normal),
            ("high", Prio::High),
            ("critical", Prio::Critical),
        ];
        for (s, expected) in cases {
            let actual = Prio::try_from(s).unwrap();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn try_from_invalid() {
        let r = Prio::try_from("unknown");
        assert!(r.is_err());
    }
}
