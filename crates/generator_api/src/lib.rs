//! Runtime-neutral contracts used by generator runtimes and other read-only consumers.

mod artifacts;
mod diagnostics;
mod model;
mod path;

pub use artifacts::{Artifact, ArtifactError, ArtifactLimits, ArtifactSet};
pub use diagnostics::{GeneratorDiagnostic, GeneratorDiagnosticLevel};
pub use model::{
    ElementDetail, ElementSummary, GeneratorModelView, ModelQueryError, MultiplicitySummary,
    QueryLimits, RelationshipSummary, SourceRange, GENERATOR_SEMANTIC_API_VERSION,
};
pub use path::{ArtifactPath, ArtifactPathError, MAX_ARTIFACT_PATH_BYTES, RESERVED_MANIFEST_NAME};
