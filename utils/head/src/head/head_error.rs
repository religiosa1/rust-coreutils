use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum HeadError {
    Io(std::io::Error),
    Overflow,
}
impl fmt::Display for HeadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HeadError::Io(err) => err.fmt(f),
            HeadError::Overflow => write!(f, "Value too large for defined data type"),
        }
    }
}

impl Error for HeadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            HeadError::Io(err) => Some(err),
            HeadError::Overflow => Some(self),
        }
    }
}

impl From<std::io::Error> for HeadError {
    fn from(err: std::io::Error) -> Self {
        HeadError::Io(err)
    }
}
