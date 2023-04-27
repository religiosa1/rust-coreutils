use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum Base64Error {
    Io(std::io::Error),
    DecodeError(base64::DecodeError),
}

impl fmt::Display for Base64Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Base64Error::Io(err) => err.fmt(f),
            Base64Error::DecodeError(err) => match err {
                base64::DecodeError::InvalidLastSymbol(index, byte)
                | base64::DecodeError::InvalidByte(index, byte) => {
                    write!(f, "Invalid input: {} {}", index, byte)
                }
                _ => err.fmt(f),
            },
        }
    }
}

impl Error for Base64Error {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Base64Error::Io(err) => Some(err),
            Base64Error::DecodeError(err) => Some(err),
        }
    }
}

impl From<std::io::Error> for Base64Error {
    fn from(err: std::io::Error) -> Self {
        Base64Error::Io(err)
    }
}
impl From<base64::DecodeError> for Base64Error {
    fn from(err: base64::DecodeError) -> Self {
        Base64Error::DecodeError(err)
    }
}
