use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum TailError {
    Io(std::io::Error),
    Overflow,
}
impl fmt::Display for TailError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TailError::Io(err) => err.fmt(f),
            TailError::Overflow => write!(f, "Value too large for defined data type"),
        }
    }
}

impl Error for TailError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TailError::Io(err) => Some(err),
            TailError::Overflow => Some(self),
        }
    }
}

impl From<std::io::Error> for TailError {
    fn from(err: std::io::Error) -> Self {
        TailError::Io(err)
    }
}
