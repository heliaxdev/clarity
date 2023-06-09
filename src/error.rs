use std::fmt;
use std::num::ParseIntError;
use std::str::Utf8Error;

/// Custom error implementation that describes possible
/// error states.
///
/// This is shared by a whole crate.
#[derive(Debug)]
pub enum Error {
    InvalidEip55,
    InvalidUtf8(Utf8Error),
    InvalidHex(ParseIntError),
    InvalidAddressLength { got: usize, expected: usize },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidEip55 => write!(f, "Invalid EIP-55 Address encoding"),
            Error::InvalidUtf8(_) => write!(f, "Failed to parse bytes as utf8"),
            Error::InvalidHex(_) => write!(f, "Invalid hex character"),
            Error::InvalidAddressLength { got, expected } => write!(
                f,
                "Invalid address length, got {}, expected {}",
                got, expected
            ),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::InvalidHex(inner) => Some(inner),
            Error::InvalidUtf8(inner) => Some(inner),
            _ => None,
        }
    }
}

impl From<Utf8Error> for Error {
    fn from(e: Utf8Error) -> Self {
        Error::InvalidUtf8(e)
    }
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Error::InvalidHex(e)
    }
}
