//! Phase 6: stable element identity, encoded once per publication.

use crate::lower::storage::SemanticModelStorage;
use crate::model::render as writer;
use crate::model::DeclarationId;
use crate::model::DeclarationKind;
use crate::model::DocumentId;
use crate::model::NameId;
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
///
/// The encoding is *derived*, not stored. An element's identity is its owner's identity plus one
/// more segment, so keeping a string per declaration meant keeping a full copy of the owner's --
/// with the document URI repeated inside every element of the publication, and a construction cost
/// of O(declarations x depth) bytes. The index therefore keeps only the per-declaration facts the
/// encoding cannot recover on its own (the occurrence ordinal) and walks the owner chain that
/// `storage` already holds when a string is actually asked for. Materialising an identity is a
/// boundary operation: `write_identity` for one, and nothing resident per element.
pub(crate) struct IdentityIndex {
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
    /// The byte length each declaration's identity encodes to.
    ///
    /// Four bytes per element instead of the encoding itself, so materialising an identity is one
    /// walk into an exactly sized buffer rather than a measuring walk followed by a writing one --
    /// and the `Box<str>` handed to the boundary is never reallocated to shrink it.
    pub(crate) lengths: Box<[u32]>,
    /// Declarations in ascending canonical-identity order: `order[rank]` is the declaration whose
    /// identity sorts at `rank`.
    ///
    /// This is what makes the publication's element handle orderable. A handle is a rank in this
    /// permutation rather than a raw storage ordinal, so comparing two handles is an integer
    /// compare that answers exactly what comparing their canonical identity strings answered --
    /// every sorted result the authority publishes keeps its order without materialising a byte.
    pub(crate) order: Box<[DeclarationId]>,
    /// The inverse permutation: each declaration's rank in `order`.
    pub(crate) rank: Box<[u32]>,
}

impl std::fmt::Debug for IdentityIndex {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // A disposable lookup accelerator; its bucket layout is not semantic content and printing
        // it would only add noise to a published model's debug output.
        formatter
            .debug_struct("IdentityIndex")
            .field("declarations", &self.occurrences.len())
            .finish_non_exhaustive()
    }
}

impl IdentityIndex {
    pub(crate) fn build(storage: &SemanticModelStorage) -> Result<Self, ResolutionError> {
        let occurrences = name_occurrences(storage)?;
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
            // The derivation walks owners, so the chain has to be acyclic and document-local; that
            // is checked once here rather than assumed by every later materialisation.
            if let Some(owner) = declaration.owner {
                let owner_declaration = storage
                    .declaration(owner)
                    .ok_or(ResolutionError::InvalidStorage)?;
                if owner.index() >= index || owner_declaration.document != declaration.document {
                    return Err(ResolutionError::InvalidStorage);
                }
            }
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
            name_paths.push(name_path);
        }
        // A qualified name identifies a declaration only when nothing else in the publication
        // renders the same one -- two same-named siblings of different kinds otherwise share it.
        let shorthand = (0..storage.declarations.len())
            .map(|index| name_paths[index].is_some_and(|path| name_path_counts[path] == 1))
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let declarations = storage.declarations.len();
        let mut next = vec![None; declarations];
        let hash_builder = RandomState::default();
        let mut heads: HashTable<DeclarationId> = HashTable::new();
        // One scratch buffer for the whole pass: the identity of each declaration is materialised
        // to be hashed and then reused, so the index costs no resident string per element.
        let mut current = String::new();
        let mut lengths = vec![0u32; declarations];
        let candidate_text = std::cell::RefCell::new(String::new());
        // Reverse order so each chain ends up in ascending `DeclarationId` order, which keeps an
        // ambiguous outcome's candidate list canonically ordered without a later sort.
        for index in (0..declarations).rev() {
            let id = DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
            current.clear();
            write_identity(storage, &occurrences, id, &mut current)?;
            lengths[index] = u32::try_from(current.len()).map_err(|_| ResolutionError::Capacity)?;
            let hash = hash_builder.hash_one(current.as_str());
            let equals = |candidate: &DeclarationId, expected: &str| {
                let mut scratch = candidate_text.borrow_mut();
                scratch.clear();
                write_identity(storage, &occurrences, *candidate, &mut scratch).is_ok()
                    && scratch.as_str() == expected
            };
            let matches = |candidate: &DeclarationId| equals(candidate, current.as_str());
            if let Some(existing) = heads.find_mut(hash, matches) {
                next[index] = Some(*existing);
                *existing = id;
            } else {
                let rehash = |candidate: &DeclarationId| {
                    let mut scratch = candidate_text.borrow_mut();
                    scratch.clear();
                    let _ = write_identity(storage, &occurrences, *candidate, &mut scratch);
                    hash_builder.hash_one(scratch.as_str())
                };
                heads
                    .try_reserve(1, rehash)
                    .map_err(|_| ResolutionError::Capacity)?;
                heads.insert_unique(hash, id, rehash);
            }
        }
        // The handle order is the canonical-identity order, settled once here. Sorting needs the
        // encodings, so they are materialised for the comparison and dropped again: the index
        // keeps the permutation (four bytes per element each way), never the strings.
        let mut encodings: Vec<Box<str>> = Vec::with_capacity(declarations);
        for index in 0..declarations {
            let id = DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
            current.clear();
            write_identity(storage, &occurrences, id, &mut current)?;
            encodings.push(current.as_str().into());
        }
        let mut order: Vec<DeclarationId> = Vec::with_capacity(declarations);
        for index in 0..declarations {
            order.push(DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?);
        }
        order.sort_unstable_by(|left, right| {
            encodings[left.index()].cmp(&encodings[right.index()])
        });
        drop(encodings);
        let mut rank = vec![0u32; declarations];
        for (position, id) in order.iter().enumerate() {
            rank[id.index()] = u32::try_from(position).map_err(|_| ResolutionError::Capacity)?;
        }
        Ok(Self {
            occurrences,
            shorthand,
            heads,
            hash_builder,
            next: next.into_boxed_slice(),
            lengths: lengths.into_boxed_slice(),
            order: order.into_boxed_slice(),
            rank: rank.into_boxed_slice(),
        })
    }

    /// The rank of a declaration in canonical-identity order.
    pub(crate) fn rank_of(&self, id: DeclarationId) -> Option<usize> {
        self.rank.get(id.index()).map(|rank| *rank as usize)
    }

    /// The declaration at one rank of the canonical-identity order.
    pub(crate) fn at_rank(&self, rank: usize) -> Option<DeclarationId> {
        self.order.get(rank).copied()
    }

    /// Whether the writer may identify this declaration by qualified name alone.
    pub(crate) fn allows_qualified_name_shorthand(&self, id: DeclarationId) -> bool {
        self.shorthand.get(id.index()).copied().unwrap_or(false)
    }

    /// This declaration's ordinal among its identically named, same-kind siblings.
    pub(crate) fn name_occurrence(&self, id: DeclarationId) -> Option<u32> {
        self.occurrences.get(id.index()).copied()
    }

    /// The canonical identity of one declaration, materialised.
    ///
    /// A boundary operation: the encoding is derived from the owner chain on demand, so a caller
    /// that only needs to compare or index elements should carry the `DeclarationId` instead.
    pub(crate) fn identity(
        &self,
        storage: &SemanticModelStorage,
        id: DeclarationId,
    ) -> Option<Box<str>> {
        if id.index() >= self.occurrences.len() {
            return None;
        }
        // The settled length gives the buffer its exact size, so materialising one identity is a
        // single walk and a single allocation -- and `into_boxed_str` below copies nothing.
        let length = *self.lengths.get(id.index())? as usize;
        let mut output = String::with_capacity(length);
        write_identity(storage, &self.occurrences, id, &mut output).ok()?;
        Some(output.into_boxed_str())
    }

    /// Every declaration carrying `identity`, in ascending `DeclarationId` order.
    pub(crate) fn declarations(
        &self,
        storage: &SemanticModelStorage,
        identity: &str,
    ) -> Vec<DeclarationId> {
        let hash = self.hash_builder.hash_one(identity);
        let mut scratch = String::new();
        let Some(head) = self
            .heads
            .find(hash, |candidate| {
                scratch.clear();
                write_identity(storage, &self.occurrences, *candidate, &mut scratch).is_ok()
                    && scratch == identity
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

/// Writes the canonical identity of `id` into `output`.
///
/// The encoding is the owner's encoding plus this declaration's own segment, so the derivation is
/// the owner chain: the document field is written once at the root and each segment follows in
/// root-to-leaf order. `IdentityIndex::build` checks that the chain strictly descends and stays
/// within one document, so this recursion is bounded by the authored nesting depth.
pub(crate) fn write_identity(
    storage: &SemanticModelStorage,
    occurrences: &[u32],
    id: DeclarationId,
    output: &mut String,
) -> Result<(), ResolutionError> {
    let declaration = storage
        .declaration(id)
        .ok_or(ResolutionError::InvalidStorage)?;
    match declaration.owner {
        Some(owner) => write_identity(storage, occurrences, owner, output)?,
        None => {
            let document = storage
                .document(declaration.document)
                .ok_or(ResolutionError::InvalidStorage)?;
            output.push_str(IDENTITY_ENCODING_VERSION);
            push_identity_field(output, &document.identity);
        }
    }
    push_identity_segment(storage, id, occurrences, output)
}

/// Appends one length-prefixed field, so a document identity or an authored name containing any
/// byte sequence -- including the encoding's own punctuation -- cannot forge a segment boundary.
/// Formatted straight into the output rather than through an intermediate `String`: an identity is
/// now materialised on demand, so a heap allocation per field would be a heap allocation per
/// element of every query result.
pub(crate) fn push_identity_field(output: &mut String, value: &str) {
    push_decimal(output, value.len() as u64);
    output.push(':');
    output.push_str(value);
}

/// A length-prefixed field whose value is itself a decimal number.
pub(crate) fn push_identity_number(output: &mut String, value: u32) {
    push_decimal(output, decimal_digits(u64::from(value)));
    output.push(':');
    push_decimal(output, u64::from(value));
}

fn decimal_digits(value: u64) -> u64 {
    if value == 0 {
        1
    } else {
        u64::from(value.ilog10()) + 1
    }
}

fn push_decimal(output: &mut String, value: u64) {
    let mut digits = [0u8; 20];
    let mut cursor = digits.len();
    let mut remaining = value;
    loop {
        cursor -= 1;
        digits[cursor] = b'0' + (remaining % 10) as u8;
        remaining /= 10;
        if remaining == 0 {
            break;
        }
    }
    // Every byte written above is an ASCII digit, so the slice is valid UTF-8.
    output.push_str(std::str::from_utf8(&digits[cursor..]).unwrap_or_default());
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
    let mut seen: BTreeMap<(DocumentId, Option<DeclarationId>, DeclarationKind, NameId), u32> =
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
            push_identity_number(
                output,
                *occurrences
                    .get(id.index())
                    .ok_or(ResolutionError::InvalidStorage)?,
            );
        }
        None => {
            output.push('a');
            push_identity_number(
                output,
                segment
                    .anonymous_ordinal
                    .ok_or(ResolutionError::InvalidStorage)?,
            );
        }
    }
    Ok(())
}
