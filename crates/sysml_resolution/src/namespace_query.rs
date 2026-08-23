//! Exact Namespace element-valued derivations over canonical structural facts.
//!
//! The resolver keeps membership relationships compactly as declaration-aligned records rather
//! than inventing public relationship identities. This API therefore exposes only exact final
//! element projections whose result can be represented without losing that distinction.

use crate::{ElementRelationship, SymbolIdentity};

pub use sysml_contract::NamespaceDerivedElementCollection;

/// The exact `importedElement` projection of one canonical NamespaceImport reference.
///
/// Imports are anonymous in the concrete language, so owner-scoped query results retain the
/// canonical import identity alongside the authored relationship. This keeps the source
/// addressable without fabricating a display-name key, while preserving the relationship's typed
/// target outcome.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamespaceImportDerivedElement {
    pub import: SymbolIdentity,
    pub relationship: ElementRelationship,
}
