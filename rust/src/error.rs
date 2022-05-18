use std::{convert::Infallible, fmt, io, str};

/// Result type using Error from this crate.
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ArgError(String),
    ConfigError(String),
    DataError(String),
    IOError(String),
    NotFound(Option<String>),
}

use Error::*;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgError(s) => write!(f, "argument: {s}"),
            ConfigError(s) => write!(f, "config: {s}"),
            DataError(s) => write!(f, "data: {s}"),
            IOError(s) => write!(f, "i/o: {s}"),
            NotFound(v) => match v {
                Some(id) => write!(f, "identifier {id} was not found"),
                None => write!(f, "provided ID was not found"),
            },
        }
    }
}

impl From<Infallible> for Error {
    fn from(err: Infallible) -> Self {
        ArgError(err.to_string())
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        IOError(err.to_string())
    }
}

impl From<str::Utf8Error> for Error {
    fn from(err: str::Utf8Error) -> Self {
        DataError(err.to_string())
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        map_sqlx_error(err)
    }
}

/// Maps from an sqlx error to an error of this library.
pub fn map_sqlx_error(error: sqlx::Error) -> Error {
    let error = match error.as_database_error() {
        Some(err) => err,
        None => {
            return if error.to_string().contains("no rows returned by a query") {
                Error::NotFound(None)
            } else {
                Error::IOError(error.to_string())
            }
        }
    };

    match error.code() {
        Some(code) => match code.to_string().as_str() {
            // FOREIGN KEY constraint failed
            "787" => Error::NotFound(None),
            _ => Error::IOError(error.to_string()),
        },
        None => Error::IOError(error.to_string()),
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::DataError(format!("failed to (de)serialize: {}", err))
    }
}
