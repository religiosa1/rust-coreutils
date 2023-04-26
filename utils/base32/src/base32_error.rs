use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum Base32Error {
    Io(std::io::Error),
    BadChar(u8),
}

impl fmt::Display for Base32Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Base32Error::Io(err) => err.fmt(f),
            Base32Error::BadChar(char) => write!(
                f,
                "Bad character (unspported by base32 alphabet): {}",
                *char
            ),
        }
    }
}

impl Error for Base32Error {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Base32Error::Io(err) => Some(err),
            Base32Error::BadChar(_) => Some(self),
        }
    }
}

impl From<std::io::Error> for Base32Error {
    fn from(err: std::io::Error) -> Self {
        Base32Error::Io(err)
    }
}
