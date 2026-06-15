use thiserror::Error;

#[derive(Debug, Error)]
pub enum KparError {
    #[error("failed to read {path}: {source}")]
    Io {
        path: String,
        source: std::io::Error,
    },
    #[error("invalid KPAR archive: {0}")]
    InvalidArchive(String),
    #[error("missing {0} in KPAR archive")]
    MissingFile(&'static str),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("checksum mismatch for {path}: expected {expected}, got {actual}")]
    ChecksumMismatch {
        path: String,
        expected: String,
        actual: String,
    },
    #[error("zip error: {0}")]
    Zip(String),
}

impl From<std::io::Error> for KparError {
    fn from(value: std::io::Error) -> Self {
        Self::Io {
            path: "<unknown>".to_string(),
            source: value,
        }
    }
}

pub type Result<T> = std::result::Result<T, KparError>;
