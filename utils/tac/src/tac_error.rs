use std::{error::Error, fmt};

#[derive(Debug)]
pub enum TacError {
    IO(std::io::Error),
    Regex(regex::Error),
}

impl fmt::Display for TacError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TacError::IO(err) => err.fmt(f),
            TacError::Regex(err) => err.fmt(f),
        }
    }
}

impl Error for TacError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TacError::IO(err) => Some(err),
            TacError::Regex(err) => Some(err),
        }
    }
}

impl From<std::io::Error> for TacError {
    fn from(err: std::io::Error) -> Self {
        TacError::IO(err)
    }
}

impl From<regex::Error> for TacError {
    fn from(err: regex::Error) -> Self {
        TacError::Regex(err)
    }
}
