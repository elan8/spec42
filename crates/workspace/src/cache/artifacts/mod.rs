//! Concrete source-derived cache artifact payloads.
//!
//! This module defines the three source-derived artifact kinds: [`ParseOutcome`],
//! [`LibraryIndex`], and [`LibraryClosure`]. Immutable semantic publications are rebuilt from
//! dependency-complete inputs and are not serialized legacy-graph cache artifacts.
//!
//! Nothing in production wires these artifacts to a call site yet. They exist as standalone,
//! fully tested `CacheArtifact` implementations ready for a future explicitly owned routing path.

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
