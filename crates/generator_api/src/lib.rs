#![recursion_limit = "256"]

//! Runtime-neutral contracts used by generator runtimes and other read-only consumers.

mod artifacts;
mod diagnostics;
mod model;
mod path;

pub use artifacts::{Artifact, ArtifactError, ArtifactLimits, ArtifactSet};
pub use diagnostics::{GeneratorDiagnostic, GeneratorDiagnosticLevel};
pub use model::{
    ElementDetail, ElementSummary, GeneratorModelView, ModelQueryError, MultiplicitySummary,
    QueryLimits, RelationshipSummary, RequirementUsageTypingSummary,
    RequirementVerificationSummary, SatisfyEndpointSummary, SatisfyPolaritySummary,
    SatisfyRelationshipSummary, SourceRange, TypingProvenanceSummary, VerificationOutcomeSummary,
    VerificationRequirementSummary, GENERATOR_SEMANTIC_API_VERSION,
};
pub type GeneratorPublicationCompleteness = sysml_query::resolved_slice::PublicationCompleteness;
pub use path::{
    ArtifactPath, ArtifactPathError, MAX_ARTIFACT_PATH_BYTES, MAX_ARTIFACT_SEGMENT_BYTES,
    RESERVED_MANIFEST_NAME,
};
pub use spec42_generator_protocol::{
    DiagramEdge, DiagramEdgeKind, DiagramElement, DiagramIncompleteReason, DiagramRelationship,
    DiagramRelationshipTarget, DiagramSemanticReference, DiagramSourceDomain, DiagramViewKind,
    DiagramViewMetadata, DiagramViewProjection, DiagramViewSummary,
};
