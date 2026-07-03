//! Structured errors for the host embedding boundary.

use serde::Serialize;
use thiserror::Error;

mod map;

pub(crate) use map::{
    map_language_service_error, map_provider_error, map_render_snapshot_error, map_view_error,
};

#[derive(Debug, Error, Serialize)]
#[serde(tag = "code", content = "details")]
pub enum WorkspaceError {
    #[error("invalid_document_uri: {message}")]
    InvalidDocumentUri {
        #[serde(rename = "message")]
        message: String,
    },
    #[error("parser_failure: {message}")]
    ParserFailure {
        uri: Option<String>,
        message: String,
    },
    #[error("unresolved_library_environment: {message}")]
    UnresolvedLibraryEnvironment { message: String },
    #[error("unsupported_view: {view}: {message}")]
    UnsupportedView { view: String, message: String },
    #[error("cancelled")]
    Cancelled,
    #[error("resource_limit_exceeded: {limit}: {message}")]
    ResourceLimitExceeded { limit: String, message: String },
    #[error("internal_invariant_failure: {message}")]
    InternalInvariantFailure { message: String },
}

impl WorkspaceError {
    pub fn invalid_document_uri(message: impl Into<String>) -> Self {
        Self::InvalidDocumentUri {
            message: message.into(),
        }
    }

    pub fn parser_failure(uri: Option<impl Into<String>>, message: impl Into<String>) -> Self {
        Self::ParserFailure {
            uri: uri.map(Into::into),
            message: message.into(),
        }
    }

    pub fn unresolved_library_environment(message: impl Into<String>) -> Self {
        Self::UnresolvedLibraryEnvironment {
            message: message.into(),
        }
    }

    pub fn unsupported_view(view: impl Into<String>, message: impl Into<String>) -> Self {
        Self::UnsupportedView {
            view: view.into(),
            message: message.into(),
        }
    }

    pub fn cancelled() -> Self {
        Self::Cancelled
    }

    pub fn resource_limit_exceeded(limit: impl Into<String>, message: impl Into<String>) -> Self {
        Self::ResourceLimitExceeded {
            limit: limit.into(),
            message: message.into(),
        }
    }

    pub fn internal_invariant_failure(message: impl Into<String>) -> Self {
        Self::InternalInvariantFailure {
            message: message.into(),
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidDocumentUri { .. } => "invalid_document_uri",
            Self::ParserFailure { .. } => "parser_failure",
            Self::UnresolvedLibraryEnvironment { .. } => "unresolved_library_environment",
            Self::UnsupportedView { .. } => "unsupported_view",
            Self::Cancelled => "cancelled",
            Self::ResourceLimitExceeded { .. } => "resource_limit_exceeded",
            Self::InternalInvariantFailure { .. } => "internal_invariant_failure",
        }
    }
}

pub type WorkspaceResult<T> = Result<T, WorkspaceError>;
