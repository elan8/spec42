use core::fmt;
use std::error::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutError {
    Validation(String),
    Unsupported(String),
    Routing(String),
    Internal(String),
}

impl fmt::Display for LayoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Validation(message) => write!(f, "validation error: {message}"),
            Self::Unsupported(message) => write!(f, "unsupported: {message}"),
            Self::Routing(message) => write!(f, "routing error: {message}"),
            Self::Internal(message) => write!(f, "internal error: {message}"),
        }
    }
}

impl Error for LayoutError {}
