use std::{convert::Infallible, fmt, io, str};

/// Result type using Error from this crate.
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    General(String),
    NotFound(Option<String>),
}

impl Error {
    pub fn new(msg: String) -> Self {
        Self::General(msg)
    }
}

#[macro_export]
macro_rules! err {
    ($fmt:expr) => {
        {
            Err($crate::error::Error::new($fmt.to_string()))
        }
    };
    ($fmt:expr, $($e:expr),*) => {
        {
            let s = format!($fmt, $($e)*);
            Err($crate::error::Error::new(s))
        }
    };
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::General(err) => write!(f, "{}", err),
            Error::NotFound(opt) => match opt {
                Some(id) => write!(f, "not found: {}", id),
                None => write!(f, "not found"),
            },
        }
    }
}

impl From<Infallible> for Error {
    fn from(err: Infallible) -> Self {
        Self::new(err.to_string())
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::new(err.to_string())
    }
}

impl From<str::Utf8Error> for Error {
    fn from(err: str::Utf8Error) -> Self {
        Self::new(err.to_string())
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
                Error::new(error.to_string())
            }
        }
    };

    match error.code() {
        Some(code) => match code.to_string().as_str() {
            // FOREIGN KEY constraint failed
            "787" => Error::NotFound(None),
            _ => Error::new(error.to_string()),
        },
        None => Error::new(error.to_string()),
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::new(err.to_string())
    }
}

impl From<toml::ser::Error> for Error {
    fn from(err: toml::ser::Error) -> Self {
        Self::new(err.to_string())
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Self::new(err.to_string())
    }
}

impl From<inquire::InquireError> for Error {
    fn from(err: inquire::InquireError) -> Self {
        Error::new(err.to_string())
    }
}
