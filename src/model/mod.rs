use crate::{err, error::Error};
use serde::Serialize;
use std::fmt;

pub mod link;
pub mod prio;
pub mod status;
pub mod tags;
pub mod todo;

pub use self::todo::*;
pub use link::*;
pub use prio::*;
pub use status::*;
pub use tags::*;

/// An identifier for Todos for simple referencing.
#[derive(Clone, Copy, Debug, Eq, Serialize)]
pub struct ID(u16);

impl ID {
    pub fn new(id: u16) -> Self {
        Self(id)
    }
}

impl TryFrom<&str> for ID {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.parse::<u16>() {
            Ok(id) => Ok(ID::new(id)),
            Err(_) => err!("invalid id: {}", value),
        }
    }
}

impl PartialEq for ID {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl fmt::Display for ID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub trait Code {
    fn encode(&self) -> String;
    fn decode(s: &str) -> Self;
}

impl Code for String {
    fn encode(&self) -> String {
        self.clone()
    }

    fn decode(s: &str) -> Self {
        s.to_string()
    }
}

/// Marker trait for values put in the CSV<T> type.
pub trait Item: fmt::Display + Eq + Code {}

impl Item for String {}

/// A container for values implementing Comma trait
/// and adds ability to serialize/deserialize to string.
#[derive(Clone, Debug, Default, Serialize)]
pub struct CSV<T>(Vec<T>)
where
    T: Item;

impl<T> CSV<T>
where
    T: Item,
{
    pub fn new(v: Vec<T>) -> Self {
        Self(v)
    }

    pub fn empty() -> Self {
        Self(vec![])
    }

    pub fn display_values(&self) -> Vec<String> {
        self.0.iter().map(|v| v.to_string()).collect()
    }

    pub fn push(&mut self, item: T) {
        self.0.push(item);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn has_any(&self, values: &[T]) -> bool {
        for v in values {
            for t in &self.0 {
                if t == v {
                    return true;
                }
            }
        }
        false
    }
}

impl<T> TryFrom<String> for CSV<T>
where
    T: Item,
{
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut values: Vec<T> = Vec::new();
        for v in value.split(',').filter_map(|s| match s.trim() {
            "" => None,
            v => Some(v),
        }) {
            let item = T::decode(v);
            values.push(item);
        }
        Ok(Self(values))
    }
}

impl<T> Code for CSV<T>
where
    T: Item,
{
    fn encode(&self) -> String {
        let v: Vec<String> = self.0.iter().map(|v| v.encode()).collect();
        v.join(",")
    }

    fn decode(s: &str) -> Self {
        Self::try_from(s.to_string()).unwrap()
    }
}

impl<T> fmt::Display for CSV<T>
where
    T: Item,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let v: Vec<String> = self.0.iter().map(|v| v.to_string()).collect();
        write!(f, "{}", v.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_string() {
        let csv: CSV<String> = CSV::try_from(String::from("1,2,3,4")).unwrap();
        assert_eq!(4, csv.len());
    }

    #[test]
    fn decode_csv() {
        let csv: CSV<String> = CSV::decode("1,2,3,4");
        assert_eq!(4, csv.len());
    }
}
