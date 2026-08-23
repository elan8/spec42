//! Phase 6: stable element identity, encoded once per publication.

use crate::lower::storage::SemanticModelStorage;
use crate::model::render as writer;
use crate::model::DeclarationId;
use crate::model::DeclarationKind;
use crate::model::DocumentId;
use crate::model::SymbolId;
use crate::resolve::results::ResolutionError;
use hashbrown::HashTable;
use std::collections::hash_map::RandomState;
use std::collections::BTreeMap;
use std::hash::BuildHasher;

/// Version tag on the canonical structural identity encoding.
///
/// The encoding is opaque to consumers, but it is compared for equality across builds of the same
/// sources, so a change to its shape has to be a deliberate, visible one.
pub(crate) const IDENTITY_ENCODING_VERSION: &str = "element/v1";

/// Canonical structural identity for every declaration, plus the lookup index over it.
///
/// An identity is the owning document followed by the ordered scope path from that document's
/// root. Every segment carries the declaration's kind, plus either its authored name -- with an
/// occurrence ordinal distinguishing identically named siblings -- or, when the declaration is
/// anonymous, its owner-local ordinal.
///
/// The shape follows the two reference implementations, which converged on it independently:
///
/// - The OMG Pilot's `Element::path()` (`Element_path_InvocationDelegate`) returns the qualified
///   name when there is one and otherwise the owner's path plus a positional index; its
///   `qualifiedName` derivation (`Element_qualifiedName_SettingDelegate`) yields null for an
///   unnamed element, for any element under an unnamed ancestor, and for every same-named sibling
///   after the first. The occurrence ordinal below is that last clause.
/// - The sibling `sysml-compiler`'s `buildStablePath` writes `[tag][name | '#' + sibling_index]`
///   per level from the root, hashing the result into a UUIDv5. The kind on every segment is that
///   tag.
///
/// Both are needed, and each covers a case the other does not. Without the kind, a `metadata def
/// SafetyFeature` and the `metadata SafetyFeature about ...` annotating it collide. Without the
/// occurrence ordinal, a source that authors two identically named siblings leaves the second one
/// unaddressable. Together they make the identity total: every declaration has one, and no two
/// share one.
///
/// The spec calls for exactly this much and no more -- `Element::elementId` is "set by tooling",
/// and `Element::path()` is "a unique location description in containment structure".
pub(crate) struct IdentityIndex {
    /// One canonical identity string per `DeclarationId`, parallel to `storage.declarations`.
    pub(crate) text: Box<[Box<str>]>,
    /// Each declaration's ordinal among its identically named, same-kind siblings.
    pub(crate) occurrences: Box<[u32]>,
    /// Whether this declaration's identity is recoverable from its qualified name alone, so the
    /// writer may use the readable shorthand: every segment named, every occurrence ordinal zero,
    /// and no other declaration in the publication sharing the same document and name path.
    pub(crate) shorthand: Box<[bool]>,
    /// Head of each distinct identity's declaration chain.
    ///
    /// Retained even though the identity is total, so a storage invariant broken elsewhere
    /// surfaces as an explicit ambiguous outcome rather than an arbitrary pick.
    pub(crate) heads: HashTable<DeclarationId>,
    pub(crate) hash_builder: RandomState,
    /// Next declaration sharing this one's identity, in ascending `DeclarationId` order.
    pub(crate) next: Box<[Option<DeclarationId>]>,
}

impl std::fmt::Debug for IdentityIndex {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // A disposable lookup accelerator; its bucket layout is not semantic content and printing
        // it would only add noise to a published model's debug output.
        formatter
            .debug_struct("IdentityIndex")
            .field("declarations", &self.text.len())
            .finish_non_exhaustive()
    }
}

impl IdentityIndex {
    pub(crate) fn build(storage: &SemanticModelStorage) -> Result<Self, ResolutionError> {
        let occurrences = name_occurrences(storage)?;
        let mut text: Vec<Box<str>> = Vec::with_capacity(storage.declarations.len());
        let mut name_paths: Vec<Option<usize>> = Vec::with_capacity(storage.declarations.len());
        let mut name_path_ids = std::collections::HashMap::new();
        name_path_ids
            .try_reserve(storage.declarations.len())
            .map_err(|_| ResolutionError::Capacity)?;
        let mut name_path_counts: Vec<u32> = Vec::new();
        for index in 0..storage.declarations.len() {
            let id = DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
            let declaration = storage
                .declaration(id)
                .ok_or(ResolutionError::InvalidStorage)?;
            let document = storage
                .document(declaration.document)
                .ok_or(ResolutionError::InvalidStorage)?;
            let mut identity = if let Some(owner) = declaration.owner {
                let owner_declaration = storage
                    .declaration(owner)
                    .ok_or(ResolutionError::InvalidStorage)?;
                if owner.index() >= index || owner_declaration.document != declaration.document {
                    return Err(ResolutionError::InvalidStorage);
                }
                text[owner.index()].to_string()
            } else {
                let mut identity = String::from(IDENTITY_ENCODING_VERSION);
                push_identity_field(&mut identity, &document.identity);
                identity
            };
            push_identity_segment(storage, id, &occurrences, &mut identity)?;
            let name_path = if declaration
                .owner
                .is_some_and(|owner| name_paths.get(owner.index()).copied().flatten().is_none())
            {
                None
            } else if let Some(name) = declaration.name {
                let parent = declaration
                    .owner
                    .and_then(|owner| name_paths[owner.index()]);
                let key = (declaration.document, parent, name);
                let path = match name_path_ids.get(&key) {
                    Some(path) => *path,
                    None => {
                        let path = name_path_counts.len();
                        name_path_counts.push(0);
                        name_path_ids.insert(key, path);
                        path
                    }
                };
                name_path_counts[path] = name_path_counts[path]
                    .checked_add(1)
                    .ok_or(ResolutionError::Capacity)?;
                Some(path)
            } else {
                None
            };
            text.push(identity.into_boxed_str());
            name_paths.push(name_path);
        }
        // A qualified name identifies a declaration only when nothing else in the publication
        // renders the same one -- two same-named siblings of different kinds otherwise share it.
        let shorthand = (0..storage.declarations.len())
            .map(|index| name_paths[index].is_some_and(|path| name_path_counts[path] == 1))
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let mut next = vec![None; text.len()];
        let hash_builder = RandomState::default();
        let mut heads: HashTable<DeclarationId> = HashTable::new();
        // Reverse order so each chain ends up in ascending `DeclarationId` order, which keeps an
        // ambiguous outcome's candidate list canonically ordered without a later sort.
        for index in (0..text.len()).rev() {
            let id = DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
            let hash = hash_builder.hash_one(text[index].as_ref());
            let matches = |candidate: &DeclarationId| text[candidate.index()] == text[index];
            if let Some(existing) = heads.find_mut(hash, matches) {
                next[index] = Some(*existing);
                *existing = id;
            } else {
                let rehash = |candidate: &DeclarationId| {
                    hash_builder.hash_one(text[candidate.index()].as_ref())
                };
                heads
                    .try_reserve(1, rehash)
                    .map_err(|_| ResolutionError::Capacity)?;
                heads.insert_unique(hash, id, rehash);
            }
        }
        Ok(Self {
            text: text.into_boxed_slice(),
            occurrences,
            shorthand,
            heads,
            hash_builder,
            next: next.into_boxed_slice(),
        })
    }

    /// Whether the writer may identify this declaration by qualified name alone.
    pub(crate) fn allows_qualified_name_shorthand(&self, id: DeclarationId) -> bool {
        self.shorthand.get(id.index()).copied().unwrap_or(false)
    }

    /// This declaration's ordinal among its identically named, same-kind siblings.
    pub(crate) fn name_occurrence(&self, id: DeclarationId) -> Option<u32> {
        self.occurrences.get(id.index()).copied()
    }

    pub(crate) fn identity(&self, id: DeclarationId) -> Option<&str> {
        self.text.get(id.index()).map(AsRef::as_ref)
    }

    /// Every declaration carrying `identity`, in ascending `DeclarationId` order.
    pub(crate) fn declarations(&self, identity: &str) -> Vec<DeclarationId> {
        let hash = self.hash_builder.hash_one(identity);
        let Some(head) = self
            .heads
            .find(hash, |candidate| {
                self.text[candidate.index()].as_ref() == identity
            })
            .copied()
        else {
            return Vec::new();
        };
        let mut chain = vec![head];
        let mut cursor = self.next[head.index()];
        while let Some(current) = cursor {
            chain.push(current);
            cursor = self.next[current.index()];
        }
        chain
    }
}

/// Appends one length-prefixed field, so a document identity or an authored name containing any
/// byte sequence -- including the encoding's own punctuation -- cannot forge a segment boundary.
pub(crate) fn push_identity_field(output: &mut String, value: &str) {
    output.push_str(&value.len().to_string());
    output.push(':');
    output.push_str(value);
}

/// The ownership chain from `id` up to the document root, ordered leaf-first.
/// The occurrence ordinal of each named declaration among its identically named siblings of the
/// same kind, in declaration order.
///
/// Zero for the overwhelmingly common case of a name that is unique in its scope, so a later
/// duplicate never disturbs the identity of the declaration that was already there. This mirrors
/// the Pilot, whose `qualifiedName` stays valid for the first same-named member and falls through
/// to a positional path for the rest.
pub(crate) fn name_occurrences(
    storage: &SemanticModelStorage,
) -> Result<Box<[u32]>, ResolutionError> {
    let mut seen: BTreeMap<(DocumentId, Option<DeclarationId>, DeclarationKind, SymbolId), u32> =
        BTreeMap::new();
    let mut occurrences = Vec::with_capacity(storage.declarations.len());
    for declaration in storage.declarations.iter() {
        let occurrence = match declaration.name {
            Some(name) => {
                let slot = seen
                    .entry((
                        declaration.document,
                        declaration.owner,
                        declaration.kind,
                        name,
                    ))
                    .or_insert(0);
                let value = *slot;
                *slot = slot.checked_add(1).ok_or(ResolutionError::Capacity)?;
                value
            }
            None => 0,
        };
        occurrences.push(occurrence);
    }
    Ok(occurrences.into_boxed_slice())
}

pub(crate) fn push_identity_segment(
    storage: &SemanticModelStorage,
    id: DeclarationId,
    occurrences: &[u32],
    output: &mut String,
) -> Result<(), ResolutionError> {
    let segment = storage
        .declaration(id)
        .ok_or(ResolutionError::InvalidStorage)?;
    push_identity_field(output, writer::declaration_kind(segment.kind));
    match segment.name {
        Some(name) => {
            output.push('n');
            push_identity_field(
                output,
                storage
                    .symbol(name)
                    .ok_or(ResolutionError::InvalidStorage)?,
            );
            push_identity_field(
                output,
                &occurrences
                    .get(id.index())
                    .ok_or(ResolutionError::InvalidStorage)?
                    .to_string(),
            );
        }
        None => {
            output.push('a');
            push_identity_field(
                output,
                &segment
                    .anonymous_ordinal
                    .ok_or(ResolutionError::InvalidStorage)?
                    .to_string(),
            );
        }
    }
    Ok(())
}
