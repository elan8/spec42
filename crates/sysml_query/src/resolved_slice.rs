//! Opaque facade for the parser-owned resolved semantic slice.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceKind {
    Workspace,
    StandardLibrary,
    Library,
    External,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceDocument {
    inner: sysml_resolution::SourceInput,
}

impl SourceDocument {
    pub fn from_memory_path(
        namespace: &str,
        path: &str,
        content: String,
        source_kind: SourceKind,
    ) -> Result<Self, SourceError> {
        let normalized = path.trim_start_matches('/').replace('\\', "/");
        if namespace.is_empty() || normalized.is_empty() {
            return Err(SourceError("source identity must not be empty"));
        }
        Ok(Self {
            inner: sysml_resolution::SourceInput::new(
                format!("memory://{namespace}/{normalized}"),
                content,
                source_kind.into(),
            ),
        })
    }
}

impl From<SourceKind> for sysml_resolution::SourceKind {
    fn from(kind: SourceKind) -> Self {
        match kind {
            SourceKind::Workspace => Self::Workspace,
            SourceKind::StandardLibrary => Self::StandardLibrary,
            SourceKind::Library => Self::Library,
            SourceKind::External => Self::External,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceError(&'static str);

impl fmt::Display for SourceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0)
    }
}

impl std::error::Error for SourceError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstructionStrategy {
    Sequential,
    Parallel,
}

#[derive(Debug)]
pub struct BuildRequest {
    inner: sysml_resolution::BuildRequest,
}

impl BuildRequest {
    pub fn resolved(
        sources: Vec<SourceDocument>,
        construction: ConstructionStrategy,
    ) -> Result<Self, BuildError> {
        let schedule = match construction {
            ConstructionStrategy::Sequential => sysml_resolution::ConstructionSchedule::Sequential,
            ConstructionStrategy::Parallel => sysml_resolution::ConstructionSchedule::Parallel,
        };
        sysml_resolution::BuildRequest::new(
            sources.into_iter().map(|source| source.inner).collect(),
            schedule,
            "parser-owned-resolution-v1",
        )
        .map(|inner| Self { inner })
        .map_err(BuildError)
    }
}

/// Opaque published semantic state. Share it behind `Arc`; do not duplicate its owner.
///
/// ```compile_fail
/// fn requires_clone<T: Clone>() {}
/// requires_clone::<sysml_query::resolved_slice::PublishedModel>();
/// ```
#[derive(Debug)]
pub struct PublishedModel {
    inner: sysml_resolution::PublishedResolution,
}

pub fn build(request: BuildRequest) -> Result<PublishedModel, BuildError> {
    sysml_resolution::build(request.inner)
        .map(|inner| PublishedModel { inner })
        .map_err(BuildError)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuildError(sysml_resolution::BuildFailure);

impl fmt::Display for BuildError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl std::error::Error for BuildError {}

impl PublishedModel {
    pub fn debug(&self) -> DebugQueries<'_> {
        DebugQueries { model: &self.inner }
    }
}

pub struct DebugQueries<'a> {
    model: &'a sysml_resolution::PublishedResolution,
}

impl DebugQueries<'_> {
    pub fn write_semantic_sexpr(&self, output: &mut dyn fmt::Write) -> fmt::Result {
        self.model.debug().write_semantic_sexpr(output)
    }

    pub fn write_diagnostics_sexpr(&self, output: &mut dyn fmt::Write) -> fmt::Result {
        self.model.debug().write_diagnostics_sexpr(output)
    }

    pub fn write_navigation_sexpr(&self, output: &mut dyn fmt::Write) -> fmt::Result {
        self.model.debug().write_navigation_sexpr(output)
    }
}

/// Storage and implementation models are not part of this facade.
///
/// ```compile_fail
/// use sysml_query::resolved_slice::{ResolutionResults, SemanticModelStorage};
/// ```
pub struct RawStorageIsNotPublic;
