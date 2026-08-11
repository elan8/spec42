//! Concrete cache artifact payloads (`UNIFY_CACHE_PLAN.md` §6.2, §6.3, §6.4).
//!
//! This module defines the three artifact kinds that plan step 4 permits building ahead of the
//! semantic-graph round-trip prerequisites (`ROUNDTRIP_SEMGRAPH_PREREQS.md` §8): [`ParseOutcome`],
//! [`LibraryIndex`], and [`LibraryClosure`]. `LibrarySemanticGraph` and `WorkspaceSemanticGraph`
//! remain undefined here; they stay gated behind the semantic-node attribute bag removal (B9) and
//! `SemanticGraphRecordV1` (B5), neither of which is complete on this branch.
//!
//! Nothing in production wires these artifacts to a call site yet (plan step 5,
//! `SemanticBuildService`, is out of scope for this slice). They exist as standalone, fully
//! tested `CacheArtifact` implementations ready for that routing work.

pub mod library_closure;
pub mod library_index;
pub mod parse_outcome;

pub use library_closure::{
    LibraryClosure, LibraryClosureIdentity, LibraryClosurePolicy, SelectedLibraryFile,
    WorkspacePackageInfluence, LIBRARY_CLOSURE_ALGORITHM_VERSION,
};
pub use library_index::{
    LibraryFileIdentity, LibraryImportFact, LibraryIndex, LibraryIndexIdentity, LibraryIndexStatus,
    LibraryPackageFact, LibraryTypeReferenceFact, LIBRARY_INDEX_ALGORITHM_VERSION,
    LIBRARY_INDEX_SCHEMA_VERSION,
};
pub use parse_outcome::{
    ParseDiagnostic, ParseDiagnosticCategory, ParseDiagnosticRange, ParseDiagnosticSeverity,
    ParseMode, ParseOutcome, ParseOutcomeIdentity, ParseStatus, PARSE_ALGORITHM_VERSION,
    PARSE_DIAGNOSTIC_SCHEMA_VERSION, PARSE_OPTIONS_VERSION,
};
