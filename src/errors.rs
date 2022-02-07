use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub enum Error {
    ParseError,
    BytesError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Shit!")
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        "Shit!"
    }

    fn cause(&self) -> Option<&dyn StdError> {
        None
    }
}
