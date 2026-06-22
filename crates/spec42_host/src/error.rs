use thiserror::Error;

#[derive(Debug, Error)]
pub enum Spec42HostError {
    #[error("{code}: {message}")]
    Coded { code: &'static str, message: String },
}

impl Spec42HostError {
    pub fn unresolved_library_environment(message: impl Into<String>) -> Self {
        Self::Coded {
            code: "unresolved_library_environment",
            message: message.into(),
        }
    }

    pub fn invalid_library_path(message: impl Into<String>) -> Self {
        Self::Coded {
            code: "invalid_document_uri",
            message: message.into(),
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            Self::Coded { code, .. } => code,
        }
    }
}

impl From<String> for Spec42HostError {
    fn from(message: String) -> Self {
        Self::unresolved_library_environment(message)
    }
}

pub type HostResult<T> = Result<T, Spec42HostError>;
