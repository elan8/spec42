//! Opaque parser-owned semantic construction and batch resolution.
//!
//! Syntax documents, dense IDs, semantic storage, solver state, and indexes remain private. The
//! public contract accepts immutable source inputs and streams owner-defined canonical output.

use std::fmt;

use source_identity::{ContentDigest, RootDigest, SourceManifest, SourceManifestEntry, SourceRole};

mod model;

use model::resolver::ResolvedSemanticModel;
use model::{BuildSchedule, CoordinatorError, OwnedSourceRecord, SemanticModelBuildCoordinator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceKind {
    Workspace,
    StandardLibrary,
    Library,
    External,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceInput {
    identity: Box<str>,
    content: String,
    kind: SourceKind,
    content_digest: ContentDigest,
}

impl SourceInput {
    pub fn new(identity: impl Into<Box<str>>, content: String, kind: SourceKind) -> Self {
        let content_digest = ContentDigest::of_bytes(content.as_bytes());
        Self {
            identity: identity.into(),
            content,
            kind,
            content_digest,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConstructionSchedule {
    Sequential,
    Parallel,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicationIdentity {
    source_digest: RootDigest,
    semantic_contract_version: Box<str>,
}

impl PublicationIdentity {
    pub fn source_digest(&self) -> &RootDigest {
        &self.source_digest
    }

    pub fn semantic_contract_version(&self) -> &str {
        &self.semantic_contract_version
    }
}

#[derive(Debug)]
pub struct BuildRequest {
    sources: Vec<SourceInput>,
    schedule: ConstructionSchedule,
    identity: PublicationIdentity,
}

impl BuildRequest {
    pub fn new(
        mut sources: Vec<SourceInput>,
        schedule: ConstructionSchedule,
        semantic_contract_version: impl Into<Box<str>>,
    ) -> Result<Self, BuildFailure> {
        sources.sort_unstable_by(|left, right| left.identity.cmp(&right.identity));
        if sources
            .windows(2)
            .any(|pair| pair[0].identity == pair[1].identity)
        {
            return Err(BuildFailure::DuplicateSourceIdentity);
        }
        let semantic_contract_version = semantic_contract_version.into();
        let entries = sources
            .iter()
            .map(|source| SourceManifestEntry {
                uri: source.identity.to_string(),
                path_hint: None,
                role: source_role(source.kind),
                content_digest: source.content_digest,
                byte_len: source.content.len() as u64,
                library_root_slot: None,
                relative_path: None,
            })
            .collect();
        let source_digest = SourceManifest::new(entries, Vec::new()).root_digest();
        Ok(Self {
            sources,
            schedule,
            identity: PublicationIdentity {
                source_digest,
                semantic_contract_version,
            },
        })
    }

    pub fn identity(&self) -> &PublicationIdentity {
        &self.identity
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildFailure {
    DuplicateSourceIdentity,
    ConstructionFailed,
}

impl fmt::Display for BuildFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for BuildFailure {}

/// An opaque, immutable resolved publication.
///
/// Publications are shared by reference; cloning the semantic owner is intentionally impossible.
///
/// ```compile_fail
/// fn requires_clone<T: Clone>() {}
/// requires_clone::<sysml_resolution::PublishedResolution>();
/// ```
///
/// Dense storage identities and indexes are private implementation details.
///
/// ```compile_fail
/// use sysml_resolution::{DeclarationId, ResolutionResults, SemanticModelStorage};
/// ```
#[derive(Debug)]
pub struct PublishedResolution {
    identity: PublicationIdentity,
    model: ResolvedSemanticModel,
}

pub fn build(request: BuildRequest) -> Result<PublishedResolution, BuildFailure> {
    let schedule = match request.schedule {
        ConstructionSchedule::Sequential => BuildSchedule::Sequential,
        ConstructionSchedule::Parallel => BuildSchedule::Parallel,
    };
    let sources = request
        .sources
        .into_iter()
        .map(|source| OwnedSourceRecord {
            identity: source.identity,
            content: source.content,
        })
        .collect();
    let model =
        SemanticModelBuildCoordinator::build(sources, schedule).map_err(|error| match error {
            CoordinatorError::DuplicateSourceIdentity => BuildFailure::DuplicateSourceIdentity,
            CoordinatorError::ConstructionFailed => BuildFailure::ConstructionFailed,
        })?;
    Ok(PublishedResolution {
        identity: request.identity,
        model,
    })
}

impl PublishedResolution {
    pub fn identity(&self) -> &PublicationIdentity {
        &self.identity
    }

    pub fn debug(&self) -> DebugQueries<'_> {
        DebugQueries {
            identity: &self.identity,
            model: &self.model,
        }
    }
}

pub struct DebugQueries<'a> {
    identity: &'a PublicationIdentity,
    model: &'a ResolvedSemanticModel,
}

impl DebugQueries<'_> {
    pub fn write_semantic_sexpr(&self, output: &mut dyn fmt::Write) -> fmt::Result {
        self.model.write_semantic_sexpr(
            &self.identity.source_digest,
            &self.identity.semantic_contract_version,
            output,
        )
    }

    pub fn write_diagnostics_sexpr(&self, output: &mut dyn fmt::Write) -> fmt::Result {
        self.model.write_diagnostics_sexpr(output)
    }

    pub fn write_navigation_sexpr(&self, output: &mut dyn fmt::Write) -> fmt::Result {
        self.model.write_navigation_sexpr(output)
    }
}

fn source_role(kind: SourceKind) -> SourceRole {
    match kind {
        SourceKind::Workspace => SourceRole::Workspace,
        SourceKind::StandardLibrary => SourceRole::StandardLibrary,
        SourceKind::Library => SourceRole::Library,
        SourceKind::External => SourceRole::External,
    }
}

/// Raw semantic storage is deliberately inaccessible.
///
/// ```compile_fail
/// use sysml_resolution::{DeclarationId, ResolutionResults, SemanticModelStorage};
/// ```
///
/// ```compile_fail
/// fn require_clone<T: Clone>() {}
/// require_clone::<sysml_resolution::BuildRequest>();
/// require_clone::<sysml_resolution::PublishedResolution>();
/// ```
pub struct RawStorageIsNotPublic;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construction_schedule_does_not_change_semantic_publication_identity() {
        let sequential =
            BuildRequest::new(Vec::new(), ConstructionSchedule::Sequential, "contract-v1").unwrap();
        let parallel =
            BuildRequest::new(Vec::new(), ConstructionSchedule::Parallel, "contract-v1").unwrap();

        assert_eq!(sequential.identity(), parallel.identity());
    }

    fn semantic_sexpr_for(source: &str) -> String {
        let request = BuildRequest::new(
            vec![SourceInput::new(
                "memory://test.sysml",
                source.to_string(),
                SourceKind::Workspace,
            )],
            ConstructionSchedule::Sequential,
            "contract-v1",
        )
        .unwrap();
        let published = build(request).unwrap();
        let mut output = String::new();
        published.debug().write_semantic_sexpr(&mut output).unwrap();
        output
    }

    /// A nested `part` usage inside an `attribute def` body (BNF `AttributeBodyElement::PartUsage`,
    /// shared with `item def`/`item` usage bodies per the OMG `14c-Language Extensions.sysml`
    /// FMEA library example) must lower as its own `part` declaration, not fall through to
    /// `unsupported_attribute_member`.
    #[test]
    fn nested_part_usage_inside_attribute_def_body_lowers_as_part() {
        let sexpr = semantic_sexpr_for(
            "package P { attribute def Show { part frame : Frame; attribute def Frame; } }",
        );
        assert!(
            sexpr.contains("P::Show::frame"),
            "expected nested part declaration, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_attribute_member"),
            "did not expect unsupported_attribute_member, got: {sexpr}"
        );
    }

    /// A nested `item` usage inside an `attribute def` body (BNF
    /// `AttributeBodyElement::ItemUsage`, resolved upstream in `0757de13` --
    /// UPSTREAM_PARSER_GAPS.md #11) must lower as its own `item` declaration via the
    /// already-existing `lower_item_usage`, not fall through to `unsupported_attribute_member`.
    #[test]
    fn nested_item_usage_inside_attribute_def_body_lowers_as_item() {
        let sexpr = semantic_sexpr_for(
            "package P { attribute def Show { item picture : Picture; attribute def Picture; } }",
        );
        assert!(
            sexpr.contains("P::Show::picture"),
            "expected nested item declaration, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_attribute_member"),
            "did not expect unsupported_attribute_member, got: {sexpr}"
        );
    }

    /// A nested `occurrence` usage inside an `attribute def` body (BNF
    /// `AttributeBodyElement::OccurrenceUsage`, e.g. the FMEA library's `#prevention occurs;`-style
    /// members) must lower as its own `occurrence` declaration via the already-existing
    /// `lower_occurrence_usage`, not fall through to `unsupported_attribute_member`.
    #[test]
    fn nested_occurrence_usage_inside_attribute_def_body_lowers_as_occurrence() {
        let sexpr = semantic_sexpr_for("package P { attribute def Show { occurrence flash; } }");
        assert!(
            sexpr.contains("P::Show::flash"),
            "expected nested occurrence declaration, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_attribute_member"),
            "did not expect unsupported_attribute_member, got: {sexpr}"
        );
    }

    /// A nested `exhibit` state usage inside an `occurrence def`/usage body (BNF
    /// `OccurrenceBodyElement::StateUsage`, e.g. `exhibit vehicleStates.on;` from the OMG spec
    /// Annex's individuals/snapshots examples) must lower as its own `state` declaration via the
    /// already-existing `lower_state_usage`, not fall through to
    /// `unsupported_occurrence_definition_member`.
    #[test]
    fn nested_state_usage_inside_occurrence_def_body_lowers_as_state() {
        let sexpr =
            semantic_sexpr_for("package P { occurrence def O { exhibit vehicleStates.on; } }");
        assert!(
            sexpr.contains("(kind state)"),
            "expected nested state declaration, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_occurrence_definition_member"),
            "did not expect unsupported_occurrence_definition_member, got: {sexpr}"
        );
    }
}
