use std::{ fmt, error::Error };

#[derive(Clone, Hash, Debug)]
pub enum ParseError {
    UnknownKey(String),
    InvalidKeyLength(String),
    Unexpected
}

impl Error for ParseError { }

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UnknownKey(x) => write!(f, "Unknown key identifier: {}", x),
            Self::InvalidKeyLength(x) => write!(f, "Invalid key length. Expected length of 1. Given: {}", x),
            Self::Unexpected => write!(f, "An unexpected error has occured")
        }
    }
}