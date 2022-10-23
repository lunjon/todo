use super::{Code, Item, ID};
use crate::{err, error::Error};
use core::fmt;

#[derive(Clone, Copy, Debug)]
pub enum Link {
    /// Blocking another todo.
    Blocks(ID),
    /// Blocked by other todo.
    BlockedBy(ID),
    RelatesTo(ID),
}

use Link::*;

impl Link {
    /// Returns the counter part of self, e.g. if self is
    /// Blocks(1) this method returns BlockedBy(other),
    pub fn bi_directional(&self, other: ID) -> Option<Self> {
        match self {
            Blocks(_) => Some(BlockedBy(other)),
            BlockedBy(_) => Some(Blocks(other)),
            _ => None,
        }
    }

    pub fn with_id(&self, id: ID) -> Self {
        match self {
            Blocks(_) => Blocks(id),
            BlockedBy(_) => BlockedBy(id),
            RelatesTo(_) => RelatesTo(id),
        }
    }

    pub fn id(&self) -> ID {
        match self {
            Blocks(id) => *id,
            BlockedBy(id) => *id,
            RelatesTo(id) => *id,
        }
    }
}

impl fmt::Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Blocks(id) => write!(f, "blocks {}", id),
            BlockedBy(id) => write!(f, "blocked by {}", id),
            RelatesTo(id) => write!(f, "relates to {}", id),
        }
    }
}

impl TryFrom<&str> for Link {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (a, b) = if let Some((a, b)) = value.split_once(':') {
            (a, b)
        } else {
            return err!("invalid link: {}", value);
        };

        let id = ID::try_from(b)?;
        match a.to_lowercase().as_str() {
            "blocks" => Ok(Self::Blocks(id)),
            "blocked-by" | "blocked_by" | "blockedby" => Ok(Self::BlockedBy(id)),
            "relates-to" | "relates_to" | "relatesto" => Ok(Self::RelatesTo(id)),
            s => err!("invalid link: {}", s),
        }
    }
}

impl TryFrom<String> for Link {
    type Error = Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl Code for Link {
    fn encode(&self) -> String {
        match self {
            Blocks(id) => format!("blocks:{}", id),
            BlockedBy(id) => format!("blockedby:{}", id),
            RelatesTo(id) => format!("relatesto:{}", id),
        }
    }

    fn decode(s: &str) -> Self {
        Self::try_from(s).unwrap()
    }
}

impl Item for Link {}

impl PartialEq for Link {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Blocks(my_id) => match other {
                Blocks(other_id) => my_id == other_id,
                _ => false,
            },
            BlockedBy(my_id) => match other {
                BlockedBy(other_id) => my_id == other_id,
                _ => false,
            },
            RelatesTo(my_id) => match other {
                RelatesTo(other_id) => my_id == other_id,
                _ => false,
            },
        }
    }
}

impl Eq for Link {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_links() {
        let tests = ["blocks:1", "blocked_by:1", "blockedby:2"];
        for t in tests {
            let res = Link::try_from(t);
            assert!(res.is_ok());
        }
    }

    #[test]
    fn reject_invalid_links() {
        let tests = ["blocks", "blockedby:", "blocks:-1", "unknown"];
        for t in tests {
            let res = Link::try_from(t);
            assert!(res.is_err());
        }
    }
}
