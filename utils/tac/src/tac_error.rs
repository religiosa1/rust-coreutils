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

macro_rules! impl_from_error {
    ($error_type:ty, $variant:ident) => {
        impl From<$error_type> for TacError {
            fn from(err: $error_type) -> Self {
                TacError::$variant(err)
            }
        }
    };
}
impl_from_error!(std::io::Error, IO);
impl_from_error!(regex::Error, Regex);
