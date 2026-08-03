//! Runtime-neutral contracts used by generator runtimes and other read-only consumers.

mod artifacts;
mod diagnostics;
mod model;

pub use artifacts::{
    Artifact, ArtifactError, ArtifactLimits, ArtifactSet, MAX_ARTIFACT_PATH_BYTES,
};
pub use diagnostics::{GeneratorDiagnostic, GeneratorDiagnosticLevel};
pub use model::{
    ElementDetail, ElementSummary, GeneratorModelView, ModelQueryError, MultiplicitySummary,
    QueryLimits, RelationshipSummary, SourceRange, GENERATOR_SEMANTIC_API_VERSION,
};
