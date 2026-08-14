//! Opaque facade for the parser-owned resolved semantic slice.

use std::fmt;

pub use sysml_resolution::{
    ElementKind, MembershipRole, NavigationTarget, OccurrenceRole, PublicationCompleteness,
    QueryOutcome, RenameOutcome, SourceLocation, StateSubactionKind, SymbolIdentity, TextPosition,
    TextRange, VisibleMember,
};

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
    pub fn from_uri(
        uri: &str,
        content: String,
        source_kind: SourceKind,
    ) -> Result<Self, SourceError> {
        if uri.is_empty() {
            return Err(SourceError("source identity must not be empty"));
        }
        Ok(Self {
            inner: sysml_resolution::SourceInput::new(uri, content, source_kind.into()),
        })
    }

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

    pub fn publication(&self) -> PublicationQueries<'_> {
        PublicationQueries { model: &self.inner }
    }

    pub fn navigation(&self) -> NavigationQueries<'_> {
        NavigationQueries { model: &self.inner }
    }

    pub fn edits(&self) -> EditQueries<'_> {
        EditQueries { model: &self.inner }
    }

    pub fn completion(&self) -> CompletionQueries<'_> {
        CompletionQueries { model: &self.inner }
    }
}

pub struct PublicationQueries<'a> {
    model: &'a sysml_resolution::PublishedResolution,
}

impl PublicationQueries<'_> {
    pub fn completeness(&self) -> PublicationCompleteness {
        self.model.completeness()
    }
}

pub struct NavigationQueries<'a> {
    model: &'a sysml_resolution::PublishedResolution,
}

impl NavigationQueries<'_> {
    pub fn target_at(
        &self,
        document: &str,
        position: TextPosition,
    ) -> QueryOutcome<NavigationTarget> {
        self.model.target_at(document, position)
    }

    pub fn references(
        &self,
        symbol: &SymbolIdentity,
        include_declaration: bool,
    ) -> QueryOutcome<Box<[SourceLocation]>> {
        self.model.references(symbol, include_declaration)
    }
}

pub struct EditQueries<'a> {
    model: &'a sysml_resolution::PublishedResolution,
}

impl EditQueries<'_> {
    pub fn prepare_rename(
        &self,
        document: &str,
        position: TextPosition,
        new_name: Option<&str>,
    ) -> RenameOutcome {
        self.model.prepare_rename(document, position, new_name)
    }
}

pub struct CompletionQueries<'a> {
    model: &'a sysml_resolution::PublishedResolution,
}

impl CompletionQueries<'_> {
    pub fn visible_members(
        &self,
        document: &str,
        position: TextPosition,
        qualifier: Option<&str>,
    ) -> QueryOutcome<Box<[VisibleMember]>> {
        self.model.visible_members(document, position, qualifier)
    }
}

pub struct DebugQueries<'a> {
    model: &'a sysml_resolution::PublishedResolution,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditorProbe {
    pub document: String,
    pub position: TextPosition,
    pub qualifier: Option<String>,
    pub rename_to: Option<String>,
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

    pub fn write_editor_queries_sexpr(
        &self,
        probes: &[EditorProbe],
        output: &mut dyn fmt::Write,
    ) -> fmt::Result {
        writeln!(output, "(editor-queries")?;
        for probe in probes {
            writeln!(
                output,
                "  (probe (document {:?}) (position {} {})",
                probe.document, probe.position.line, probe.position.character
            )?;
            let target = self.model.target_at(&probe.document, probe.position);
            write_target_outcome(output, "target", &target)?;
            if let QueryOutcome::Resolved(target)
            | QueryOutcome::Recovered(target)
            | QueryOutcome::UnsupportedWith(target) = &target
            {
                write_locations_outcome(
                    output,
                    "references",
                    &self.model.references(&target.symbol, true),
                )?;
            }
            write_rename_outcome(
                output,
                &self.model.prepare_rename(
                    &probe.document,
                    probe.position,
                    probe.rename_to.as_deref(),
                ),
            )?;
            write_members_outcome(
                output,
                &self.model.visible_members(
                    &probe.document,
                    probe.position,
                    probe.qualifier.as_deref(),
                ),
            )?;
            writeln!(output, "  )")?;
        }
        write!(output, ")")
    }
}

fn write_range(output: &mut dyn fmt::Write, range: TextRange) -> fmt::Result {
    write!(
        output,
        "(range (start {} {}) (end {} {}))",
        range.start.line, range.start.character, range.end.line, range.end.character
    )
}

fn write_location(output: &mut dyn fmt::Write, location: &SourceLocation) -> fmt::Result {
    write!(output, "(location (document {:?}) ", location.document)?;
    write_range(output, location.range)?;
    write!(output, " (role {:?}))", location.role)
}

fn write_target(output: &mut dyn fmt::Write, target: &NavigationTarget) -> fmt::Result {
    write!(output, "(candidate (name {:?}) ", target.name)?;
    write_location(output, &target.location)?;
    write!(output, ")")
}

fn write_target_outcome(
    output: &mut dyn fmt::Write,
    label: &str,
    outcome: &QueryOutcome<NavigationTarget>,
) -> fmt::Result {
    write!(output, "    ({label} ")?;
    match outcome {
        QueryOutcome::Resolved(target) => {
            write!(output, "(status resolved) ")?;
            write_target(output, target)?;
        }
        QueryOutcome::Recovered(target) => {
            write!(output, "(status recovery) ")?;
            write_target(output, target)?;
        }
        QueryOutcome::UnsupportedWith(target) => {
            write!(output, "(status unsupported) ")?;
            write_target(output, target)?;
        }
        QueryOutcome::Ambiguous(targets) => {
            write!(output, "(status ambiguous) (candidates")?;
            for target in targets {
                write!(output, " ")?;
                write_target(output, target)?;
            }
            write!(output, ")")?;
        }
        QueryOutcome::Unresolved => write!(output, "(status unresolved)")?,
        QueryOutcome::Unsupported => write!(output, "(status unsupported)")?,
        QueryOutcome::Recovery => write!(output, "(status recovery)")?,
        QueryOutcome::Incomplete => write!(output, "(status incomplete)")?,
    }
    writeln!(output, ")")
}

fn write_locations_outcome(
    output: &mut dyn fmt::Write,
    label: &str,
    outcome: &QueryOutcome<Box<[SourceLocation]>>,
) -> fmt::Result {
    write!(output, "    ({label} ")?;
    match outcome {
        QueryOutcome::Resolved(values)
        | QueryOutcome::Recovered(values)
        | QueryOutcome::UnsupportedWith(values) => {
            write!(output, "(locations")?;
            for value in values.iter() {
                write!(output, " ")?;
                write_location(output, value)?;
            }
            write!(output, ")")?;
        }
        _ => write!(output, "(status unavailable)")?,
    }
    writeln!(output, ")")
}

fn write_rename_outcome(output: &mut dyn fmt::Write, outcome: &RenameOutcome) -> fmt::Result {
    write!(output, "    (rename ")?;
    match outcome {
        RenameOutcome::Ready {
            name,
            range,
            occurrences,
            ..
        } => {
            write!(output, "(status ready) (name {name:?}) ")?;
            write_range(output, *range)?;
            write!(output, " (occurrences {})", occurrences.len())?;
        }
        RenameOutcome::Collision(targets) => {
            write!(output, "(status collision) (candidates")?;
            for target in targets.iter() {
                write!(output, " ")?;
                write_target(output, target)?;
            }
            write!(output, ")")?;
        }
        RenameOutcome::Ambiguous(targets) => {
            write!(output, "(status ambiguous) (candidates {}))", targets.len())?
        }
        RenameOutcome::InvalidName => write!(output, "(status invalid-name)")?,
        RenameOutcome::Unresolved => write!(output, "(status unresolved)")?,
        RenameOutcome::Unsupported => write!(output, "(status unsupported)")?,
        RenameOutcome::Recovery => write!(output, "(status recovery)")?,
        RenameOutcome::Incomplete => write!(output, "(status incomplete)")?,
    }
    writeln!(output, ")")
}

fn write_members_outcome(
    output: &mut dyn fmt::Write,
    outcome: &QueryOutcome<Box<[VisibleMember]>>,
) -> fmt::Result {
    write!(output, "    (visible-members ")?;
    match outcome {
        QueryOutcome::Resolved(values)
        | QueryOutcome::Recovered(values)
        | QueryOutcome::UnsupportedWith(values) => {
            write!(output, "(candidates")?;
            for value in values.iter() {
                write!(
                    output,
                    " (member (name {:?}) (qualified-name {:?}) (kind {:?})",
                    value.name,
                    value.qualified_name,
                    value.kind.as_str()
                )?;
                if let Some(role) = value.role {
                    write!(output, " (role {:?})", role.as_str())?;
                }
                write!(output, ")")?;
            }
            write!(output, ")")?;
        }
        _ => write!(output, "(status unavailable)")?,
    }
    writeln!(output, ")")
}

/// Storage and implementation models are not part of this facade.
///
/// ```compile_fail
/// use sysml_query::resolved_slice::{ResolutionResults, SemanticModelStorage};
/// ```
pub struct RawStorageIsNotPublic;

#[cfg(test)]
mod tests {
    use super::PublishedModel;

    #[test]
    fn immutable_publication_can_be_shared_by_async_hosts() {
        fn requires_send_sync<T: Send + Sync>() {}
        requires_send_sync::<PublishedModel>();
    }
}
