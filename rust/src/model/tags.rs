use crate::error::Error;
use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
pub struct Tags {
    tags: Vec<String>,
}

impl Tags {
    pub fn new(tags: Vec<String>) -> Self {
        Self { tags }
    }

    pub fn has_any(&self, values: &[String]) -> bool {
        for v in values {
            for t in &self.tags {
                if t == v {
                    return true;
                }
            }
        }
        false
    }

    pub fn values(&self) -> Vec<String> {
        self.tags.clone()
    }
}

impl fmt::Display for Tags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tags.join(" "))
    }
}

impl TryFrom<&str> for Tags {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let tags: Vec<String> = s.split_whitespace().map(String::from).collect();
        if tags.iter().any(|t| t.len() > 20) {
            Err(Error::DataError(
                "invalid tag found: length greater than 20".to_string(),
            ))
        } else {
            Ok(Self { tags })
        }
    }
}

impl TryFrom<String> for Tags {
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
        let tags = Tags::try_from("one two").unwrap();
        assert_eq!(tags.tags.len(), 2);
    }

    #[test]
    fn try_from_empty() {
        let tags = Tags::try_from("").unwrap();
        assert!(tags.tags.is_empty());
    }

    #[test]
    fn has_any() {
        let tags = Tags::try_from("one two").unwrap();
        let a = vec!["one".to_string()];
        let b = vec!["three".to_string()];
        assert!(tags.has_any(&a));
        assert!(!tags.has_any(&b));
    }
}
