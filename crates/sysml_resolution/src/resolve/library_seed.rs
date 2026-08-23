//! Phase 3: admitting and seeding a settled library stratum.

use crate::lower::storage::SemanticModelStorage;
use crate::resolve::results::ResolutionStatus;
use source_identity::SourceRole;

/// What a settled library build hands to the publications that follow it.
///
/// The outcomes are indexed by authored-reference ordinal. That is only meaningful because sources
/// are admitted library-first and lowering is per-document: the library's declarations and
/// references occupy exactly the same dense prefix in a workspace build as they did in the build
/// that settled them.
///
/// The two name sets are the evidence for whether reusing those outcomes is sound at all. See
/// [`SettledLibrary::admits`].
#[derive(Debug)]
pub(crate) struct SettledLibrary {
    pub(crate) outcomes: Box<[ResolutionStatus]>,
    /// Names declared at the library's own root, which a workspace declaration must not shadow or
    /// duplicate.
    pub(crate) root_names: std::collections::BTreeSet<Box<str>>,
    /// First path segments of every library reference the library-only build left unresolved or
    /// ambiguous. A workspace root with one of these names could newly satisfy -- or newly make
    /// ambiguous -- a reference whose outcome is about to be reused.
    pub(crate) unsettled_roots: std::collections::BTreeSet<Box<str>>,
}

impl SettledLibrary {
    /// Whether these outcomes may be reused for a publication containing `storage`.
    ///
    /// Global-root lookup is the one channel through which a workspace declaration can change what
    /// a library reference resolves to, and it is reachable in exactly two ways: a workspace root
    /// sharing a library root's name, or a workspace root answering a lookup the library itself
    /// left unsettled. Both are name comparisons over the roots, so the check costs a walk of the
    /// workspace's top-level declarations rather than a re-solve.
    pub(crate) fn admits(&self, storage: &SemanticModelStorage) -> bool {
        if self.outcomes.len() > storage.references.len() {
            return false;
        }
        // The prefix must still be the library's. If lowering ever stopped putting library
        // documents first, seeding would silently answer for the wrong references.
        for reference in storage.references.iter().take(self.outcomes.len()) {
            let Some(declaration) = storage.declaration(reference.source) else {
                return false;
            };
            let Some(document) = storage.document(declaration.document) else {
                return false;
            };
            if document.role == SourceRole::Workspace {
                return false;
            }
        }
        for (index, declaration) in storage.declarations.iter().enumerate() {
            if declaration.owner.is_some() {
                continue;
            }
            let Some(document) = storage.document(declaration.document) else {
                return false;
            };
            if document.role != SourceRole::Workspace {
                continue;
            }
            let Some(name) = declaration.name.and_then(|name| storage.symbol(name)) else {
                continue;
            };
            if self.root_names.contains(name) || self.unsettled_roots.contains(name) {
                let _ = index;
                return false;
            }
        }
        true
    }
}
