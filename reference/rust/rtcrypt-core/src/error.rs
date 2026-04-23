use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum RtError {
    InvalidInput(String),
    ValidationFailed(String),
    KeyNotFound(String),
    Unauthorized(String),
    Internal(String),
}

impl Display for RtError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidInput(v) => write!(f, "invalid input: {v}"),
            Self::ValidationFailed(v) => write!(f, "validation failed: {v}"),
            Self::KeyNotFound(v) => write!(f, "key not found: {v}"),
            Self::Unauthorized(v) => write!(f, "unauthorized: {v}"),
            Self::Internal(v) => write!(f, "internal error: {v}"),
        }
    }
}

impl std::error::Error for RtError {}
