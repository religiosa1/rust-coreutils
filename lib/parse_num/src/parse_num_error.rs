use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParseNumError {
    Empty,
    BadNumericValue,
    BadMultiplierValue(u32),
}

impl fmt::Display for ParseNumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseNumError::Empty => write!(f, "Empty string as NUM"),
            ParseNumError::BadNumericValue => write!(f, "bad numeric value"),
            ParseNumError::BadMultiplierValue(n) => {
                write!(f, "bad multiplier value at position {}", n)
            }
        }
    }
}

impl Error for ParseNumError {}
