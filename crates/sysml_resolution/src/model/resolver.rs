//! Private batch resolution over the dense canonical semantic storage.
//!
//! Resolution first materializes canonical direct-name ranges, then solves import targets and
//! effective imported scopes together to a fixed point. Each pass visits the preclassified import
//! slots and indexed candidate ranges only; it never rescans declarations or all references for
//! an individual lookup. Downstream reference families read the frozen effective index after the
//! import barrier converges.

use super::element_kind;
use super::evaluation;
use super::*;
use crate::diagnostics::UNCODED_PARSE_ERROR;
use crate::element_kind::{MembershipRole, RequirementConstraintKind};
use crate::evaluation::{
    AuthoredUnit, ElementEvaluation, EvaluationPolicy, EvaluationState, ExpectedMeasurement,
    ResolvedUnit, UnitResolution,
};
use crate::{
    ActionDerivedFactCollection, ActionDerivedFactOutcome, ActionDerivedFactPrerequisite,
    AnnotationForm as InspectionAnnotationForm, BindingConnector, BindingConnectorCheckKind,
    BindingConnectorValidationOutcome, BindingEndpoint, Conformance, ConformanceObstacle,
    DefinitionUsageDerivedKind, DefinitionUsageDerivedOutcome, DefinitionUsageDerivedPrerequisite,
    DerivedElementOwner, Diagnostic, DiagnosticCategory, DiagnosticCode, DiagnosticLocation,
    DiagnosticOrigin, DiagnosticSeverity, Documentation, EffectiveType, EffectiveTypeOrigin,
    ElementDerivedDocumentationCollection, ElementRelationship, ElementSearch, ElementSource,
    FeatureDerivedRelationshipCollection, LibrarySpecializationAnchorBranch,
    NamespaceDerivedElementCollection, NamespaceImportDerivedElement, NavigationTarget,
    OccurrenceRole, PublicationCompleteness as PublicCompleteness, PublishedDiagnostics,
    QueryOutcome, RedefinitionCheckKind, RedefinitionCheckOutcome, RedefinitionCheckPrerequisite,
    RelatedLocation, RelationshipProvenance, RelationshipTarget, RenameOutcome,
    RequirementDerivedFactCollection, RequirementDerivedFactOutcome,
    RequirementDerivedFactPrerequisite, RequirementUsageTyping, RequirementVerification,
    SatisfyEndpoint, SatisfyPolarity, SatisfyRelationship, SourceLocation, SpecializationCheckKind,
    SpecializationCheckOutcome, SpecializationCheckPrerequisite, SpecializationScope,
    SubsettingConformance, SymbolIdentity, TextPosition, TextRange, TypeDerivedElementCollection,
    TypeDerivedFactCollection, TypeDerivedFactOutcome, TypeDerivedFactPrerequisite,
    TypeDerivedFactValue, TypeDerivedRelationshipCollection, TypeFeaturingCheckKind,
    TypeFeaturingCheckOutcome, TypeFeaturingCheckPrerequisite, TypeReference, VerificationOutcome,
    VerificationRequirement, VisibleMember,
};
use spec42_constraint_manifest::{
    ElementDerivedOwnerKind, LibrarySpecializationPredicate, NamespaceImportDerivedElementKind,
};

mod binding;
mod conformance;
mod details;
mod expression;
mod expression_conformance;
mod host_conformance;
mod inspection;
mod structural;
mod types;
pub(crate) mod writer;

/// The note attached to each declaration an ambiguous reference could have named.
const RELATED_AMBIGUOUS_CANDIDATE: &str = "Candidate this reference could name.";

/// The settled diagnostics of one publication with the per-document ranges that index them.
///
/// The two are derived together and are only correct together, so they are returned together
/// rather than as two calls a caller could interleave.
type DerivedDiagnostics = (Box<[Diagnostic]>, Box<[(u32, u32)]>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ResolutionError {
    Capacity,
    InvalidStorage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct NameKey {
    owner: Option<DeclarationId>,
    name: SymbolId,
}

/// Position-addressable lookup tables for one document, built once at the publication barrier.
///
/// Before this existed, `target_at` scanned every authored reference and every declaration in the
/// whole model on every call, which the design doc's complexity contract forbids: a
/// source-position question must not cost the size of the workspace.
///
/// Reference spans and declaration *identifier* spans are leaf token ranges that cannot nest, so
/// both are binary-searchable. A declaration's *full* span does nest -- a package contains a part
/// contains an attribute -- so the containing-element lookup scans this document's declarations
/// and keeps the innermost match. That is document-local rather than model-wide, which is the
/// property that matters here; a genuine interval tree would only help a single very large
/// document.
#[derive(Debug, Default)]
struct DocumentPositions {
    /// Authored reference ranges, ordered by start.
    references: Box<[(TextRange, AuthoredReferenceId)]>,
    /// Declaration identifier ranges, ordered by start. Only named declarations appear.
    identifiers: Box<[(TextRange, DeclarationId)]>,
    /// Every declaration's full span, as a containment tree.
    spans: SpanTree,
}

/// Declaration spans arranged so containment can be answered without reading the whole document.
///
/// Declaration spans nest, which rules out the binary search [`leaf_ranges_containing`] uses: an
/// enclosing package sorts before its members yet ends after all of them, so "does this span end
/// before the position" is not monotone over the ordering and `partition_point` has no meaning.
///
/// Entries are ordered so that a declaration is immediately followed by its descendants, and each
/// entry records the exclusive end of its own subtree. A containment query then descends the
/// nesting, skipping every subtree that cannot contain the position in one step. What it visits is
/// bounded by the declarations that *begin before the position at each enclosing level* -- not by
/// the document's declaration count, which is what a filter over the flat table costs.
#[derive(Debug, Default)]
struct SpanTree {
    entries: Box<[(TextRange, DeclarationId)]>,
    subtree_end: Box<[u32]>,
}

impl SpanTree {
    fn build(mut entries: Vec<(TextRange, DeclarationId)>) -> Self {
        // Start ascending, then end *descending*, so an enclosing declaration precedes the members
        // it shares a start with rather than sorting after them.
        entries.sort_by(|left, right| {
            left.0
                .start
                .cmp(&right.0.start)
                .then_with(|| right.0.end.cmp(&left.0.end))
        });
        // Every entry still open when a later one begins is one of its ancestors, so a stack of
        // open entries closes exactly those the new entry is not nested in.
        let mut subtree_end = vec![0u32; entries.len()];
        let mut open: Vec<usize> = Vec::new();
        for (index, (range, _)) in entries.iter().enumerate() {
            while open
                .last()
                .is_some_and(|ancestor| entries[*ancestor].0.end <= range.start)
            {
                let ancestor = open
                    .pop()
                    .expect("the stack was just observed to be non-empty");
                subtree_end[ancestor] = index as u32;
            }
            open.push(index);
        }
        for ancestor in open {
            subtree_end[ancestor] = entries.len() as u32;
        }
        Self {
            entries: entries.into_boxed_slice(),
            subtree_end: subtree_end.into_boxed_slice(),
        }
    }

    /// The innermost declaration whose span contains `position`.
    fn innermost_containing(&self, position: TextPosition) -> Option<DeclarationId> {
        let mut innermost = None;
        let mut index = 0usize;
        let mut level_end = self.entries.len();
        while index < level_end {
            let (range, id) = self.entries[index];
            record_visited_index_entries(1);
            if range.start > position {
                // Siblings are ordered by start, so nothing further along this level, or inside
                // it, can begin at or before the position.
                break;
            }
            if range_contains(range, position) {
                innermost = Some(id);
                level_end = self.subtree_end[index] as usize;
                index += 1;
            } else {
                index = self.subtree_end[index] as usize;
            }
        }
        innermost
    }

    /// Every span, in source order.
    fn iter(&self) -> impl Iterator<Item = &(TextRange, DeclarationId)> {
        self.entries.iter()
    }
}

/// Document lookup by identity, plus each document's position tables.
#[derive(Debug)]
struct DocumentIndex {
    by_identity: HashTable<DocumentId>,
    hash_builder: RandomState,
    positions: Box<[DocumentPositions]>,
}

impl DocumentIndex {
    fn build(storage: &SemanticModelStorage) -> Result<Self, ResolutionError> {
        let hash_builder = RandomState::default();
        let mut by_identity: HashTable<DocumentId> = HashTable::new();
        for index in 0..storage.documents.len() {
            let id = DocumentId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
            let identity = storage.documents[index].identity.as_ref();
            let hash = hash_builder.hash_one(identity);
            let rehash = |candidate: &DocumentId| {
                hash_builder.hash_one(storage.documents[candidate.index()].identity.as_ref())
            };
            by_identity
                .try_reserve(1, rehash)
                .map_err(|_| ResolutionError::Capacity)?;
            by_identity.insert_unique(hash, id, rehash);
        }

        let mut references: Vec<Vec<(TextRange, AuthoredReferenceId)>> =
            vec![Vec::new(); storage.documents.len()];
        for (index, reference) in storage.references.iter().enumerate() {
            let Some(source) = storage.declaration(reference.source) else {
                continue;
            };
            let Ok(id) = AuthoredReferenceId::from_index(index) else {
                continue;
            };
            // A reference whose span cannot be mapped is skipped rather than failing the build:
            // it simply cannot answer a position query, and the authored fact itself is unaffected.
            if let Ok(range) = document_range(storage, source.document, &reference.span) {
                references[source.document.index()].push((range, id));
            }
        }

        let mut identifiers: Vec<Vec<(TextRange, DeclarationId)>> =
            vec![Vec::new(); storage.documents.len()];
        let mut spans: Vec<Vec<(TextRange, DeclarationId)>> =
            vec![Vec::new(); storage.documents.len()];
        for index in 0..storage.declarations.len() {
            let Ok(id) = DeclarationId::from_index(index) else {
                continue;
            };
            let declaration = &storage.declarations[index];
            if let Ok(range) = document_range(storage, declaration.document, &declaration.span) {
                spans[declaration.document.index()].push((range, id));
            }
            let Some(name) = declaration.name.and_then(|name| storage.symbol(name)) else {
                continue;
            };
            if let Ok(range) =
                declaration_identifier_range(storage, declaration.document, &declaration.span, name)
            {
                identifiers[declaration.document.index()].push((range, id));
            }
        }

        let positions = (0..storage.documents.len())
            .map(|index| {
                let mut document_references = std::mem::take(&mut references[index]);
                let mut document_identifiers = std::mem::take(&mut identifiers[index]);
                let document_spans = std::mem::take(&mut spans[index]);
                document_references.sort_by_key(|(range, _)| *range);
                document_identifiers.sort_by_key(|(range, _)| *range);
                DocumentPositions {
                    references: document_references.into_boxed_slice(),
                    identifiers: document_identifiers.into_boxed_slice(),
                    spans: SpanTree::build(document_spans),
                }
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();

        Ok(Self {
            by_identity,
            hash_builder,
            positions,
        })
    }

    fn document(&self, storage: &SemanticModelStorage, identity: &str) -> Option<DocumentId> {
        let hash = self.hash_builder.hash_one(identity);
        self.by_identity
            .find(hash, |candidate| {
                storage.documents[candidate.index()].identity.as_ref() == identity
            })
            .copied()
    }

    fn positions(&self, document: DocumentId) -> Option<&DocumentPositions> {
        self.positions.get(document.index())
    }
}

#[cfg(test)]
thread_local! {
    static VISITED_INDEX_ENTRIES: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

/// Charge `entries` index entries to the running complexity measurement.
///
/// Compiled away outside tests. It exists because the inspection path's cost bound -- every lookup
/// is a range slice or a descent, never a walk over the model -- is a property Rust cannot check
/// and a passing query cannot demonstrate: a scan and an index return the same answer. The
/// measurement is what makes a regression to a scan a test failure rather than a slowdown.
#[inline]
fn record_visited_index_entries(entries: usize) {
    #[cfg(test)]
    VISITED_INDEX_ENTRIES.with(|counter| {
        counter.set(counter.get().saturating_add(entries as u64));
    });
    #[cfg(not(test))]
    let _ = entries;
}

/// Runs `query` and reports how many index entries it visited.
#[cfg(test)]
pub(crate) fn measure_visited_index_entries<T>(query: impl FnOnce() -> T) -> (T, u64) {
    VISITED_INDEX_ENTRIES.with(|counter| counter.set(0));
    let value = query();
    (value, VISITED_INDEX_ENTRIES.with(std::cell::Cell::get))
}

/// Every entry of a start-ordered leaf-range table whose range contains `position`.
///
/// The ranges cannot nest, so a binary search to the first candidate and a short forward walk
/// visits only entries that actually start at or before the position and are still open.
fn leaf_ranges_containing<T: Copy>(
    entries: &[(TextRange, T)],
    position: TextPosition,
) -> impl Iterator<Item = T> + '_ {
    let start = entries.partition_point(|(range, _)| range.end < position);
    entries[start..]
        .iter()
        .inspect(|_| record_visited_index_entries(1))
        .take_while(move |(range, _)| range.start <= position)
        .filter(move |(range, _)| range_contains(*range, position))
        .map(|(_, value)| *value)
}

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
struct IdentityIndex {
    /// One canonical identity string per `DeclarationId`, parallel to `storage.declarations`.
    text: Box<[Box<str>]>,
    /// Each declaration's ordinal among its identically named, same-kind siblings.
    occurrences: Box<[u32]>,
    /// Whether this declaration's identity is recoverable from its qualified name alone, so the
    /// writer may use the readable shorthand: every segment named, every occurrence ordinal zero,
    /// and no other declaration in the publication sharing the same document and name path.
    shorthand: Box<[bool]>,
    /// Head of each distinct identity's declaration chain.
    ///
    /// Retained even though the identity is total, so a storage invariant broken elsewhere
    /// surfaces as an explicit ambiguous outcome rather than an arbitrary pick.
    heads: HashTable<DeclarationId>,
    hash_builder: RandomState,
    /// Next declaration sharing this one's identity, in ascending `DeclarationId` order.
    next: Box<[Option<DeclarationId>]>,
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
    fn build(storage: &SemanticModelStorage) -> Result<Self, ResolutionError> {
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
    fn allows_qualified_name_shorthand(&self, id: DeclarationId) -> bool {
        self.shorthand.get(id.index()).copied().unwrap_or(false)
    }

    /// This declaration's ordinal among its identically named, same-kind siblings.
    fn name_occurrence(&self, id: DeclarationId) -> Option<u32> {
        self.occurrences.get(id.index()).copied()
    }

    fn identity(&self, id: DeclarationId) -> Option<&str> {
        self.text.get(id.index()).map(AsRef::as_ref)
    }

    /// Every declaration carrying `identity`, in ascending `DeclarationId` order.
    fn declarations(&self, identity: &str) -> Vec<DeclarationId> {
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
fn push_identity_field(output: &mut String, value: &str) {
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
fn name_occurrences(storage: &SemanticModelStorage) -> Result<Box<[u32]>, ResolutionError> {
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

fn push_identity_segment(
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CandidateRange {
    start: u32,
    len: u32,
}

impl CandidateRange {
    fn from_bounds(start: usize, end: usize) -> Result<Self, ResolutionError> {
        let start = u32::try_from(start).map_err(|_| ResolutionError::Capacity)?;
        let len = u32::try_from(
            end.checked_sub(start as usize)
                .ok_or(ResolutionError::InvalidStorage)?,
        )
        .map_err(|_| ResolutionError::Capacity)?;
        start.checked_add(len).ok_or(ResolutionError::Capacity)?;
        Ok(Self { start, len })
    }

    fn slice<'a, T>(&self, values: &'a [T]) -> Option<&'a [T]> {
        let end = self.start.checked_add(self.len)?;
        values.get(self.start as usize..end as usize)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct NameIndex {
    keys: Box<[NameKey]>,
    ranges: Box<[CandidateRange]>,
    candidates: Box<[DeclarationId]>,
}

/// Publication-owned reverse edges from a resolved target to its authored reference sites.
///
/// The declaration occurrence is deliberately not stored here: it has different provenance and
/// remains an explicit `include_declaration` policy in the query API. Reference ids within each
/// target range stay in authored canonical order because the CSR is filled in ascending id order.
#[derive(Debug)]
struct ReverseReferenceIndex {
    ranges: Box<[(u32, u32)]>,
    references: Box<[AuthoredReferenceId]>,
}

impl ReverseReferenceIndex {
    fn build(declarations: usize, resolution: &ResolutionResults) -> Result<Self, ResolutionError> {
        let mut counts = vec![0u32; declarations];
        for outcome in resolution.outcomes.iter().copied() {
            if let ResolutionStatus::Resolved(target) = outcome {
                let count = counts
                    .get_mut(target.index())
                    .ok_or(ResolutionError::InvalidStorage)?;
                *count = count.checked_add(1).ok_or(ResolutionError::Capacity)?;
            }
        }

        let mut ranges = Vec::with_capacity(declarations);
        let mut starts = Vec::with_capacity(declarations);
        let mut end = 0u32;
        for count in counts {
            let start = end;
            end = end.checked_add(count).ok_or(ResolutionError::Capacity)?;
            ranges.push((start, end));
            starts.push(start);
        }
        let reference_count = usize::try_from(end).map_err(|_| ResolutionError::Capacity)?;
        let mut references = vec![AuthoredReferenceId(0); reference_count];
        for (index, outcome) in resolution.outcomes.iter().copied().enumerate() {
            let ResolutionStatus::Resolved(target) = outcome else {
                continue;
            };
            let cursor = starts
                .get_mut(target.index())
                .ok_or(ResolutionError::InvalidStorage)?;
            let slot = references
                .get_mut(*cursor as usize)
                .ok_or(ResolutionError::InvalidStorage)?;
            *slot =
                AuthoredReferenceId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
            *cursor = cursor.checked_add(1).ok_or(ResolutionError::Capacity)?;
        }
        Ok(Self {
            ranges: ranges.into_boxed_slice(),
            references: references.into_boxed_slice(),
        })
    }

    fn references(&self, target: DeclarationId) -> &[AuthoredReferenceId] {
        let Some(&(start, end)) = self.ranges.get(target.index()) else {
            return &[];
        };
        self.references
            .get(start as usize..end as usize)
            .unwrap_or_default()
    }
}

/// Effective member enumeration for every lexical scope, including the root scope.
///
/// This is the persistent, frequently-enumerated scope-map family: owned, imported and inherited
/// candidates are merged once at publication. Lookup by a particular name continues to use the
/// canonical origin-specific indexes so shadowing and ambiguity retain their semantic meaning.
#[derive(Debug)]
struct EffectiveScopeIndex {
    ranges: Box<[(u32, u32)]>,
    members: Box<[DeclarationId]>,
}

impl EffectiveScopeIndex {
    fn build(
        declarations: usize,
        direct: &NameIndex,
        imported: &NameIndex,
        inherited: &NameIndex,
    ) -> Result<Self, ResolutionError> {
        let mut ranges = Vec::with_capacity(declarations.saturating_add(1));
        let mut members = Vec::new();
        for slot in 0..=declarations {
            let owner = if slot == 0 {
                None
            } else {
                Some(DeclarationId::from_index(slot - 1).map_err(|_| ResolutionError::Capacity)?)
            };
            let mut scope_members = Vec::new();
            for index in [direct, imported, inherited] {
                for (_, candidates) in index.entries_for_owner(owner) {
                    scope_members.extend_from_slice(candidates);
                }
            }
            scope_members.sort_unstable();
            scope_members.dedup();
            let start = u32::try_from(members.len()).map_err(|_| ResolutionError::Capacity)?;
            members.extend(scope_members);
            let end = u32::try_from(members.len()).map_err(|_| ResolutionError::Capacity)?;
            ranges.push((start, end));
        }
        Ok(Self {
            ranges: ranges.into_boxed_slice(),
            members: members.into_boxed_slice(),
        })
    }

    fn members(&self, owner: Option<DeclarationId>) -> &[DeclarationId] {
        let slot = owner.map_or(0usize, |owner| owner.index().saturating_add(1));
        let Some(&(start, end)) = self.ranges.get(slot) else {
            return &[];
        };
        let members = self
            .members
            .get(start as usize..end as usize)
            .unwrap_or_default();
        record_visited_index_entries(members.len());
        members
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EffectiveVisibility {
    Public,
    Private,
    Protected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct EffectiveMembership {
    visibility: EffectiveVisibility,
    /// The authored membership kind, retained so inspection need not rescan the record table.
    kind: MembershipKind,
    /// Whether the visibility above was written or defaulted.
    authored: bool,
}

/// Dense, declaration-aligned membership facts used by resolution. Authored `Default` is settled
/// once here, with its owning declaration context, rather than being reinterpreted by each lookup.
#[derive(Debug)]
struct MembershipIndex {
    by_declaration: Box<[EffectiveMembership]>,
}

impl MembershipIndex {
    fn build(
        declarations: &[Declaration],
        memberships: &[MembershipRecord],
    ) -> Result<Self, ResolutionError> {
        let mut by_declaration = vec![None; declarations.len()];
        for membership in memberships {
            let declaration = declarations
                .get(membership.member.index())
                .ok_or(ResolutionError::InvalidStorage)?;
            let slot = by_declaration
                .get_mut(membership.member.index())
                .ok_or(ResolutionError::InvalidStorage)?;
            if slot.is_some()
                || matches!(declaration.kind, DeclarationKind::Import)
                    != matches!(membership.kind, MembershipKind::Import)
            {
                return Err(ResolutionError::InvalidStorage);
            }
            let visibility = match membership.visibility {
                Visibility::Public => EffectiveVisibility::Public,
                Visibility::Private => EffectiveVisibility::Private,
                Visibility::Protected => EffectiveVisibility::Protected,
                Visibility::Default if membership.kind == MembershipKind::Import => {
                    EffectiveVisibility::Private
                }
                Visibility::Default => match declaration.owner {
                    None => EffectiveVisibility::Public,
                    Some(owner)
                        if declarations.get(owner.index()).is_some_and(|owner| {
                            matches!(
                                owner.kind,
                                DeclarationKind::Package | DeclarationKind::LibraryPackage
                            )
                        }) =>
                    {
                        EffectiveVisibility::Public
                    }
                    Some(_) => EffectiveVisibility::Private,
                },
            };
            *slot = Some(EffectiveMembership {
                visibility,
                kind: membership.kind,
                authored: membership.visibility != Visibility::Default,
            });
        }
        let by_declaration = by_declaration
            .into_iter()
            .collect::<Option<Vec<_>>>()
            .ok_or(ResolutionError::InvalidStorage)?;
        Ok(Self {
            by_declaration: by_declaration.into_boxed_slice(),
        })
    }

    fn get(&self, declaration: DeclarationId) -> Option<EffectiveMembership> {
        self.by_declaration.get(declaration.index()).copied()
    }

    fn is_public(&self, declaration: DeclarationId) -> bool {
        self.get(declaration)
            .is_some_and(|membership| membership.visibility == EffectiveVisibility::Public)
    }
}

impl NameIndex {
    fn build(mut entries: Vec<(NameKey, DeclarationId)>) -> Result<Self, ResolutionError> {
        entries.sort_unstable_by_key(name_entry_sort_key);
        entries.dedup();

        let mut keys = Vec::new();
        let mut ranges = Vec::new();
        let mut candidates = Vec::new();
        keys.try_reserve(entries.len())
            .map_err(|_| ResolutionError::Capacity)?;
        ranges
            .try_reserve(entries.len())
            .map_err(|_| ResolutionError::Capacity)?;
        candidates
            .try_reserve(entries.len())
            .map_err(|_| ResolutionError::Capacity)?;

        let mut cursor = 0;
        while cursor < entries.len() {
            let key = entries[cursor].0;
            let start = candidates.len();
            while cursor < entries.len() && entries[cursor].0 == key {
                candidates.push(entries[cursor].1);
                cursor += 1;
            }
            keys.push(key);
            ranges.push(CandidateRange::from_bounds(start, candidates.len())?);
        }

        Ok(Self {
            keys: keys.into_boxed_slice(),
            ranges: ranges.into_boxed_slice(),
            candidates: candidates.into_boxed_slice(),
        })
    }

    fn candidates(&self, owner: Option<DeclarationId>, name: SymbolId) -> &[DeclarationId] {
        let key = NameKey { owner, name };
        let Ok(index) = self.keys.binary_search(&key) else {
            return &[];
        };
        let candidates = self.ranges[index]
            .slice(&self.candidates)
            .unwrap_or_default();
        record_visited_index_entries(candidates.len());
        candidates
    }

    fn entries_for_owner(
        &self,
        owner: Option<DeclarationId>,
    ) -> impl Iterator<Item = (SymbolId, &[DeclarationId])> {
        let start = self.keys.partition_point(|key| key.owner < owner);
        let end = self.keys.partition_point(|key| key.owner <= owner);
        self.keys[start..end]
            .iter()
            .zip(&self.ranges[start..end])
            .filter_map(|(key, range)| {
                range
                    .slice(&self.candidates)
                    .map(|candidates| (key.name, candidates))
            })
    }
}

/// The tuple's canonical `Ord` encoded as one integer comparison.
///
/// `None` sorts before every owner and `Some(u32::MAX)` still fits because the owner occupies 33
/// bits above the two complete 32-bit name/candidate fields. The encoding is injective, so this is
/// purely a cheaper sorting representation rather than a hash or a competing identity policy.
fn name_entry_sort_key((key, candidate): &(NameKey, DeclarationId)) -> u128 {
    let owner = key.owner.map_or(0, |owner| u128::from(owner.0) + 1);
    (owner << 64) | (u128::from(key.name.0) << 32) | u128::from(candidate.0)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ResolutionStatus {
    Resolved(DeclarationId),
    Unresolved,
    Ambiguous(CandidateRange),
    Unsupported,
    NonConverged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SolverStatus {
    Converged,
    NonConverged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct ResolutionWork {
    passes: u32,
    import_evaluations: u64,
    downstream_evaluations: u64,
    indexed_name_lookups: u64,
    direct_index_entries: u64,
    effective_index_entries: u64,
}

/// A resolver-synthesized relationship fact that has no authored reference site. The narrow slice
/// currently covered here is same-name inherited-member redefinition against an immediate
/// (directly specialized) parent's own directly owned feature. Multi-level/diamond inherited
/// redefinition is intentionally out of scope: an ambiguous or absent immediate-parent match is
/// left unresolved rather than guessed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ImpliedRelationship {
    kind: ReferenceKind,
    source: DeclarationId,
    target: DeclarationId,
}

/// One exact unconditional `specializesFromLibrary` check extracted from the pinned XMI through
/// `specifications/constraint_manifest.toml`. Conditional OCL bodies never enter this table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct LibrarySpecializationRuleKey(&'static str);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct LibrarySpecializationAnchorKey {
    rule: LibrarySpecializationRuleKey,
    branch: LibrarySpecializationAnchorBranch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LibrarySpecializationRule {
    rule_id: &'static str,
    metaclass: &'static str,
    anchor: &'static str,
}

include!(concat!(env!("OUT_DIR"), "/library_specialization_rules.rs"));

/// One exact conditional `specializesFromLibrary` check whose predicate is a closed manifest
/// contract. The resolver evaluates the predicate from its owned declaration facts; it never
/// reparses OCL or infers applicability from a rule name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ConditionalLibrarySpecializationRule {
    rule_id: &'static str,
    metaclass: &'static str,
    predicate: LibrarySpecializationPredicate,
    owner_metaclasses: &'static [&'static str],
    true_anchor: Option<&'static str>,
    anchor: &'static str,
}

include!(concat!(
    env!("OUT_DIR"),
    "/conditional_library_specialization_rules.rs"
));

/// One exact unconditional `redefinesFromLibrary` check extracted from the pinned XMI. Rules are
/// generated separately from specializations: the function name is a distinct normative contract
/// and a consumer must not reinterpret one as the other.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LibraryRedefinitionRule {
    rule_id: &'static str,
    metaclass: &'static str,
    anchor: &'static str,
}

include!(concat!(env!("OUT_DIR"), "/library_redefinition_rules.rs"));

/// One closed exact Feature relationship-collection derivation emitted from the pinned manifest.
/// The generated table, rather than a query-side rule-name convention, decides which collection
/// exists and which source metaclass owns it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FeatureDerivedRelationshipRule {
    rule_id: &'static str,
    metaclass: &'static str,
    collection: FeatureDerivedRelationshipCollection,
}

include!(concat!(
    env!("OUT_DIR"),
    "/feature_derived_relationship_rules.rs"
));

/// One closed Type relationship derivation emitted from the pinned manifest. The exact rule ID
/// determines the query collection; consumers never infer it from a display name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TypeDerivedRelationshipRule {
    rule_id: &'static str,
    metaclass: &'static str,
    collection: TypeDerivedRelationshipCollection,
}

include!(concat!(
    env!("OUT_DIR"),
    "/type_derived_relationship_rules.rs"
));

/// One exact final element-valued Type derivation emitted from the pinned manifest. Intermediate
/// Membership relationship identities remain private; this table therefore admits only element
/// projections that canonical owner and membership facts can answer losslessly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TypeDerivedElementRule {
    rule_id: &'static str,
    metaclass: &'static str,
    collection: TypeDerivedElementCollection,
}

include!(concat!(env!("OUT_DIR"), "/type_derived_element_rules.rs"));

/// One exact Type derivation that cannot yet return values because its canonical fact owner is
/// intentionally absent. The generated table preserves rule identity and result category.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TypeDerivedFactRule {
    rule_id: &'static str,
    metaclass: &'static str,
    collection: TypeDerivedFactCollection,
}

include!(concat!(env!("OUT_DIR"), "/type_derived_fact_rules.rs"));

/// A complete Definition/Usage derivation selected only by the generated manifest projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DefinitionUsageDerivedRule {
    rule_id: &'static str,
    metaclass: &'static str,
    kind: DefinitionUsageDerivedKind,
}

include!(concat!(
    env!("OUT_DIR"),
    "/definition_usage_derived_rules.rs"
));

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ActionDerivedFactRule {
    rule_id: &'static str,
    metaclass: &'static str,
    collection: ActionDerivedFactCollection,
}

include!(concat!(env!("OUT_DIR"), "/action_derived_fact_rules.rs"));

/// One exact Systems::Requirements derivation emitted by the manifest. The query collection is
/// closed and rule-keyed; it is not an interpretation of arbitrary membership names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RequirementDerivedFactRule {
    rule_id: &'static str,
    metaclass: &'static str,
    collection: RequirementDerivedFactCollection,
}

include!(concat!(
    env!("OUT_DIR"),
    "/requirement_derived_fact_rules.rs"
));

/// One exact TypeFeaturing check emitted from the manifest. The table owns the normative rule
/// identity and metaclass; the resolver consumes only canonical FeatureMembership and effective
/// TypeFeaturing facts to decide it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TypeFeaturingCheckRule {
    rule_id: &'static str,
    metaclass: &'static str,
    kind: TypeFeaturingCheckKind,
}

include!(concat!(env!("OUT_DIR"), "/type_featuring_check_rules.rs"));

/// One exact redefinition check body emitted from the manifest. This only identifies a normative
/// predicate; it never acts as a second relationship store or an OCL interpreter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RedefinitionCheckRule {
    rule_id: &'static str,
    metaclass: &'static str,
    kind: RedefinitionCheckKind,
}

include!(concat!(env!("OUT_DIR"), "/redefinition_check_rules.rs"));

/// One exact specialization predicate emitted from the manifest.  This generated table is the
/// sole rule-to-kind binding; it never contains semantic relationship values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SpecializationCheckRule {
    rule_id: &'static str,
    metaclass: &'static str,
    kind: SpecializationCheckKind,
}

include!(concat!(env!("OUT_DIR"), "/specialization_check_rules.rs"));

/// The closed exact `Element::owner` derivation emitted from the pinned manifest.
///
/// The table is deliberately separate from relationship collections: `owner` is a derived
/// scalar over canonical declaration structure, not a synthetic relationship or a path lookup.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ElementDerivedOwnerRule {
    rule_id: &'static str,
    metaclass: &'static str,
    kind: ElementDerivedOwnerKind,
}

include!(concat!(env!("OUT_DIR"), "/element_derived_owner_rules.rs"));

/// One exact Element documentation-form derivation emitted from the pinned manifest.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ElementDerivedDocumentationRule {
    rule_id: &'static str,
    metaclass: &'static str,
    collection: ElementDerivedDocumentationCollection,
}

include!(concat!(
    env!("OUT_DIR"),
    "/element_derived_documentation_rules.rs"
));

/// One exact Namespace element-valued derivation emitted from the pinned manifest. The table
/// selects which direct canonical structural projection is available; query callers do not map
/// rule names to declaration filters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NamespaceDerivedElementRule {
    rule_id: &'static str,
    metaclass: &'static str,
    collection: NamespaceDerivedElementCollection,
}

include!(concat!(
    env!("OUT_DIR"),
    "/namespace_derived_element_rules.rs"
));

/// The closed exact `NamespaceImport::importedElement` projection emitted from the pinned
/// manifest. It is deliberately separate from Namespace collections because its source is one
/// anonymous import declaration and its target retains the canonical reference outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NamespaceImportDerivedElementRule {
    rule_id: &'static str,
    metaclass: &'static str,
    kind: NamespaceImportDerivedElementKind,
}

include!(concat!(
    env!("OUT_DIR"),
    "/namespace_import_derived_element_rules.rs"
));

/// One exact BindingConnector validation body emitted from the pinned manifest.
///
/// The generated contract proves that a caller-selected rule kind has a single normative rule
/// and metaclass. The binding index remains the sole owner of connector facts and validation
/// prerequisites.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BindingConnectorCheckRule {
    rule_id: &'static str,
    metaclass: &'static str,
    kind: BindingConnectorCheckKind,
}

include!(concat!(
    env!("OUT_DIR"),
    "/binding_connector_check_rules.rs"
));

fn feature_derived_relationship_rule(
    collection: FeatureDerivedRelationshipCollection,
) -> Option<&'static FeatureDerivedRelationshipRule> {
    GENERATED_FEATURE_DERIVED_RELATIONSHIP_RULES
        .iter()
        .find(|rule| rule.collection == collection)
}

fn namespace_derived_element_rule(
    collection: NamespaceDerivedElementCollection,
) -> Option<&'static NamespaceDerivedElementRule> {
    GENERATED_NAMESPACE_DERIVED_ELEMENT_RULES
        .iter()
        .find(|rule| rule.collection == collection)
}

fn namespace_import_derived_element_rule() -> Option<&'static NamespaceImportDerivedElementRule> {
    match GENERATED_NAMESPACE_IMPORT_DERIVED_ELEMENT_RULES {
        [rule] => Some(rule),
        _ => None,
    }
}

fn binding_connector_check_rule(
    kind: BindingConnectorCheckKind,
) -> Option<&'static BindingConnectorCheckRule> {
    GENERATED_BINDING_CONNECTOR_CHECK_RULES
        .iter()
        .find(|rule| rule.kind == kind)
}

fn feature_derived_relationship_kinds(
    collection: FeatureDerivedRelationshipCollection,
) -> &'static [ReferenceKind] {
    match collection {
        FeatureDerivedRelationshipCollection::OwnedFeatureChaining => {
            &[ReferenceKind::FeatureChaining]
        }
        FeatureDerivedRelationshipCollection::OwnedRedefinition => &[ReferenceKind::Redefinition],
        // KerML `Redefinition` is a subtype of `Subsetting`, so its owned relationship belongs in
        // `ownedSubsetting` as well. The storage preserves its more specific reference kind.
        FeatureDerivedRelationshipCollection::OwnedSubsetting => {
            &[ReferenceKind::Subsetting, ReferenceKind::Redefinition]
        }
        FeatureDerivedRelationshipCollection::OwnedTyping => &[ReferenceKind::FeatureTyping],
        FeatureDerivedRelationshipCollection::OwnedTypeFeaturing => &[ReferenceKind::TypeFeaturing],
    }
}

fn type_derived_relationship_rule(
    collection: TypeDerivedRelationshipCollection,
) -> Option<&'static TypeDerivedRelationshipRule> {
    GENERATED_TYPE_DERIVED_RELATIONSHIP_RULES
        .iter()
        .find(|rule| rule.collection == collection)
}

fn type_derived_element_rule(
    collection: TypeDerivedElementCollection,
) -> Option<&'static TypeDerivedElementRule> {
    GENERATED_TYPE_DERIVED_ELEMENT_RULES
        .iter()
        .find(|rule| rule.collection == collection)
}

fn type_derived_fact_rule(
    collection: TypeDerivedFactCollection,
) -> Option<&'static TypeDerivedFactRule> {
    GENERATED_TYPE_DERIVED_FACT_RULES
        .iter()
        .find(|rule| rule.collection == collection)
}

fn definition_usage_derived_rule(
    kind: DefinitionUsageDerivedKind,
) -> Option<&'static DefinitionUsageDerivedRule> {
    GENERATED_DEFINITION_USAGE_DERIVED_RULES
        .iter()
        .find(|rule| rule.kind == kind)
}

fn action_derived_fact_rule(
    collection: ActionDerivedFactCollection,
) -> Option<&'static ActionDerivedFactRule> {
    GENERATED_ACTION_DERIVED_FACT_RULES
        .iter()
        .find(|rule| rule.collection == collection)
}

fn requirement_derived_fact_rule(
    collection: RequirementDerivedFactCollection,
) -> Option<&'static RequirementDerivedFactRule> {
    GENERATED_REQUIREMENT_DERIVED_FACT_RULES
        .iter()
        .find(|rule| rule.collection == collection)
}

fn type_featuring_check_rule(
    kind: TypeFeaturingCheckKind,
) -> Option<&'static TypeFeaturingCheckRule> {
    GENERATED_TYPE_FEATURING_CHECK_RULES
        .iter()
        .find(|rule| rule.kind == kind)
}

fn redefinition_check_rule(kind: RedefinitionCheckKind) -> Option<&'static RedefinitionCheckRule> {
    GENERATED_REDEFINITION_CHECK_RULES
        .iter()
        .find(|rule| rule.kind == kind)
}

fn specialization_check_rule(
    kind: SpecializationCheckKind,
) -> Option<&'static SpecializationCheckRule> {
    GENERATED_SPECIALIZATION_CHECK_RULES
        .iter()
        .find(|rule| rule.kind == kind)
}

fn element_derived_owner_rule() -> Option<&'static ElementDerivedOwnerRule> {
    GENERATED_ELEMENT_DERIVED_OWNER_RULES.first()
}

fn element_derived_documentation_rule(
    collection: ElementDerivedDocumentationCollection,
) -> Option<&'static ElementDerivedDocumentationRule> {
    GENERATED_ELEMENT_DERIVED_DOCUMENTATION_RULES
        .iter()
        .find(|rule| rule.collection == collection)
}

fn type_derived_relationship_kinds(
    collection: TypeDerivedRelationshipCollection,
) -> &'static [ReferenceKind] {
    match collection {
        TypeDerivedRelationshipCollection::OwnedSpecialization => &[
            ReferenceKind::Subclassification,
            ReferenceKind::Subsetting,
            ReferenceKind::Redefinition,
            ReferenceKind::FeatureTyping,
        ],
        TypeDerivedRelationshipCollection::OwnedUnioning
        | TypeDerivedRelationshipCollection::UnioningType => &[ReferenceKind::Unioning],
        TypeDerivedRelationshipCollection::OwnedIntersecting
        | TypeDerivedRelationshipCollection::IntersectingType => &[ReferenceKind::Intersecting],
        TypeDerivedRelationshipCollection::OwnedDifferencing
        | TypeDerivedRelationshipCollection::DifferencingType => &[ReferenceKind::Differencing],
        TypeDerivedRelationshipCollection::OwnedDisjoining => &[ReferenceKind::Disjoining],
    }
}

fn library_specialization_rules(
    metaclass: &str,
) -> impl Iterator<Item = &'static LibrarySpecializationRule> + '_ {
    GENERATED_LIBRARY_SPECIALIZATION_RULES
        .iter()
        .filter(move |rule| rule.metaclass == metaclass)
}

fn conditional_library_specialization_rules(
    metaclass: &str,
) -> impl Iterator<Item = &'static ConditionalLibrarySpecializationRule> + '_ {
    GENERATED_CONDITIONAL_LIBRARY_SPECIALIZATION_RULES
        .iter()
        .filter(move |rule| rule.metaclass == metaclass)
}

fn library_redefinition_rules(
    metaclass: &str,
) -> impl Iterator<Item = &'static LibraryRedefinitionRule> + '_ {
    GENERATED_LIBRARY_REDEFINITION_RULES
        .iter()
        .filter(move |rule| rule.metaclass == metaclass)
}

/// The root standard-library packages every generated library rule anchors into, deduplicated
/// and sorted. The library-closure authority seeds these so that a publication always admits
/// the documents its implied specializations and redefinitions resolve against.
pub(crate) fn library_anchor_packages() -> Vec<&'static str> {
    let mut packages = GENERATED_LIBRARY_SPECIALIZATION_RULES
        .iter()
        .map(|rule| rule.anchor)
        .chain(
            GENERATED_CONDITIONAL_LIBRARY_SPECIALIZATION_RULES
                .iter()
                .flat_map(|rule| [Some(rule.anchor), rule.true_anchor].into_iter().flatten()),
        )
        .chain(
            GENERATED_LIBRARY_REDEFINITION_RULES
                .iter()
                .map(|rule| rule.anchor),
        )
        .filter_map(|anchor| anchor.split("::").next())
        .collect::<Vec<_>>();
    packages.sort_unstable();
    packages.dedup();
    packages
}

/// Maps an exact XMI source metaclass to the parser's owned declaration projection.
///
/// `PayloadFeature` is currently not a lowered declaration kind. Keeping that absence explicit
/// is intentional: a same-named feature, a generic `Feature`, or an enclosing declaration must
/// never be substituted as the source of a normative implied redefinition. Extend this adapter
/// only when lowering publishes an exact `PayloadFeature` projection.
fn lowered_redefinition_source_kind(metaclass: &str) -> Option<DeclarationKind> {
    match metaclass {
        "PayloadFeature" => None,
        _ => None,
    }
}

#[cfg(test)]
fn generated_library_specialization_rule_count() -> usize {
    GENERATED_LIBRARY_SPECIALIZATION_RULES.len()
}

#[cfg(test)]
fn generated_conditional_library_specialization_rule_count() -> usize {
    GENERATED_CONDITIONAL_LIBRARY_SPECIALIZATION_RULES.len()
}

#[cfg(test)]
fn generated_library_redefinition_rule_count() -> usize {
    GENERATED_LIBRARY_REDEFINITION_RULES.len()
}

/// The canonical standard-library target named by one `specializesFromLibrary` check.
///
/// Anchor outcomes are an owned semantic fact. A workspace declaration with matching spelling can
/// never substitute for a language anchor, and several standard-library candidates remain an
/// explicit ambiguity rather than an accidental traversal choice.
#[derive(Debug, Clone, PartialEq, Eq)]
enum LibrarySpecializationAnchor {
    Resolved(DeclarationId),
    Missing,
    Ambiguous(Box<[DeclarationId]>),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct LibrarySpecializationAnchorFacts {
    /// Every generated rule branch receives exactly one outcome at the semantic publication
    /// barrier. The typed `(rule, branch)` identity, rather than rendered anchor or metaclass,
    /// owns identity: an anchor can be deliberately shared by independent normative rules.
    by_rule:
        std::collections::BTreeMap<LibrarySpecializationAnchorKey, LibrarySpecializationAnchor>,
}

impl LibrarySpecializationAnchorFacts {
    /// Compatibility projection for legacy single-anchor rules and the `else` branch of exact
    /// polarity contracts.
    fn outcome(&self, rule_id: &str) -> Option<&LibrarySpecializationAnchor> {
        self.outcome_for(rule_id, LibrarySpecializationAnchorBranch::Default)
    }

    fn outcome_for(
        &self,
        rule_id: &str,
        branch: LibrarySpecializationAnchorBranch,
    ) -> Option<&LibrarySpecializationAnchor> {
        self.by_rule.iter().find_map(|(key, outcome)| {
            (key.rule.0 == rule_id && key.branch == branch).then_some(outcome)
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct LibrarySpecializationDiagnosticKey {
    anchor: &'static str,
    document: DocumentId,
}

#[derive(Debug)]
struct ResolutionResults {
    outcomes: Box<[ResolutionStatus]>,
    ambiguous_candidates: Box<[DeclarationId]>,
    inherited_names: NameIndex,
    solver_status: SolverStatus,
    implied_relationships: Box<[ImpliedRelationship]>,
    library_specialization_anchors: LibrarySpecializationAnchorFacts,
    #[cfg(test)]
    work: ResolutionWork,
}

impl ResolutionResults {
    fn outcome(&self, id: AuthoredReferenceId) -> Option<ResolutionStatus> {
        self.outcomes.get(id.index()).copied()
    }

    fn ambiguous_candidates(&self, range: CandidateRange) -> &[DeclarationId] {
        range.slice(&self.ambiguous_candidates).unwrap_or_default()
    }

    fn library_specialization_anchor(&self, rule_id: &str) -> Option<&LibrarySpecializationAnchor> {
        self.library_specialization_anchors.outcome(rule_id)
    }
}

/// The published outcome of evaluating one `PendingEvaluationFact`: the declaration whose
/// constraint/calc expression it belongs to, and what evaluation settled for it.
///
/// The folding algebra's own `EvaluatedValue` is deliberately not kept here. It is the lattice the
/// fixed point needs, not a fact about the model, and `EvaluationState` already carries the value
/// where there is one -- keeping both would make two representations of one answer, with nothing
/// stopping them from disagreeing.
#[derive(Debug, Clone, PartialEq)]
struct EvaluationFact {
    declaration: DeclarationId,
    state: EvaluationState,
}

/// Slice 2 of the constraint/calc expression fact family (see `4ca42166` for slice 1). Runs
/// strictly after `resolve_dense`'s resolution fixed point settles, mirroring where slice 1's own
/// `ExpressionOperand` references are resolved: literal-only expressions never depend on
/// resolution and could in principle fold earlier, but folding here keeps evaluation a single
/// pass over one immutable resolved state, with no risk of an evaluation fact observing a
/// mid-fixed-point resolution outcome.
///
/// If resolution itself did not converge, no evaluation fact is published: an expression whose
/// operand's own resolution outcome is not settled cannot be conservatively classified as
/// resolved/unresolved, so evaluation stays vacuous for that publication (an empty `evaluation`
/// section, `has-evaluation` false) rather than guessing.
///
/// Slice 3 (constant propagation, see `EvalNode`/`fold_eval_node_pending`): a `HasOperand` fact's
/// tree is folded against a declaration -> constant-value map that is itself the result of a
/// bounded fixed point over every pending evaluation fact (`Literal` facts -- including slice 3's
/// literal-only attribute default values, see `lower_attribute_default_value` -- seed the map with
/// no dependency; each `HasOperand` fact settles once every operand it needs is itself settled,
/// either to a concrete constant, to `NonConstant`/`UnresolvedOperand`, or because its resolved
/// target has no evaluation fact at all). A dependency chain through resolved operand references
/// can be at most one hop longer per pass, so the fixed point is bounded by
/// `evaluation_facts.len() + 1` passes (mirrors `build_ancestor_closures`'s bound); any fact still
/// unsettled after the bound is exhausted is a genuine cross-declaration dependency cycle and
/// publishes the explicit `EvaluatedValue::NonConverged` outcome, never a fabricated value, an
/// infinite loop, or a panic.
fn compute_evaluation(
    storage: &SemanticModelStorage,
    resolution: &ResolutionResults,
    policy: EvaluationPolicy,
) -> SettledEvaluation {
    if policy == EvaluationPolicy::Skip {
        // A declared outcome, not an absent one: every element that has an expression reports
        // that no attempt was made, which a consumer can tell apart from "nothing to evaluate".
        //
        // An unsupported shape still reports `Unsupported`: the policy decides whether evaluation
        // ran, and the shape is outside the evaluated slice whether it ran or not.
        let skipped = |shape: &ExpressionEvalShape| match shape {
            ExpressionEvalShape::Unsupported => EvaluationState::Unsupported,
            _ => EvaluationState::NotRun,
        };
        return SettledEvaluation::Settled {
            facts: storage
                .evaluation_facts
                .iter()
                .map(|pending| EvaluationFact {
                    declaration: pending.declaration,
                    state: skipped(&pending.shape),
                })
                .collect(),
            filters: settled_filters(storage, |condition| skipped(&condition.shape)),
        };
    }
    if resolution.solver_status != SolverStatus::Converged {
        return SettledEvaluation::Vacuous;
    }

    // operand_targets[declaration][ordinal] = the ExpressionOperand reference's resolved target,
    // or None when that reference did not resolve to exactly one declaration.
    let mut operand_targets: std::collections::BTreeMap<DeclarationId, Vec<Option<DeclarationId>>> =
        Default::default();
    for (index, reference) in storage.references.iter().enumerate() {
        if reference.kind != ReferenceKind::ExpressionOperand {
            continue;
        }
        let Ok(id) = AuthoredReferenceId::from_index(index) else {
            continue;
        };
        let target = match resolution.outcome(id) {
            Some(ResolutionStatus::Resolved(target)) => Some(target),
            _ => None,
        };
        let ordinal = reference.ordinal as usize;
        let slot = operand_targets.entry(reference.source).or_default();
        if slot.len() <= ordinal {
            slot.resize(ordinal + 1, None);
        }
        slot[ordinal] = target;
    }

    // Every declaration whose expression can ever settle to a constant -- the only declarations
    // constant propagation can look up a value for. A resolved operand reference whose target is
    // *not* in this set has no known constant, settling immediately as `NonConstant`.
    //
    // An unsupported shape is excluded on purpose. It publishes a fact, but never a value, so a
    // dependent expression must settle against it now rather than wait for a value that cannot
    // arrive and then be reported as a dependency cycle.
    let has_fact: std::collections::BTreeSet<DeclarationId> = storage
        .evaluation_facts
        .iter()
        .filter(|pending| !matches!(pending.shape, ExpressionEvalShape::Unsupported))
        .map(|pending| pending.declaration)
        .collect();

    let mut outcomes: std::collections::BTreeMap<DeclarationId, EvaluatedValue> =
        Default::default();
    for pending in storage.evaluation_facts.iter() {
        match &pending.shape {
            ExpressionEvalShape::Literal(value) | ExpressionEvalShape::ConstantFolded(value) => {
                outcomes.insert(pending.declaration, value.clone());
            }
            ExpressionEvalShape::HasOperand(_) | ExpressionEvalShape::Unsupported => {}
        }
    }

    let pass_limit = storage.evaluation_facts.len().saturating_add(1);
    for _ in 0..pass_limit {
        let mut changed = false;
        for pending in storage.evaluation_facts.iter() {
            let ExpressionEvalShape::HasOperand(tree) = &pending.shape else {
                continue;
            };
            if outcomes.contains_key(&pending.declaration) {
                continue;
            }
            let targets = operand_targets.get(&pending.declaration);
            let mut resolve_operand = |ordinal: u32| -> Option<EvaluatedValue> {
                match targets.and_then(|targets| targets.get(ordinal as usize).copied().flatten()) {
                    None => Some(EvaluatedValue::UnresolvedOperand),
                    Some(target) => match outcomes.get(&target) {
                        Some(value) => Some(value.clone()),
                        None if has_fact.contains(&target) => None,
                        None => Some(EvaluatedValue::NonConstant),
                    },
                }
            };
            if let Some(value) = fold_eval_node_pending(tree, &mut resolve_operand) {
                outcomes.insert(pending.declaration, value);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }

    // Anything still unsettled after the bound is a genuine cross-declaration dependency cycle
    // (directly or transitively self-referential), not a longer-than-expected acyclic chain --
    // the bound already covers every acyclic chain up to the total fact count.
    for pending in storage.evaluation_facts.iter() {
        if matches!(pending.shape, ExpressionEvalShape::Unsupported) {
            continue;
        }
        outcomes
            .entry(pending.declaration)
            .or_insert(EvaluatedValue::NonConverged);
    }

    let mut facts = Vec::with_capacity(storage.evaluation_facts.len());
    for pending in storage.evaluation_facts.iter() {
        if matches!(pending.shape, ExpressionEvalShape::Unsupported) {
            facts.push(EvaluationFact {
                declaration: pending.declaration,
                state: EvaluationState::Unsupported,
            });
            continue;
        }
        let Some(outcome) = outcomes.get(&pending.declaration) else {
            continue;
        };
        facts.push(EvaluationFact {
            declaration: pending.declaration,
            state: evaluation::evaluation_state(outcome, &pending.shape),
        });
    }

    // Filter conditions settle after the declaration fixed point, against its result. They are
    // read-only consumers of it: a condition defines no declaration's value, so nothing can depend
    // on one, and folding them here cannot change what any declaration evaluated to.
    let filters = settled_filters(storage, |condition| {
        fold_settled_expression(
            &condition.shape,
            condition.owner,
            &operand_targets,
            &outcomes,
        )
    });

    SettledEvaluation::Settled {
        facts: facts.into_boxed_slice(),
        filters,
    }
}

/// One settled record per authored `filter` condition, carrying its authored facts alongside the
/// state `state_of` decides for it.
///
/// The authored half is copied here rather than left to a later join: a state array parallel to
/// `SemanticModelStorage::filter_conditions` was a second invariant to keep, and the one branch
/// that could not produce a state for every condition broke it by publishing none.
fn settled_filters(
    storage: &SemanticModelStorage,
    mut state_of: impl FnMut(&AuthoredFilterCondition) -> EvaluationState,
) -> Box<[expression::SettledFilter]> {
    storage
        .filter_conditions
        .iter()
        .map(|condition| expression::SettledFilter {
            owner: condition.owner,
            document: condition.document,
            form: condition.form,
            span: condition.span.clone(),
            state: state_of(condition),
            predicate: condition.predicate.clone(),
        })
        .collect()
}

/// Everything the evaluation pass settled.
///
/// Two states, not one with empty collections. A publication whose resolution did not converge has
/// no stable outcomes to evaluate against, so it settles nothing at all -- which is a different
/// fact from settling that nothing has a value, and the completeness the publication carries is
/// where a consumer reads it.
#[derive(Debug)]
enum SettledEvaluation {
    /// One outcome per classified declaration expression, and one per authored filter condition.
    Settled {
        facts: Box<[EvaluationFact]>,
        filters: Box<[expression::SettledFilter]>,
    },
    /// Resolution did not converge, so no expression was evaluated.
    Vacuous,
}

/// Folds one already-classified expression against the constants a settled publication holds.
///
/// The declaration fixed point above settles every expression a declaration *owns*. A `filter`
/// condition is not one: it is written inside a declaration whose own value it does not define, so
/// it never contributes a constant and never participates in the fixed point. It is folded once,
/// afterwards, against the same settled operand targets and constants, which is exactly the
/// "consume settled facts, publish nothing back" shape the barrier requires.
fn fold_settled_expression(
    shape: &ExpressionEvalShape,
    owner: DeclarationId,
    operand_targets: &std::collections::BTreeMap<DeclarationId, Vec<Option<DeclarationId>>>,
    outcomes: &std::collections::BTreeMap<DeclarationId, EvaluatedValue>,
) -> EvaluationState {
    match shape {
        ExpressionEvalShape::Unsupported => EvaluationState::Unsupported,
        ExpressionEvalShape::Literal(value) | ExpressionEvalShape::ConstantFolded(value) => {
            evaluation::evaluation_state(value, shape)
        }
        ExpressionEvalShape::HasOperand(tree) => {
            let targets = operand_targets.get(&owner);
            let value = fold_eval_node(tree, &mut |ordinal: u32| {
                match targets.and_then(|targets| targets.get(ordinal as usize).copied().flatten()) {
                    None => EvaluatedValue::UnresolvedOperand,
                    // A target with no settled constant of its own is correctly not a constant:
                    // the declaration fixed point has already run to completion, so an absent
                    // entry is an answer, not a pending one.
                    Some(target) => outcomes
                        .get(&target)
                        .cloned()
                        .unwrap_or(EvaluatedValue::NonConstant),
                }
            });
            evaluation::evaluation_state(&value, shape)
        }
    }
}

#[derive(Debug)]
pub(crate) struct ResolvedSemanticModel {
    storage: SemanticModelStorage,
    direct_names: NameIndex,
    effective_imports: NameIndex,
    identities: IdentityIndex,
    documents: DocumentIndex,
    memberships: MembershipIndex,
    reverse_references: ReverseReferenceIndex,
    effective_scopes: EffectiveScopeIndex,
    facts: inspection::ElementFactIndex,
    /// Canonical paired binding-connector endpoints, assembled once after resolution.
    bindings: binding::BindingConnectorIndex,
    types: types::TypeIndex,
    resolution: ResolutionResults,
    evaluation: Box<[EvaluationFact]>,
    /// Settled unit, measurement and filter facts over the expressions this publication admitted.
    expressions: expression::ExpressionIndex,
    /// Settled at the publication barrier alongside the indexes, so reading them is a lookup and
    /// a broken storage invariant fails the build instead of a later query.
    diagnostics: Box<[Diagnostic]>,
    /// Where each document's diagnostics begin and end inside `diagnostics`, by `DocumentId`.
    ///
    /// The derivation groups one document's diagnostics contiguously, so a document-scoped query
    /// is a slice of the settled sequence rather than a scan of it. Built at the same barrier so
    /// the two can never disagree.
    diagnostics_by_document: Box<[(u32, u32)]>,
    metadata: PublicationMetadata,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PublicationPhase {
    Resolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PublicationCompleteness {
    Complete,
    ParseRecovery,
    UnsupportedSyntax,
    NonConverged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PublicationMetadata {
    phase: PublicationPhase,
    completeness: PublicationCompleteness,
    has_evaluation: bool,
}

impl ResolvedSemanticModel {
    pub(crate) fn completeness(&self) -> PublicCompleteness {
        match self.metadata.completeness {
            PublicationCompleteness::Complete => PublicCompleteness::Complete,
            PublicationCompleteness::ParseRecovery => PublicCompleteness::ParseRecovery,
            PublicationCompleteness::UnsupportedSyntax => PublicCompleteness::UnsupportedSyntax,
            PublicationCompleteness::NonConverged => PublicCompleteness::NonConverged,
        }
    }

    pub(crate) fn affected_documents(
        &self,
        changed_document: &str,
    ) -> QueryOutcome<Box<[crate::AffectedDocument]>> {
        let Some(changed) = self
            .storage
            .documents
            .iter()
            .position(|document| document.identity.as_ref() == changed_document)
            .map(|index| DocumentId(index as u32))
        else {
            return QueryOutcome::Unresolved;
        };

        // `provider -> consumer`. Both resolved and ambiguous outcomes are owned facts: every
        // ambiguous candidate can affect the consumer if it changes. Unresolved/unsupported
        // dependency-shaping references remain explicit through the publication completeness or
        // the conservative result below; they are never guessed from authored text.
        let mut reverse = vec![Vec::<DocumentId>::new(); self.storage.documents.len()];
        let mut unsettled_dependency = false;
        for (ordinal, reference) in self.storage.references.iter().enumerate() {
            if !matches!(
                reference.kind,
                ReferenceKind::NamespaceImport
                    | ReferenceKind::MembershipImport
                    | ReferenceKind::FilterImport
                    | ReferenceKind::AliasBinding
            ) {
                continue;
            }
            let Some(source_document) = self
                .storage
                .declaration(reference.source)
                .map(|declaration| declaration.document)
            else {
                unsettled_dependency = true;
                continue;
            };
            let reference_id = AuthoredReferenceId(ordinal as u32);
            let targets: Vec<DeclarationId> = match self.resolution.outcome(reference_id) {
                Some(ResolutionStatus::Resolved(target)) => vec![target],
                Some(ResolutionStatus::Ambiguous(range)) => {
                    self.resolution.ambiguous_candidates(range).to_vec()
                }
                Some(
                    ResolutionStatus::Unresolved
                    | ResolutionStatus::Unsupported
                    | ResolutionStatus::NonConverged,
                )
                | None => {
                    unsettled_dependency = true;
                    continue;
                }
            };
            for target in targets {
                if let Some(target_document) = self
                    .storage
                    .declaration(target)
                    .map(|declaration| declaration.document)
                {
                    if target_document != source_document {
                        reverse[target_document.index()].push(source_document);
                    }
                }
            }
        }

        let mut seen = vec![false; self.storage.documents.len()];
        let mut pending = vec![changed];
        seen[changed.index()] = true;
        while let Some(provider) = pending.pop() {
            for consumer in reverse[provider.index()].iter().copied() {
                if !seen[consumer.index()] {
                    seen[consumer.index()] = true;
                    pending.push(consumer);
                }
            }
        }
        let mut affected = seen
            .into_iter()
            .enumerate()
            .filter(|(index, affected)| *affected && *index != changed.index())
            .filter_map(|(index, _)| self.storage.documents.get(index))
            .map(|document| crate::AffectedDocument {
                identity: document.identity.clone(),
                source: match document.role {
                    source_identity::SourceRole::Workspace => crate::ElementSource::Workspace,
                    source_identity::SourceRole::StandardLibrary => {
                        crate::ElementSource::StandardLibrary
                    }
                    source_identity::SourceRole::Library => crate::ElementSource::Library,
                    source_identity::SourceRole::External => crate::ElementSource::External,
                },
            })
            .collect::<Vec<_>>();
        affected.sort_by(|left, right| left.identity.cmp(&right.identity));
        let affected = affected.into_boxed_slice();
        if unsettled_dependency {
            match self.metadata.completeness {
                PublicationCompleteness::NonConverged => QueryOutcome::Incomplete,
                PublicationCompleteness::UnsupportedSyntax => {
                    QueryOutcome::UnsupportedWith(affected)
                }
                _ => QueryOutcome::Recovered(affected),
            }
        } else {
            self.resolved_outcome(affected)
        }
    }

    fn resolved_outcome<T>(&self, value: T) -> QueryOutcome<T> {
        match self.metadata.completeness {
            PublicationCompleteness::Complete => QueryOutcome::Resolved(value),
            PublicationCompleteness::ParseRecovery => QueryOutcome::Recovered(value),
            PublicationCompleteness::UnsupportedSyntax => QueryOutcome::UnsupportedWith(value),
            PublicationCompleteness::NonConverged => QueryOutcome::Incomplete,
        }
    }

    /// The canonical structural identity of one declaration.
    ///
    /// Stable across builds of the same sources, unlike the dense storage ordinal, so a consumer
    /// may hold one across a rebuild; see `IdentityIndex`.
    fn symbol_identity(&self, id: DeclarationId) -> Option<SymbolIdentity> {
        self.identities
            .identity(id)
            .map(|text| SymbolIdentity(text.into()))
    }

    /// Every declaration carrying `identity`.
    ///
    /// More than one only when the source authors identically named siblings; callers publish that
    /// as an explicit ambiguous outcome rather than choosing between them.
    fn identity_declarations(&self, identity: &SymbolIdentity) -> Vec<DeclarationId> {
        self.identities.declarations(&identity.0)
    }

    fn declaration_target(&self, id: DeclarationId) -> Option<NavigationTarget> {
        let declaration = self.storage.declaration(id)?;
        let name = self.storage.symbol(declaration.name?)?;
        Some(NavigationTarget {
            symbol: self.symbol_identity(id)?,
            name: name.into(),
            location: SourceLocation {
                document: self
                    .storage
                    .document(declaration.document)?
                    .identity
                    .clone(),
                range: declaration_identifier_range(
                    &self.storage,
                    declaration.document,
                    &declaration.span,
                    name,
                )
                .ok()?,
                role: OccurrenceRole::Declaration,
            },
        })
    }

    pub(crate) fn target_at(
        &self,
        document: &str,
        position: TextPosition,
    ) -> QueryOutcome<NavigationTarget> {
        let Some(document_id) = self.documents.document(&self.storage, document) else {
            return QueryOutcome::Unresolved;
        };
        let Some(positions) = self.documents.positions(document_id) else {
            return QueryOutcome::Unresolved;
        };
        let mut reference_matches = Vec::new();
        for reference_id in leaf_ranges_containing(&positions.references, position) {
            match self.resolution.outcome(reference_id) {
                Some(ResolutionStatus::Resolved(target)) => {
                    if let Some(target) = self.declaration_target(target) {
                        reference_matches.push(target);
                    }
                }
                Some(ResolutionStatus::Ambiguous(range)) => {
                    let mut targets = self
                        .resolution
                        .ambiguous_candidates(range)
                        .iter()
                        .filter_map(|id| self.declaration_target(*id))
                        .collect::<Vec<_>>();
                    targets.sort_by(target_order);
                    targets.dedup_by(|a, b| a.symbol == b.symbol);
                    return QueryOutcome::Ambiguous(targets.into_boxed_slice());
                }
                Some(ResolutionStatus::Unsupported) => return QueryOutcome::Unsupported,
                Some(ResolutionStatus::NonConverged) => return QueryOutcome::Incomplete,
                Some(ResolutionStatus::Unresolved) | None => return QueryOutcome::Unresolved,
            }
        }
        reference_matches.sort_by(target_order);
        reference_matches.dedup_by(|a, b| a.symbol == b.symbol);
        if reference_matches.len() == 1 {
            return self.resolved_outcome(reference_matches.remove(0));
        }
        if reference_matches.len() > 1 {
            return QueryOutcome::Ambiguous(reference_matches.into_boxed_slice());
        }
        let mut declarations = leaf_ranges_containing(&positions.identifiers, position)
            .filter_map(|id| self.declaration_target(id))
            .collect::<Vec<_>>();
        declarations.sort_by(target_order);
        match declarations.len() {
            0 => QueryOutcome::Unresolved,
            1 => self.resolved_outcome(declarations.remove(0)),
            _ => QueryOutcome::Ambiguous(declarations.into_boxed_slice()),
        }
    }

    pub(crate) fn references(
        &self,
        symbol: &SymbolIdentity,
        include_declaration: bool,
    ) -> QueryOutcome<Box<[SourceLocation]>> {
        if matches!(
            self.metadata.completeness,
            PublicationCompleteness::NonConverged
        ) {
            return QueryOutcome::Incomplete;
        }
        let mut targets = self.identity_declarations(symbol);
        if targets.len() > 1 {
            // The caller's identity names identically authored siblings. Answering for one of
            // them would silently pick; answering for all of them as one list would merge distinct
            // elements' references, so each candidate's own list is published separately.
            let per_candidate = targets
                .into_iter()
                .map(
                    |target| match self.references_for(target, include_declaration) {
                        QueryOutcome::Resolved(locations)
                        | QueryOutcome::Recovered(locations)
                        | QueryOutcome::UnsupportedWith(locations) => locations,
                        _ => Box::default(),
                    },
                )
                .collect::<Vec<_>>();
            return QueryOutcome::Ambiguous(per_candidate.into_boxed_slice());
        }
        let Some(target) = targets.pop() else {
            return QueryOutcome::Unresolved;
        };
        self.references_for(target, include_declaration)
    }

    fn references_for(
        &self,
        target: DeclarationId,
        include_declaration: bool,
    ) -> QueryOutcome<Box<[SourceLocation]>> {
        let Some(target_declaration) = self.storage.declaration(target) else {
            return QueryOutcome::Unresolved;
        };
        let mut locations = Vec::new();
        if include_declaration {
            if let Some(target) = self.declaration_target(target) {
                locations.push(target.location);
            }
        }
        let references = self.reverse_references.references(target);
        record_visited_index_entries(references.len());
        for id in references {
            let Some(reference) = self.storage.references.get(id.index()) else {
                return QueryOutcome::Incomplete;
            };
            let Some(source) = self.storage.declaration(reference.source) else {
                return QueryOutcome::Incomplete;
            };
            let Some(name) = self
                .storage
                .symbol(target_declaration.name.unwrap_or(SymbolId(u32::MAX)))
            else {
                return QueryOutcome::Incomplete;
            };
            let range = identifier_range(&self.storage, source.document, &reference.span, name)
                .or_else(|_| document_range(&self.storage, source.document, &reference.span));
            let Ok(range) = range else {
                return QueryOutcome::Incomplete;
            };
            locations.push(SourceLocation {
                document: self
                    .storage
                    .document(source.document)
                    .map(|d| d.identity.clone())
                    .unwrap_or_default(),
                range,
                role: OccurrenceRole::Reference,
            });
        }
        locations.sort_by(location_order);
        locations.dedup();
        self.resolved_outcome(locations.into_boxed_slice())
    }

    pub(crate) fn prepare_rename(
        &self,
        document: &str,
        position: TextPosition,
        new_name: Option<&str>,
    ) -> RenameOutcome {
        let target = match self.target_at(document, position) {
            QueryOutcome::Resolved(target)
            | QueryOutcome::Recovered(target)
            | QueryOutcome::UnsupportedWith(target) => target,
            QueryOutcome::Ambiguous(targets) => return RenameOutcome::Ambiguous(targets),
            QueryOutcome::Unsupported => return RenameOutcome::Unsupported,
            QueryOutcome::Recovery => return RenameOutcome::Recovery,
            QueryOutcome::Incomplete => return RenameOutcome::Incomplete,
            QueryOutcome::Unresolved => return RenameOutcome::Unresolved,
        };
        if let Some(name) = new_name {
            if !valid_identifier(name) {
                return RenameOutcome::InvalidName;
            }
            let mut candidates = self.identity_declarations(&target.symbol);
            if candidates.len() > 1 {
                let mut ambiguous = candidates
                    .into_iter()
                    .filter_map(|candidate| self.declaration_target(candidate))
                    .collect::<Vec<_>>();
                ambiguous.sort_by(target_order);
                return RenameOutcome::Ambiguous(ambiguous.into_boxed_slice());
            }
            let Some(id) = candidates.pop() else {
                return RenameOutcome::Incomplete;
            };
            let Some(declaration) = self.storage.declaration(id) else {
                return RenameOutcome::Incomplete;
            };
            if let Some(symbol) = self.storage.symbols.find(name) {
                let mut collisions = self
                    .direct_names
                    .candidates(declaration.owner, symbol)
                    .iter()
                    .filter(|candidate| **candidate != id)
                    .filter_map(|candidate| self.declaration_target(*candidate))
                    .collect::<Vec<_>>();
                collisions.sort_by(target_order);
                if !collisions.is_empty() {
                    return RenameOutcome::Collision(collisions.into_boxed_slice());
                }
            }
        }
        let occurrences = match self.references(&target.symbol, true) {
            QueryOutcome::Resolved(value) => value,
            _ => return RenameOutcome::Incomplete,
        };
        let range = occurrences
            .iter()
            .find(|location| {
                location.document.as_ref() == document && range_contains(location.range, position)
            })
            .map(|location| location.range)
            .unwrap_or(target.location.range);
        RenameOutcome::Ready {
            symbol: target.symbol,
            name: target.name,
            range,
            occurrences,
        }
    }

    pub(crate) fn visible_members(
        &self,
        document: &str,
        position: TextPosition,
        qualifier: Option<&str>,
    ) -> QueryOutcome<Box<[VisibleMember]>> {
        let recovered = matches!(
            self.metadata.completeness,
            PublicationCompleteness::ParseRecovery
        );
        let unsupported = matches!(
            self.metadata.completeness,
            PublicationCompleteness::UnsupportedSyntax
        );
        if matches!(
            self.metadata.completeness,
            PublicationCompleteness::NonConverged
        ) {
            return QueryOutcome::Incomplete;
        }
        let Some(document_id) = self.documents.document(&self.storage, document) else {
            return QueryOutcome::Unresolved;
        };
        let Some(positions) = self.documents.positions(document_id) else {
            return QueryOutcome::Unresolved;
        };
        let owner = positions.spans.innermost_containing(position);
        let mut ids = Vec::new();
        if let Some(qualifier) = qualifier {
            let scopes = match self.resolve_qualifier_scopes(owner, qualifier) {
                Ok(scopes) if !scopes.is_empty() => scopes,
                Ok(_) => return QueryOutcome::Unresolved,
                Err(_) => return QueryOutcome::Incomplete,
            };
            if scopes.len() > 1 {
                let candidates = scopes
                    .into_iter()
                    .map(|scope| {
                        self.visible_member_records(self.effective_scopes.members(Some(scope)))
                    })
                    .collect::<Vec<_>>()
                    .into_boxed_slice();
                return QueryOutcome::Ambiguous(candidates);
            }
            ids.extend_from_slice(self.effective_scopes.members(Some(scopes[0])));
        } else {
            let mut scope = owner;
            loop {
                ids.extend_from_slice(self.effective_scopes.members(scope));
                let Some(current) = scope else {
                    break;
                };
                scope = self
                    .storage
                    .declaration(current)
                    .and_then(|declaration| declaration.owner);
            }
        }
        let members = self.visible_member_records(&ids);
        if recovered {
            QueryOutcome::Recovered(members)
        } else if unsupported {
            QueryOutcome::UnsupportedWith(members)
        } else {
            QueryOutcome::Resolved(members)
        }
    }

    fn resolve_qualifier_scopes(
        &self,
        owner: Option<DeclarationId>,
        qualifier: &str,
    ) -> Result<Vec<DeclarationId>, ResolutionError> {
        let Some(segments) = qualifier
            .split("::")
            .filter(|segment| !segment.is_empty())
            .map(|segment| self.storage.symbols.find(segment))
            .collect::<Option<Vec<_>>>()
        else {
            return Ok(Vec::new());
        };
        let Some(first) = segments.first().copied() else {
            return Ok(Vec::new());
        };
        let mut candidates = Vec::new();
        let mut work = ResolutionWork::default();
        lookup_lexical_into(
            &self.storage.declarations,
            &ResolutionIndexes {
                direct_names: &self.direct_names,
                exported_names: &self.direct_names,
                effective_imports: Some(&self.effective_imports),
                exported_imports: Some(&self.effective_imports),
                inherited_names: Some(&self.resolution.inherited_names),
            },
            owner,
            first,
            // An interactive lookup has no redefining feature to exclude.
            LookupTarget {
                domain: DeclarationDomain::Any,
                excluded: None,
            },
            &mut candidates,
            &mut work,
        )?;
        let mut next = Vec::new();
        for segment in &segments[1..] {
            next.clear();
            for candidate in candidates.iter().copied() {
                let direct = self.direct_names.candidates(Some(candidate), *segment);
                if direct.is_empty() {
                    next.extend_from_slice(
                        self.effective_imports.candidates(Some(candidate), *segment),
                    );
                } else {
                    next.extend_from_slice(direct);
                }
            }
            next.sort_unstable();
            next.dedup();
            std::mem::swap(&mut candidates, &mut next);
        }
        Ok(candidates)
    }

    fn visible_member_records(&self, ids: &[DeclarationId]) -> Box<[VisibleMember]> {
        let mut ids = ids.to_vec();
        ids.sort_unstable();
        ids.dedup();
        let mut members = ids
            .into_iter()
            .filter_map(|id| {
                let target = self.declaration_target(id)?;
                let declaration = self.storage.declaration(id)?;
                let qualified_name = declaration_qualified_name(&self.storage, id)?;
                let container_name = declaration
                    .owner
                    .and_then(|owner| self.storage.declaration(owner)?.name)
                    .and_then(|name| self.storage.symbol(name))
                    .map(Into::into);
                Some(VisibleMember {
                    symbol: target.symbol,
                    name: target.name,
                    kind: element_kind::element_kind(declaration.kind),
                    role: element_kind::membership_role(declaration.kind),
                    qualified_name: qualified_name.into_boxed_str(),
                    container_name,
                    declaring_document: target.location.document,
                    declaration_range: target.location.range,
                })
            })
            .collect::<Vec<_>>();
        members.sort_by(|a, b| {
            a.name
                .cmp(&b.name)
                .then_with(|| a.declaring_document.cmp(&b.declaring_document))
                .then_with(|| a.declaration_range.cmp(&b.declaration_range))
        });
        members.into_boxed_slice()
    }

    /// Every diagnostic this publication settled, as facts rather than rendered text.
    ///
    /// Derived once while the model is sealed, for the same reason the other indexes are: a query
    /// against a settled publication reads a fact instead of recomputing one, and a storage
    /// inconsistency fails the build rather than surfacing later as a silently missing diagnostic.
    ///
    /// Only workspace-authored documents contribute. Library sources take part in the same
    /// semantic system, but their own diagnostics are not the authoring surface, and this also
    /// keeps the barrier's cost proportional to the workspace rather than to the library.
    fn derive_diagnostics(
        &self,
        reported: &[Box<str>],
    ) -> Result<DerivedDiagnostics, ResolutionError> {
        let mut diagnostics = Vec::new();
        let mut by_document = vec![(0u32, 0u32); self.storage.documents.len()];
        // Built once, then sliced per document. Every rule below asks "what did this document
        // declare", and answering that by scanning every declaration each time made the barrier
        // quadratic in the admitted corpus -- invisible while only workspace documents were
        // derived, and the reason a library's own documents could not be.
        let declarations_by_document = self.declarations_by_document()?;
        for document_index in self.reported_document_indices(reported) {
            let document = &self.storage.documents[document_index];
            let document_id = DocumentId(document_index as u32);
            let first = diagnostics.len();

            for error in document.parse_errors.iter() {
                let range = parse_error_range(&document.parsed, error)
                    .ok_or(ResolutionError::InvalidStorage)?;
                diagnostics.push(Diagnostic {
                    // The parser owns both the code and the sentence; neither is re-derived here.
                    message: error.message.as_str().into(),
                    subject: None,
                    code: DiagnosticCode::Parser {
                        code: match error.code.as_deref() {
                            Some(code) => code.into(),
                            None => UNCODED_PARSE_ERROR.into(),
                        },
                        category: parser_diagnostic_category(error.category),
                    },
                    severity: match error.severity {
                        Some(sysml_v2_parser::DiagnosticSeverity::Warning) => {
                            DiagnosticSeverity::Warning
                        }
                        Some(sysml_v2_parser::DiagnosticSeverity::Error) | None => {
                            DiagnosticSeverity::Error
                        }
                    },
                    origin: DiagnosticOrigin::Parser,
                    location: DiagnosticLocation {
                        document: document.identity.clone(),
                        range,
                    },
                    related: Box::default(),
                });
            }

            for record in self
                .storage
                .unsupported
                .iter()
                .filter(|record| record.document == document_id)
            {
                let code = unsupported_construct_code(record.family);
                diagnostics.push(Diagnostic {
                    message: code.describe().into(),
                    subject: None,
                    code,
                    severity: DiagnosticSeverity::Warning,
                    origin: DiagnosticOrigin::Semantic,
                    location: DiagnosticLocation {
                        document: document.identity.clone(),
                        range: document_range(&self.storage, document_id, &record.span)?,
                    },
                    related: Box::default(),
                });
            }

            for (index, reference) in self.storage.references.iter().enumerate() {
                let source = self
                    .storage
                    .declaration(reference.source)
                    .ok_or(ResolutionError::InvalidStorage)?;
                if source.document != document_id {
                    continue;
                }
                let reference_id = AuthoredReferenceId::from_index(index)
                    .map_err(|_| ResolutionError::Capacity)?;
                let status = self
                    .resolution
                    .outcome(reference_id)
                    .ok_or(ResolutionError::InvalidStorage)?;
                let Some((severity, code)) = reference_diagnostic(reference.kind, status) else {
                    continue;
                };
                let mut related = Vec::new();
                if let ResolutionStatus::Ambiguous(candidates) = status {
                    // Every candidate, in the resolver's canonical candidate order: choosing one
                    // would settle an ambiguity the publication deliberately left open.
                    for target in self.resolution.ambiguous_candidates(candidates) {
                        related
                            .push(self.related_declaration(*target, RELATED_AMBIGUOUS_CANDIDATE)?);
                    }
                }
                diagnostics.push(Diagnostic {
                    message: code.describe().into(),
                    subject: self.symbol_identity(reference.source),
                    code,
                    severity,
                    origin: DiagnosticOrigin::Semantic,
                    location: DiagnosticLocation {
                        document: document.identity.clone(),
                        range: document_range(&self.storage, document_id, &reference.span)?,
                    },
                    related: related.into_boxed_slice(),
                });
            }

            let declared = declarations_by_document
                .get(document_index)
                .map(Vec::as_slice)
                .unwrap_or_default();
            self.collect_conformance(document_id, declared, &mut diagnostics)?;
            self.collect_structural_conformance(document_id, declared, &mut diagnostics)?;
            self.collect_expression_conformance(document_id, declared, &mut diagnostics)?;
            self.collect_host_conformance(document_id, declared, &mut diagnostics)?;
            self.collect_library_context(document_id, first, &mut diagnostics)?;

            // Ordering is owned here so no consumer has to sort, and so the order cannot vary with
            // which storage collection a diagnostic happened to come from. The sort is stable, so
            // parser, unsupported-construct, and reference diagnostics that share a range keep the
            // order they were derived in.
            diagnostics[first..].sort_by(|left, right| {
                left.location
                    .range
                    .cmp(&right.location.range)
                    .then_with(|| left.code.as_str().cmp(right.code.as_str()))
            });
            by_document[document_index] = (
                u32::try_from(first).map_err(|_| ResolutionError::Capacity)?,
                u32::try_from(diagnostics.len()).map_err(|_| ResolutionError::Capacity)?,
            );
        }
        Ok((
            diagnostics.into_boxed_slice(),
            by_document.into_boxed_slice(),
        ))
    }

    /// The documents whose diagnostics this publication derives, in canonical order.
    ///
    /// Every workspace-authored document, plus any admitted document the host explicitly named.
    ///
    /// The default is provenance: a workspace does not inherit its library's diagnostics, and
    /// deriving every admitted document would make the barrier cost the whole library on every
    /// rebuild. But provenance is not the same question as *authoring surface*: an editor with a
    /// library file open is authoring it, and only the host knows that. Naming the document is how
    /// it says so, and it is a build input rather than a query option because a diagnostic must be
    /// settled before the publication is visible.
    fn reported_document_indices(&self, reported: &[Box<str>]) -> Vec<usize> {
        let mut indices = (0..self.storage.documents.len())
            .filter(|index| {
                let document = &self.storage.documents[*index];
                document.role == source_identity::SourceRole::Workspace
                    || reported
                        .iter()
                        .any(|identity| identity.as_ref() == document.identity.as_ref())
            })
            .collect::<Vec<_>>();
        indices.sort_by(|left, right| {
            self.storage.documents[*left]
                .identity
                .cmp(&self.storage.documents[*right].identity)
                .then_with(|| left.cmp(right))
        });
        indices
    }

    /// Every declaration each admitted document authored, indexed by document.
    ///
    /// One pass over storage, so the diagnostic barrier costs the corpus once rather than once per
    /// document per rule.
    fn declarations_by_document(&self) -> Result<Vec<Vec<DeclarationId>>, ResolutionError> {
        let mut by_document = vec![Vec::new(); self.storage.documents.len()];
        for index in 0..self.storage.declarations.len() {
            let id = DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
            let declaration = self
                .storage
                .declaration(id)
                .ok_or(ResolutionError::InvalidStorage)?;
            by_document
                .get_mut(declaration.document.index())
                .ok_or(ResolutionError::InvalidStorage)?
                .push(id);
        }
        Ok(by_document)
    }

    /// The diagnostics one document owns, as the settled slice rather than a filtered scan.
    ///
    /// A document this publication did not admit has no diagnostics, which is a different answer
    /// from "no diagnostic was reported": the caller asked about a document that is not part of
    /// this model, and the empty slice says so alongside the publication's completeness.
    pub(crate) fn published_document_diagnostics(&self, document: &str) -> PublishedDiagnostics {
        let diagnostics = match self.documents.document(&self.storage, document) {
            Some(id) => match self.diagnostics_by_document.get(id.index()) {
                Some((start, end)) => self.diagnostics[*start as usize..*end as usize].into(),
                None => Box::default(),
            },
            None => Box::default(),
        };
        PublishedDiagnostics {
            completeness: self.completeness(),
            diagnostics,
        }
    }

    pub(crate) fn published_diagnostics(&self) -> PublishedDiagnostics {
        PublishedDiagnostics {
            completeness: self.completeness(),
            diagnostics: self.diagnostics.clone(),
        }
    }

    pub(crate) fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    pub(crate) fn write_semantic_sexpr(
        &self,
        source_digest: &source_identity::RootDigest,
        semantic_contract_version: &str,
        output: &mut dyn std::fmt::Write,
    ) -> std::fmt::Result {
        writer::write_semantic(self, source_digest, semantic_contract_version, output)
    }

    pub(crate) fn write_navigation_sexpr(
        &self,
        output: &mut dyn std::fmt::Write,
    ) -> std::fmt::Result {
        writer::write_navigation_only(self, output)
    }

    pub(crate) fn write_diagnostics_sexpr(
        &self,
        output: &mut dyn std::fmt::Write,
    ) -> std::fmt::Result {
        writer::write_diagnostics(self, output)
    }

    pub(crate) fn write_types_sexpr(&self, output: &mut dyn std::fmt::Write) -> std::fmt::Result {
        writer::write_types_only(self, output)
    }

    /// Resolves one published identity to the single declaration it names.
    ///
    /// Every type query needs this, and every one of them owes the caller the same three explicit
    /// answers: the publication did not converge, the identity names nothing, or it names several
    /// identically authored siblings and choosing between them would be a guess.
    fn single_declaration<T>(
        &self,
        symbol: &SymbolIdentity,
    ) -> Result<DeclarationId, QueryOutcome<T>> {
        if matches!(
            self.metadata.completeness,
            PublicationCompleteness::NonConverged
        ) {
            return Err(QueryOutcome::Incomplete);
        }
        let mut candidates = self.identity_declarations(symbol);
        if candidates.len() > 1 {
            return Err(QueryOutcome::Unresolved);
        }
        candidates.pop().ok_or(QueryOutcome::Unresolved)
    }

    fn symbols(&self, declarations: impl Iterator<Item = DeclarationId>) -> Box<[SymbolIdentity]> {
        let mut symbols = declarations
            .filter_map(|id| self.symbol_identity(id))
            .collect::<Vec<_>>();
        symbols.sort();
        symbols.dedup();
        symbols.into_boxed_slice()
    }

    pub(crate) fn direct_types(
        &self,
        symbol: &SymbolIdentity,
    ) -> QueryOutcome<Box<[TypeReference]>> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let mut types = self
            .types
            .direct_types(declaration)
            .iter()
            .filter_map(|(target, provenance)| {
                Some(TypeReference {
                    symbol: self.symbol_identity(*target)?,
                    provenance: match provenance {
                        types::FactProvenance::Authored => RelationshipProvenance::Authored,
                        types::FactProvenance::Implied => RelationshipProvenance::Implied,
                    },
                })
            })
            .collect::<Vec<_>>();
        types.sort_by(|left, right| left.symbol.cmp(&right.symbol));
        self.resolved_outcome(types.into_boxed_slice())
    }

    /// Projects one exact KerML Feature relationship collection from the canonical relationship
    /// index. No relationship is re-derived here: authored and implied edges, their provenance,
    /// and target-resolution state are all the same facts an element inspection publishes.
    pub(crate) fn feature_derived_relationships(
        &self,
        symbol: &SymbolIdentity,
        collection: FeatureDerivedRelationshipCollection,
    ) -> QueryOutcome<Box<[ElementRelationship]>> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let Some(rule) = feature_derived_relationship_rule(collection) else {
            // A public enum value with no generated pinned-manifest contract is not a silently
            // empty collection. It is an incomplete implementation boundary.
            return QueryOutcome::Unsupported;
        };
        if rule.metaclass != "Feature"
            || self
                .memberships
                .get(declaration)
                .is_none_or(|membership| membership.kind != MembershipKind::Feature)
        {
            // The rule is defined on raw KerML Feature. A non-feature source has no valid empty
            // answer, and a lowering projection we do not yet classify as a Feature must remain
            // explicit rather than being accepted by display name or owning syntax.
            return QueryOutcome::Unsupported;
        }
        let _rule_id = rule.rule_id;
        self.resolved_outcome(
            self.relationships_of_kinds(
                declaration,
                feature_derived_relationship_kinds(collection),
            ),
        )
    }

    /// Projects one exact KerML Type relationship collection or operand projection from the
    /// canonical relationship index. Operand queries intentionally return their relationships,
    /// retaining authored/implied provenance and unresolved target state.
    pub(crate) fn type_derived_relationships(
        &self,
        symbol: &SymbolIdentity,
        collection: TypeDerivedRelationshipCollection,
    ) -> QueryOutcome<Box<[ElementRelationship]>> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let Some(rule) = type_derived_relationship_rule(collection) else {
            return QueryOutcome::Unsupported;
        };
        if rule.metaclass != "Type"
            || self
                .storage
                .declaration(declaration)
                .is_none_or(|declaration| !DeclarationDomain::Type.accepts(declaration.kind))
        {
            return QueryOutcome::Unsupported;
        }
        let _rule_id = rule.rule_id;
        self.resolved_outcome(
            self.relationships_of_kinds(declaration, type_derived_relationship_kinds(collection)),
        )
    }

    /// Projects one exact final Type member-element collection from canonical declaration owner
    /// membership, and modifier facts. It deliberately does not reconstruct `FeatureMembership`
    /// objects or consult source spelling, because callers ask for the selected member elements
    /// themselves.
    pub(crate) fn type_derived_elements(
        &self,
        symbol: &SymbolIdentity,
        collection: TypeDerivedElementCollection,
    ) -> QueryOutcome<Box<[SymbolIdentity]>> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let Some(rule) = type_derived_element_rule(collection) else {
            return QueryOutcome::Unsupported;
        };
        if rule.metaclass != "Type"
            || self
                .storage
                .declaration(declaration)
                .is_none_or(|value| !DeclarationDomain::Type.accepts(value.kind))
        {
            return QueryOutcome::Unsupported;
        }
        let _rule_id = rule.rule_id;
        let values = self.symbols(
            self.storage
                .declarations
                .iter()
                .enumerate()
                .filter_map(|(index, candidate_declaration)| {
                    let candidate = DeclarationId::from_index(index).ok()?;
                    (candidate != declaration && candidate_declaration.owner == Some(declaration))
                        .then_some(candidate)
                })
                .filter(|candidate| match collection {
                    TypeDerivedElementCollection::OwnedFeature => self
                        .memberships
                        .get(*candidate)
                        .is_some_and(|membership| membership.kind == MembershipKind::Feature),
                    TypeDerivedElementCollection::OwnedEndFeature => {
                        self.memberships
                            .get(*candidate)
                            .is_some_and(|membership| membership.kind == MembershipKind::Feature)
                            && self
                                .storage
                                .declaration_facts(*candidate)
                                .is_some_and(|facts| facts.modifiers.end)
                    }
                }),
        );
        self.resolved_outcome(values)
    }

    /// Every feature membership one Type owns directly, in canonical declaration order.
    fn owned_feature_members(&self, owner: DeclarationId) -> Vec<DeclarationId> {
        self.storage
            .declarations
            .iter()
            .enumerate()
            .filter_map(|(index, candidate_declaration)| {
                let candidate = DeclarationId::from_index(index).ok()?;
                (candidate != owner
                    && candidate_declaration.owner == Some(owner)
                    && self
                        .memberships
                        .get(candidate)
                        .is_some_and(|membership| membership.kind == MembershipKind::Feature))
                .then_some(candidate)
            })
            .collect()
    }

    /// Records every member `member` redefines, from settled `Redefinition` facts.
    ///
    /// Authored and implied redefinitions are read alike: an implied redefinition is a canonical
    /// relationship of the publication, and a member it replaces is inherited no more than one a
    /// written `:>>` replaces.
    fn collect_redefined_members(
        &self,
        member: DeclarationId,
        into: &mut std::collections::BTreeSet<DeclarationId>,
    ) {
        for reference_id in self.outgoing_reference_ids(member) {
            if self.storage.references[reference_id.index()].kind != ReferenceKind::Redefinition {
                continue;
            }
            if let Some(ResolutionStatus::Resolved(target)) = self.resolution.outcome(*reference_id)
            {
                into.insert(target);
            }
        }
        for index in self.outgoing_implied_indices(member) {
            let implied = &self.resolution.implied_relationships[*index as usize];
            if implied.kind == ReferenceKind::Redefinition {
                into.insert(implied.target);
            }
        }
    }

    /// The canonical inherited FeatureMembership closure of one Type.
    ///
    /// KerML derives `inheritedMembership` from the memberships of the general types reached
    /// through specialization, minus the ones a nearer member redefines. The closure itself is the
    /// specialization index this resolver already owns, so a member reached through two paths is
    /// inherited once and a cyclic hierarchy -- which has no closure -- inherits nothing.
    fn inherited_feature_members(&self, declaration: DeclarationId) -> Vec<DeclarationId> {
        let ancestors = self
            .types
            .specialization()
            .scoped_ancestors(declaration)
            .map(|(ancestor, _)| ancestor)
            .collect::<Vec<_>>();
        if ancestors.is_empty() {
            return Vec::new();
        }
        let mut candidates = Vec::new();
        for ancestor in ancestors {
            candidates.extend(self.owned_feature_members(ancestor));
        }
        let owned = self.owned_feature_members(declaration);
        let mut redefined = std::collections::BTreeSet::new();
        for member in owned.iter().chain(candidates.iter()) {
            self.collect_redefined_members(*member, &mut redefined);
        }
        candidates.retain(|candidate| !redefined.contains(candidate));
        candidates
    }

    /// Whether one member of a Type belongs to the selected derived collection.
    fn type_derived_fact_selects(
        &self,
        collection: TypeDerivedFactCollection,
        member: DeclarationId,
    ) -> bool {
        let facts = self.storage.declaration_facts(member);
        match collection {
            TypeDerivedFactCollection::OwnedFeatureMembership
            | TypeDerivedFactCollection::Multiplicity
            | TypeDerivedFactCollection::OwnedConjugator => false,
            TypeDerivedFactCollection::FeatureMembership
            | TypeDerivedFactCollection::Feature
            | TypeDerivedFactCollection::InheritedMembership
            | TypeDerivedFactCollection::InheritedFeature => true,
            TypeDerivedFactCollection::EndFeature => facts.is_some_and(|facts| facts.modifiers.end),
            TypeDerivedFactCollection::DirectedFeature => {
                facts.is_some_and(|facts| facts.direction.is_some())
            }
            TypeDerivedFactCollection::Input => facts.is_some_and(|facts| {
                matches!(
                    facts.direction,
                    Some(ParameterDirection::In | ParameterDirection::InOut)
                )
            }),
            TypeDerivedFactCollection::Output => facts.is_some_and(|facts| {
                matches!(
                    facts.direction,
                    Some(ParameterDirection::Out | ParameterDirection::InOut)
                )
            }),
        }
    }

    /// Returns one exact Type derivation over canonical membership and specialization-closure
    /// facts, or the explicit first missing prerequisite for the derivations whose normative
    /// result is a relationship or multiplicity identity that compact declaration-aligned storage
    /// does not own.
    pub(crate) fn type_derived_fact(
        &self,
        symbol: &SymbolIdentity,
        collection: TypeDerivedFactCollection,
    ) -> QueryOutcome<TypeDerivedFactOutcome> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let Some(rule) = type_derived_fact_rule(collection) else {
            return self.resolved_outcome(TypeDerivedFactOutcome::Unsupported {
                prerequisite: TypeDerivedFactPrerequisite::RuleNotPublished,
            });
        };
        if rule.metaclass != "Type"
            || self
                .storage
                .declaration(declaration)
                .is_none_or(|value| !DeclarationDomain::Type.accepts(value.kind))
        {
            return QueryOutcome::Unsupported;
        }
        let _rule_id = rule.rule_id;
        let unavailable = match collection {
            TypeDerivedFactCollection::OwnedFeatureMembership => {
                Some(TypeDerivedFactPrerequisite::FeatureMembershipIdentity)
            }
            TypeDerivedFactCollection::Multiplicity => {
                Some(TypeDerivedFactPrerequisite::MultiplicityIdentity)
            }
            TypeDerivedFactCollection::OwnedConjugator => {
                Some(TypeDerivedFactPrerequisite::ConjugationRelationshipIdentity)
            }
            _ => None,
        };
        if let Some(prerequisite) = unavailable {
            return self.resolved_outcome(TypeDerivedFactOutcome::Unsupported { prerequisite });
        }
        let inherited = self.inherited_feature_members(declaration);
        let members = match collection {
            TypeDerivedFactCollection::InheritedMembership
            | TypeDerivedFactCollection::InheritedFeature => inherited,
            _ => self
                .owned_feature_members(declaration)
                .into_iter()
                .chain(inherited)
                .collect(),
        };
        // `FeatureMembership`-valued collections still name their member element, never a
        // fabricated Membership relationship identity: that identity remains unpublished, and its
        // own derivation stays explicitly unsupported above.
        let membership_valued = matches!(
            collection,
            TypeDerivedFactCollection::InheritedMembership
                | TypeDerivedFactCollection::FeatureMembership
        );
        let values = self
            .symbols(
                members
                    .into_iter()
                    .filter(|member| self.type_derived_fact_selects(collection, *member)),
            )
            .into_vec()
            .into_iter()
            .map(|member| {
                if membership_valued {
                    TypeDerivedFactValue::FeatureMembership { member }
                } else {
                    TypeDerivedFactValue::Feature(member)
                }
            })
            .collect::<Vec<_>>();
        self.resolved_outcome(TypeDerivedFactOutcome::Values(values.into_boxed_slice()))
    }

    /// Returns one exact Systems::DefinitionAndUsage derived property from the canonical direct
    /// declaration owner, feature-membership, kind, and modifier facts.
    ///
    /// This intentionally stops at the first unavailable owner for the broader `feature`,
    /// `directedFeature`, VariantMembership, and time-variation predicates.  A direct child scan
    /// is never substituted for an inherited collection, and a VariantMembership relationship is
    /// never fabricated from an element role.
    pub(crate) fn definition_usage_derived(
        &self,
        symbol: &SymbolIdentity,
        kind: DefinitionUsageDerivedKind,
    ) -> QueryOutcome<DefinitionUsageDerivedOutcome> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let Some(rule) = definition_usage_derived_rule(kind) else {
            return self.resolved_outcome(DefinitionUsageDerivedOutcome::Unsupported {
                prerequisite: DefinitionUsageDerivedPrerequisite::RuleNotPublished,
            });
        };
        let Some(source) = self.storage.declaration(declaration) else {
            return QueryOutcome::Incomplete;
        };
        if !definition_usage_source_matches(rule.metaclass, source.kind) {
            return QueryOutcome::Unsupported;
        }
        let _rule_id = rule.rule_id;
        match kind {
            // `usage` and `directedUsage` select over the *effective* feature membership of the
            // definition or usage -- everything it owns plus everything it inherits -- so they
            // read the same canonical specialization closure `Type::inheritedMembership` does,
            // rather than the direct child scan the `owned`/`nested` collections below use.
            DefinitionUsageDerivedKind::DefinitionDirectedUsage
            | DefinitionUsageDerivedKind::UsageDirectedUsage
            | DefinitionUsageDerivedKind::DefinitionUsage
            | DefinitionUsageDerivedKind::UsageUsage => {
                let directed = matches!(
                    kind,
                    DefinitionUsageDerivedKind::DefinitionDirectedUsage
                        | DefinitionUsageDerivedKind::UsageDirectedUsage
                );
                let values = self.symbols(
                    self.owned_feature_members(declaration)
                        .into_iter()
                        .chain(self.inherited_feature_members(declaration))
                        .filter(|member| {
                            self.storage
                                .declaration(*member)
                                .is_some_and(|member| is_usage_declaration(member.kind))
                                && (!directed
                                    || self
                                        .storage
                                        .declaration_facts(*member)
                                        .is_some_and(|facts| facts.direction.is_some()))
                        }),
                );
                self.resolved_outcome(DefinitionUsageDerivedOutcome::Elements(values))
            }
            DefinitionUsageDerivedKind::DefinitionVariant
            | DefinitionUsageDerivedKind::DefinitionVariantMembership
            | DefinitionUsageDerivedKind::UsageVariant
            | DefinitionUsageDerivedKind::UsageVariantMembership => {
                self.resolved_outcome(DefinitionUsageDerivedOutcome::Unsupported {
                    prerequisite: DefinitionUsageDerivedPrerequisite::VariantMembershipIdentity,
                })
            }
            DefinitionUsageDerivedKind::UsageMayTimeVary => {
                self.resolved_outcome(DefinitionUsageDerivedOutcome::Unsupported {
                    prerequisite:
                        DefinitionUsageDerivedPrerequisite::EffectiveOccurrenceTimeVariationFacts,
                })
            }
            DefinitionUsageDerivedKind::UsageIsReference => {
                let is_composite = self
                    .storage
                    .declaration_facts(declaration)
                    .is_some_and(|facts| facts.modifiers.composite);
                self.resolved_outcome(DefinitionUsageDerivedOutcome::Boolean(!is_composite))
            }
            _ => {
                let values = self.symbols(self.storage.declarations.iter().enumerate().filter_map(
                    |(index, candidate)| {
                        let candidate_id = DeclarationId::from_index(index).ok()?;
                        (candidate_id != declaration
                            && candidate.owner == Some(declaration)
                            && self
                                .memberships
                                .get(candidate_id)
                                .is_some_and(|membership| {
                                    membership.kind == MembershipKind::Feature
                                })
                            && definition_usage_candidate_matches(kind, candidate.kind))
                        .then_some(candidate_id)
                    },
                ));
                self.resolved_outcome(DefinitionUsageDerivedOutcome::Elements(values))
            }
        }
    }

    /// Returns one exact Systems::Requirements projection from published feature-membership
    /// roles or documentation records. The generated row fixes both source metaclass and
    /// property identity, while this implementation reads only canonical lowered facts.
    pub(crate) fn requirement_derived_fact(
        &self,
        symbol: &SymbolIdentity,
        collection: RequirementDerivedFactCollection,
    ) -> QueryOutcome<RequirementDerivedFactOutcome> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let Some(rule) = requirement_derived_fact_rule(collection) else {
            return self.resolved_outcome(RequirementDerivedFactOutcome::Unsupported {
                prerequisite: RequirementDerivedFactPrerequisite::RuleNotPublished,
            });
        };
        let Some(source) = self.storage.declaration(declaration) else {
            return QueryOutcome::Incomplete;
        };
        if !requirement_derived_source_matches(rule.metaclass, source.kind) {
            return QueryOutcome::Unsupported;
        }
        let _rule_id = rule.rule_id;
        if collection.requires_text() {
            let values = self
                .documentation(declaration)
                .into_vec()
                .into_iter()
                .filter(|value| value.form == InspectionAnnotationForm::Documentation)
                .map(|value| value.text)
                .collect::<Vec<_>>()
                .into_boxed_slice();
            return self.resolved_outcome(RequirementDerivedFactOutcome::Text(values));
        }
        let Some(role) = requirement_derived_membership_role(collection) else {
            return self.resolved_outcome(RequirementDerivedFactOutcome::Unsupported {
                prerequisite: RequirementDerivedFactPrerequisite::CanonicalMembershipRole,
            });
        };
        let values = self.symbols(self.storage.declarations.iter().enumerate().filter_map(
            |(index, candidate)| {
                let candidate_id = DeclarationId::from_index(index).ok()?;
                (candidate.owner == Some(declaration)
                    && self
                        .memberships
                        .get(candidate_id)
                        .is_some_and(|membership| membership.kind == MembershipKind::Feature)
                    && element_kind::membership_role(candidate.kind) == Some(role))
                .then_some(candidate_id)
            },
        ));
        self.resolved_outcome(RequirementDerivedFactOutcome::Elements(values))
    }

    /// The exact Actions derivation boundary.  The current model preserves selected action forms
    /// and references but not the normative ordered argument/input-parameter or inherited-usage
    /// identities, so this returns the first unavailable canonical fact rather than guessing.
    pub(crate) fn action_derived_fact(
        &self,
        symbol: &SymbolIdentity,
        collection: ActionDerivedFactCollection,
    ) -> QueryOutcome<ActionDerivedFactOutcome> {
        let declaration = match self.single_declaration(symbol) {
            Ok(value) => value,
            Err(outcome) => return outcome,
        };
        let Some(rule) = action_derived_fact_rule(collection) else {
            return self.resolved_outcome(ActionDerivedFactOutcome::Unsupported {
                prerequisite: ActionDerivedFactPrerequisite::RuleNotPublished,
            });
        };
        let _rule_id = rule.rule_id;
        let prerequisite = match collection {
            ActionDerivedFactCollection::ActionDefinitionAction => {
                ActionDerivedFactPrerequisite::EffectiveUsageClosure
            }
            ActionDerivedFactCollection::AssignmentReferent => {
                ActionDerivedFactPrerequisite::OwnedMembershipIdentity
            }
            ActionDerivedFactCollection::ForLoopVariable => {
                ActionDerivedFactPrerequisite::OrderedOwnedFeatureIdentity
            }
            ActionDerivedFactCollection::LoopBodyAction
            | ActionDerivedFactCollection::AcceptPayloadParameter
            | ActionDerivedFactCollection::WhileArgument
            | ActionDerivedFactCollection::UntilArgument
            | ActionDerivedFactCollection::IfThenAction
            | ActionDerivedFactCollection::IfElseAction
            | ActionDerivedFactCollection::IfArgument => {
                ActionDerivedFactPrerequisite::OrderedInputParameterIdentity
            }
            ActionDerivedFactCollection::TerminateOccurrenceArgument
            | ActionDerivedFactCollection::SendSenderArgument
            | ActionDerivedFactCollection::SendReceiverArgument
            | ActionDerivedFactCollection::SendPayloadArgument => {
                ActionDerivedFactPrerequisite::ActionMetaclassIdentity
            }
            _ => ActionDerivedFactPrerequisite::OrderedActionArgumentIdentity,
        };
        let _source_kind = self
            .storage
            .declaration(declaration)
            .map(|value| value.kind);
        self.resolved_outcome(ActionDerivedFactOutcome::Unsupported { prerequisite })
    }

    /// Decides the exact FeatureMembership TypeFeaturing implication from the canonical
    /// membership and effective TypeFeaturing facts. It deliberately does not inspect source
    /// spelling or reconstruct `isFeaturingType` downstream.
    pub(crate) fn type_featuring_check(
        &self,
        symbol: &SymbolIdentity,
        kind: TypeFeaturingCheckKind,
    ) -> QueryOutcome<TypeFeaturingCheckOutcome> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let Some(rule) = type_featuring_check_rule(kind) else {
            return self.resolved_outcome(TypeFeaturingCheckOutcome::Unsupported {
                prerequisite: TypeFeaturingCheckPrerequisite::RuleNotPublished,
            });
        };
        if rule.metaclass != "Feature" {
            return self.resolved_outcome(TypeFeaturingCheckOutcome::Unsupported {
                prerequisite: TypeFeaturingCheckPrerequisite::RuleNotPublished,
            });
        }
        let _normative_rule = rule.rule_id;
        if self.types.featuring_requires_snapshots(declaration) {
            return self.resolved_outcome(TypeFeaturingCheckOutcome::Unsupported {
                prerequisite: TypeFeaturingCheckPrerequisite::VariableFeatureSnapshots,
            });
        }
        if !self
            .memberships
            .get(declaration)
            .is_some_and(|membership| membership.kind == MembershipKind::Feature)
        {
            return self.resolved_outcome(TypeFeaturingCheckOutcome::Unsupported {
                prerequisite: TypeFeaturingCheckPrerequisite::FeatureMembershipFacts,
            });
        }
        let outcome = if self.types.featuring_types(declaration).is_empty() {
            TypeFeaturingCheckOutcome::Violated
        } else {
            TypeFeaturingCheckOutcome::Satisfied
        };
        self.resolved_outcome(outcome)
    }

    /// Returns the first missing canonical prerequisite for one exact redefinition check.
    ///
    /// Authored and implied redefinition edges are already settled in this publication. None of
    /// these predicates is reducible to merely having an edge: each selects a particular endpoint
    /// through a metamodel role (for example an end position, state subaction kind, or constructor
    /// result). Those role facts are not yet published as canonical query inputs, so this method
    /// deliberately does not walk source syntax, inspect names, or turn an arbitrary redefinition
    /// into a satisfied result.
    pub(crate) fn redefinition_check(
        &self,
        kind: RedefinitionCheckKind,
    ) -> QueryOutcome<RedefinitionCheckOutcome> {
        let Some(rule) = redefinition_check_rule(kind) else {
            return self.resolved_outcome(RedefinitionCheckOutcome::Unsupported {
                prerequisite: RedefinitionCheckPrerequisite::RuleNotPublished,
            });
        };
        let _normative_rule = (rule.rule_id, rule.metaclass);
        let prerequisite = match kind {
            RedefinitionCheckKind::FeatureEnd => {
                RedefinitionCheckPrerequisite::EndFeaturePositionAndInheritedEnds
            }
            RedefinitionCheckKind::FeatureFlowFeature => {
                RedefinitionCheckPrerequisite::FlowEndOrdinalAndLibraryAnchors
            }
            RedefinitionCheckKind::FeatureOwnedCrossFeatureSpecialization => {
                RedefinitionCheckPrerequisite::CrossFeatureAndSubsettingEndpoints
            }
            RedefinitionCheckKind::FeatureParameter => {
                RedefinitionCheckPrerequisite::ParameterDirectionAndInheritedPosition
            }
            RedefinitionCheckKind::FeatureResult => {
                RedefinitionCheckPrerequisite::FunctionOrExpressionResult
            }
            RedefinitionCheckKind::ConstructorExpressionResultFeature => {
                RedefinitionCheckPrerequisite::ConstructorResultAndInstantiatedTypeFeatures
            }
            RedefinitionCheckKind::FeatureChainExpressionSourceTarget => {
                RedefinitionCheckPrerequisite::FeatureChainSourceTarget
            }
            RedefinitionCheckKind::FeatureChainExpressionTarget => {
                RedefinitionCheckPrerequisite::FeatureChainSourceTargetAndLibraryAnchor
            }
            RedefinitionCheckKind::ActionUsageStateAction => {
                RedefinitionCheckPrerequisite::StateSubactionMembershipAndKind
            }
            RedefinitionCheckKind::AssignmentActionUsageAccessedFeature
            | RedefinitionCheckKind::AssignmentActionUsageReferent
            | RedefinitionCheckKind::AssignmentActionUsageStartingAt => {
                RedefinitionCheckPrerequisite::AssignmentActionInputParameterEndpoints
            }
            RedefinitionCheckKind::ForLoopActionUsageVar => {
                RedefinitionCheckPrerequisite::ForLoopVariableProjection
            }
            RedefinitionCheckKind::RequirementUsageObjective => {
                RedefinitionCheckPrerequisite::ObjectiveMembershipAndCaseObjective
            }
            RedefinitionCheckKind::RenderingUsage => {
                RedefinitionCheckPrerequisite::ViewRenderingMembership
            }
        };
        self.resolved_outcome(RedefinitionCheckOutcome::Unsupported { prerequisite })
    }

    /// Returns the first unpublished canonical input for one exact specialization predicate.
    ///
    /// The resolver owns generic authored and implied specialization facts, but no predicate in
    /// this group can be discharged just because such an edge exists: each selects an endpoint,
    /// applicability role, or library anchor that remains a separate fact boundary.
    pub(crate) fn specialization_check(
        &self,
        kind: SpecializationCheckKind,
    ) -> QueryOutcome<SpecializationCheckOutcome> {
        let Some(rule) = specialization_check_rule(kind) else {
            return self.resolved_outcome(SpecializationCheckOutcome::Unsupported {
                prerequisite: SpecializationCheckPrerequisite::RuleNotPublished,
            });
        };
        let _normative_rule = (rule.rule_id, rule.metaclass);
        let prerequisite = match kind {
            SpecializationCheckKind::FeatureCrossing => {
                SpecializationCheckPrerequisite::CrossFeatureProjection
            }
            SpecializationCheckKind::FeatureObject | SpecializationCheckKind::FeatureOccurrence => {
                SpecializationCheckPrerequisite::FeatureTypingMetaclassAndLibraryAnchor
            }
            SpecializationCheckKind::FeatureOwnedCrossFeature => {
                SpecializationCheckPrerequisite::OwnedCrossFeatureOwnerTypes
            }
            SpecializationCheckKind::FeaturePortion
            | SpecializationCheckKind::FeatureSubobject
            | SpecializationCheckKind::FeatureSuboccurrence => {
                SpecializationCheckPrerequisite::FeatureModifiersOwnerTypingAndLibraryAnchor
            }
            SpecializationCheckKind::FeatureValuation => {
                SpecializationCheckPrerequisite::FeatureValueEvaluationResults
            }
            SpecializationCheckKind::MetadataFeatureSemantic => {
                SpecializationCheckPrerequisite::SemanticMetadataProjection
            }
            SpecializationCheckKind::ConnectorBinaryObject
            | SpecializationCheckKind::ConnectorObject => {
                SpecializationCheckPrerequisite::ConnectorAssociationProjectionAndLibraryAnchor
            }
            SpecializationCheckKind::StepOwnedPerformance
            | SpecializationCheckKind::StepSubperformance => {
                SpecializationCheckPrerequisite::StepOwnershipTypingAndLibraryAnchor
            }
            SpecializationCheckKind::SelectExpressionResult
            | SpecializationCheckKind::IndexExpressionResult => {
                SpecializationCheckPrerequisite::ExpressionArgumentResult
            }
            SpecializationCheckKind::ConstructorExpressionResult => {
                SpecializationCheckPrerequisite::ExpressionResultAndInstantiatedType
            }
            SpecializationCheckKind::ConstructorExpression => {
                SpecializationCheckPrerequisite::LibraryAnchorAndImpliedSpecialization
            }
            SpecializationCheckKind::FeatureChainExpressionResult => {
                SpecializationCheckPrerequisite::FeatureChainSourceTargetAndSubsetting
            }
            SpecializationCheckKind::FeatureReferenceExpressionResult => {
                SpecializationCheckPrerequisite::FeatureReferenceReferentAndResult
            }
            SpecializationCheckKind::InvocationExpressionBehaviorResult => {
                SpecializationCheckPrerequisite::InvocationInstantiatedTypeAndResult
            }
            SpecializationCheckKind::InvocationExpression => {
                SpecializationCheckPrerequisite::InvocationInstantiatedType
            }
            SpecializationCheckKind::MergeNodeIncomingSuccession
            | SpecializationCheckKind::DecisionNodeOutgoingSuccession => {
                SpecializationCheckPrerequisite::SuccessionEndpointAndSubsetting
            }
            SpecializationCheckKind::StateUsageExclusiveState
            | SpecializationCheckKind::StateUsageSubstate => {
                SpecializationCheckPrerequisite::StateSubactionKindAndLibraryAnchor
            }
            SpecializationCheckKind::TransitionUsageAction
            | SpecializationCheckKind::TransitionUsageState => {
                SpecializationCheckPrerequisite::TransitionOwnerSourceAndLibraryAnchor
            }
            SpecializationCheckKind::TransitionUsagePayload => {
                SpecializationCheckPrerequisite::TransitionTriggerPayloadEndpoints
            }
            SpecializationCheckKind::TransitionUsageSuccessionSource => {
                SpecializationCheckPrerequisite::TransitionSuccessionSource
            }
            SpecializationCheckKind::TransitionUsageTransitionFeature => {
                SpecializationCheckPrerequisite::TransitionFeatureRolesAndLibraryAnchors
            }
            SpecializationCheckKind::IncludeUseCase => {
                SpecializationCheckPrerequisite::UseCaseOwnerAndLibraryAnchor
            }
            SpecializationCheckKind::UsageVariationDefinition
            | SpecializationCheckKind::UsageVariationUsage => {
                SpecializationCheckPrerequisite::UsageVariationOwner
            }
            SpecializationCheckKind::OccurrenceDefinitionMultiplicity => {
                SpecializationCheckPrerequisite::IndividualMultiplicityAndLibraryAnchor
            }
            SpecializationCheckKind::OccurrenceUsageSuboccurrence => {
                SpecializationCheckPrerequisite::OccurrenceOwnerTypingAndLibraryAnchor
            }
        };
        self.resolved_outcome(SpecializationCheckOutcome::Unsupported { prerequisite })
    }

    /// Projects the exact `deriveElementOwner` result from the canonical declaration ownership
    /// fact. The query neither reads source text nor follows rendered qualified names.
    pub(crate) fn derived_element_owner(
        &self,
        symbol: &SymbolIdentity,
    ) -> QueryOutcome<DerivedElementOwner> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let Some(rule) = element_derived_owner_rule() else {
            return QueryOutcome::Unsupported;
        };
        if rule.metaclass != "Element" || rule.kind != ElementDerivedOwnerKind::Owner {
            return QueryOutcome::Unsupported;
        }
        let _rule_id = rule.rule_id;
        let value = self
            .storage
            .declaration(declaration)
            .and_then(|declaration| declaration.owner)
            .and_then(|owner| self.symbol_identity(owner))
            .map_or(DerivedElementOwner::NoOwner, DerivedElementOwner::Owner);
        self.resolved_outcome(value)
    }

    /// Projects one exact Root Element documentation-form derivation from canonical
    /// documentation records. It does not inspect source syntax or recreate ownership paths.
    pub(crate) fn element_derived_documentation(
        &self,
        symbol: &SymbolIdentity,
        collection: ElementDerivedDocumentationCollection,
    ) -> QueryOutcome<Box<[Documentation]>> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let Some(rule) = element_derived_documentation_rule(collection) else {
            return QueryOutcome::Unsupported;
        };
        if rule.metaclass != "Element" {
            return QueryOutcome::Unsupported;
        }
        let _rule_id = rule.rule_id;
        let form = match collection {
            ElementDerivedDocumentationCollection::Documentation => {
                InspectionAnnotationForm::Documentation
            }
            ElementDerivedDocumentationCollection::TextualRepresentation => {
                InspectionAnnotationForm::TextualRepresentation
            }
        };
        let values = self
            .documentation(declaration)
            .into_vec()
            .into_iter()
            .filter(|documentation| documentation.form == form)
            .collect::<Vec<_>>()
            .into_boxed_slice();
        self.resolved_outcome(values)
    }

    /// Projects one exact Namespace element-valued derivation from settled declaration ownership
    /// and membership facts. It neither traverses rendered names nor turns compact membership
    /// records into a second relationship store.
    pub(crate) fn namespace_derived_elements(
        &self,
        symbol: &SymbolIdentity,
        collection: NamespaceDerivedElementCollection,
    ) -> QueryOutcome<Box<[SymbolIdentity]>> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let Some(rule) = namespace_derived_element_rule(collection) else {
            return QueryOutcome::Unsupported;
        };
        if rule.metaclass != "Namespace"
            || self
                .storage
                .declaration(declaration)
                .is_none_or(|value| !DeclarationDomain::Namespace.accepts(value.kind))
        {
            return QueryOutcome::Unsupported;
        }
        let _rule_id = rule.rule_id;
        let values = self.symbols(
            self.storage
                .declarations
                .iter()
                .enumerate()
                .filter_map(|(index, candidate_declaration)| {
                    let candidate = DeclarationId::from_index(index).ok()?;
                    (candidate != declaration && candidate_declaration.owner == Some(declaration))
                        .then_some(candidate)
                })
                .filter(|candidate| match collection {
                    NamespaceDerivedElementCollection::OwnedMember => self
                        .memberships
                        .get(*candidate)
                        .is_some_and(|membership| membership.kind == MembershipKind::Owning),
                    NamespaceDerivedElementCollection::OwnedImport => self
                        .storage
                        .declaration(*candidate)
                        .is_some_and(|candidate| candidate.kind == DeclarationKind::Import),
                }),
        );
        self.resolved_outcome(values)
    }

    /// Projects `deriveNamespaceImportImportedElement` for every direct authored NamespaceImport
    /// owned by a Namespace. The concrete grammar gives an import no authorable name, so the
    /// owner-scoped result carries its canonical identity rather than asking callers to infer one
    /// from rendered source. Each value retains the same typed target outcome as inspection.
    pub(crate) fn namespace_import_derived_elements(
        &self,
        symbol: &SymbolIdentity,
    ) -> QueryOutcome<Box<[NamespaceImportDerivedElement]>> {
        let namespace = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let Some(rule) = namespace_import_derived_element_rule() else {
            return QueryOutcome::Unsupported;
        };
        if rule.metaclass != "NamespaceImport"
            || rule.kind != NamespaceImportDerivedElementKind::ImportedElement
            || self
                .storage
                .declaration(namespace)
                .is_none_or(|value| !DeclarationDomain::Namespace.accepts(value.kind))
        {
            return QueryOutcome::Unsupported;
        }
        let _rule_id = rule.rule_id;
        let mut values = Vec::new();
        for (index, declaration) in self.storage.declarations.iter().enumerate() {
            if declaration.owner != Some(namespace) || declaration.kind != DeclarationKind::Import {
                continue;
            }
            let import = match DeclarationId::from_index(index) {
                Ok(import) => import,
                Err(_) => return QueryOutcome::Unsupported,
            };
            let relationships =
                self.relationships_of_kinds(import, &[ReferenceKind::NamespaceImport]);
            let relationship = match relationships.as_ref() {
                [] => continue,
                [relationship] => relationship,
                _ => return QueryOutcome::Unsupported,
            };
            let Some(import) = self.symbol_identity(import) else {
                return QueryOutcome::Unsupported;
            };
            values.push(NamespaceImportDerivedElement {
                import,
                relationship: relationship.clone(),
            });
        }
        values.sort_by(|left, right| left.import.cmp(&right.import));
        self.resolved_outcome(values.into_boxed_slice())
    }

    pub(crate) fn requirement_usage_typing(
        &self,
        symbol: &SymbolIdentity,
    ) -> QueryOutcome<RequirementUsageTyping> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        if self
            .storage
            .declaration(declaration)
            .is_none_or(|value| value.kind != DeclarationKind::RequirementUsage)
        {
            return QueryOutcome::Unsupported;
        }
        let relationships = self.relationships(declaration);
        let mut typings = relationships
            .iter()
            .filter(|relationship| relationship.kind == "featureTyping");
        let value = match (typings.next(), typings.next()) {
            (None, _) => RequirementUsageTyping::Missing,
            (Some(_), Some(_)) => {
                let candidates = self
                    .types
                    .direct_types(declaration)
                    .iter()
                    .filter_map(|(target, _)| self.symbol_identity(*target))
                    .collect::<Vec<_>>();
                RequirementUsageTyping::Ambiguous(candidates.into_boxed_slice())
            }
            (Some(relationship), None) => match &relationship.target {
                RelationshipTarget::Resolved(target) => {
                    let target_is_requirement_definition = self
                        .identity_declarations(target)
                        .into_iter()
                        .any(|target| {
                            self.storage.declaration(target).is_some_and(|declaration| {
                                declaration.kind == DeclarationKind::RequirementDefinition
                            })
                        });
                    if target_is_requirement_definition {
                        RequirementUsageTyping::Resolved(TypeReference {
                            symbol: target.clone(),
                            provenance: relationship.provenance,
                        })
                    } else {
                        RequirementUsageTyping::Unsupported
                    }
                }
                RelationshipTarget::Ambiguous(values) => {
                    RequirementUsageTyping::Ambiguous(values.clone())
                }
                RelationshipTarget::Unresolved => RequirementUsageTyping::Unresolved,
                RelationshipTarget::Unsupported => RequirementUsageTyping::Unsupported,
            },
        };
        self.resolved_outcome(value)
    }

    pub(crate) fn satisfy_relationships(&self) -> QueryOutcome<Box<[SatisfyRelationship]>> {
        let endpoint = |reference: Option<AuthoredReferenceId>| match reference
            .as_ref()
            .and_then(|reference| self.resolution.outcome(*reference))
        {
            Some(ResolutionStatus::Resolved(target)) => self
                .symbol_identity(target)
                .map(SatisfyEndpoint::Resolved)
                .unwrap_or(SatisfyEndpoint::Unresolved),
            Some(ResolutionStatus::Ambiguous(candidates)) => SatisfyEndpoint::Ambiguous(
                self.resolution
                    .ambiguous_candidates(candidates)
                    .iter()
                    .filter_map(|candidate| self.symbol_identity(*candidate))
                    .collect(),
            ),
            Some(ResolutionStatus::Unsupported) => SatisfyEndpoint::Unsupported,
            Some(ResolutionStatus::Unresolved) | Some(ResolutionStatus::NonConverged) => {
                SatisfyEndpoint::Unresolved
            }
            None => SatisfyEndpoint::Unsupported,
        };
        let mut values = self
            .storage
            .declarations
            .iter()
            .enumerate()
            .filter_map(|(index, declaration)| {
                let document = self.storage.document(declaration.document)?;
                if document.role != SourceRole::Workspace
                    || declaration.kind != DeclarationKind::Satisfy
                {
                    return None;
                }
                let id = DeclarationId::from_index(index).ok()?;
                let requirement = self
                    .storage
                    .references
                    .iter()
                    .enumerate()
                    .find(|(_, value)| {
                        value.source == id && value.kind == ReferenceKind::SatisfySource
                    })
                    .and_then(|(index, _)| AuthoredReferenceId::from_index(index).ok());
                let satisfying = self
                    .storage
                    .references
                    .iter()
                    .enumerate()
                    .find(|(_, value)| {
                        value.source == id && value.kind == ReferenceKind::SatisfyTarget
                    })
                    .and_then(|(index, _)| AuthoredReferenceId::from_index(index).ok());
                let facts = self.storage.declaration_facts(id)?;
                Some(SatisfyRelationship {
                    identity: self.symbol_identity(id)?,
                    requirement: endpoint(requirement),
                    satisfying_element: endpoint(satisfying),
                    polarity: if facts.negated.unwrap_or(false) {
                        SatisfyPolarity::NotSatisfied
                    } else {
                        SatisfyPolarity::Satisfied
                    },
                    provenance: RelationshipProvenance::Authored,
                    location: self.source_location(id)?,
                })
            })
            .collect::<Vec<_>>();
        values.sort_by(|left, right| {
            left.location
                .document
                .cmp(&right.location.document)
                .then_with(|| left.location.range.cmp(&right.location.range))
                .then_with(|| left.identity.cmp(&right.identity))
        });
        self.resolved_outcome(values.into_boxed_slice())
    }

    /// The publication-owned paired endpoints of every workspace-authored binding connector.
    ///
    /// `BindingConnectorIndex` is assembled at the barrier from the two directional reference
    /// slots. This projection only converts its settled declaration identities into public ones;
    /// it never searches references, so it cannot pair ends from distinct authored statements.
    pub(crate) fn binding_connectors(&self) -> QueryOutcome<Box<[BindingConnector]>> {
        let endpoint = |endpoint: &binding::BindingEndpointFact| match endpoint {
            binding::BindingEndpointFact::Resolved(target) => self
                .symbol_identity(*target)
                .map(BindingEndpoint::Resolved)
                .unwrap_or(BindingEndpoint::Unresolved),
            binding::BindingEndpointFact::Ambiguous(candidates) => BindingEndpoint::Ambiguous(
                candidates
                    .iter()
                    .filter_map(|candidate| self.symbol_identity(*candidate))
                    .collect(),
            ),
            binding::BindingEndpointFact::Unresolved => BindingEndpoint::Unresolved,
            binding::BindingEndpointFact::Unsupported => BindingEndpoint::Unsupported,
        };
        let mut values = self
            .bindings
            .facts()
            .iter()
            .filter_map(|fact| {
                let declaration = self.storage.declaration(fact.connector)?;
                let document = self.storage.document(declaration.document)?;
                if document.role != SourceRole::Workspace {
                    return None;
                }
                Some(BindingConnector {
                    identity: self.symbol_identity(fact.connector)?,
                    source: endpoint(&fact.source),
                    target: endpoint(&fact.target),
                    provenance: match fact.provenance {
                        types::FactProvenance::Authored => RelationshipProvenance::Authored,
                        types::FactProvenance::Implied => RelationshipProvenance::Implied,
                    },
                    location: self.source_location(fact.connector)?,
                })
            })
            .collect::<Vec<_>>();
        values.sort_by(|left, right| {
            left.location
                .document
                .cmp(&right.location.document)
                .then_with(|| left.location.range.cmp(&right.location.range))
                .then_with(|| left.identity.cmp(&right.identity))
        });
        self.resolved_outcome(values.into_boxed_slice())
    }

    /// The explicit applicability state of one closed binding-connector validation rule.
    ///
    /// The rule reads the binding index, but its FeatureReferenceExpression target/result inputs
    /// are not yet canonical facts. Returning a typed unsupported result preserves that boundary
    /// instead of guessing an endpoint from a name or source expression.
    pub(crate) fn binding_connector_validation(
        &self,
        rule: BindingConnectorCheckKind,
    ) -> QueryOutcome<BindingConnectorValidationOutcome> {
        let Some(contract) = binding_connector_check_rule(rule) else {
            return self.resolved_outcome(BindingConnectorValidationOutcome::Unsupported {
                prerequisite: crate::BindingConnectorValidationPrerequisite::RuleNotPublished,
            });
        };
        // Both fields are manifest-owned contract data. Touch them here so a generated table
        // cannot quietly become a kind-only lookalike while the query retains no rule-ID map.
        let _normative_rule = (contract.rule_id, contract.metaclass);
        self.resolved_outcome(self.bindings.validation(rule))
    }

    pub(crate) fn requirement_verifications(&self) -> QueryOutcome<Box<[RequirementVerification]>> {
        let endpoint = |reference: Option<AuthoredReferenceId>| match reference
            .and_then(|reference| self.resolution.outcome(reference))
        {
            Some(ResolutionStatus::Resolved(target)) => self
                .symbol_identity(target)
                .map(VerificationRequirement::Resolved)
                .unwrap_or(VerificationRequirement::Unresolved),
            Some(ResolutionStatus::Ambiguous(candidates)) => VerificationRequirement::Ambiguous(
                self.resolution
                    .ambiguous_candidates(candidates)
                    .iter()
                    .filter_map(|candidate| self.symbol_identity(*candidate))
                    .collect(),
            ),
            Some(ResolutionStatus::Unsupported) => VerificationRequirement::Unsupported,
            Some(ResolutionStatus::Unresolved) | Some(ResolutionStatus::NonConverged) => {
                VerificationRequirement::Unresolved
            }
            None => VerificationRequirement::Unsupported,
        };
        let mut values = Vec::new();
        for (index, declaration) in self.storage.declarations.iter().enumerate() {
            let Some(document) = self.storage.document(declaration.document) else {
                continue;
            };
            if document.role != SourceRole::Workspace
                || declaration.kind != DeclarationKind::VerifyRequirement
            {
                continue;
            }
            let Some(id) = DeclarationId::from_index(index).ok() else {
                continue;
            };
            let Some(objective) = declaration
                .owner
                .and_then(|id| self.storage.declaration(id))
            else {
                continue;
            };
            if objective.kind != DeclarationKind::RequirementUsage {
                continue;
            }
            let Some(case_id) = objective.owner else {
                continue;
            };
            let Some(case) = self.storage.declaration(case_id) else {
                continue;
            };
            if !matches!(
                case.kind,
                DeclarationKind::VerificationCaseDefinition
                    | DeclarationKind::VerificationCaseUsage
            ) {
                continue;
            }
            let reference = self
                .storage
                .references
                .iter()
                .enumerate()
                .find(|(_, reference)| {
                    reference.source == id
                        && reference.kind == ReferenceKind::VerifyRequirementTarget
                })
                .and_then(|(index, _)| AuthoredReferenceId::from_index(index).ok());
            let (Some(identity), Some(verification_case), Some(location)) = (
                self.symbol_identity(id),
                self.symbol_identity(case_id),
                self.source_location(id),
            ) else {
                continue;
            };
            values.push(RequirementVerification {
                identity,
                verification_case,
                requirement: endpoint(reference),
                provenance: RelationshipProvenance::Authored,
                location,
                outcome: VerificationOutcome::Unsupported,
            });
        }
        values.sort_by(|left, right| {
            left.location
                .document
                .cmp(&right.location.document)
                .then_with(|| left.location.range.cmp(&right.location.range))
                .then_with(|| left.identity.cmp(&right.identity))
        });
        self.resolved_outcome(values.into_boxed_slice())
    }

    pub(crate) fn effective_types(
        &self,
        symbol: &SymbolIdentity,
    ) -> QueryOutcome<Box<[EffectiveType]>> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let mut types = self
            .types
            .effective_types(declaration)
            .iter()
            .filter_map(|(target, source)| {
                Some(EffectiveType {
                    symbol: self.symbol_identity(*target)?,
                    origin: match source {
                        types::EffectiveTypeSource::Direct => EffectiveTypeOrigin::Direct,
                        types::EffectiveTypeSource::Inherited(from) => {
                            EffectiveTypeOrigin::Inherited(self.symbol_identity(*from)?)
                        }
                    },
                })
            })
            .collect::<Vec<_>>();
        types.sort_by(|left, right| left.symbol.cmp(&right.symbol));
        self.resolved_outcome(types.into_boxed_slice())
    }

    /// The settled standard-library anchor used by `checkPartDefinitionSpecialization`.
    ///
    /// A missing anchor remains `Unresolved`; multiple standard-library candidates remain
    /// `Ambiguous` with every canonical identity. Callers therefore never need to recover the
    /// anchor from a rendered name or substitute a workspace declaration.
    pub(crate) fn part_definition_specialization_anchor(&self) -> QueryOutcome<SymbolIdentity> {
        self.library_specialization_anchor("sysml-2.0:8.3.11.2:checkPartDefinitionSpecialization")
    }

    pub(crate) fn library_specialization_anchor(
        &self,
        rule_id: &str,
    ) -> QueryOutcome<SymbolIdentity> {
        self.library_rule_anchor(rule_id)
    }

    /// The canonical anchor outcome for one explicitly selected branch of a generated
    /// specialization rule. `Default` preserves the legacy single-anchor projection.
    pub(crate) fn library_specialization_anchor_branch(
        &self,
        rule_id: &str,
        branch: LibrarySpecializationAnchorBranch,
    ) -> QueryOutcome<SymbolIdentity> {
        self.library_anchor_outcome(
            self.resolution
                .library_specialization_anchors
                .outcome_for(rule_id, branch),
        )
    }

    /// The canonical standard-library anchor outcome for any generated exact library rule.
    ///
    /// The stable manifest rule ID selects the fact; this intentionally does not infer a rule from
    /// a metaclass, display name, or anchor text.
    pub(crate) fn library_rule_anchor(&self, rule_id: &str) -> QueryOutcome<SymbolIdentity> {
        self.library_anchor_outcome(self.resolution.library_specialization_anchor(rule_id))
    }

    fn library_anchor_outcome(
        &self,
        outcome: Option<&LibrarySpecializationAnchor>,
    ) -> QueryOutcome<SymbolIdentity> {
        match outcome {
            Some(LibrarySpecializationAnchor::Resolved(anchor)) => self
                .symbol_identity(*anchor)
                .map_or(QueryOutcome::Unresolved, |anchor| {
                    self.resolved_outcome(anchor)
                }),
            Some(LibrarySpecializationAnchor::Ambiguous(candidates)) => QueryOutcome::Ambiguous(
                candidates
                    .iter()
                    .filter_map(|candidate| self.symbol_identity(*candidate))
                    .collect(),
            ),
            Some(LibrarySpecializationAnchor::Missing) | None => QueryOutcome::Unresolved,
        }
    }

    /// Whether an exact unconditional `redefinesFromLibrary` contract can be applied to a
    /// lowered declaration in this publication.
    ///
    /// This is deliberately separate from [`Self::library_rule_anchor`]: the latter reports the
    /// normative library fact even when the parser has not yet lowered the rule's source
    /// metaclass. Returning `Unsupported` here makes that boundary observable instead of silently
    /// treating an unavailable source projection as an absent implied edge.
    pub(crate) fn library_redefinition_applicability(&self, rule_id: &str) -> QueryOutcome<()> {
        let Some(rule) = GENERATED_LIBRARY_REDEFINITION_RULES
            .iter()
            .find(|rule| rule.rule_id == rule_id)
        else {
            return QueryOutcome::Unresolved;
        };
        if lowered_redefinition_source_kind(rule.metaclass).is_some() {
            self.resolved_outcome(())
        } else {
            QueryOutcome::Unsupported
        }
    }

    pub(crate) fn direct_supertypes(
        &self,
        symbol: &SymbolIdentity,
        scope: SpecializationScope,
    ) -> QueryOutcome<Box<[SymbolIdentity]>> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let bit = internal_scope(scope);
        let symbols = self.symbols(
            self.types
                .supertypes(declaration)
                .iter()
                .filter(|(_, scopes)| types::scopes_of(*scopes).any(|candidate| candidate == bit))
                .map(|(target, _)| *target),
        );
        self.resolved_outcome(symbols)
    }

    /// Every supertype of `symbol`, including `symbol` itself.
    ///
    /// Reflexive, matching the Pilot's `allSupertypes() = OrderedSet{self}->closure(supertypes)`.
    /// A caller that wants the strict set removes itself; a caller that expected reflexivity and
    /// did not get it would silently answer "does not conform" for a type against itself.
    pub(crate) fn all_supertypes(
        &self,
        symbol: &SymbolIdentity,
        scope: SpecializationScope,
    ) -> QueryOutcome<Box<[SymbolIdentity]>> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let bit = internal_scope(scope);
        let symbols = self.symbols(
            std::iter::once(declaration).chain(
                self.types
                    .specialization()
                    .scoped_ancestors(declaration)
                    .filter(move |(_, scopes)| scopes.contains(&bit))
                    .map(|(ancestor, _)| ancestor),
            ),
        );
        self.resolved_outcome(symbols)
    }

    pub(crate) fn direct_subtypes(
        &self,
        symbol: &SymbolIdentity,
        scope: SpecializationScope,
    ) -> QueryOutcome<Box<[SymbolIdentity]>> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let bit = internal_scope(scope);
        let symbols = self.symbols(
            self.types
                .subtypes(declaration)
                .iter()
                .filter(|(_, scopes)| types::scopes_of(*scopes).any(|candidate| candidate == bit))
                .map(|(source, _)| *source),
        );
        self.resolved_outcome(symbols)
    }

    pub(crate) fn featuring_type(
        &self,
        symbol: &SymbolIdentity,
    ) -> QueryOutcome<Option<SymbolIdentity>> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        if self.types.featuring_requires_snapshots(declaration) {
            return QueryOutcome::Unsupported;
        }
        let featuring = self
            .types
            .featuring_types(declaration)
            .iter()
            .filter_map(|(owner, _)| self.symbol_identity(*owner))
            .collect::<Vec<_>>();
        match featuring.as_slice() {
            [] => self.resolved_outcome(None),
            [owner] => self.resolved_outcome(Some(owner.clone())),
            _ => QueryOutcome::Ambiguous(featuring.into_iter().map(Some).collect()),
        }
    }

    /// Every effective featuring type produced by the canonical TypeFeaturing/FeatureChaining
    /// fact family, retaining authored versus implied provenance.
    pub(crate) fn featuring_types(
        &self,
        symbol: &SymbolIdentity,
    ) -> QueryOutcome<Box<[TypeReference]>> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let values = self
            .types
            .featuring_types(declaration)
            .iter()
            .filter_map(|(target, provenance)| {
                self.symbol_identity(*target).map(|symbol| TypeReference {
                    symbol,
                    provenance: match provenance {
                        types::FactProvenance::Authored => RelationshipProvenance::Authored,
                        types::FactProvenance::Implied => RelationshipProvenance::Implied,
                    },
                })
            })
            .collect::<Vec<_>>();
        let values = values.into_boxed_slice();
        if self.types.featuring_requires_snapshots(declaration) {
            if values.is_empty() {
                QueryOutcome::Unsupported
            } else {
                QueryOutcome::UnsupportedWith(values)
            }
        } else {
            self.resolved_outcome(values)
        }
    }

    pub(crate) fn conforms_to(
        &self,
        specific: &SymbolIdentity,
        general: &SymbolIdentity,
        scope: SpecializationScope,
    ) -> QueryOutcome<Conformance> {
        let specific = match self.single_declaration(specific) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let general = match self.single_declaration(general) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        self.resolved_outcome(self.conformance(specific, general, scope))
    }

    fn conformance(
        &self,
        specific: DeclarationId,
        general: DeclarationId,
        scope: SpecializationScope,
    ) -> Conformance {
        if specific == general {
            return Conformance::Conforms;
        }
        // A declaration that reaches itself has a malformed hierarchy. Its closure is still
        // complete, so an answer could be produced -- but producing one would turn a modelling
        // error into a published semantic fact, which is exactly what the explicit-state rule
        // exists to prevent.
        if self.types.specialization().is_cyclic(specific) {
            return Conformance::Indeterminate(ConformanceObstacle::CyclicSpecialization);
        }
        if self
            .types
            .specialization()
            .reaches(specific, general, internal_scope(scope))
        {
            return Conformance::Conforms;
        }
        // Specialization did not reach it. Conformance is a question about what a type
        // *classifies*, and KerML's type relationships constrain that without stating a
        // generalization, so the set entailments are consulted before concluding.
        if self.set_inclusion(specific, general, scope, &mut Vec::new()) {
            return Conformance::Conforms;
        }
        Conformance::DoesNotConform
    }

    /// Whether `specific`'s instances are all `general`'s, following only entailments that are
    /// sound for KerML's four type relationships.
    ///
    /// Each is one direction of one relationship's set meaning, and nothing here is symmetric:
    ///
    /// - a union owner is included in `general` only when *every* operand is, since it classifies
    ///   whatever any operand does;
    /// - an intersection owner is included in `general` when *any* operand is, since it classifies
    ///   only what every operand does;
    /// - a difference owner is included in `general` when its *first* operand is -- the remaining
    ///   operands are exclusions and carry no positive inclusion;
    /// - `general` being a union owner admits `specific` when `specific` is included in any of its
    ///   operands, since each operand is included in the union;
    /// - `Disjoint` states that two types share no instances and entails no inclusion at all, so
    ///   it appears nowhere below.
    ///
    /// `visiting` bounds the recursion. A malformed model can make these relationships mutually
    /// recursive, and the answer for a type currently being decided is "not established by this
    /// path" rather than a second attempt.
    fn set_inclusion(
        &self,
        specific: DeclarationId,
        general: DeclarationId,
        scope: SpecializationScope,
        visiting: &mut Vec<DeclarationId>,
    ) -> bool {
        if specific == general {
            return true;
        }
        if visiting.contains(&specific) {
            return false;
        }
        if self
            .types
            .specialization()
            .reaches(specific, general, internal_scope(scope))
        {
            return true;
        }
        visiting.push(specific);
        let included = self.set_inclusion_uncached(specific, general, scope, visiting);
        visiting.pop();
        included
    }

    fn set_inclusion_uncached(
        &self,
        specific: DeclarationId,
        general: DeclarationId,
        scope: SpecializationScope,
        visiting: &mut Vec<DeclarationId>,
    ) -> bool {
        let mut union_operands = self
            .types
            .operands_of(specific, types::SetOperator::Union)
            .peekable();
        if union_operands.peek().is_some()
            && union_operands.all(|operand| self.set_inclusion(operand, general, scope, visiting))
        {
            return true;
        }
        if self
            .types
            .operands_of(specific, types::SetOperator::Intersection)
            .any(|operand| self.set_inclusion(operand, general, scope, visiting))
        {
            return true;
        }
        if self
            .types
            .operands_of(specific, types::SetOperator::Difference)
            .next()
            .is_some_and(|reduced| self.set_inclusion(reduced, general, scope, visiting))
        {
            return true;
        }
        self.types
            .operands_of(general, types::SetOperator::Union)
            .any(|operand| self.set_inclusion(specific, operand, scope, visiting))
    }

    /// KerML §7.4.12: every type of the general feature must be conformed to by some type of the
    /// specific feature.
    ///
    /// An untyped side conforms: a feature that declares no typing of its own takes the other's,
    /// so there is nothing to violate. Effective types are used rather than declared ones, so a
    /// feature that inherits its typing along a redefinition chain is not mistaken for untyped.
    pub(crate) fn feature_typing_conforms(
        &self,
        specific: &SymbolIdentity,
        general: &SymbolIdentity,
    ) -> QueryOutcome<Conformance> {
        let specific = match self.single_declaration(specific) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let general = match self.single_declaration(general) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        self.resolved_outcome(self.typing_conformance(specific, general))
    }

    fn typing_conformance(&self, specific: DeclarationId, general: DeclarationId) -> Conformance {
        // The specific side contributes only the typings it does not owe to the general side.
        // Counting an inherited typing would make the rule vacuous: every redefining feature
        // inherits the redefined feature's type, so that type would always be there to satisfy
        // the check, and a feature retyped to something unrelated would still pass.
        let specific_types = self
            .types
            .effective_types(specific)
            .iter()
            .filter(|(_, source)| match source {
                types::EffectiveTypeSource::Direct => true,
                // Discard a typing the specific side inherited along a chain that runs *through*
                // the general side: the general side has that typing too, so it cannot be the
                // thing that fails to conform to it.
                //
                // The test is that `general` reaches `from`, not the reverse. `trueEvaluations
                // subsets booleanEvaluations subsets evaluations` inherits `Evaluation` from
                // `evaluations`, and `evaluations` is an ancestor of the general side rather than
                // a descendant, so asking whether `evaluations` reaches `booleanEvaluations`
                // answered no and kept a typing both sides share. The rule then demanded that
                // `Evaluation` conform to `BooleanEvaluation` -- the general side's own narrower
                // typing -- and reported the Kernel Semantic Library's own declarations as
                // incompatible.
                types::EffectiveTypeSource::Inherited(from) => {
                    *from != general
                        && !self.types.specialization().reaches(
                            general,
                            *from,
                            types::SpecializationScope::FeatureSpecialization,
                        )
                }
            })
            .map(|(target, _)| *target)
            .collect::<Vec<_>>();
        let general_types = self.types.effective_types(general);
        if specific_types.is_empty() || general_types.is_empty() {
            return Conformance::Conforms;
        }
        let mut obstacle = None;
        for (general_type, _) in general_types {
            let mut satisfied = false;
            for specific_type in &specific_types {
                match self.conformance(
                    *specific_type,
                    *general_type,
                    SpecializationScope::AnySpecialization,
                ) {
                    Conformance::Conforms => {
                        satisfied = true;
                        break;
                    }
                    Conformance::Indeterminate(reason) => obstacle = Some(reason),
                    Conformance::DoesNotConform => {}
                }
            }
            if !satisfied {
                // An unanswerable pair cannot be reported as a violation: the rule was never
                // evaluated for it.
                return match obstacle {
                    Some(reason) => Conformance::Indeterminate(reason),
                    None => Conformance::DoesNotConform,
                };
            }
        }
        Conformance::Conforms
    }

    /// KerML §8.4.3.4, with its two halves kept apart.
    pub(crate) fn subsetting_conforms(
        &self,
        subsetting: &SymbolIdentity,
        subsetted: &SymbolIdentity,
    ) -> QueryOutcome<SubsettingConformance> {
        let subsetting = match self.single_declaration(subsetting) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let subsetted = match self.single_declaration(subsetted) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        // A feature owned only by namespaces has no featuring type, so the domain half of the rule
        // has nothing to compare and cannot be violated.
        let featuring = match (
            self.types.featuring_type(subsetting),
            self.types.featuring_type(subsetted),
        ) {
            (Some(specific), Some(general)) => {
                self.conformance(specific, general, SpecializationScope::AnySpecialization)
            }
            _ => Conformance::Conforms,
        };
        self.resolved_outcome(SubsettingConformance {
            featuring,
            types: self.typing_conformance(subsetting, subsetted),
        })
    }
}

fn internal_scope(scope: SpecializationScope) -> types::SpecializationScope {
    match scope {
        SpecializationScope::AnySpecialization => types::SpecializationScope::AnySpecialization,
        SpecializationScope::Subclassification => types::SpecializationScope::Subclassification,
        SpecializationScope::FeatureSpecialization => {
            types::SpecializationScope::FeatureSpecialization
        }
    }
}

fn document_range(
    storage: &SemanticModelStorage,
    document: DocumentId,
    span: &Span,
) -> Result<TextRange, ResolutionError> {
    let parsed = &storage
        .document(document)
        .ok_or(ResolutionError::InvalidStorage)?
        .parsed;
    let range = parsed.range(span).ok_or(ResolutionError::InvalidStorage)?;
    Ok(TextRange {
        start: TextPosition {
            line: range.start.line.saturating_sub(1),
            character: u32::try_from(range.start.column.saturating_sub(1))
                .map_err(|_| ResolutionError::Capacity)?,
        },
        end: TextPosition {
            line: range.end.line.saturating_sub(1),
            character: u32::try_from(range.end.column.saturating_sub(1))
                .map_err(|_| ResolutionError::Capacity)?,
        },
    })
}

fn parser_diagnostic_category(
    category: Option<sysml_v2_parser::DiagnosticCategory>,
) -> DiagnosticCategory {
    match category {
        Some(sysml_v2_parser::DiagnosticCategory::ParseError) => {
            DiagnosticCategory::MalformedSyntax
        }
        Some(sysml_v2_parser::DiagnosticCategory::UnsupportedGrammarForm) => {
            DiagnosticCategory::UnsupportedSyntax
        }
        Some(sysml_v2_parser::DiagnosticCategory::UnresolvedSymbol) => {
            DiagnosticCategory::Unresolved
        }
        None => DiagnosticCategory::UnclassifiedParser,
    }
}

fn parse_error_range(document: &ParsedDocument, error: &ParseError) -> Option<TextRange> {
    let start_offset = error.offset?;
    let end_offset = start_offset.checked_add(error.length.unwrap_or(1))?;
    let start = document.source.position_at(start_offset)?;
    let end = document.source.position_at(end_offset).unwrap_or(start);
    Some(TextRange {
        start: TextPosition {
            line: start.line.saturating_sub(1),
            character: u32::try_from(start.column.saturating_sub(1)).ok()?,
        },
        end: TextPosition {
            line: end.line.saturating_sub(1),
            character: u32::try_from(end.column.saturating_sub(1)).ok()?,
        },
    })
}

/// The public code for a construct this publication does not model.
///
/// Exhaustive by construction: a new lowering family cannot be added without deciding its code.
fn unsupported_construct_code(family: UnsupportedFamily) -> DiagnosticCode {
    match family {
        UnsupportedFamily::PackageMember => DiagnosticCode::UnsupportedPackageMember,
        UnsupportedFamily::PartDefinitionMember => DiagnosticCode::UnsupportedPartDefinitionMember,
        UnsupportedFamily::PartUsageMember => DiagnosticCode::UnsupportedPartUsageMember,
        UnsupportedFamily::AttributeMember => DiagnosticCode::UnsupportedAttributeMember,
        UnsupportedFamily::RequirementDefinitionMember => {
            DiagnosticCode::UnsupportedRequirementDefinitionMember
        }
        UnsupportedFamily::PortDefinitionMember => DiagnosticCode::UnsupportedPortDefinitionMember,
        UnsupportedFamily::PortUsageMember => DiagnosticCode::UnsupportedPortUsageMember,
        UnsupportedFamily::ActionDefinitionMember => {
            DiagnosticCode::UnsupportedActionDefinitionMember
        }
        UnsupportedFamily::ActionUsageMember => DiagnosticCode::UnsupportedActionUsageMember,
        UnsupportedFamily::StateDefinitionMember => {
            DiagnosticCode::UnsupportedStateDefinitionMember
        }
        UnsupportedFamily::ConnectionDefinitionMember => {
            DiagnosticCode::UnsupportedConnectionDefinitionMember
        }
        UnsupportedFamily::InterfaceDefinitionMember => {
            DiagnosticCode::UnsupportedInterfaceDefinitionMember
        }
        UnsupportedFamily::ViewDefinitionMember => DiagnosticCode::UnsupportedViewDefinitionMember,
        UnsupportedFamily::ConstraintDefinitionMember => {
            DiagnosticCode::UnsupportedConstraintDefinitionMember
        }
        UnsupportedFamily::CalcDefinitionMember => DiagnosticCode::UnsupportedCalcDefinitionMember,
        UnsupportedFamily::RenderingDefinitionMember => {
            DiagnosticCode::UnsupportedRenderingDefinitionMember
        }
        UnsupportedFamily::OccurrenceDefinitionMember => {
            DiagnosticCode::UnsupportedOccurrenceDefinitionMember
        }
        UnsupportedFamily::AnalysisCaseDefinitionMember => {
            DiagnosticCode::UnsupportedAnalysisCaseDefinitionMember
        }
        UnsupportedFamily::CaseDefinitionMember => DiagnosticCode::UnsupportedCaseDefinitionMember,
        UnsupportedFamily::VerificationCaseDefinitionMember => {
            DiagnosticCode::UnsupportedVerificationCaseDefinitionMember
        }
        UnsupportedFamily::UseCaseDefinitionMember => {
            DiagnosticCode::UnsupportedUseCaseDefinitionMember
        }
        UnsupportedFamily::ReferenceUsageMember => DiagnosticCode::UnsupportedReferenceUsageMember,
        UnsupportedFamily::RelationshipBodyMember => {
            DiagnosticCode::UnsupportedRelationshipBodyMember
        }
        UnsupportedFamily::ParserUnsupported => DiagnosticCode::UnsupportedParserConstruct,
    }
}

/// What one authored reference's settled outcome reports, or `None` when it resolved.
///
/// A resolved reference has nothing to report; that is not the same answer as any of the failure
/// states below, and the three failure classes stay distinct all the way to the consumer.
fn reference_diagnostic(
    kind: ReferenceKind,
    status: ResolutionStatus,
) -> Option<(DiagnosticSeverity, DiagnosticCode)> {
    match status {
        ResolutionStatus::Resolved(_) => None,
        ResolutionStatus::Unresolved => Some((
            DiagnosticSeverity::Warning,
            match kind {
                ReferenceKind::FeatureTyping => DiagnosticCode::UnresolvedTypeReference,
                ReferenceKind::Subclassification => DiagnosticCode::UnresolvedSpecializesReference,
                ReferenceKind::NamespaceImport | ReferenceKind::MembershipImport => {
                    DiagnosticCode::UnresolvedImportTarget
                }
                // A view names what it shows; an expose target that resolves to nothing means the
                // view shows nothing, which is a different thing to a reader than an unresolved
                // name in a declaration.
                ReferenceKind::ViewExpose => DiagnosticCode::ViewExposeUnresolved,
                _ => DiagnosticCode::UnresolvedReference,
            },
        )),
        ResolutionStatus::Unsupported => Some((
            DiagnosticSeverity::Warning,
            match kind {
                ReferenceKind::NamespaceImport
                | ReferenceKind::MembershipImport
                | ReferenceKind::FilterImport => DiagnosticCode::UnsupportedFilteredImport,
                _ => DiagnosticCode::UnsupportedReference,
            },
        )),
        ResolutionStatus::NonConverged => Some((
            DiagnosticSeverity::Error,
            DiagnosticCode::NonConvergedResolution,
        )),
        ResolutionStatus::Ambiguous(_) => Some((
            DiagnosticSeverity::Error,
            match kind {
                ReferenceKind::NamespaceImport | ReferenceKind::MembershipImport => {
                    DiagnosticCode::AmbiguousImportTarget
                }
                _ => DiagnosticCode::AmbiguousReference,
            },
        )),
    }
}

/// Where a *declaration* writes its own name.
///
/// Distinct from [`identifier_range`], which searches a reference span and takes the last
/// word-boundary match because a qualified path names its target in the final segment. A
/// declaration span covers the whole declaration including its body, so the same rule finds the
/// last mention of the name anywhere inside -- for `part def Vehicle { part engine : Vehicle; }`
/// it points at the body's reference rather than at the declared name.
///
/// The declared name is in the header, after the keywords and after an optional `<shortName>`, so
/// the search is bounded to the text before the body opener and skips the short-name group. A
/// declaration whose header is unrecoverable -- a parse recovery that lost its `{` or `;` -- falls
/// back to the whole-span search rather than losing its location entirely.
fn declaration_identifier_range(
    storage: &SemanticModelStorage,
    document: DocumentId,
    span: &Span,
    identifier: &str,
) -> Result<TextRange, ResolutionError> {
    let parsed = &storage
        .document(document)
        .ok_or(ResolutionError::InvalidStorage)?
        .parsed;
    let source = parsed
        .source
        .slice(span)
        .ok_or(ResolutionError::InvalidStorage)?;
    let header = source
        .find(['{', ';'])
        .map_or(source, |body| &source[..body]);
    let relative = word_boundary_matches(header, identifier)
        .find(|start| !inside_short_name(header, *start))
        .or_else(|| word_boundary_matches(header, identifier).next())
        .or_else(|| word_boundary_matches(source, identifier).last())
        .ok_or(ResolutionError::InvalidStorage)?;
    identifier_text_range(parsed, span, relative, identifier.len())
}

/// Whether `start` falls inside an unclosed `<`...`>` short-name group.
fn inside_short_name(header: &str, start: usize) -> bool {
    let before = &header[..start];
    before
        .rfind('<')
        .is_some_and(|open| !before[open..].contains('>'))
}

/// Every occurrence of `identifier` in `text` that is not part of a longer identifier.
fn word_boundary_matches<'a>(
    text: &'a str,
    identifier: &'a str,
) -> impl Iterator<Item = usize> + 'a {
    text.match_indices(identifier)
        .filter(move |(start, _)| {
            let before = text[..*start].chars().next_back();
            let after = text[*start + identifier.len()..].chars().next();
            !before.is_some_and(identifier_character) && !after.is_some_and(identifier_character)
        })
        .map(|(start, _)| start)
}

fn identifier_range(
    storage: &SemanticModelStorage,
    document: DocumentId,
    span: &Span,
    identifier: &str,
) -> Result<TextRange, ResolutionError> {
    let parsed = &storage
        .document(document)
        .ok_or(ResolutionError::InvalidStorage)?
        .parsed;
    let source = parsed
        .source
        .slice(span)
        .ok_or(ResolutionError::InvalidStorage)?;
    let relative = word_boundary_matches(source, identifier)
        .last()
        .ok_or(ResolutionError::InvalidStorage)?;
    identifier_text_range(parsed, span, relative, identifier.len())
}

fn identifier_text_range(
    parsed: &ParsedDocument,
    span: &Span,
    relative: usize,
    length: usize,
) -> Result<TextRange, ResolutionError> {
    let start_offset = span
        .offset
        .checked_add(relative)
        .ok_or(ResolutionError::Capacity)?;
    let end_offset = start_offset
        .checked_add(length)
        .ok_or(ResolutionError::Capacity)?;
    let start = parsed
        .source
        .position_at(start_offset)
        .ok_or(ResolutionError::InvalidStorage)?;
    let end = parsed
        .source
        .position_at(end_offset)
        .ok_or(ResolutionError::InvalidStorage)?;
    Ok(TextRange {
        start: TextPosition {
            line: start.line.saturating_sub(1),
            character: u32::try_from(start.column.saturating_sub(1))
                .map_err(|_| ResolutionError::Capacity)?,
        },
        end: TextPosition {
            line: end.line.saturating_sub(1),
            character: u32::try_from(end.column.saturating_sub(1))
                .map_err(|_| ResolutionError::Capacity)?,
        },
    })
}

fn identifier_character(character: char) -> bool {
    character.is_alphanumeric() || matches!(character, '_' | '-')
}

fn valid_identifier(value: &str) -> bool {
    let mut characters = value.chars();
    characters
        .next()
        .is_some_and(|first| first.is_alphabetic() || first == '_')
        && characters.all(identifier_character)
}

fn range_contains(range: TextRange, position: TextPosition) -> bool {
    range.start <= position && position <= range.end
}

fn target_order(left: &NavigationTarget, right: &NavigationTarget) -> std::cmp::Ordering {
    left.location
        .document
        .cmp(&right.location.document)
        .then_with(|| left.location.range.cmp(&right.location.range))
        .then_with(|| left.name.cmp(&right.name))
}

fn location_order(left: &SourceLocation, right: &SourceLocation) -> std::cmp::Ordering {
    left.document
        .cmp(&right.document)
        .then_with(|| left.range.cmp(&right.range))
        .then_with(|| left.role.cmp(&right.role))
}

fn declaration_qualified_name(
    storage: &SemanticModelStorage,
    mut declaration: DeclarationId,
) -> Option<String> {
    let mut names = Vec::new();
    loop {
        let value = storage.declaration(declaration)?;
        if let Some(name) = value.name.and_then(|name| storage.symbol(name)) {
            names.push(name);
        }
        let Some(owner) = value.owner else {
            break;
        };
        declaration = owner;
    }
    names.reverse();
    Some(names.join("::"))
}

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
    outcomes: Box<[ResolutionStatus]>,
    /// Names declared at the library's own root, which a workspace declaration must not shadow or
    /// duplicate.
    root_names: std::collections::BTreeSet<Box<str>>,
    /// First path segments of every library reference the library-only build left unresolved or
    /// ambiguous. A workspace root with one of these names could newly satisfy -- or newly make
    /// ambiguous -- a reference whose outcome is about to be reused.
    unsettled_roots: std::collections::BTreeSet<Box<str>>,
}

impl SettledLibrary {
    /// Whether these outcomes may be reused for a publication containing `storage`.
    ///
    /// Global-root lookup is the one channel through which a workspace declaration can change what
    /// a library reference resolves to, and it is reachable in exactly two ways: a workspace root
    /// sharing a library root's name, or a workspace root answering a lookup the library itself
    /// left unsettled. Both are name comparisons over the roots, so the check costs a walk of the
    /// workspace's top-level declarations rather than a re-solve.
    fn admits(&self, storage: &SemanticModelStorage) -> bool {
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

impl ResolvedSemanticModel {
    /// Everything a later publication needs to reuse this one as a library.
    ///
    /// The parsed documents come out by reference-counted handle rather than by copy, so reuse
    /// shares one parse of the library across every publication built against it.
    pub(crate) fn prepared_library(
        &self,
    ) -> Result<super::PreparedLibrary, super::CoordinatorError> {
        let documents = self
            .storage
            .documents
            .iter()
            .map(|document| super::PreparedDocument {
                identity: document.identity.clone(),
                role: document.role,
                parsed: Arc::clone(&document.parsed),
                parse_errors: document.parse_errors.to_vec(),
            })
            .collect();
        Ok(super::PreparedLibrary {
            documents,
            settled: self
                .settled_library()
                .map_err(|_| super::CoordinatorError::ConstructionFailed)?,
        })
    }

    /// The reusable settled state of a library-only publication.
    pub(super) fn settled_library(&self) -> Result<SettledLibrary, ResolutionError> {
        let mut root_names = std::collections::BTreeSet::new();
        for declaration in self.storage.declarations.iter() {
            if declaration.owner.is_some() {
                continue;
            }
            if let Some(name) = declaration.name.and_then(|name| self.storage.symbol(name)) {
                root_names.insert(name.into());
            }
        }
        let mut unsettled_roots = std::collections::BTreeSet::new();
        for (index, reference) in self.storage.references.iter().enumerate() {
            let id =
                AuthoredReferenceId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
            let settled = matches!(
                self.resolution.outcome(id),
                Some(ResolutionStatus::Resolved(_)) | Some(ResolutionStatus::Unsupported)
            );
            if settled {
                continue;
            }
            let Some((segments, _)) = self.storage.paths.get(reference.path) else {
                continue;
            };
            let Some(first) = segments.first().and_then(|id| self.storage.symbol(*id)) else {
                continue;
            };
            unsettled_roots.insert(first.into());
        }
        Ok(SettledLibrary {
            outcomes: self.resolution.outcomes.clone(),
            root_names,
            unsettled_roots,
        })
    }
}

impl SemanticModelStorage {
    pub(super) fn resolve(
        self,
        policy: EvaluationPolicy,
        library: Option<&SettledLibrary>,
        reported: &[Box<str>],
    ) -> Result<ResolvedSemanticModel, ResolutionError> {
        let has_recovery = self
            .documents
            .iter()
            .any(|document| !document.parse_errors.is_empty())
            || !self.recovery.is_empty();
        let has_unsupported = !self.unsupported.is_empty();
        let seed = library
            .filter(|library| library.admits(&self))
            .map(|library| library.outcomes.as_ref());
        let (direct_names, effective_imports, memberships, mut resolution) = resolve_dense(
            &self.declarations,
            &self.memberships,
            &self.paths,
            &self.references,
            seed,
        )?;
        // `checkPartDefinitionSpecialization` is an implied semantic fact, so its anchor and
        // relationships are settled here, before every index and diagnostic consumer below. The
        // lookup is owned by semantic construction: neither a renderer nor a validation rule gets
        // to rediscover `Parts::Part` from text or a display path.
        let library_anchors = library_specialization_anchors(&self);
        if matches!(resolution.solver_status, SolverStatus::Converged) {
            let mut implied = resolution.implied_relationships.into_vec();
            implied.extend(
                synthesize_generated_library_specializations(
                    &self,
                    &self.references,
                    &resolution.outcomes,
                    &library_anchors,
                )?
                .into_vec(),
            );
            implied.extend(
                synthesize_generated_library_redefinitions(
                    &self,
                    &self.references,
                    &library_anchors,
                )?
                .into_vec(),
            );
            implied.extend(
                synthesize_feature_membership_type_featurings(&self, &self.references)?.into_vec(),
            );
            implied.sort_by_key(|relationship| {
                (
                    relationship.kind,
                    relationship.source.0,
                    relationship.target.0,
                )
            });
            implied.dedup();
            resolution.implied_relationships = implied.into_boxed_slice();
        }
        resolution.library_specialization_anchors = library_anchors;
        let completeness = if has_recovery {
            PublicationCompleteness::ParseRecovery
        } else if has_unsupported {
            PublicationCompleteness::UnsupportedSyntax
        } else if !matches!(resolution.solver_status, SolverStatus::Converged) {
            PublicationCompleteness::NonConverged
        } else {
            PublicationCompleteness::Complete
        };
        let settled = compute_evaluation(&self, &resolution, policy);
        let (evaluation, filter_conditions) = match settled {
            SettledEvaluation::Settled { facts, filters } => (facts, Some(filters)),
            SettledEvaluation::Vacuous => (Box::default(), None),
        };
        let has_evaluation = !evaluation.is_empty();
        let identities = IdentityIndex::build(&self)?;
        let documents = DocumentIndex::build(&self)?;
        let reverse_references =
            ReverseReferenceIndex::build(self.declarations.len(), &resolution)?;
        let effective_scopes = EffectiveScopeIndex::build(
            self.declarations.len(),
            &direct_names,
            &effective_imports,
            &resolution.inherited_names,
        )?;
        let facts = inspection::ElementFactIndex::build(&self, &resolution, &evaluation)?;
        let bindings = binding::BindingConnectorIndex::build(&self, &resolution)?;
        // A barrier product, not a solver family: every type fact here is derived from settled
        // outcomes and feeds nothing back into scope, imports or inheritance. The resolver's own
        // ancestor closure for inherited names stays separate and unchanged -- widening that one
        // would silently change name resolution.
        let type_facts = types::TypeIndex::build(&self, &resolution)?;
        let mut model = ResolvedSemanticModel {
            storage: self,
            direct_names,
            effective_imports,
            identities,
            documents,
            memberships,
            reverse_references,
            effective_scopes,
            facts,
            bindings,
            types: type_facts,
            resolution,
            evaluation,
            expressions: expression::ExpressionIndex::default(),
            diagnostics: Box::default(),
            diagnostics_by_document: Box::default(),
            metadata: PublicationMetadata {
                phase: PublicationPhase::Resolved,
                completeness,
                has_evaluation,
            },
        };
        // Expression facts read the type closure and the settled evaluation, so they are assembled
        // once the model holds both, and before diagnostics, which report what they settled.
        model.expressions = expression::ExpressionIndex::build(&model, filter_conditions)?;
        // Last barrier product: diagnostics report what every earlier phase settled, so they are
        // derived from the assembled model rather than from any one phase's intermediate state.
        let (diagnostics, diagnostics_by_document) = model.derive_diagnostics(reported)?;
        model.diagnostics = diagnostics;
        model.diagnostics_by_document = diagnostics_by_document;
        Ok(model)
    }
}

trait ResolutionReferenceFact {
    fn source(&self) -> DeclarationId;
    fn kind(&self) -> ReferenceKind;
    fn path(&self) -> SymbolPathId;
    fn flags(&self) -> RelationshipFlags;
}

impl ResolutionReferenceFact for AuthoredReference {
    fn source(&self) -> DeclarationId {
        self.source
    }

    fn kind(&self) -> ReferenceKind {
        self.kind
    }

    fn path(&self) -> SymbolPathId {
        self.path
    }

    fn flags(&self) -> RelationshipFlags {
        self.flags
    }
}

fn resolve_dense<R: ResolutionReferenceFact>(
    declarations: &[Declaration],
    memberships: &[MembershipRecord],
    paths: &SymbolPathArena,
    references: &[R],
    seed: Option<&[ResolutionStatus]>,
) -> Result<(NameIndex, NameIndex, MembershipIndex, ResolutionResults), ResolutionError> {
    let supported_import_count = references
        .iter()
        .filter(|reference| supported_import_domain(*reference).is_some())
        .count();
    // Each effective edge is drawn from a finite declaration/import product. The deliberately
    // conservative bound makes failure explicit without assuming acyclic imports or relying on a
    // machine-dependent timeout.
    let pass_limit = declarations
        .len()
        .checked_mul(supported_import_count.saturating_add(1))
        .and_then(|limit| limit.checked_add(1))
        .ok_or(ResolutionError::Capacity)?
        .max(1);
    resolve_dense_with_limit(
        declarations,
        memberships,
        paths,
        references,
        pass_limit,
        seed,
    )
}

fn resolve_dense_with_limit<R: ResolutionReferenceFact>(
    declarations: &[Declaration],
    memberships: &[MembershipRecord],
    paths: &SymbolPathArena,
    references: &[R],
    pass_limit: usize,
    seed: Option<&[ResolutionStatus]>,
) -> Result<(NameIndex, NameIndex, MembershipIndex, ResolutionResults), ResolutionError> {
    let membership_records = memberships;
    let memberships = MembershipIndex::build(declarations, memberships)?;
    let direct_names = build_direct_name_index(declarations, None)?;
    let exported_names = build_direct_name_index(declarations, Some(&memberships))?;
    let mut outcomes = vec![ResolutionStatus::Unsupported; references.len()];
    // A settled library's outcomes are installed before the first pass and its references are then
    // left out of every slot list below, so no pass re-evaluates them. They are still *read* --
    // by the name, import and inheritance indexes each pass rebuilds -- which is what makes the
    // workspace resolve against a library that is already settled rather than against nothing.
    let settled = seed.map_or(0, <[ResolutionStatus]>::len);
    if let Some(seed) = seed {
        outcomes[..settled].copy_from_slice(seed);
    }
    let all_import_slots: Vec<usize> = references
        .iter()
        .enumerate()
        .filter_map(|(index, reference)| supported_import_domain(reference).map(|_| index))
        .collect();
    let import_slots: Vec<usize> = all_import_slots
        .iter()
        .copied()
        .filter(|index| *index >= settled)
        .collect();
    // Subclassification is resolved first because the ancestor-scoped inherited-member lookup used
    // by FeatureTyping is built directly from settled Subclassification outcomes; splitting the two
    // kinds avoids depending on source order between an owned specialization and an owned typing
    // reference within the same document.
    let subclass_slots: Vec<usize> = references
        .iter()
        .enumerate()
        .filter(|(index, _)| *index >= settled)
        .filter_map(|(index, reference)| {
            // The four KerML type-relationship kinds join this pass rather than getting their own:
            // each names a `Type` through the same lexical lookup a `specializes` clause uses, and
            // none of them reads inherited scope. They stay distinct `ReferenceKind`s so the
            // published relationship never collapses into a specialization.
            matches!(
                reference.kind(),
                ReferenceKind::Subclassification
                    | ReferenceKind::Unioning
                    | ReferenceKind::Intersecting
                    | ReferenceKind::Differencing
                    | ReferenceKind::Disjoining
            )
            .then_some(index)
        })
        .collect();
    let typing_slots: Vec<usize> = references
        .iter()
        .enumerate()
        .filter(|(index, _)| *index >= settled)
        .filter_map(|(index, reference)| {
            matches!(
                reference.kind(),
                ReferenceKind::FeatureTyping | ReferenceKind::TypeFeaturing
            )
            .then_some(index)
        })
        .collect();
    // A metadata annotation's target (`@Safety{...}`'s `Safety`) must be a type -- specifically a
    // metadata def -- exactly like `FeatureTyping`, so `MetadataAnnotation` joins the same
    // `DeclarationDomain::Type`/ancestor-scoped lexical lookup as `typing_slots` below, kept as its
    // own slot list (and its own `ReferenceKind`) purely so the annotation relationship stays
    // distinct from ordinary typing/specialization in query output.
    // A filter condition's `@Name` metadata-classification test (`ReferenceKind::
    // FilterMetadataTest`, e.g. `filter @Safety;`'s `Safety`) names a metadata def exactly like a
    // `MetadataAnnotation` target, so it joins the same `DeclarationDomain::Type` slot list rather
    // than a separate one -- both resolve through the identical lexical lookup fixed point, kept
    // as a distinct `ReferenceKind` purely so filter and annotation relationships stay distinct in
    // query output.
    let metadata_annotation_slots: Vec<usize> = references
        .iter()
        .enumerate()
        .filter(|(index, _)| *index >= settled)
        .filter_map(|(index, reference)| {
            matches!(
                reference.kind(),
                ReferenceKind::MetadataAnnotation
                    | ReferenceKind::FilterMetadataTest
                    | ReferenceKind::AcceptPayloadType
                    | ReferenceKind::TypeCheckTarget
                    | ReferenceKind::MetaCastTarget
                    | ReferenceKind::FlowPayloadType
            )
            .then_some(index)
        })
        .collect();
    // `Subsetting` (`:>`) and `Redefinition` (`:>>`) targets are always features, never types, so
    // they resolve against `DeclarationDomain::Any` -- like `AliasBinding`/`ConnectorEnd` -- rather
    // than joining the Subclassification/FeatureTyping `Type` domain pass. They do read the same
    // ancestor-scoped `inherited_names` index as `FeatureTyping` (built from settled
    // Subclassification outcomes below), because both explicit redefinition (`:>> status = ...;`)
    // and subsetting of an inherited feature must be able to reach a same-named or differently-named
    // feature owned by an ancestor, not just a directly owned member.
    let subsetting_slots: Vec<usize> = references
        .iter()
        .enumerate()
        .filter(|(index, _)| *index >= settled)
        .filter_map(|(index, reference)| {
            (reference.kind() == ReferenceKind::Subsetting).then_some(index)
        })
        .collect();
    let redefinition_slots: Vec<usize> = references
        .iter()
        .enumerate()
        .filter(|(index, _)| *index >= settled)
        .filter_map(|(index, reference)| {
            (reference.kind() == ReferenceKind::Redefinition).then_some(index)
        })
        .collect();
    // An alias target can be any element (not just a Type), so `AliasBinding` resolves against
    // `DeclarationDomain::Any` rather than joining the Subclassification/FeatureTyping `Type`
    // domain passes; it does not read inherited scope either, so it can settle alongside
    // Subclassification, independently of the ancestor closures built below.
    let alias_slots: Vec<usize> = references
        .iter()
        .enumerate()
        .filter(|(index, _)| *index >= settled)
        .filter_map(|(index, reference)| {
            (reference.kind() == ReferenceKind::AliasBinding).then_some(index)
        })
        .collect();
    // A connector end can reference any feature (not just a Type), exactly like an alias target,
    // so `ConnectorEnd` resolves against `DeclarationDomain::Any` alongside `AliasBinding` rather
    // than joining the Subclassification/FeatureTyping `Type` domain passes; it does not read
    // inherited scope either.
    let connector_end_slots: Vec<usize> = references
        .iter()
        .enumerate()
        .filter(|(index, _)| *index >= settled)
        .filter_map(|(index, reference)| {
            (reference.kind() == ReferenceKind::ConnectorEnd).then_some(index)
        })
        .collect();
    // A succession end (`first`/`then` in a `FirstStmt`) can reference any owned action feature
    // (not just a Type), exactly like a connector end, so `Succession` resolves against
    // `DeclarationDomain::Any` alongside `ConnectorEnd` rather than joining the Subclassification/
    // FeatureTyping `Type` domain passes; it does not read inherited scope either.
    let succession_slots: Vec<usize> = references
        .iter()
        .enumerate()
        .filter(|(index, _)| *index >= settled)
        .filter_map(|(index, reference)| {
            (reference.kind() == ReferenceKind::Succession).then_some(index)
        })
        .collect();
    // Entry/do/exit action bindings, a state's initial-state (`then`) target, and a constraint/
    // calc expression's feature-reference operands (`ExpressionOperand`) can each reference any
    // owned feature (not just a Type), exactly like `Succession`, so they resolve against
    // `DeclarationDomain::Any` alongside it rather than joining the Subclassification/FeatureTyping
    // `Type` domain passes; none of them read inherited scope either.
    // A `satisfy <requirement> by <element>;` statement's source/target operands can each
    // reference any owned feature (not just a Type), exactly like `Succession`/`TransitionSource`,
    // so they join `state_binding_slots`'s `DeclarationDomain::Any` pass rather than the
    // Subclassification/FeatureTyping `Type` domain passes.
    // An `Expression::Invocation`/`Constructor` callee (`InvocationCallee`) can likewise name any
    // owned feature (a calc/function) or a type (a constructor), not just a Type, so it joins this
    // same `DeclarationDomain::Any` pass.
    let state_binding_slots: Vec<usize> = references
        .iter()
        .enumerate()
        .filter(|(index, _)| *index >= settled)
        .filter_map(|(index, reference)| {
            matches!(
                reference.kind(),
                ReferenceKind::EntryActionBinding
                    | ReferenceKind::DoActionBinding
                    | ReferenceKind::ExitActionBinding
                    | ReferenceKind::InitialState
                    | ReferenceKind::ExpressionOperand
                    | ReferenceKind::TransitionSource
                    | ReferenceKind::TransitionTarget
                    | ReferenceKind::TransitionTrigger
                    | ReferenceKind::TransitionEffect
                    | ReferenceKind::SatisfySource
                    | ReferenceKind::SatisfyTarget
                    | ReferenceKind::BindSource
                    | ReferenceKind::BindTarget
                    | ReferenceKind::Variant
                    | ReferenceKind::IncludeUseCase
                    | ReferenceKind::ViewExpose
                    | ReferenceKind::InvocationCallee
                    | ReferenceKind::ThenTarget
                    | ReferenceKind::AcceptVia
                    | ReferenceKind::SendTarget
                    | ReferenceKind::TerminateTarget
                    | ReferenceKind::FlowSource
                    | ReferenceKind::FlowTarget
                    | ReferenceKind::StakeholderTarget
                    | ReferenceKind::PurposeTarget
                    | ReferenceKind::VerifyRequirementTarget
                    | ReferenceKind::AssignTarget
                    | ReferenceKind::DependencyClient
                    | ReferenceKind::DependencySupplier
                    | ReferenceKind::PerformParameterTarget
                    | ReferenceKind::FeatureChaining
            )
            .then_some(index)
        })
        .collect();
    // `MemberAccessOperand` (a dotted feature-chain access, e.g. `t.bead`, `f.a`) must run after
    // `typing_slots` has resolved and `inherited_names` has been extended with usage-typing entries
    // below (`extend_inherited_names_with_usage_typing`), because its interior segments are looked
    // up as members of each hop's resolved *type*, not the hop's own declaration -- exactly the
    // index that extension builds. It cannot join `state_binding_slots` above, which runs before
    // that extension exists.
    let member_access_slots: Vec<usize> = references
        .iter()
        .enumerate()
        .filter(|(index, _)| *index >= settled)
        .filter_map(|(index, reference)| {
            matches!(
                reference.kind(),
                ReferenceKind::MemberAccessOperand
                    | ReferenceKind::AllocateSource
                    | ReferenceKind::AllocateTarget
            )
            .then_some(index)
        })
        .collect();
    let mut work = ResolutionWork {
        direct_index_entries: u64::try_from(direct_names.candidates.len())
            .map_err(|_| ResolutionError::Capacity)?,
        ..ResolutionWork::default()
    };
    let mut effective_imports = NameIndex::build(Vec::new())?;
    let mut exported_imports = NameIndex::build(Vec::new())?;
    let mut ambiguous_candidates = Vec::new();
    let mut candidates = Vec::new();
    let mut next_candidates = Vec::new();
    let mut converged = false;

    for _ in 0..pass_limit {
        work.passes = work
            .passes
            .checked_add(1)
            .ok_or(ResolutionError::Capacity)?;
        ambiguous_candidates.clear();
        for index in import_slots.iter().copied() {
            work.import_evaluations = work
                .import_evaluations
                .checked_add(1)
                .ok_or(ResolutionError::Capacity)?;
            let reference = &references[index];
            outcomes[index] = resolve_reference(
                declarations,
                paths,
                reference,
                supported_import_domain(reference).ok_or(ResolutionError::InvalidStorage)?,
                ResolutionIndexes {
                    direct_names: &direct_names,
                    exported_names: &exported_names,
                    effective_imports: Some(&effective_imports),
                    exported_imports: Some(&exported_imports),
                    inherited_names: None,
                },
                ResolutionScratch {
                    ambiguous_candidates: &mut ambiguous_candidates,
                    candidates: &mut candidates,
                    next_candidates: &mut next_candidates,
                    work: &mut work,
                },
            )?;
        }
        let (next_effective_imports, next_exported_imports) = build_effective_import_indexes(
            declarations,
            &memberships,
            references,
            &all_import_slots,
            &exported_names,
            &exported_imports,
            &outcomes,
        )?;
        if next_effective_imports == effective_imports && next_exported_imports == exported_imports
        {
            effective_imports = next_effective_imports;
            exported_imports = next_exported_imports;
            converged = true;
            break;
        }
        effective_imports = next_effective_imports;
        exported_imports = next_exported_imports;
    }

    let solver_status = if converged {
        SolverStatus::Converged
    } else {
        for index in import_slots
            .iter()
            .chain(&subclass_slots)
            .chain(&typing_slots)
            .chain(&state_binding_slots)
            .chain(&subsetting_slots)
            .chain(&redefinition_slots)
            .chain(&alias_slots)
            .chain(&connector_end_slots)
            .chain(&succession_slots)
            .chain(&metadata_annotation_slots)
            .chain(&member_access_slots)
            .copied()
        {
            outcomes[index] = ResolutionStatus::NonConverged;
        }
        ambiguous_candidates.clear();
        SolverStatus::NonConverged
    };

    let mut inherited_names = NameIndex::build(Vec::new())?;
    if converged {
        for index in subclass_slots.iter().copied() {
            work.downstream_evaluations = work
                .downstream_evaluations
                .checked_add(1)
                .ok_or(ResolutionError::Capacity)?;
            outcomes[index] = resolve_reference(
                declarations,
                paths,
                &references[index],
                DeclarationDomain::Type,
                ResolutionIndexes {
                    direct_names: &direct_names,
                    exported_names: &exported_names,
                    effective_imports: Some(&effective_imports),
                    exported_imports: Some(&exported_imports),
                    inherited_names: None,
                },
                ResolutionScratch {
                    ambiguous_candidates: &mut ambiguous_candidates,
                    candidates: &mut candidates,
                    next_candidates: &mut next_candidates,
                    work: &mut work,
                },
            )?;
        }

        for index in alias_slots.iter().copied() {
            work.downstream_evaluations = work
                .downstream_evaluations
                .checked_add(1)
                .ok_or(ResolutionError::Capacity)?;
            outcomes[index] = resolve_reference(
                declarations,
                paths,
                &references[index],
                DeclarationDomain::Any,
                ResolutionIndexes {
                    direct_names: &direct_names,
                    exported_names: &exported_names,
                    effective_imports: Some(&effective_imports),
                    exported_imports: Some(&exported_imports),
                    inherited_names: None,
                },
                ResolutionScratch {
                    ambiguous_candidates: &mut ambiguous_candidates,
                    candidates: &mut candidates,
                    next_candidates: &mut next_candidates,
                    work: &mut work,
                },
            )?;
        }

        for index in connector_end_slots.iter().copied() {
            work.downstream_evaluations = work
                .downstream_evaluations
                .checked_add(1)
                .ok_or(ResolutionError::Capacity)?;
            outcomes[index] = resolve_reference(
                declarations,
                paths,
                &references[index],
                DeclarationDomain::Any,
                ResolutionIndexes {
                    direct_names: &direct_names,
                    exported_names: &exported_names,
                    effective_imports: Some(&effective_imports),
                    exported_imports: Some(&exported_imports),
                    inherited_names: None,
                },
                ResolutionScratch {
                    ambiguous_candidates: &mut ambiguous_candidates,
                    candidates: &mut candidates,
                    next_candidates: &mut next_candidates,
                    work: &mut work,
                },
            )?;
        }

        for index in succession_slots.iter().copied() {
            work.downstream_evaluations = work
                .downstream_evaluations
                .checked_add(1)
                .ok_or(ResolutionError::Capacity)?;
            outcomes[index] = resolve_reference(
                declarations,
                paths,
                &references[index],
                DeclarationDomain::Any,
                ResolutionIndexes {
                    direct_names: &direct_names,
                    exported_names: &exported_names,
                    effective_imports: Some(&effective_imports),
                    exported_imports: Some(&exported_imports),
                    inherited_names: None,
                },
                ResolutionScratch {
                    ambiguous_candidates: &mut ambiguous_candidates,
                    candidates: &mut candidates,
                    next_candidates: &mut next_candidates,
                    work: &mut work,
                },
            )?;
        }

        for index in state_binding_slots.iter().copied() {
            work.downstream_evaluations = work
                .downstream_evaluations
                .checked_add(1)
                .ok_or(ResolutionError::Capacity)?;
            outcomes[index] = resolve_reference(
                declarations,
                paths,
                &references[index],
                DeclarationDomain::Any,
                ResolutionIndexes {
                    direct_names: &direct_names,
                    exported_names: &exported_names,
                    effective_imports: Some(&effective_imports),
                    exported_imports: Some(&exported_imports),
                    inherited_names: None,
                },
                ResolutionScratch {
                    ambiguous_candidates: &mut ambiguous_candidates,
                    candidates: &mut candidates,
                    next_candidates: &mut next_candidates,
                    work: &mut work,
                },
            )?;
        }

        // Alias bindings form a functional graph (each alias has at most one outgoing edge, its
        // own resolved target). A cycle (`alias A for B; alias B for A;`) is detected explicitly,
        // bounded by declaration count, and published as a typed `NonConverged` outcome on each
        // implicated alias's own `AliasBinding` reference -- mirroring the Subclassification
        // ancestor-closure cycle handling above -- rather than looping or panicking.
        let cyclic_alias_sources =
            detect_cyclic_alias_bindings(declarations, references, &outcomes)?;
        for index in alias_slots.iter().copied() {
            if cyclic_alias_sources.contains(&references[index].source()) {
                outcomes[index] = ResolutionStatus::NonConverged;
            }
        }

        // Ancestor-scoped inherited-member lookup is built once here, over the now-settled
        // Subclassification outcomes above, as its own bounded fixed point: diamond ancestry
        // (Left -> Base and Right -> Base) is visited once per declaration because the closure is a
        // set, and a specialization cycle is detected explicitly rather than looped forever.
        let (ancestor_closures, cyclic_ancestry) =
            build_ancestor_closures(declarations, references, &outcomes)?;
        inherited_names = build_inherited_name_index(&direct_names, &ancestor_closures)?;

        for index in typing_slots.iter().copied() {
            work.downstream_evaluations = work
                .downstream_evaluations
                .checked_add(1)
                .ok_or(ResolutionError::Capacity)?;
            let reference = &references[index];
            if owner_chain_is_cyclic(declarations, reference.source(), &cyclic_ancestry)? {
                outcomes[index] = ResolutionStatus::NonConverged;
                continue;
            }
            outcomes[index] = resolve_reference(
                declarations,
                paths,
                reference,
                DeclarationDomain::Type,
                ResolutionIndexes {
                    direct_names: &direct_names,
                    exported_names: &exported_names,
                    effective_imports: Some(&effective_imports),
                    exported_imports: Some(&exported_imports),
                    inherited_names: Some(&inherited_names),
                },
                ResolutionScratch {
                    ambiguous_candidates: &mut ambiguous_candidates,
                    candidates: &mut candidates,
                    next_candidates: &mut next_candidates,
                    work: &mut work,
                },
            )?;
        }

        for index in metadata_annotation_slots.iter().copied() {
            work.downstream_evaluations = work
                .downstream_evaluations
                .checked_add(1)
                .ok_or(ResolutionError::Capacity)?;
            let reference = &references[index];
            if owner_chain_is_cyclic(declarations, reference.source(), &cyclic_ancestry)? {
                outcomes[index] = ResolutionStatus::NonConverged;
                continue;
            }
            outcomes[index] = resolve_reference(
                declarations,
                paths,
                reference,
                DeclarationDomain::Type,
                ResolutionIndexes {
                    direct_names: &direct_names,
                    exported_names: &exported_names,
                    effective_imports: Some(&effective_imports),
                    exported_imports: Some(&exported_imports),
                    inherited_names: Some(&inherited_names),
                },
                ResolutionScratch {
                    ambiguous_candidates: &mut ambiguous_candidates,
                    candidates: &mut candidates,
                    next_candidates: &mut next_candidates,
                    work: &mut work,
                },
            )?;
        }

        // A redefinition/subsetting reference owned by a *usage* (not a def/type) cannot reach its
        // target through the usage's own ancestor closure -- a plain usage has no Subclassification
        // ancestors of its own. Instead, the target must be reached
        // by first following the usage's own settled `FeatureTyping` reference to its type, then
        // searching that type's directly-owned members and its already-computed ancestor closure
        // (built above from Subclassification, independently of this step). This is why it runs
        // only now, after `typing_slots` has converged -- unlike the Subclassification-based
        // `inherited_names` above, which does not depend on FeatureTyping at all. Example: `need :
        // Need { attribute :>> status = ...; }` where `status` is owned by `ManagedRequirement`,
        // reached only via `Need -> UserRequirement -> ManagedRequirement`.
        inherited_names = extend_inherited_names_with_usage_typing(
            &direct_names,
            inherited_names,
            references,
            &outcomes,
            &typing_slots,
        )?;

        // `MemberAccessOperand` (dotted feature-chain access, e.g. `t.bead`) resolves its root
        // segment through the same `DeclarationDomain::Any` lexical lookup every other operand
        // kind uses, then chases each subsequent segment as a member owned (directly or through
        // inheritance) by the *type* of the previously resolved segment, via the just-extended
        // `inherited_names` index -- see `resolve_member_access_reference`'s own doc comment.
        for index in member_access_slots.iter().copied() {
            work.downstream_evaluations = work
                .downstream_evaluations
                .checked_add(1)
                .ok_or(ResolutionError::Capacity)?;
            outcomes[index] = resolve_member_access_reference(
                declarations,
                paths,
                &references[index],
                ResolutionIndexes {
                    direct_names: &direct_names,
                    exported_names: &exported_names,
                    effective_imports: Some(&effective_imports),
                    exported_imports: Some(&exported_imports),
                    inherited_names: Some(&inherited_names),
                },
                ResolutionScratch {
                    ambiguous_candidates: &mut ambiguous_candidates,
                    candidates: &mut candidates,
                    next_candidates: &mut next_candidates,
                    work: &mut work,
                },
            )?;
        }

        for index in subsetting_slots.iter().copied() {
            work.downstream_evaluations = work
                .downstream_evaluations
                .checked_add(1)
                .ok_or(ResolutionError::Capacity)?;
            let reference = &references[index];
            if owner_chain_is_cyclic(declarations, reference.source(), &cyclic_ancestry)? {
                outcomes[index] = ResolutionStatus::NonConverged;
                continue;
            }
            outcomes[index] = resolve_reference(
                declarations,
                paths,
                reference,
                DeclarationDomain::Any,
                ResolutionIndexes {
                    direct_names: &direct_names,
                    exported_names: &exported_names,
                    effective_imports: Some(&effective_imports),
                    exported_imports: Some(&exported_imports),
                    inherited_names: Some(&inherited_names),
                },
                ResolutionScratch {
                    ambiguous_candidates: &mut ambiguous_candidates,
                    candidates: &mut candidates,
                    next_candidates: &mut next_candidates,
                    work: &mut work,
                },
            )?;
        }

        for index in redefinition_slots.iter().copied() {
            work.downstream_evaluations = work
                .downstream_evaluations
                .checked_add(1)
                .ok_or(ResolutionError::Capacity)?;
            let reference = &references[index];
            if owner_chain_is_cyclic(declarations, reference.source(), &cyclic_ancestry)? {
                outcomes[index] = ResolutionStatus::NonConverged;
                continue;
            }
            outcomes[index] = resolve_reference(
                declarations,
                paths,
                reference,
                DeclarationDomain::Any,
                ResolutionIndexes {
                    direct_names: &direct_names,
                    exported_names: &exported_names,
                    effective_imports: Some(&effective_imports),
                    exported_imports: Some(&exported_imports),
                    inherited_names: Some(&inherited_names),
                },
                ResolutionScratch {
                    ambiguous_candidates: &mut ambiguous_candidates,
                    candidates: &mut candidates,
                    next_candidates: &mut next_candidates,
                    work: &mut work,
                },
            )?;
        }
    }
    #[cfg(test)]
    {
        work.effective_index_entries = u64::try_from(effective_imports.candidates.len())
            .map_err(|_| ResolutionError::Capacity)?;
    }

    // The implied-redefinition family reads only settled Subclassification outcomes and the
    // already-built owner-scoped direct-name index above; it is not mutually recursive with the
    // import/typing fixed point above and therefore runs once, after that fixed point settles,
    // rather than joining it as another per-pass family.
    let implied_relationships = if converged {
        let mut implied = synthesize_implied_redefinitions(
            declarations,
            membership_records,
            references,
            &direct_names,
            &outcomes,
        )?
        .into_vec();
        let cyclic_alias_sources =
            detect_cyclic_alias_bindings(declarations, references, &outcomes)?;
        implied.extend(
            synthesize_implied_alias_bindings(
                declarations,
                references,
                &outcomes,
                &cyclic_alias_sources,
            )?
            .into_vec(),
        );
        implied.into_boxed_slice()
    } else {
        Box::default()
    };

    Ok((
        direct_names,
        effective_imports,
        memberships,
        ResolutionResults {
            outcomes: outcomes.into_boxed_slice(),
            ambiguous_candidates: ambiguous_candidates.into_boxed_slice(),
            inherited_names,
            solver_status,
            implied_relationships,
            library_specialization_anchors: LibrarySpecializationAnchorFacts::default(),
            #[cfg(test)]
            work,
        },
    ))
}

/// Synthesizes implied same-name inherited-member redefinition facts.
///
/// Scope: a feature member `f` directly owned by a type `Child`, where `Child` has a resolved
/// `Subclassification` reference to `Parent`, and `Parent` directly (not transitively) owns
/// exactly one feature member also named `f`. This deliberately does not chase multi-level or
/// diamond ancestry: if the immediate parent has zero or more than one directly owned same-name
/// feature candidate, no implied fact is synthesized for that pair rather than guessing. A member
/// that already carries an explicit `:>>` redefinition to any target is left to that authored fact
/// and is never also given an implied one.
fn synthesize_implied_redefinitions<R: ResolutionReferenceFact>(
    declarations: &[Declaration],
    memberships: &[MembershipRecord],
    references: &[R],
    direct_names: &NameIndex,
    outcomes: &[ResolutionStatus],
) -> Result<Box<[ImpliedRelationship]>, ResolutionError> {
    let mut membership_kind: Vec<Option<MembershipKind>> = vec![None; declarations.len()];
    for membership in memberships {
        if let Some(slot) = membership_kind.get_mut(membership.member.index()) {
            *slot = Some(membership.kind);
        }
    }
    let is_feature = |id: DeclarationId| {
        membership_kind.get(id.index()).copied().flatten() == Some(MembershipKind::Feature)
    };

    let mut explicitly_redefines: std::collections::BTreeSet<DeclarationId> = Default::default();
    for reference in references {
        if reference.kind() == ReferenceKind::Redefinition {
            explicitly_redefines.insert(reference.source());
        }
    }

    let mut implied = Vec::new();
    for (index, reference) in references.iter().enumerate() {
        if reference.kind() != ReferenceKind::Subclassification {
            continue;
        }
        let ResolutionStatus::Resolved(parent) = outcomes[index] else {
            continue;
        };
        let child = reference.source();
        for (name, member_candidates) in direct_names.entries_for_owner(Some(child)) {
            for &member in member_candidates {
                if !is_feature(member) || explicitly_redefines.contains(&member) {
                    continue;
                }
                let parent_candidates = direct_names.candidates(Some(parent), name);
                let mut matches = parent_candidates.iter().copied().filter(|c| is_feature(*c));
                let Some(single_match) = matches.next() else {
                    continue;
                };
                if matches.next().is_some() {
                    // Ambiguous immediate-parent candidates: leave unresolved rather than guess.
                    continue;
                }
                implied.push(ImpliedRelationship {
                    kind: ReferenceKind::Redefinition,
                    source: member,
                    target: single_match,
                });
            }
        }
    }
    implied.sort_by_key(|relationship| (relationship.source.0, relationship.target.0));
    implied.dedup();
    Ok(implied.into_boxed_slice())
}

/// Detects `alias` targets that eventually cycle back to their own starting alias declaration
/// (`alias A for B; alias B for A;`). Alias bindings form a functional graph -- each alias
/// declaration has at most one outgoing edge, its own resolved `AliasBinding` target -- so a walk
/// from any alias source bounded by `declarations.len() + 1` hops either terminates at a non-alias
/// target, runs off an unresolved edge, or revisits its own start, which is the only case flagged
/// here. Only alias declarations that themselves author a resolved `AliasBinding` reference are
/// candidates, so the returned set is always a subset of alias declarations.
fn detect_cyclic_alias_bindings<R: ResolutionReferenceFact>(
    declarations: &[Declaration],
    references: &[R],
    outcomes: &[ResolutionStatus],
) -> Result<std::collections::BTreeSet<DeclarationId>, ResolutionError> {
    let mut direct_target: Vec<Option<DeclarationId>> = vec![None; declarations.len()];
    for (index, reference) in references.iter().enumerate() {
        if reference.kind() != ReferenceKind::AliasBinding {
            continue;
        }
        if let ResolutionStatus::Resolved(target) = outcomes[index] {
            if let Some(slot) = direct_target.get_mut(reference.source().index()) {
                *slot = Some(target);
            }
        }
    }
    let pass_limit = declarations
        .len()
        .checked_add(1)
        .ok_or(ResolutionError::Capacity)?;
    let mut cyclic = std::collections::BTreeSet::new();
    for index in 0..declarations.len() {
        let Some(mut current) = direct_target[index] else {
            continue;
        };
        let start = DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
        let mut steps = 0usize;
        loop {
            if current == start {
                cyclic.insert(start);
                break;
            }
            let Some(next) = direct_target.get(current.index()).copied().flatten() else {
                break;
            };
            current = next;
            steps = steps.checked_add(1).ok_or(ResolutionError::Capacity)?;
            if steps > pass_limit {
                // Defensive only: a functional graph over a bounded declaration count cannot
                // require more hops than there are declarations without having already revisited
                // `start` above.
                cyclic.insert(start);
                break;
            }
        }
    }
    Ok(cyclic)
}

/// Synthesizes implied "typing/specialization/... through an alias" relationship facts: when an
/// authored reference (for example a `FeatureTyping` on `device : DeviceAlias`) resolves to an
/// alias declaration, this follows that alias's own resolved `AliasBinding` chain -- transitively,
/// through alias-of-alias -- to the ultimate non-alias target and publishes an `implied` (per
/// provenance) relationship of the *same* reference kind
/// straight from the original source to that ultimate target. This makes aliasing "transparent"
/// for downstream typing without weakening or replacing the alias's own authored `AliasBinding`
/// fact, which remains published as its own (authored-provenance) reference/relationship. A cycle
/// in the alias chain (already reported via `detect_cyclic_alias_bindings`) or an unresolved link
/// simply yields no implied fact for that source, rather than guessing.
fn synthesize_implied_alias_bindings<R: ResolutionReferenceFact>(
    declarations: &[Declaration],
    references: &[R],
    outcomes: &[ResolutionStatus],
    cyclic_alias_sources: &std::collections::BTreeSet<DeclarationId>,
) -> Result<Box<[ImpliedRelationship]>, ResolutionError> {
    let mut alias_target: std::collections::BTreeMap<DeclarationId, DeclarationId> =
        Default::default();
    for (index, reference) in references.iter().enumerate() {
        if reference.kind() != ReferenceKind::AliasBinding {
            continue;
        }
        if cyclic_alias_sources.contains(&reference.source()) {
            continue;
        }
        if let ResolutionStatus::Resolved(target) = outcomes[index] {
            alias_target.insert(reference.source(), target);
        }
    }
    let is_alias = |id: DeclarationId| {
        declarations
            .get(id.index())
            .is_some_and(|declaration| declaration.kind == DeclarationKind::Alias)
    };

    let mut implied = Vec::new();
    for (index, reference) in references.iter().enumerate() {
        if reference.kind() == ReferenceKind::AliasBinding {
            continue;
        }
        let ResolutionStatus::Resolved(mut current) = outcomes[index] else {
            continue;
        };
        if !is_alias(current) {
            continue;
        }
        let mut visited = std::collections::BTreeSet::new();
        let mut ultimate = None;
        loop {
            if !visited.insert(current) {
                // Cyclic alias chain: leave unresolved rather than guess.
                ultimate = None;
                break;
            }
            match alias_target.get(&current) {
                Some(&next) if is_alias(next) => current = next,
                Some(&next) => {
                    ultimate = Some(next);
                    break;
                }
                None => break,
            }
        }
        if let Some(target) = ultimate {
            implied.push(ImpliedRelationship {
                kind: reference.kind(),
                source: reference.source(),
                target,
            });
        }
    }
    implied.sort_by_key(|relationship| (relationship.source.0, relationship.target.0));
    implied.dedup();
    Ok(implied.into_boxed_slice())
}

fn library_specialization_anchors(
    storage: &SemanticModelStorage,
) -> LibrarySpecializationAnchorFacts {
    let anchors = GENERATED_LIBRARY_SPECIALIZATION_RULES
        .iter()
        .map(|rule| {
            (
                rule.rule_id,
                LibrarySpecializationAnchorBranch::Default,
                rule.anchor,
            )
        })
        .chain(
            GENERATED_CONDITIONAL_LIBRARY_SPECIALIZATION_RULES
                .iter()
                .flat_map(|rule| {
                    std::iter::once((
                        rule.rule_id,
                        LibrarySpecializationAnchorBranch::Default,
                        rule.anchor,
                    ))
                    .chain(rule.true_anchor.map(|anchor| {
                        (
                            rule.rule_id,
                            LibrarySpecializationAnchorBranch::PredicateTrue,
                            anchor,
                        )
                    }))
                }),
        )
        .chain(GENERATED_LIBRARY_REDEFINITION_RULES.iter().map(|rule| {
            (
                rule.rule_id,
                LibrarySpecializationAnchorBranch::Default,
                rule.anchor,
            )
        }))
        .map(|(rule_id, branch, anchor)| {
            (
                LibrarySpecializationAnchorKey {
                    rule: LibrarySpecializationRuleKey(rule_id),
                    branch,
                },
                resolve_library_specialization_anchor(storage, anchor),
            )
        })
        .collect();
    LibrarySpecializationAnchorFacts { by_rule: anchors }
}

fn resolve_library_specialization_anchor(
    storage: &SemanticModelStorage,
    anchor: &'static str,
) -> LibrarySpecializationAnchor {
    let parts = anchor.split("::").collect::<Vec<_>>();
    let Some((&last, owners)) = parts.split_last() else {
        return LibrarySpecializationAnchor::Missing;
    };
    let mut candidates = storage
        .declarations
        .iter()
        .enumerate()
        .filter_map(|(index, declaration)| {
            (storage
                .document(declaration.document)
                .is_some_and(|document| document.role == SourceRole::StandardLibrary)
                && declaration
                    .name
                    .is_some_and(|name| storage.symbol(name) == Some(last))
                && anchor_owner_path_matches(storage, declaration.owner, owners))
            .then(|| DeclarationId::from_index(index).ok())
            .flatten()
        })
        .collect::<Vec<_>>();
    candidates.sort_unstable();
    candidates.dedup();
    match candidates.len() {
        0 => LibrarySpecializationAnchor::Missing,
        1 => LibrarySpecializationAnchor::Resolved(candidates[0]),
        _ => LibrarySpecializationAnchor::Ambiguous(candidates.into_boxed_slice()),
    }
}

/// Checks the structural containment path of a normative anchor, outermost first.
///
/// This deliberately stops at the source root and therefore cannot treat an arbitrary nested
/// package named `Parts` as the language-owned library namespace.
fn anchor_owner_path_matches(
    storage: &SemanticModelStorage,
    owner: Option<DeclarationId>,
    expected: &[&str],
) -> bool {
    let mut cursor = owner;
    for name in expected.iter().rev() {
        let Some(current) = cursor else {
            return false;
        };
        let Some(declaration) = storage.declaration(current) else {
            return false;
        };
        if !declaration
            .name
            .is_some_and(|id| storage.symbol(id) == Some(*name))
        {
            return false;
        }
        cursor = declaration.owner;
    }
    cursor.is_none()
}

/// Applies every exact unconditional manifest rule through the typed declaration-kind projection.
/// The generated table is the authority for both applicability metaclass and library anchor.
fn synthesize_generated_library_specializations(
    storage: &SemanticModelStorage,
    references: &[AuthoredReference],
    outcomes: &[ResolutionStatus],
    anchor_facts: &LibrarySpecializationAnchorFacts,
) -> Result<Box<[ImpliedRelationship]>, ResolutionError> {
    let (ancestors, cyclic) = build_ancestor_closures(&storage.declarations, references, outcomes)?;
    let mut implied = Vec::new();
    for (index, declaration) in storage.declarations.iter().enumerate() {
        let source = DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
        for metaclass in std::iter::once(library_rule_metaclass(declaration.kind))
            .chain((declaration.kind == DeclarationKind::Flow).then_some("Flow"))
        {
            for rule in library_specialization_rules(metaclass) {
                let Some(LibrarySpecializationAnchor::Resolved(anchor)) =
                    anchor_facts.outcome(rule.rule_id)
                else {
                    continue;
                };
                if source == *anchor
                    || cyclic.contains(&source)
                    || ancestors
                        .get(source.index())
                        .is_some_and(|set| set.contains(anchor))
                {
                    continue;
                }
                implied.push(ImpliedRelationship {
                    kind: ReferenceKind::Subclassification,
                    source,
                    target: *anchor,
                });
            }
            for rule in conditional_library_specialization_rules(metaclass) {
                if !conditional_library_specialization_predicate_holds_with_resolution(
                    storage, source, rule, references, outcomes,
                ) {
                    continue;
                }
                let branch =
                    conditional_library_specialization_anchor_branch(storage, source, rule);
                let Some(LibrarySpecializationAnchor::Resolved(anchor)) =
                    anchor_facts.outcome_for(rule.rule_id, branch)
                else {
                    continue;
                };
                if source == *anchor
                    || cyclic.contains(&source)
                    || ancestors
                        .get(source.index())
                        .is_some_and(|set| set.contains(anchor))
                {
                    continue;
                }
                implied.push(ImpliedRelationship {
                    kind: ReferenceKind::Subclassification,
                    source,
                    target: *anchor,
                });
            }
        }
    }
    implied.sort_by_key(|relationship| (relationship.source.0, relationship.target.0));
    implied.dedup();
    Ok(implied.into_boxed_slice())
}

/// Evaluates only the exact predicate vocabulary emitted by the pinned-manifest extractor.
/// The predicate is the manifest's closed enum, so adding a contract requires an exhaustive
/// resolver decision rather than falling through a similarly spelled string label.
fn conditional_library_specialization_predicate_holds(
    storage: &SemanticModelStorage,
    source: DeclarationId,
    rule: &ConditionalLibrarySpecializationRule,
) -> bool {
    let Some(declaration) = storage.declaration(source) else {
        return false;
    };
    let Some(facts) = storage.declaration_facts(source) else {
        return false;
    };
    match rule.predicate {
        LibrarySpecializationPredicate::IsIndividual => {
            declaration.kind == DeclarationKind::OccurrenceDefinition && facts.modifiers.individual
        }
        LibrarySpecializationPredicate::PortionKindSnapshot => {
            declaration.kind == DeclarationKind::OccurrenceUsage
                && facts.portion_kind == Some(PortionKind::Snapshot)
        }
        LibrarySpecializationPredicate::PortionKindTimeslice => {
            declaration.kind == DeclarationKind::OccurrenceUsage
                && facts.portion_kind == Some(PortionKind::Timeslice)
        }
        LibrarySpecializationPredicate::CompositeOwnedBy => {
            facts.modifiers.composite
                && declaration.owner.is_some_and(|owner| {
                    storage.declaration(owner).is_some_and(|owner| {
                        rule.owner_metaclasses
                            .contains(&library_rule_metaclass(owner.kind))
                    })
                })
        }
        LibrarySpecializationPredicate::OwnedBy => declaration.owner.is_some_and(|owner| {
            storage.declaration(owner).is_some_and(|owner| {
                rule.owner_metaclasses
                    .contains(&library_rule_metaclass(owner.kind))
            })
        }),
        LibrarySpecializationPredicate::IsSubactionUsage => {
            facts.modifiers.composite
                && declaration.owner.is_some_and(|owner| {
                    storage.declaration(owner).is_some_and(|owner| {
                        matches!(
                            owner.kind,
                            DeclarationKind::ActionDefinition
                                | DeclarationKind::ActionUsage
                                | DeclarationKind::AcceptActionUsage
                        )
                    })
                })
        }
        LibrarySpecializationPredicate::IsNotTriggerAction => {
            declaration.kind == DeclarationKind::AcceptActionUsage
                && facts.is_trigger_action == Some(false)
        }
        LibrarySpecializationPredicate::IsSubactionUsageAndNotTriggerAction => {
            declaration.kind == DeclarationKind::AcceptActionUsage
                && facts.is_trigger_action == Some(false)
                && facts.modifiers.composite
                && declaration.owner.is_some_and(|owner| {
                    storage.declaration(owner).is_some_and(|owner| {
                        matches!(
                            owner.kind,
                            DeclarationKind::ActionDefinition
                                | DeclarationKind::ActionUsage
                                | DeclarationKind::AcceptActionUsage
                        )
                    })
                })
        }
        LibrarySpecializationPredicate::IsTriggerAction => {
            declaration.kind == DeclarationKind::AcceptActionUsage
                && facts.is_trigger_action == Some(true)
        }
        LibrarySpecializationPredicate::HasElseActionBranch => {
            declaration.kind == DeclarationKind::If && facts.has_else_action.is_some()
        }
        LibrarySpecializationPredicate::OwnedEndFeatureCountIsTwo
        | LibrarySpecializationPredicate::ConnectorEndCountIsTwo
        | LibrarySpecializationPredicate::AssociationEndCountIsTwo
        | LibrarySpecializationPredicate::EndFeatureCountIsTwo => {
            positional_end_count(storage, source) == 2
        }
        LibrarySpecializationPredicate::FlowEndCountIsTwo => {
            declaration.kind == DeclarationKind::FlowDefinition
                && positional_end_count(storage, source) == 2
        }
        LibrarySpecializationPredicate::OwnedEndFeaturesNotEmpty => {
            declaration.kind == DeclarationKind::Flow
                && facts.owned_end_feature_count.is_some_and(|count| count > 0)
        }
        LibrarySpecializationPredicate::OwnedTypingDataType
        | LibrarySpecializationPredicate::EndOwnedByAssociationOrConnector
        | LibrarySpecializationPredicate::ConnectorAssociationStructure => false,
        LibrarySpecializationPredicate::PolarityBranch => facts.negated.is_some(),
        LibrarySpecializationPredicate::FramedConcernMembership => {
            element_kind::membership_role(declaration.kind) == Some(MembershipRole::FramedConcern)
        }
        LibrarySpecializationPredicate::RequirementConstraintMembershipKind => matches!(
            element_kind::membership_role(declaration.kind),
            Some(MembershipRole::RequirementConstraint(
                RequirementConstraintKind::Assumption | RequirementConstraintKind::Requirement
            ))
        ),
        LibrarySpecializationPredicate::ActorMembershipOwningRequirement => {
            element_kind::membership_role(declaration.kind) == Some(MembershipRole::Actor)
        }
        LibrarySpecializationPredicate::StakeholderMembership => {
            element_kind::membership_role(declaration.kind) == Some(MembershipRole::Stakeholder)
        }
        LibrarySpecializationPredicate::RequirementVerificationMembership => {
            element_kind::membership_role(declaration.kind)
                == Some(MembershipRole::RequirementVerification)
        }
    }
}

/// Evaluates the exact predicates whose prerequisite is a direct, already-settled
/// `FeatureTyping` relationship. The relationship and its target are canonical authored and
/// resolved facts; this owner never rereads syntax or derives a type from a display name.
fn conditional_library_specialization_predicate_holds_with_resolution(
    storage: &SemanticModelStorage,
    source: DeclarationId,
    rule: &ConditionalLibrarySpecializationRule,
    references: &[AuthoredReference],
    outcomes: &[ResolutionStatus],
) -> bool {
    match rule.predicate {
        LibrarySpecializationPredicate::OwnedTypingDataType => {
            direct_owned_typing_targets(storage, source, references, outcomes).any(|target| {
                storage
                    .declaration(target)
                    .is_some_and(|declaration| declaration.kind == DeclarationKind::KermlDataType)
            })
        }
        LibrarySpecializationPredicate::EndOwnedByAssociationOrConnector => storage
            .declaration(source)
            .zip(storage.declaration_facts(source))
            .is_some_and(|(declaration, facts)| {
                facts.modifiers.end
                    && declaration.owner.is_some_and(|owner| {
                        storage.declaration(owner).is_some_and(|owner| {
                            matches!(
                                owner.kind,
                                DeclarationKind::KermlAssociation | DeclarationKind::KermlConnector
                            )
                        })
                    })
            }),
        LibrarySpecializationPredicate::ConnectorAssociationStructure => {
            declaration_has_direct_association_structure_typing(
                storage, source, references, outcomes,
            )
        }
        _ => conditional_library_specialization_predicate_holds(storage, source, rule),
    }
}

/// `Connector::association` is a derived subset of the connector's `type` collection constrained
/// to `Association`. The canonical direct `FeatureTyping` relationship already owns that source
/// collection; filtering its settled target by the exact `AssociationStructure` metaclass here
/// gives the two closed Connector predicates their sole semantic input without inspecting syntax.
fn declaration_has_direct_association_structure_typing(
    storage: &SemanticModelStorage,
    source: DeclarationId,
    references: &[AuthoredReference],
    outcomes: &[ResolutionStatus],
) -> bool {
    storage
        .declaration(source)
        .is_some_and(|declaration| declaration.kind == DeclarationKind::KermlConnector)
        && direct_owned_typing_targets(storage, source, references, outcomes).any(|target| {
            storage.declaration(target).is_some_and(|declaration| {
                declaration.kind == DeclarationKind::KermlAssociationStructure
            })
        })
}

fn direct_owned_typing_targets<'a>(
    storage: &'a SemanticModelStorage,
    source: DeclarationId,
    references: &'a [AuthoredReference],
    outcomes: &'a [ResolutionStatus],
) -> impl Iterator<Item = DeclarationId> + 'a {
    references
        .iter()
        .enumerate()
        .filter(move |(_, reference)| {
            reference.source == source && reference.kind == ReferenceKind::FeatureTyping
        })
        .filter_map(move |(index, _)| match outcomes.get(index) {
            Some(ResolutionStatus::Resolved(target)) => Some(*target),
            _ => None,
        })
        .filter(move |target| storage.declaration(*target).is_some())
}

/// Selects the already-published branch of a closed conditional contract. The `then` branch is a
/// typed key shared by exact polarity and membership-role contracts; every other predicate retains
/// the compatibility default anchor. A missing canonical fact was rejected by predicate
/// evaluation above, rather than silently selecting a branch.
fn conditional_library_specialization_anchor_branch(
    storage: &SemanticModelStorage,
    source: DeclarationId,
    rule: &ConditionalLibrarySpecializationRule,
) -> LibrarySpecializationAnchorBranch {
    let predicate_true = match rule.predicate {
        LibrarySpecializationPredicate::PolarityBranch => storage
            .declaration_facts(source)
            .is_some_and(|facts| facts.negated == Some(true)),
        LibrarySpecializationPredicate::HasElseActionBranch => storage
            .declaration_facts(source)
            .is_some_and(|facts| facts.has_else_action == Some(true)),
        LibrarySpecializationPredicate::RequirementConstraintMembershipKind => {
            storage.declaration(source).is_some_and(|declaration| {
                element_kind::membership_role(declaration.kind)
                    == Some(MembershipRole::RequirementConstraint(
                        RequirementConstraintKind::Assumption,
                    ))
            })
        }
        LibrarySpecializationPredicate::ActorMembershipOwningRequirement => storage
            .declaration(source)
            .and_then(|declaration| declaration.owner)
            .and_then(|owner| storage.declaration(owner))
            .is_some_and(|owner| {
                matches!(
                    owner.kind,
                    DeclarationKind::RequirementDefinition | DeclarationKind::RequirementUsage
                )
            }),
        LibrarySpecializationPredicate::IsIndividual
        | LibrarySpecializationPredicate::PortionKindSnapshot
        | LibrarySpecializationPredicate::PortionKindTimeslice
        | LibrarySpecializationPredicate::CompositeOwnedBy
        | LibrarySpecializationPredicate::OwnedEndFeatureCountIsTwo
        | LibrarySpecializationPredicate::ConnectorEndCountIsTwo
        | LibrarySpecializationPredicate::AssociationEndCountIsTwo
        | LibrarySpecializationPredicate::EndFeatureCountIsTwo
        | LibrarySpecializationPredicate::FlowEndCountIsTwo
        | LibrarySpecializationPredicate::OwnedEndFeaturesNotEmpty
        | LibrarySpecializationPredicate::OwnedTypingDataType
        | LibrarySpecializationPredicate::EndOwnedByAssociationOrConnector
        | LibrarySpecializationPredicate::ConnectorAssociationStructure
        | LibrarySpecializationPredicate::OwnedBy
        | LibrarySpecializationPredicate::IsSubactionUsage
        | LibrarySpecializationPredicate::IsNotTriggerAction
        | LibrarySpecializationPredicate::IsSubactionUsageAndNotTriggerAction
        | LibrarySpecializationPredicate::IsTriggerAction
        | LibrarySpecializationPredicate::FramedConcernMembership
        | LibrarySpecializationPredicate::StakeholderMembership
        | LibrarySpecializationPredicate::RequirementVerificationMembership => false,
    };
    if predicate_true {
        LibrarySpecializationAnchorBranch::PredicateTrue
    } else {
        LibrarySpecializationAnchorBranch::Default
    }
}

/// The canonical structural representation for the exact XMI end collections is each child
/// declaration's owned positional-end fact. The generated rule's metaclass and closed predicate
/// still distinguish `connectorEnd`, `associationEnd`, `endFeature`, and `ownedEndFeature`; this
/// helper only owns their shared storage projection.
fn positional_end_count(storage: &SemanticModelStorage, owner: DeclarationId) -> usize {
    storage
        .declarations
        .iter()
        .enumerate()
        .filter(|(index, declaration)| {
            declaration.owner == Some(owner)
                && DeclarationId::from_index(*index)
                    .ok()
                    .and_then(|member| storage.declaration_facts(member))
                    .is_some_and(|facts| facts.positional_end.is_some())
        })
        .count()
}

/// Applies exact unconditional `redefinesFromLibrary` rules after authored references have
/// settled. An authored redefinition always suppresses the generated edge for that source, so an
/// implied fact never disguises or competes with source intent. Rules whose metaclass has no
/// lowered declaration projection produce no edge; their canonical anchor remains queryable by
/// stable rule ID rather than being guessed from a similarly named declaration.
fn synthesize_generated_library_redefinitions(
    storage: &SemanticModelStorage,
    references: &[AuthoredReference],
    anchor_facts: &LibrarySpecializationAnchorFacts,
) -> Result<Box<[ImpliedRelationship]>, ResolutionError> {
    let authored_sources: std::collections::BTreeSet<_> = references
        .iter()
        .filter(|reference| reference.kind == ReferenceKind::Redefinition)
        .map(|reference| reference.source)
        .collect();
    let mut implied = Vec::new();
    for (index, declaration) in storage.declarations.iter().enumerate() {
        let source = DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
        if authored_sources.contains(&source) {
            continue;
        }
        for metaclass in std::iter::once(library_rule_metaclass(declaration.kind))
            .chain((declaration.kind == DeclarationKind::Flow).then_some("Flow"))
        {
            for rule in library_redefinition_rules(metaclass) {
                let Some(LibrarySpecializationAnchor::Resolved(anchor)) =
                    anchor_facts.outcome(rule.rule_id)
                else {
                    continue;
                };
                if source != *anchor {
                    implied.push(ImpliedRelationship {
                        kind: ReferenceKind::Redefinition,
                        source,
                        target: *anchor,
                    });
                }
            }
        }
    }
    implied.sort_by_key(|relationship| (relationship.source.0, relationship.target.0));
    implied.dedup();
    Ok(implied.into_boxed_slice())
}

/// Materializes KerML's `checkFeatureFeatureMembershipTypeFeaturing` semantic consequence.
///
/// A non-`var` Feature owned through a FeatureMembership is featured by that membership's owning
/// Type. This runs after reference convergence and writes the same canonical relationship store as
/// authored `featured by` clauses. Any authored TypeFeaturing for the source suppresses the
/// boilerplate edge, preserving explicit source intent and avoiding a second competing fact.
///
/// `var` Features have a different normative target (the owning type's `snapshots` feature). That
/// prerequisite is not lowered yet, so this function publishes no guessed edge; the type-featuring
/// query reports the corresponding explicit unsupported outcome.
fn synthesize_feature_membership_type_featurings(
    storage: &SemanticModelStorage,
    references: &[AuthoredReference],
) -> Result<Box<[ImpliedRelationship]>, ResolutionError> {
    let authored_sources = references
        .iter()
        .filter(|reference| reference.kind == ReferenceKind::TypeFeaturing)
        .map(|reference| reference.source)
        .collect::<std::collections::BTreeSet<_>>();
    let mut implied = Vec::new();
    for membership in storage.memberships.iter() {
        if membership.kind != MembershipKind::Feature
            || authored_sources.contains(&membership.member)
        {
            continue;
        }
        let Some(feature) = storage.declaration(membership.member) else {
            return Err(ResolutionError::InvalidStorage);
        };
        if storage
            .declaration_facts(membership.member)
            .is_some_and(|facts| facts.modifiers.var)
        {
            continue;
        }
        let Some(owner) = feature.owner else {
            continue;
        };
        let Some(owner_declaration) = storage.declaration(owner) else {
            return Err(ResolutionError::InvalidStorage);
        };
        if matches!(
            owner_declaration.kind,
            DeclarationKind::Namespace
                | DeclarationKind::Package
                | DeclarationKind::LibraryPackage
                | DeclarationKind::Import
                | DeclarationKind::Alias
        ) {
            continue;
        }
        implied.push(ImpliedRelationship {
            kind: ReferenceKind::TypeFeaturing,
            source: membership.member,
            target: owner,
        });
    }
    implied.sort_by_key(|relationship| (relationship.source.0, relationship.target.0));
    implied.dedup();
    Ok(implied.into_boxed_slice())
}

/// The generated schema-v3 metaclass spelling for a lowered declaration.
///
/// The public kind is the canonical projection for ordinary declarations. Two source forms retain
/// a more descriptive public spelling (`FlowConnection*` and `Calculation*`) while the XMI uses
/// the shorter abstract-syntax names; normalize those at this semantic boundary once rather than
/// letting generated-rule consumers maintain aliases.
fn library_rule_metaclass(kind: DeclarationKind) -> &'static str {
    match kind {
        DeclarationKind::FlowDefinition => "FlowDefinition",
        DeclarationKind::Flow => "FlowUsage",
        DeclarationKind::CalcDefinition => "CalculationDefinition",
        DeclarationKind::CalcUsage => "CalculationUsage",
        _ => element_kind::element_kind(kind).as_str(),
    }
}

/// Compatibility spelling for specialization-only consumers. New generated-rule owners use
/// `library_rule_metaclass`, which is the single normalization boundary for both exact families.
fn library_specialization_metaclass(kind: DeclarationKind) -> &'static str {
    library_rule_metaclass(kind)
}

/// Computes, for every declaration, the transitive set of Subclassification ancestors reached
/// through resolved Subclassification outcomes, as a bounded fixed point over the immutable
/// previous-pass state (mirroring the effective-import fixed point above): each pass reads the
/// prior complete closure array and writes a fresh next array, swapped only at the pass barrier.
///
/// A diamond (`Diamond :> Left, Right` where both specialize `Base`) naturally dedups to a single
/// `Base` entry because each declaration's closure is a set. A specialization cycle is detected
/// explicitly: if a declaration's own closure would come to include itself, that declaration is
/// reported as cyclic instead of being handed an ever-growing or self-referential ancestor list.
/// Because each pass only ever unions in previously-discovered ancestors, the closure array is
/// bounded by the total declaration count and this loop is guaranteed to reach a fixed point well
/// inside `declarations.len() + 1` passes even in the presence of a cycle; it never spins forever.
type AncestorClosures = (
    Vec<Box<[DeclarationId]>>,
    std::collections::BTreeSet<DeclarationId>,
);

fn build_ancestor_closures<R: ResolutionReferenceFact>(
    declarations: &[Declaration],
    references: &[R],
    outcomes: &[ResolutionStatus],
) -> Result<AncestorClosures, ResolutionError> {
    let mut direct_parents: Vec<std::collections::BTreeSet<DeclarationId>> =
        vec![Default::default(); declarations.len()];
    for (index, reference) in references.iter().enumerate() {
        if reference.kind() != ReferenceKind::Subclassification {
            continue;
        }
        if let ResolutionStatus::Resolved(target) = outcomes[index] {
            let slot = direct_parents
                .get_mut(reference.source().index())
                .ok_or(ResolutionError::InvalidStorage)?;
            slot.insert(target);
        }
    }

    let mut closure = direct_parents.clone();
    let pass_limit = declarations
        .len()
        .checked_add(1)
        .ok_or(ResolutionError::Capacity)?;
    for _ in 0..pass_limit {
        let mut next = closure.clone();
        let mut changed = false;
        for (index, parents) in direct_parents.iter().enumerate() {
            for parent in parents {
                for ancestor in
                    std::iter::once(*parent).chain(closure[parent.index()].iter().copied())
                {
                    if next[index].insert(ancestor) {
                        changed = true;
                    }
                }
            }
        }
        closure = next;
        if !changed {
            break;
        }
    }

    let mut cyclic = std::collections::BTreeSet::new();
    let mut closures = Vec::with_capacity(declarations.len());
    for (index, ancestors) in closure.into_iter().enumerate() {
        let id = DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
        if ancestors.contains(&id) {
            cyclic.insert(id);
            closures.push(Box::default());
        } else {
            closures.push(ancestors.into_iter().collect::<Vec<_>>().into_boxed_slice());
        }
    }
    Ok((closures, cyclic))
}

/// Builds the ancestor-scoped inherited-member lookup index: for each non-cyclic declaration with
/// a non-empty ancestor closure, every name directly owned by any ancestor becomes a candidate for
/// that declaration. `NameIndex::build` sorts and dedups `(owner, name, candidate)` triples, so a
/// member reached through two different ancestor paths to the same target (the diamond case)
/// collapses to one candidate, while two different ancestors that directly own two different
/// same-named members remain two distinct candidates and therefore resolve as ambiguous.
fn build_inherited_name_index(
    direct_names: &NameIndex,
    ancestor_closures: &[Box<[DeclarationId]>],
) -> Result<NameIndex, ResolutionError> {
    let mut entries = Vec::new();
    for (index, ancestors) in ancestor_closures.iter().enumerate() {
        if ancestors.is_empty() {
            continue;
        }
        let child = DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
        for &ancestor in ancestors.iter() {
            for (name, candidates) in direct_names.entries_for_owner(Some(ancestor)) {
                for &candidate in candidates {
                    entries.push((
                        NameKey {
                            owner: Some(child),
                            name,
                        },
                        candidate,
                    ));
                }
            }
        }
    }
    NameIndex::build(entries)
}

/// Extends the Subclassification-derived `inherited_names` index (`build_inherited_name_index`)
/// with entries reachable only by first following a usage's own settled `FeatureTyping` reference
/// to its type: for every declaration `usage` whose `FeatureTyping` reference resolved to `target`,
/// every name directly owned by `target` and every name already reachable in `target`'s own
/// ancestor-scoped `inherited_names` entry becomes a candidate for `(usage, name)` here too. This
/// lets an explicit `:>>`/`:>` reference owned by a plain usage (which has no Subclassification
/// ancestors of its own) reach a member owned by an ancestor of the usage's *type* -- for example
/// `need : Need { attribute :>> status = ...; }` reaching `ManagedRequirement::status` through
/// `Need`'s own ancestor closure. Must run after `typing_slots` has resolved, since it reads their
/// settled outcomes; the original entries are preserved unchanged, so plain type-owned
/// Subclassification lookups (Subsetting/Redefinition owned directly by a def) are unaffected.
fn extend_inherited_names_with_usage_typing<R: ResolutionReferenceFact>(
    direct_names: &NameIndex,
    inherited_names: NameIndex,
    references: &[R],
    outcomes: &[ResolutionStatus],
    typing_slots: &[usize],
) -> Result<NameIndex, ResolutionError> {
    let mut entries: Vec<(NameKey, DeclarationId)> = Vec::new();
    for &index in typing_slots {
        let ResolutionStatus::Resolved(target) = outcomes[index] else {
            continue;
        };
        let usage = references[index].source();
        for (name, candidates) in direct_names.entries_for_owner(Some(target)) {
            for &candidate in candidates {
                entries.push((
                    NameKey {
                        owner: Some(usage),
                        name,
                    },
                    candidate,
                ));
            }
        }
        for (name, candidates) in inherited_names.entries_for_owner(Some(target)) {
            for &candidate in candidates {
                entries.push((
                    NameKey {
                        owner: Some(usage),
                        name,
                    },
                    candidate,
                ));
            }
        }
    }
    if entries.is_empty() {
        return Ok(inherited_names);
    }
    for (index, keys) in inherited_names.keys.iter().enumerate() {
        for &candidate in inherited_names.ranges[index]
            .slice(&inherited_names.candidates)
            .unwrap_or_default()
        {
            entries.push((*keys, candidate));
        }
    }
    NameIndex::build(entries)
}

/// True when `source`'s owning namespace chain passes through a declaration whose Subclassification
/// ancestry was found to be cyclic. A FeatureTyping reference owned (directly or via an enclosing
/// scope) by such a declaration cannot have its inherited scope computed and is published as an
/// explicit `NonConverged` outcome rather than silently falling back to local/import-only lookup or
/// looping.
fn owner_chain_is_cyclic(
    declarations: &[Declaration],
    source: DeclarationId,
    cyclic_ancestry: &std::collections::BTreeSet<DeclarationId>,
) -> Result<bool, ResolutionError> {
    let mut owner = declarations
        .get(source.index())
        .ok_or(ResolutionError::InvalidStorage)?
        .owner;
    while let Some(current) = owner {
        if cyclic_ancestry.contains(&current) {
            return Ok(true);
        }
        owner = declarations
            .get(current.index())
            .ok_or(ResolutionError::InvalidStorage)?
            .owner;
    }
    Ok(false)
}

fn supported_import_domain(reference: &impl ResolutionReferenceFact) -> Option<DeclarationDomain> {
    match reference.kind() {
        // A view's `expose` names any member, so it resolves through the same lookup an ordinary
        // reference does rather than the import domains.
        ReferenceKind::ViewExpose => Some(DeclarationDomain::Any),
        ReferenceKind::NamespaceImport
            if reference.flags().wildcard && !reference.flags().recursive =>
        {
            Some(DeclarationDomain::Namespace)
        }
        ReferenceKind::MembershipImport
            if !reference.flags().wildcard && !reference.flags().recursive =>
        {
            Some(DeclarationDomain::Any)
        }
        ReferenceKind::NamespaceImport
        | ReferenceKind::MembershipImport
        | ReferenceKind::FilterImport
        | ReferenceKind::FeatureTyping
        | ReferenceKind::TypeFeaturing
        | ReferenceKind::FeatureChaining
        | ReferenceKind::Subclassification
        | ReferenceKind::Subsetting
        | ReferenceKind::Redefinition
        | ReferenceKind::References
        | ReferenceKind::Crosses
        | ReferenceKind::Intersects
        | ReferenceKind::Unioning
        | ReferenceKind::Intersecting
        | ReferenceKind::Differencing
        | ReferenceKind::Disjoining
        | ReferenceKind::AliasBinding
        | ReferenceKind::ConnectorEnd
        | ReferenceKind::Succession
        | ReferenceKind::EntryActionBinding
        | ReferenceKind::DoActionBinding
        | ReferenceKind::ExitActionBinding
        | ReferenceKind::InitialState
        | ReferenceKind::ExpressionOperand
        | ReferenceKind::TransitionSource
        | ReferenceKind::TransitionTarget
        | ReferenceKind::TransitionTrigger
        | ReferenceKind::TransitionEffect
        | ReferenceKind::MetadataAnnotation
        | ReferenceKind::FilterMetadataTest
        | ReferenceKind::SatisfySource
        | ReferenceKind::SatisfyTarget
        | ReferenceKind::AllocateSource
        | ReferenceKind::AllocateTarget
        | ReferenceKind::BindSource
        | ReferenceKind::BindTarget
        | ReferenceKind::Variant
        | ReferenceKind::IncludeUseCase
        | ReferenceKind::MemberAccessOperand
        | ReferenceKind::InvocationCallee
        | ReferenceKind::ThenTarget
        | ReferenceKind::AcceptVia
        | ReferenceKind::SendTarget
        | ReferenceKind::AcceptPayloadType
        | ReferenceKind::TerminateTarget
        | ReferenceKind::FlowSource
        | ReferenceKind::FlowTarget
        | ReferenceKind::TypeCheckTarget
        | ReferenceKind::MetaCastTarget
        | ReferenceKind::StakeholderTarget
        | ReferenceKind::PurposeTarget
        | ReferenceKind::VerifyRequirementTarget
        | ReferenceKind::AssignTarget
        | ReferenceKind::DependencyClient
        | ReferenceKind::DependencySupplier
        | ReferenceKind::PerformParameterTarget
        | ReferenceKind::FlowPayloadType => None,
    }
}

/// Exact Definition/Usage metaclass applicability over the lowering's canonical kind fact.
///
/// These are deliberately not display-name checks. A new lowered declaration kind makes this
/// policy reviewable here rather than silently appearing in a broad derived collection.
fn definition_usage_source_matches(metaclass: &str, kind: DeclarationKind) -> bool {
    match metaclass {
        "Definition" => matches!(
            kind,
            DeclarationKind::PartDefinition
                | DeclarationKind::AttributeDefinition
                | DeclarationKind::EnumerationDefinition
                | DeclarationKind::RequirementDefinition
                | DeclarationKind::PortDefinition
                | DeclarationKind::ItemDefinition
                | DeclarationKind::ActionDefinition
                | DeclarationKind::StateDefinition
                | DeclarationKind::MetadataDefinition
                | DeclarationKind::ConnectionDefinition
                | DeclarationKind::OccurrenceDefinition
                | DeclarationKind::AnalysisCaseDefinition
                | DeclarationKind::InterfaceDefinition
                | DeclarationKind::ViewDefinition
                | DeclarationKind::CaseDefinition
                | DeclarationKind::VerificationCaseDefinition
                | DeclarationKind::UseCaseDefinition
                | DeclarationKind::ViewpointDefinition
                | DeclarationKind::RenderingDefinition
                | DeclarationKind::AllocationDefinition
                | DeclarationKind::FlowDefinition
                | DeclarationKind::ConstraintDefinition
                | DeclarationKind::ConcernDefinition
                | DeclarationKind::CalcDefinition
                | DeclarationKind::ClassDefinition
                | DeclarationKind::ExtendedDefinition
                | DeclarationKind::IndividualDefinition
        ),
        "Usage" => is_usage_declaration(kind),
        _ => false,
    }
}

fn requirement_derived_source_matches(metaclass: &str, kind: DeclarationKind) -> bool {
    matches!(
        (metaclass, kind),
        (
            "RequirementDefinition",
            DeclarationKind::RequirementDefinition
        ) | ("RequirementUsage", DeclarationKind::RequirementUsage)
    )
}

fn requirement_derived_membership_role(
    collection: RequirementDerivedFactCollection,
) -> Option<MembershipRole> {
    use RequirementDerivedFactCollection as Collection;
    match collection {
        Collection::DefinitionActorParameter | Collection::UsageActorParameter => {
            Some(MembershipRole::Actor)
        }
        Collection::DefinitionSubjectParameter | Collection::UsageSubjectParameter => {
            Some(MembershipRole::Subject)
        }
        Collection::DefinitionRequiredConstraint | Collection::UsageRequiredConstraint => Some(
            MembershipRole::RequirementConstraint(RequirementConstraintKind::Requirement),
        ),
        Collection::DefinitionAssumedConstraint | Collection::UsageAssumedConstraint => Some(
            MembershipRole::RequirementConstraint(RequirementConstraintKind::Assumption),
        ),
        Collection::DefinitionFramedConcern | Collection::UsageFramedConcern => {
            Some(MembershipRole::FramedConcern)
        }
        Collection::DefinitionText | Collection::UsageText => None,
    }
}

/// The currently lowered SysML `Usage` subtypes. This is a semantic applicability table, not a
/// fallback: the private declaration kind is the canonical lowering fact and the match remains
/// exhaustive enough to force review when a new usage form is admitted.
fn is_usage_declaration(kind: DeclarationKind) -> bool {
    matches!(
        kind,
        DeclarationKind::PartUsage
            | DeclarationKind::AttributeUsage
            | DeclarationKind::EnumerationUsage
            | DeclarationKind::EnumerationLiteral
            | DeclarationKind::RequirementUsage
            | DeclarationKind::PortUsage
            | DeclarationKind::ItemUsage
            | DeclarationKind::ActionUsage
            | DeclarationKind::AcceptActionUsage
            | DeclarationKind::StateUsage
            | DeclarationKind::MetadataUsage
            | DeclarationKind::ConnectionUsage
            | DeclarationKind::OccurrenceUsage
            | DeclarationKind::AnalysisCaseUsage
            | DeclarationKind::ViewUsage
            | DeclarationKind::CaseUsage
            | DeclarationKind::VerificationCaseUsage
            | DeclarationKind::UseCaseUsage
            | DeclarationKind::ViewpointUsage
            | DeclarationKind::RenderingUsage
            | DeclarationKind::InterfaceUsage
            | DeclarationKind::ConstraintUsage
            | DeclarationKind::AssertConstraintUsage
            | DeclarationKind::AssumeConstraintUsage
            | DeclarationKind::RequireConstraintUsage
            | DeclarationKind::ConcernUsage
            | DeclarationKind::CalcUsage
            | DeclarationKind::ReferenceUsage
            | DeclarationKind::DefaultReferenceUsage
            | DeclarationKind::ParameterUsage
            | DeclarationKind::SubjectUsage
            | DeclarationKind::PerformActionUsage
            | DeclarationKind::Transition
            | DeclarationKind::Satisfy
            | DeclarationKind::Allocate
            | DeclarationKind::Bind
            | DeclarationKind::Assign
            | DeclarationKind::While
            | DeclarationKind::Loop
            | DeclarationKind::If
            | DeclarationKind::ForLoop
            | DeclarationKind::ForLoopVariable
            | DeclarationKind::Decide
            | DeclarationKind::Merge
            | DeclarationKind::Fork
            | DeclarationKind::Join
            | DeclarationKind::ThenContinuation
            | DeclarationKind::Flow
            | DeclarationKind::StakeholderUsage
            | DeclarationKind::RequirementActor
            | DeclarationKind::CaseActor
            | DeclarationKind::Frame
            | DeclarationKind::VerifyRequirement
            | DeclarationKind::BareConnect
            | DeclarationKind::EntryActionBinding
            | DeclarationKind::DoActionBinding
            | DeclarationKind::ExitActionBinding
            | DeclarationKind::InitialState
            | DeclarationKind::FinalState
            | DeclarationKind::PerformParameterBinding
    )
}

/// Whether a direct canonical member satisfies the exact selected `selectByKind` result.
fn definition_usage_candidate_matches(
    collection: DefinitionUsageDerivedKind,
    kind: DeclarationKind,
) -> bool {
    use DefinitionUsageDerivedKind as Collection;
    match collection {
        Collection::DefinitionOwnedAction | Collection::UsageNestedAction => matches!(
            kind,
            DeclarationKind::ActionUsage
                | DeclarationKind::AcceptActionUsage
                | DeclarationKind::PerformActionUsage
                | DeclarationKind::EntryActionBinding
                | DeclarationKind::DoActionBinding
                | DeclarationKind::ExitActionBinding
        ),
        Collection::DefinitionOwnedAllocation | Collection::UsageNestedAllocation => {
            matches!(kind, DeclarationKind::Allocate)
        }
        Collection::DefinitionOwnedAnalysisCase | Collection::UsageNestedAnalysisCase => {
            matches!(kind, DeclarationKind::AnalysisCaseUsage)
        }
        Collection::DefinitionOwnedAttribute | Collection::UsageNestedAttribute => {
            matches!(kind, DeclarationKind::AttributeUsage)
        }
        Collection::DefinitionOwnedCalculation | Collection::UsageNestedCalculation => {
            matches!(kind, DeclarationKind::CalcUsage)
        }
        Collection::DefinitionOwnedCase | Collection::UsageNestedCase => {
            matches!(kind, DeclarationKind::CaseUsage)
        }
        Collection::DefinitionOwnedConcern | Collection::UsageNestedConcern => {
            matches!(kind, DeclarationKind::ConcernUsage | DeclarationKind::Frame)
        }
        Collection::DefinitionOwnedConnection | Collection::UsageNestedConnection => matches!(
            kind,
            DeclarationKind::ConnectionUsage | DeclarationKind::BareConnect | DeclarationKind::Bind
        ),
        Collection::DefinitionOwnedConstraint | Collection::UsageNestedConstraint => matches!(
            kind,
            DeclarationKind::ConstraintUsage
                | DeclarationKind::AssertConstraintUsage
                | DeclarationKind::AssumeConstraintUsage
                | DeclarationKind::RequireConstraintUsage
        ),
        Collection::DefinitionOwnedEnumeration | Collection::UsageNestedEnumeration => matches!(
            kind,
            DeclarationKind::EnumerationUsage | DeclarationKind::EnumerationLiteral
        ),
        Collection::DefinitionOwnedFlow | Collection::UsageNestedFlow => {
            matches!(kind, DeclarationKind::Flow)
        }
        // The pinned XMI body selects ReferenceUsage for both `ownedInterface` and
        // `nestedInterface`; the exact body, not the property suffix, is authoritative.
        Collection::DefinitionOwnedInterface
        | Collection::UsageNestedInterface
        | Collection::DefinitionOwnedReference
        | Collection::UsageNestedReference => matches!(
            kind,
            DeclarationKind::ReferenceUsage
                | DeclarationKind::DefaultReferenceUsage
                | DeclarationKind::ParameterUsage
                | DeclarationKind::SubjectUsage
                | DeclarationKind::PerformParameterBinding
        ),
        Collection::DefinitionOwnedItem | Collection::UsageNestedItem => {
            matches!(kind, DeclarationKind::ItemUsage)
        }
        Collection::DefinitionOwnedMetadata | Collection::UsageNestedMetadata => {
            matches!(kind, DeclarationKind::MetadataUsage)
        }
        Collection::DefinitionOwnedOccurrence | Collection::UsageNestedOccurrence => {
            matches!(kind, DeclarationKind::OccurrenceUsage)
        }
        Collection::DefinitionOwnedPart | Collection::UsageNestedPart => matches!(
            kind,
            DeclarationKind::PartUsage
                | DeclarationKind::StakeholderUsage
                | DeclarationKind::RequirementActor
                | DeclarationKind::CaseActor
        ),
        Collection::DefinitionOwnedPort | Collection::UsageNestedPort => {
            matches!(kind, DeclarationKind::PortUsage)
        }
        Collection::DefinitionOwnedRendering | Collection::UsageNestedRendering => {
            matches!(kind, DeclarationKind::RenderingUsage)
        }
        Collection::DefinitionOwnedRequirement | Collection::UsageNestedRequirement => matches!(
            kind,
            DeclarationKind::RequirementUsage | DeclarationKind::VerifyRequirement
        ),
        Collection::DefinitionOwnedState | Collection::UsageNestedState => {
            matches!(
                kind,
                DeclarationKind::StateUsage | DeclarationKind::FinalState
            )
        }
        Collection::DefinitionOwnedTransition | Collection::UsageNestedTransition => {
            matches!(kind, DeclarationKind::Transition)
        }
        Collection::DefinitionOwnedUsage | Collection::UsageNestedUsage => {
            is_usage_declaration(kind)
        }
        Collection::DefinitionOwnedUseCase | Collection::UsageNestedUseCase => {
            matches!(kind, DeclarationKind::UseCaseUsage)
        }
        Collection::DefinitionOwnedVerificationCase | Collection::UsageNestedVerificationCase => {
            matches!(kind, DeclarationKind::VerificationCaseUsage)
        }
        Collection::DefinitionOwnedView | Collection::UsageNestedView => {
            matches!(kind, DeclarationKind::ViewUsage)
        }
        Collection::DefinitionOwnedViewpoint | Collection::UsageNestedViewpoint => {
            matches!(kind, DeclarationKind::ViewpointUsage)
        }
        // These variants are returned before candidate selection by the rule-scoped method.
        Collection::DefinitionDirectedUsage
        | Collection::DefinitionUsage
        | Collection::DefinitionVariant
        | Collection::DefinitionVariantMembership
        | Collection::UsageDirectedUsage
        | Collection::UsageIsReference
        | Collection::UsageMayTimeVary
        | Collection::UsageUsage
        | Collection::UsageVariant
        | Collection::UsageVariantMembership => false,
    }
}

#[derive(Debug, Clone, Copy)]
enum DeclarationDomain {
    Any,
    Namespace,
    Type,
}

impl DeclarationDomain {
    fn accepts(self, kind: DeclarationKind) -> bool {
        match self {
            Self::Any => true,
            Self::Namespace => matches!(
                kind,
                DeclarationKind::Namespace
                    | DeclarationKind::Package
                    | DeclarationKind::LibraryPackage
            ),
            // An alias is a transparent proxy: whether it is Type-domain-compatible is a property
            // of its (possibly not-yet-resolved) ultimate target, not of the alias declaration
            // itself, so it is provisionally accepted here. `synthesize_implied_alias_bindings`
            // below chases the alias's own resolved `AliasBinding` reference to publish the real
            // (implied) typing/specialization fact against the ultimate non-alias target.
            Self::Type => matches!(
                kind,
                DeclarationKind::PartDefinition
                    | DeclarationKind::AttributeDefinition
                    | DeclarationKind::EnumerationDefinition
                    | DeclarationKind::RequirementDefinition
                    | DeclarationKind::PortDefinition
                    | DeclarationKind::ItemDefinition
                    | DeclarationKind::ActionDefinition
                    | DeclarationKind::StateDefinition
                    | DeclarationKind::MetadataDefinition
                    | DeclarationKind::ConnectionDefinition
                    | DeclarationKind::InterfaceDefinition
                    | DeclarationKind::OccurrenceDefinition
                    | DeclarationKind::AnalysisCaseDefinition
                    | DeclarationKind::ViewDefinition
                    | DeclarationKind::CaseDefinition
                    | DeclarationKind::VerificationCaseDefinition
                    | DeclarationKind::UseCaseDefinition
                    | DeclarationKind::ViewpointDefinition
                    | DeclarationKind::RenderingDefinition
                    | DeclarationKind::AllocationDefinition
                    | DeclarationKind::FlowDefinition
                    | DeclarationKind::ConstraintDefinition
                    | DeclarationKind::ConcernDefinition
                    | DeclarationKind::CalcDefinition
                    | DeclarationKind::ClassDefinition
                    // The KerML type metaclasses. Every one of these is a `Type` in the
                    // metamodel, so each is a legitimate FeatureTyping/Subclassification target;
                    // without them no reference into the KerML kernel libraries can resolve, and
                    // `attribute mass : ScalarValues::Real` fails against a `datatype` that is
                    // right there in the admitted standard library. `KermlMultiplicity` is
                    // deliberately absent: `Multiplicity <: Feature`, and this domain admits
                    // definition-like types only, exactly as it already excludes SysML usages.
                    | DeclarationKind::KermlClassifier
                    | DeclarationKind::KermlStructure
                    | DeclarationKind::KermlAssociation
                    | DeclarationKind::KermlAssociationStructure
                    | DeclarationKind::KermlDataType
                    | DeclarationKind::KermlMetaclass
                    | DeclarationKind::KermlBehavior
                    | DeclarationKind::KermlFunction
                    | DeclarationKind::KermlPredicate
                    | DeclarationKind::KermlInteraction
                    | DeclarationKind::KermlType
                    | DeclarationKind::Alias
            ),
        }
    }
}

fn build_direct_name_index(
    declarations: &[Declaration],
    public_only: Option<&MembershipIndex>,
) -> Result<NameIndex, ResolutionError> {
    let mut entries = Vec::new();
    entries
        .try_reserve(declarations.len())
        .map_err(|_| ResolutionError::Capacity)?;
    for (index, declaration) in declarations.iter().enumerate() {
        let declaration_id =
            DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
        if public_only.is_some_and(|memberships| !memberships.is_public(declaration_id)) {
            continue;
        }
        if let Some(name) = declaration.name {
            entries.push((
                NameKey {
                    owner: declaration.owner,
                    name,
                },
                declaration_id,
            ));
        }
    }
    NameIndex::build(entries)
}

fn build_effective_import_indexes<R: ResolutionReferenceFact>(
    declarations: &[Declaration],
    memberships: &MembershipIndex,
    references: &[R],
    import_slots: &[usize],
    exported_names: &NameIndex,
    previous_exported_imports: &NameIndex,
    outcomes: &[ResolutionStatus],
) -> Result<(NameIndex, NameIndex), ResolutionError> {
    let mut entries = Vec::new();
    let mut exported_entries = Vec::new();
    for index in import_slots.iter().copied() {
        let reference = references
            .get(index)
            .ok_or(ResolutionError::InvalidStorage)?;
        let ResolutionStatus::Resolved(target) = outcomes[index] else {
            continue;
        };
        let import_owner = declarations
            .get(reference.source().index())
            .ok_or(ResolutionError::InvalidStorage)?
            .owner;
        let import_is_public = memberships.is_public(reference.source());
        match reference.kind() {
            ReferenceKind::NamespaceImport => {
                for (name, candidates) in exported_names.entries_for_owner(Some(target)) {
                    extend_import_entries(
                        &mut entries,
                        &mut exported_entries,
                        import_owner,
                        name,
                        candidates,
                        import_is_public,
                    );
                }
                for (name, candidates) in previous_exported_imports.entries_for_owner(Some(target))
                {
                    extend_import_entries(
                        &mut entries,
                        &mut exported_entries,
                        import_owner,
                        name,
                        candidates,
                        import_is_public,
                    );
                }
            }
            ReferenceKind::MembershipImport => {
                let declaration = declarations
                    .get(target.index())
                    .ok_or(ResolutionError::InvalidStorage)?;
                if memberships.is_public(target) {
                    if let Some(name) = declaration.name {
                        extend_import_entries(
                            &mut entries,
                            &mut exported_entries,
                            import_owner,
                            name,
                            std::slice::from_ref(&target),
                            import_is_public,
                        );
                    }
                }
            }
            ReferenceKind::FilterImport
            | ReferenceKind::FeatureTyping
            | ReferenceKind::TypeFeaturing
            | ReferenceKind::FeatureChaining
            | ReferenceKind::Subclassification
            | ReferenceKind::Subsetting
            | ReferenceKind::Redefinition
            | ReferenceKind::References
            | ReferenceKind::Crosses
            | ReferenceKind::Intersects
            | ReferenceKind::Unioning
            | ReferenceKind::Intersecting
            | ReferenceKind::Differencing
            | ReferenceKind::Disjoining
            | ReferenceKind::AliasBinding
            | ReferenceKind::ConnectorEnd
            | ReferenceKind::Succession
            | ReferenceKind::EntryActionBinding
            | ReferenceKind::DoActionBinding
            | ReferenceKind::ExitActionBinding
            | ReferenceKind::InitialState
            | ReferenceKind::ExpressionOperand
            | ReferenceKind::TransitionSource
            | ReferenceKind::TransitionTarget
            | ReferenceKind::TransitionTrigger
            | ReferenceKind::TransitionEffect
            | ReferenceKind::MetadataAnnotation
            | ReferenceKind::FilterMetadataTest
            | ReferenceKind::SatisfySource
            | ReferenceKind::SatisfyTarget
            | ReferenceKind::AllocateSource
            | ReferenceKind::AllocateTarget
            | ReferenceKind::BindSource
            | ReferenceKind::BindTarget
            | ReferenceKind::Variant
            | ReferenceKind::IncludeUseCase
            | ReferenceKind::ViewExpose
            | ReferenceKind::MemberAccessOperand
            | ReferenceKind::InvocationCallee
            | ReferenceKind::ThenTarget
            | ReferenceKind::AcceptVia
            | ReferenceKind::SendTarget
            | ReferenceKind::AcceptPayloadType
            | ReferenceKind::TerminateTarget
            | ReferenceKind::FlowSource
            | ReferenceKind::FlowTarget
            | ReferenceKind::TypeCheckTarget
            | ReferenceKind::MetaCastTarget
            | ReferenceKind::StakeholderTarget
            | ReferenceKind::PurposeTarget
            | ReferenceKind::VerifyRequirementTarget
            | ReferenceKind::AssignTarget
            | ReferenceKind::DependencyClient
            | ReferenceKind::DependencySupplier
            | ReferenceKind::PerformParameterTarget
            | ReferenceKind::FlowPayloadType => {}
        }
    }
    Ok((
        NameIndex::build(entries)?,
        NameIndex::build(exported_entries)?,
    ))
}

fn extend_import_entries(
    local: &mut Vec<(NameKey, DeclarationId)>,
    exported: &mut Vec<(NameKey, DeclarationId)>,
    owner: Option<DeclarationId>,
    name: SymbolId,
    candidates: &[DeclarationId],
    import_is_public: bool,
) {
    let key = NameKey { owner, name };
    local.extend(candidates.iter().copied().map(|candidate| (key, candidate)));
    if import_is_public {
        exported.extend(candidates.iter().copied().map(|candidate| (key, candidate)));
    }
}

struct ResolutionIndexes<'a> {
    direct_names: &'a NameIndex,
    exported_names: &'a NameIndex,
    effective_imports: Option<&'a NameIndex>,
    exported_imports: Option<&'a NameIndex>,
    /// Ancestor-scoped inherited-member lookup, keyed by `(child type declaration, name)`. Absent
    /// for the Subclassification pass itself (it is built from Subclassification's own settled
    /// outcomes) and present for reference kinds resolved afterward, such as FeatureTyping.
    inherited_names: Option<&'a NameIndex>,
}

struct ResolutionScratch<'a> {
    ambiguous_candidates: &'a mut Vec<DeclarationId>,
    candidates: &'a mut Vec<DeclarationId>,
    next_candidates: &'a mut Vec<DeclarationId>,
    work: &'a mut ResolutionWork,
}

fn resolve_reference<R: ResolutionReferenceFact>(
    declarations: &[Declaration],
    paths: &SymbolPathArena,
    reference: &R,
    domain: DeclarationDomain,
    indexes: ResolutionIndexes<'_>,
    scratch: ResolutionScratch<'_>,
) -> Result<ResolutionStatus, ResolutionError> {
    let (segments, rooted) = paths
        .get(reference.path())
        .ok_or(ResolutionError::InvalidStorage)?;
    let source = declarations
        .get(reference.source().index())
        .ok_or(ResolutionError::InvalidStorage)?;
    // KerML Redefinition relates a feature to a *different* feature, so the redefining feature is
    // not in its own redefinition scope: the Pilot's `KerMLScope` excludes it, and
    // Redefinition excludes owned
    // first-scope candidates".
    //
    // Without this, `feature annotatedElement : Element[1..*] redefines annotatedElement;` -- the
    // shape the KerML abstract-syntax library uses throughout to narrow an inherited feature --
    // finds itself in its owner's owned tier, and that tier shadows the inherited one the author
    // meant. The feature then specializes itself, which makes its whole conformance hierarchy
    // cyclic and every conformance question about it unanswerable.
    //
    // Deliberately Redefinition alone. The anonymous subsetting shorthand `:> annotatedElement :
    // T;` has the same shape, but a metadata definition commonly authors several of them, and the
    // lowering gives each the subsetted feature's name as its *declared* name. Excluding only the
    // reference's own source there does not reach the inherited feature; it reaches the sibling
    // shorthand, so two of them resolve to each other and the self-loop becomes a two-cycle. That
    // is a lowering defect -- an unnamed member must not acquire a declared name -- and it is
    // recorded in planning/UPSTREAM_PARSER_GAPS.md rather than compensated for here.
    let excluded = (reference.kind() == ReferenceKind::Redefinition).then(|| reference.source());
    scratch.candidates.clear();
    scratch.next_candidates.clear();
    if rooted {
        record_lookup(scratch.work)?;
        scratch
            .candidates
            .extend_from_slice(indexes.exported_names.candidates(None, segments[0]));
    } else {
        // When the first segment is also the last (a plain unqualified reference), the winning
        // precedence tier's candidates *are* the final resolution target, so domain compatibility
        // is applied per tier below (an incompatible-domain local binding still shadows a
        // compatible outer/imported one. When more
        // segments follow, this first segment denotes an intermediate namespace/type owner, not
        // the reference's final target, so no domain filtering applies here: `Any` accepts
        // everything and the tier logic degrades to plain name-presence shadowing.
        let first_segment_domain = if segments.len() == 1 {
            domain
        } else {
            DeclarationDomain::Any
        };
        let lexical_scope = if reference.kind() == ReferenceKind::ExpressionOperand {
            Some(reference.source())
        } else {
            source.owner
        };
        lookup_lexical_into(
            declarations,
            &indexes,
            lexical_scope,
            segments[0],
            LookupTarget {
                domain: first_segment_domain,
                excluded,
            },
            scratch.candidates,
            scratch.work,
        )?;
    }

    for segment in &segments[1..] {
        scratch.next_candidates.clear();
        for candidate in scratch.candidates.iter().copied() {
            record_lookup(scratch.work)?;
            // A qualified name's later segments name a member OWNED by the previous segment's
            // resolved declaration, exactly as if that member were being looked up from within
            // its owner's own scope (KerML's "public by default unless owned by a non-Package
            // namespace" visibility rule governs whether a *wildcard* import/general lookup can
            // reach a name, not whether an explicit qualified reference that already names the
            // owner can reach its direct member). `direct_names` is therefore the right index
            // here -- the same one `extend_inherited_names_with_usage_typing`'s inherited-member
            // traversal reads from for redefinition targets like `ManagedRequirement::status`
            // (owned by a Type, not a Package, yet still reachable). `exported_names` is reserved
            // for cross-file import propagation (`NamespaceImport`/`MembershipImport`), which is
            // a different visibility question -- whether a *different* file can pull the name in
            // via `import`, not whether this file's own qualified reference can name it directly.
            // Import fallback still applies when the owner itself was reached only through an
            // import (the member is not locally owned at all), so cross-package traversal through
            // imported namespaces keeps working.
            let direct = indexes.direct_names.candidates(Some(candidate), *segment);
            if !direct.is_empty() {
                scratch.next_candidates.extend_from_slice(direct);
            } else if let Some(imports) = indexes.exported_imports {
                record_lookup(scratch.work)?;
                scratch
                    .next_candidates
                    .extend_from_slice(imports.candidates(Some(candidate), *segment));
            }
        }
        scratch.next_candidates.sort_unstable();
        scratch.next_candidates.dedup();
        std::mem::swap(scratch.candidates, scratch.next_candidates);
    }
    // The non-rooted, single-segment case already applied domain-aware tier selection inside
    // `lookup_lexical_into` above (including the incompatible-domain shadow rule), so re-filtering
    // here would silently discard a shadowed candidate and regress it to `Unresolved`. Every other
    // case (rooted lookups, and the final segment of a multi-segment qualified name) still needs
    // this domain check applied to its result.
    if rooted || segments.len() > 1 {
        scratch.candidates.retain(|candidate| {
            declarations
                .get(candidate.index())
                .is_some_and(|declaration| domain.accepts(declaration.kind))
        });
    }
    // A qualified or absolute path can name the specializing feature just as an unqualified one
    // can, and it is no more well-formed for having been spelled out.
    if let Some(excluded) = excluded {
        scratch
            .candidates
            .retain(|candidate| *candidate != excluded);
    }
    status_from_candidates(scratch.candidates, scratch.ambiguous_candidates)
}

/// Walks the enclosing-namespace chain from `owner` outward. At each level, owned members take
/// precedence over inherited (ancestor-scoped) members, which take precedence over imports, per
/// the canonical scope-origin precedence ("owned members, then
/// inherited/general members, then imports"). `inherited_names` is `None` for reference kinds that
/// do not read inherited scope (for example Subclassification itself).
///
/// `domain` is the broad metamodel-domain filter for the reference's final target (section 6 step
/// 8). Per section 11.1 ("an incompatible inner `Type` still shadows a compatible outer type,
/// followed by validation"), domain compatibility never changes *which* precedence tier wins: at
/// each tier, a domain-compatible candidate is preferred when one exists, but a tier that has any
/// same-name binding still shadows lower-precedence tiers even if every candidate at that tier is
/// domain-incompatible. Passing `DeclarationDomain::Any` disables this preference entirely (every
/// candidate matches), which is what callers use when this lookup does not produce the reference's
/// final target (an interior segment of a qualified name).
/// What a lexical lookup is looking for, as opposed to where it looks.
#[derive(Debug, Clone, Copy)]
struct LookupTarget {
    /// The broad metamodel-domain filter for the reference's final target.
    domain: DeclarationDomain,
    /// A declaration that is not in its own scope for this reference, if any.
    excluded: Option<DeclarationId>,
}

fn lookup_lexical_into(
    declarations: &[Declaration],
    indexes: &ResolutionIndexes<'_>,
    mut owner: Option<DeclarationId>,
    name: SymbolId,
    target: LookupTarget,
    candidates: &mut Vec<DeclarationId>,
    work: &mut ResolutionWork,
) -> Result<(), ResolutionError> {
    let LookupTarget { domain, excluded } = target;
    let select_tier = |raw: &[DeclarationId], out: &mut Vec<DeclarationId>| {
        let compatible = raw
            .iter()
            .copied()
            .filter(|candidate| {
                declarations
                    .get(candidate.index())
                    .is_some_and(|declaration| domain.accepts(declaration.kind))
            })
            .collect::<Vec<_>>();
        if compatible.is_empty() {
            out.extend_from_slice(raw);
        } else {
            out.extend(compatible);
        }
    };
    // Applied before the tier is tested for emptiness, so a tier whose only binding is the
    // excluded declaration does not shadow the tiers below it.
    let visible = |raw: &[DeclarationId], scratch: &mut Vec<DeclarationId>| -> bool {
        let Some(excluded) = excluded else {
            return false;
        };
        if !raw.contains(&excluded) {
            return false;
        }
        scratch.clear();
        scratch.extend(raw.iter().copied().filter(|entry| *entry != excluded));
        true
    };
    let mut filtered = Vec::new();
    loop {
        record_lookup(work)?;
        let mut direct = indexes.direct_names.candidates(owner, name);
        if visible(direct, &mut filtered) {
            direct = &filtered;
        }
        if !direct.is_empty() {
            select_tier(direct, candidates);
            return Ok(());
        }
        if let Some(inherited) = indexes.inherited_names {
            record_lookup(work)?;
            let inherited = inherited.candidates(owner, name);
            if !inherited.is_empty() {
                select_tier(inherited, candidates);
                return Ok(());
            }
        }
        if let Some(imports) = indexes.effective_imports {
            record_lookup(work)?;
            let imported = imports.candidates(owner, name);
            if !imported.is_empty() {
                select_tier(imported, candidates);
                return Ok(());
            }
        }
        let Some(current) = owner else {
            return Ok(());
        };
        owner = declarations
            .get(current.index())
            .ok_or(ResolutionError::InvalidStorage)?
            .owner;
    }
}

/// Resolves a `ReferenceKind::MemberAccessOperand` reference (a dotted feature-chain access, e.g.
/// `t.bead`, `f.a`, chained `a.b.c`): the reference's `path()` is the flattened chain built by
/// `SemanticModelBuilder::push_member_access_reference` -- the root segment(s) followed by each
/// subsequent dotted member segment, all in one `SymbolPathId`.
///
/// The root segment resolves through `DeclarationDomain::Any` lexical lookup, beginning in the
/// source declaration's owned scope and then walking its enclosing-namespace chain. This lets a
/// constraint/calc expression reach its own parameters without changing the enclosing-scope result
/// for sources that own no declarations. Each subsequent segment is then looked up as a member OWNED
/// (directly or through inheritance) by the *type* of the previously resolved segment, never as a
/// member of the previous segment's own declaration -- reusing `inherited_names`, which by the
/// time this runs has already been extended with usage-typing entries
/// (`extend_inherited_names_with_usage_typing`): for a declaration `usage` whose own `FeatureTyping`
/// reference resolved to `target`, `inherited_names.candidates(Some(usage), name)` already contains
/// every name owned (directly or by inheritance) by `target`. This is exactly "member of the
/// previously resolved segment's type."
///
/// If the root segment does not resolve to exactly one candidate, or any subsequent segment finds
/// no owned-by-type member, the whole chain publishes `Unresolved`; if an intermediate or final
/// segment is ambiguous, the whole chain publishes `Ambiguous`. The chain never partially resolves
/// -- there is no way to publish "the first two segments resolved but the third didn't."
fn resolve_member_access_reference<R: ResolutionReferenceFact>(
    declarations: &[Declaration],
    paths: &SymbolPathArena,
    reference: &R,
    indexes: ResolutionIndexes<'_>,
    scratch: ResolutionScratch<'_>,
) -> Result<ResolutionStatus, ResolutionError> {
    let (segments, rooted) = paths
        .get(reference.path())
        .ok_or(ResolutionError::InvalidStorage)?;
    if rooted {
        // A dotted member-access chain is never `::`-absolute; defensive, not reachable from the
        // lowering side today.
        return Ok(ResolutionStatus::Unsupported);
    }
    let _source = declarations
        .get(reference.source().index())
        .ok_or(ResolutionError::InvalidStorage)?;
    scratch.candidates.clear();
    scratch.next_candidates.clear();
    lookup_lexical_into(
        declarations,
        &indexes,
        Some(reference.source()),
        segments[0],
        // A member-access chain is never a Redefinition reference.
        LookupTarget {
            domain: DeclarationDomain::Any,
            excluded: None,
        },
        scratch.candidates,
        scratch.work,
    )?;
    for segment in &segments[1..] {
        if scratch.candidates.len() != 1 {
            // Zero candidates (Unresolved) or more than one (Ambiguous) -- either way the chain
            // cannot continue past this hop; publish that outcome directly rather than silently
            // dropping the remaining segments.
            return status_from_candidates(scratch.candidates, scratch.ambiguous_candidates);
        }
        let candidate = scratch.candidates[0];
        scratch.next_candidates.clear();
        record_lookup(scratch.work)?;
        if let Some(inherited) = indexes.inherited_names {
            scratch
                .next_candidates
                .extend_from_slice(inherited.candidates(Some(candidate), *segment));
        }
        scratch.next_candidates.sort_unstable();
        scratch.next_candidates.dedup();
        std::mem::swap(scratch.candidates, scratch.next_candidates);
    }
    status_from_candidates(scratch.candidates, scratch.ambiguous_candidates)
}

fn record_lookup(work: &mut ResolutionWork) -> Result<(), ResolutionError> {
    work.indexed_name_lookups = work
        .indexed_name_lookups
        .checked_add(1)
        .ok_or(ResolutionError::Capacity)?;
    Ok(())
}

fn status_from_candidates(
    candidates: &[DeclarationId],
    ambiguous_candidates: &mut Vec<DeclarationId>,
) -> Result<ResolutionStatus, ResolutionError> {
    match candidates {
        [] => Ok(ResolutionStatus::Unresolved),
        [candidate] => Ok(ResolutionStatus::Resolved(*candidate)),
        _ => {
            let start = ambiguous_candidates.len();
            ambiguous_candidates.extend_from_slice(candidates);
            Ok(ResolutionStatus::Ambiguous(CandidateRange::from_bounds(
                start,
                ambiguous_candidates.len(),
            )?))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_feature_relationship_collection_contracts_are_complete_and_closed() {
        assert_eq!(GENERATED_FEATURE_DERIVED_RELATIONSHIP_RULES.len(), 5);
        assert_eq!(
            GENERATED_FEATURE_DERIVED_RELATIONSHIP_RULES
                .iter()
                .map(|rule| (rule.rule_id, rule.metaclass, rule.collection))
                .collect::<Vec<_>>(),
            vec![
                (
                    "kerml-1.0:8.3.3.3.4:deriveFeatureOwnedFeatureChaining",
                    "Feature",
                    FeatureDerivedRelationshipCollection::OwnedFeatureChaining,
                ),
                (
                    "kerml-1.0:8.3.3.3.4:deriveFeatureOwnedRedefinition",
                    "Feature",
                    FeatureDerivedRelationshipCollection::OwnedRedefinition,
                ),
                (
                    "kerml-1.0:8.3.3.3.4:deriveFeatureOwnedSubsetting",
                    "Feature",
                    FeatureDerivedRelationshipCollection::OwnedSubsetting,
                ),
                (
                    "kerml-1.0:8.3.3.3.4:deriveFeatureOwnedTypeFeaturing",
                    "Feature",
                    FeatureDerivedRelationshipCollection::OwnedTypeFeaturing,
                ),
                (
                    "kerml-1.0:8.3.3.3.4:deriveFeatureOwnedTyping",
                    "Feature",
                    FeatureDerivedRelationshipCollection::OwnedTyping,
                ),
            ]
        );
    }

    #[test]
    fn generated_type_fact_contract_rows_are_canonical_rule_id_ordered() {
        assert!(GENERATED_TYPE_DERIVED_FACT_RULES
            .windows(2)
            .all(|pair| pair[0].rule_id < pair[1].rule_id));
    }

    #[test]
    fn generated_unconditional_library_specialization_table_covers_all_manifest_check_rules() {
        assert_eq!(generated_library_specialization_rule_count(), 85);
        assert_eq!(
            generated_conditional_library_specialization_rule_count(),
            56
        );
        let unique_rules = GENERATED_LIBRARY_SPECIALIZATION_RULES
            .iter()
            .map(|rule| rule.rule_id)
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(
            unique_rules.len(),
            85,
            "rule IDs must remain unique map keys"
        );
        assert!(GENERATED_LIBRARY_SPECIALIZATION_RULES.iter().all(|rule| {
            !rule.rule_id.is_empty() && !rule.metaclass.is_empty() && !rule.anchor.is_empty()
        }));
        assert!(GENERATED_CONDITIONAL_LIBRARY_SPECIALIZATION_RULES
            .iter()
            .all(|rule| {
                !rule.rule_id.is_empty()
                    && !rule.metaclass.is_empty()
                    && !rule.anchor.is_empty()
                    && (rule.predicate != LibrarySpecializationPredicate::CompositeOwnedBy
                        || rule.owner_metaclasses.len() == 2)
            }));
        assert_eq!(generated_library_redefinition_rule_count(), 1);
        assert_eq!(
            GENERATED_LIBRARY_REDEFINITION_RULES,
            &[LibraryRedefinitionRule {
                rule_id: "kerml-1.0:8.3.4.9.5:checkPayloadFeatureRedefinition",
                metaclass: "PayloadFeature",
                anchor: "Transfers::Transfer::payload",
            }]
        );
        for rule in GENERATED_LIBRARY_SPECIALIZATION_RULES {
            assert_eq!(
                library_specialization_rules(rule.metaclass).count(),
                1,
                "{} must retain its exact generated applicability metaclass",
                rule.rule_id
            );
        }

        // The stored fact owner is total over the generated table even when the admitted model
        // contains no library declarations. That makes every missing prerequisite explicit, and
        // catches a manifest/generator change that adds a rule without publication coverage.
        let anchors = library_specialization_anchors(&storage_with_one_filter());
        assert_eq!(anchors.by_rule.len(), 148);
        for rule in GENERATED_LIBRARY_SPECIALIZATION_RULES {
            assert!(matches!(
                anchors.outcome(rule.rule_id),
                Some(LibrarySpecializationAnchor::Missing)
            ));
        }
        for rule in GENERATED_CONDITIONAL_LIBRARY_SPECIALIZATION_RULES {
            assert!(matches!(
                anchors.outcome(rule.rule_id),
                Some(LibrarySpecializationAnchor::Missing)
            ));
            if rule.true_anchor.is_some() {
                assert!(matches!(
                    anchors.outcome_for(
                        rule.rule_id,
                        LibrarySpecializationAnchorBranch::PredicateTrue,
                    ),
                    Some(LibrarySpecializationAnchor::Missing)
                ));
            }
        }
        for rule in GENERATED_LIBRARY_REDEFINITION_RULES {
            assert!(matches!(
                anchors.outcome(rule.rule_id),
                Some(LibrarySpecializationAnchor::Missing)
            ));
        }
        let shared_anchor_rules = GENERATED_LIBRARY_SPECIALIZATION_RULES
            .iter()
            .filter(|rule| rule.anchor == "Performances::literalIntegerEvaluations")
            .collect::<Vec<_>>();
        assert_eq!(shared_anchor_rules.len(), 2);
        assert_ne!(
            shared_anchor_rules[0].rule_id,
            shared_anchor_rules[1].rule_id
        );
        assert!(matches!(
            anchors.outcome(shared_anchor_rules[0].rule_id),
            Some(LibrarySpecializationAnchor::Missing)
        ));
        assert!(matches!(
            anchors.outcome(shared_anchor_rules[1].rule_id),
            Some(LibrarySpecializationAnchor::Missing)
        ));
    }

    #[test]
    fn generated_conditional_specialization_rows_preserve_the_typed_manifest_contract() {
        let manifest_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("specifications/constraint_manifest.toml");
        let manifest = spec42_constraint_manifest::ConstraintManifest::load_toml(&manifest_path)
            .expect("load typed manifest for generated-row drift test");
        let mut expected = manifest
            .specifications
            .iter()
            .flat_map(|specification| &specification.constraints)
            .filter_map(|entry| {
                entry
                    .conditional_specializes_from_library
                    .as_ref()
                    .map(|contract| {
                        (
                            entry.rule_id.clone(),
                            entry.metaclass.clone(),
                            contract.predicate,
                            contract.owner_metaclasses.clone(),
                            contract.true_anchor.clone(),
                            contract.anchor.clone(),
                        )
                    })
            })
            .collect::<Vec<_>>();
        expected.sort_by(|left, right| left.0.cmp(&right.0));
        let actual = GENERATED_CONDITIONAL_LIBRARY_SPECIALIZATION_RULES
            .iter()
            .map(|rule| {
                (
                    rule.rule_id.to_string(),
                    rule.metaclass.to_string(),
                    rule.predicate,
                    rule.owner_metaclasses
                        .iter()
                        .map(|metaclass| (*metaclass).to_string())
                        .collect::<Vec<_>>(),
                    rule.true_anchor.map(str::to_string),
                    rule.anchor.to_string(),
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }

    #[test]
    fn membership_role_specialization_predicates_use_the_canonical_role_and_owner_facts() {
        let storage = storage_with_membership_role_specializations();
        let rule = |id| {
            GENERATED_CONDITIONAL_LIBRARY_SPECIALIZATION_RULES
                .iter()
                .find(|rule| rule.rule_id == id)
                .expect("generated membership-role rule")
        };
        let id = |index| DeclarationId::from_index(index).expect("test declaration id");

        let framed = rule("sysml-2.0:8.3.21.4:checkConcernUsageFramedConcernSpecialization");
        assert!(conditional_library_specialization_predicate_holds(
            &storage,
            id(2),
            framed
        ));
        assert!(!conditional_library_specialization_predicate_holds(
            &storage,
            id(9),
            framed
        ));

        let constraint =
            rule("sysml-2.0:8.3.20.4:checkConstraintUsageRequirementConstraintSpecialization");
        assert!(conditional_library_specialization_predicate_holds(
            &storage,
            id(3),
            constraint
        ));
        assert_eq!(
            conditional_library_specialization_anchor_branch(&storage, id(3), constraint),
            LibrarySpecializationAnchorBranch::PredicateTrue,
        );
        assert_eq!(
            conditional_library_specialization_anchor_branch(&storage, id(4), constraint),
            LibrarySpecializationAnchorBranch::Default,
        );

        let actor = rule("sysml-2.0:8.3.11.3:checkPartUsageActorSpecialization");
        assert!(conditional_library_specialization_predicate_holds(
            &storage,
            id(5),
            actor
        ));
        assert_eq!(
            conditional_library_specialization_anchor_branch(&storage, id(5), actor),
            LibrarySpecializationAnchorBranch::PredicateTrue,
        );
        assert_eq!(
            conditional_library_specialization_anchor_branch(&storage, id(6), actor),
            LibrarySpecializationAnchorBranch::Default,
        );
        assert!(!conditional_library_specialization_predicate_holds(
            &storage,
            id(9),
            actor
        ));

        let stakeholder = rule("sysml-2.0:8.3.11.3:checkPartUsageStakeholderSpecialization");
        assert!(conditional_library_specialization_predicate_holds(
            &storage,
            id(8),
            stakeholder
        ));

        let verification =
            rule("sysml-2.0:8.3.21.9:checkRequirementUsageRequirementVerificationSpecialization");
        assert!(conditional_library_specialization_predicate_holds(
            &storage,
            id(10),
            verification
        ));
    }

    #[test]
    fn accept_action_specialization_predicates_use_trigger_and_subaction_facts() {
        let storage = storage_with_accept_action_specializations();
        let rule = |id| {
            GENERATED_CONDITIONAL_LIBRARY_SPECIALIZATION_RULES
                .iter()
                .find(|rule| rule.rule_id == id)
                .expect("generated accept-action rule")
        };
        let id = |index| DeclarationId::from_index(index).expect("test declaration id");
        let ordinary = rule("sysml-2.0:8.3.17.2:checkAcceptActionUsageSpecialization");
        let subaction = rule("sysml-2.0:8.3.17.2:checkAcceptActionUsageSubactionSpecialization");
        let trigger = rule("sysml-2.0:8.3.17.2:checkAcceptActionUsageTriggerActionSpecialization");

        // A top-level accept action is non-trigger but not a subaction.
        assert!(conditional_library_specialization_predicate_holds(
            &storage,
            id(1),
            ordinary
        ));
        assert!(!conditional_library_specialization_predicate_holds(
            &storage,
            id(1),
            subaction
        ));
        assert!(!conditional_library_specialization_predicate_holds(
            &storage,
            id(1),
            trigger
        ));

        // The exact owner/composite facts add the subaction specialization without changing the
        // non-trigger one.
        assert!(conditional_library_specialization_predicate_holds(
            &storage,
            id(3),
            ordinary
        ));
        assert!(conditional_library_specialization_predicate_holds(
            &storage,
            id(3),
            subaction
        ));
        assert!(!conditional_library_specialization_predicate_holds(
            &storage,
            id(3),
            trigger
        ));

        // A transition trigger is explicitly suppressed from both non-trigger rules.
        assert!(!conditional_library_specialization_predicate_holds(
            &storage,
            id(5),
            ordinary
        ));
        assert!(!conditional_library_specialization_predicate_holds(
            &storage,
            id(5),
            subaction
        ));
        assert!(conditional_library_specialization_predicate_holds(
            &storage,
            id(5),
            trigger
        ));
    }

    #[test]
    fn if_action_specialization_selects_the_canonical_else_action_branch() {
        let id = |index| DeclarationId::from_index(index).expect("test declaration id");
        let storage = SemanticModelStorage {
            documents: Box::new([]),
            declarations: vec![
                declaration(DocumentId(0), None, None, DeclarationKind::Package),
                declaration(DocumentId(0), Some(id(0)), None, DeclarationKind::If),
                declaration(DocumentId(0), Some(id(0)), None, DeclarationKind::If),
                declaration(DocumentId(0), Some(id(0)), None, DeclarationKind::If),
            ]
            .into_boxed_slice(),
            declaration_facts: vec![
                DeclarationFacts::none(),
                DeclarationFacts::none(),
                DeclarationFacts {
                    has_else_action: Some(false),
                    ..DeclarationFacts::none()
                },
                DeclarationFacts {
                    has_else_action: Some(true),
                    ..DeclarationFacts::none()
                },
            ]
            .into_boxed_slice(),
            memberships: Box::new([]),
            references: Box::new([]),
            documentation: Box::new([]),
            feature_values: Box::new([]),
            unsupported: Box::new([]),
            recovery: Box::new([]),
            symbols: SymbolTableBuilder::default().freeze(),
            paths: SymbolPathArenaBuilder::default().freeze(),
            evaluation_facts: Box::new([]),
            unit_tokens: Box::new([]),
            filter_conditions: Box::new([]),
            invocations: Box::new([]),
        };
        let rule = GENERATED_CONDITIONAL_LIBRARY_SPECIALIZATION_RULES
            .iter()
            .find(|rule| rule.rule_id == "sysml-2.0:8.3.17.10:checkIfActionUsageSpecialization")
            .expect("generated if-action rule");

        assert!(!conditional_library_specialization_predicate_holds(
            &storage,
            id(1),
            rule
        ));
        assert!(conditional_library_specialization_predicate_holds(
            &storage,
            id(2),
            rule
        ));
        assert!(conditional_library_specialization_predicate_holds(
            &storage,
            id(3),
            rule
        ));
        assert_eq!(
            conditional_library_specialization_anchor_branch(&storage, id(2), rule),
            LibrarySpecializationAnchorBranch::Default,
        );
        assert_eq!(
            conditional_library_specialization_anchor_branch(&storage, id(3), rule),
            LibrarySpecializationAnchorBranch::PredicateTrue,
        );
    }

    #[test]
    fn flow_specialization_predicates_use_owned_endpoint_facts_and_suppress_incomplete_forms() {
        let id = |index| DeclarationId::from_index(index).expect("test declaration id");
        let storage = SemanticModelStorage {
            documents: Box::new([]),
            declarations: vec![
                declaration(DocumentId(0), None, None, DeclarationKind::Package),
                declaration(
                    DocumentId(0),
                    Some(id(0)),
                    None,
                    DeclarationKind::FlowDefinition,
                ),
                declaration(
                    DocumentId(0),
                    Some(id(1)),
                    None,
                    DeclarationKind::ConnectionUsage,
                ),
                declaration(
                    DocumentId(0),
                    Some(id(1)),
                    None,
                    DeclarationKind::ConnectionUsage,
                ),
                declaration(
                    DocumentId(0),
                    Some(id(0)),
                    None,
                    DeclarationKind::FlowDefinition,
                ),
                declaration(
                    DocumentId(0),
                    Some(id(4)),
                    None,
                    DeclarationKind::ConnectionUsage,
                ),
                declaration(DocumentId(0), Some(id(0)), None, DeclarationKind::Flow),
                declaration(DocumentId(0), Some(id(0)), None, DeclarationKind::Flow),
            ]
            .into_boxed_slice(),
            declaration_facts: vec![
                DeclarationFacts::none(),
                DeclarationFacts::none(),
                DeclarationFacts {
                    positional_end: Some(0),
                    ..DeclarationFacts::none()
                },
                DeclarationFacts {
                    positional_end: Some(1),
                    ..DeclarationFacts::none()
                },
                DeclarationFacts::none(),
                DeclarationFacts {
                    positional_end: Some(0),
                    ..DeclarationFacts::none()
                },
                DeclarationFacts {
                    owned_end_feature_count: Some(2),
                    ..DeclarationFacts::none()
                },
                DeclarationFacts::none(),
            ]
            .into_boxed_slice(),
            memberships: Box::new([]),
            references: Box::new([]),
            documentation: Box::new([]),
            feature_values: Box::new([]),
            unsupported: Box::new([]),
            recovery: Box::new([]),
            symbols: SymbolTableBuilder::default().freeze(),
            paths: SymbolPathArenaBuilder::default().freeze(),
            evaluation_facts: Box::new([]),
            unit_tokens: Box::new([]),
            filter_conditions: Box::new([]),
            invocations: Box::new([]),
        };
        let rule = |rule_id| {
            GENERATED_CONDITIONAL_LIBRARY_SPECIALIZATION_RULES
                .iter()
                .find(|rule| rule.rule_id == rule_id)
                .expect("generated flow rule")
        };
        let binary = rule("sysml-2.0:8.3.16.2:checkFlowDefinitionBinarySpecialization");
        let flow_usage = rule("sysml-2.0:8.3.16.3:checkFlowUsageFlowSpecialization");
        let flow_with_ends = rule("kerml-1.0:8.3.4.9.2:checkFlowWithEndsSpecialization");

        assert!(conditional_library_specialization_predicate_holds(
            &storage,
            id(1),
            binary
        ));
        assert!(!conditional_library_specialization_predicate_holds(
            &storage,
            id(4),
            binary
        ));
        assert!(conditional_library_specialization_predicate_holds(
            &storage,
            id(6),
            flow_usage
        ));
        assert!(conditional_library_specialization_predicate_holds(
            &storage,
            id(6),
            flow_with_ends
        ));
        assert!(!conditional_library_specialization_predicate_holds(
            &storage,
            id(7),
            flow_usage
        ));
        assert!(!conditional_library_specialization_predicate_holds(
            &storage,
            id(7),
            flow_with_ends
        ));
    }

    #[test]
    fn parser_categories_are_mapped_without_code_or_message_heuristics() {
        assert_eq!(
            parser_diagnostic_category(Some(sysml_v2_parser::DiagnosticCategory::ParseError)),
            DiagnosticCategory::MalformedSyntax
        );
        assert_eq!(
            parser_diagnostic_category(Some(
                sysml_v2_parser::DiagnosticCategory::UnsupportedGrammarForm
            )),
            DiagnosticCategory::UnsupportedSyntax
        );
        assert_eq!(
            parser_diagnostic_category(Some(sysml_v2_parser::DiagnosticCategory::UnresolvedSymbol)),
            DiagnosticCategory::Unresolved
        );
        assert_eq!(
            parser_diagnostic_category(None),
            DiagnosticCategory::UnclassifiedParser
        );
    }

    #[derive(Debug, Clone, Copy)]
    struct TestReference {
        source: DeclarationId,
        kind: ReferenceKind,
        path: SymbolPathId,
        flags: RelationshipFlags,
    }

    impl ResolutionReferenceFact for TestReference {
        fn source(&self) -> DeclarationId {
            self.source
        }

        fn kind(&self) -> ReferenceKind {
            self.kind
        }

        fn path(&self) -> SymbolPathId {
            self.path
        }

        fn flags(&self) -> RelationshipFlags {
            self.flags
        }
    }

    fn declaration(
        document: DocumentId,
        owner: Option<DeclarationId>,
        name: Option<SymbolId>,
        kind: DeclarationKind,
    ) -> Declaration {
        Declaration {
            document,
            owner,
            name,
            anonymous_ordinal: name.is_none().then_some(0),
            kind,
            span: Span::dummy(),
        }
    }

    /// A storage holding nothing but one authored `filter` condition.
    ///
    /// Enough for the evaluation pass, which reads the condition table, the evaluation candidates
    /// and the references, and nothing else.
    fn storage_with_one_filter() -> SemanticModelStorage {
        SemanticModelStorage {
            documents: Box::new([]),
            declarations: Box::new([]),
            declaration_facts: Box::new([]),
            memberships: Box::new([]),
            references: Box::new([]),
            documentation: Box::new([]),
            feature_values: Box::new([]),
            unsupported: Box::new([]),
            recovery: Box::new([]),
            symbols: SymbolTableBuilder::default().freeze(),
            paths: SymbolPathArenaBuilder::default().freeze(),
            evaluation_facts: Box::new([]),
            unit_tokens: Box::new([]),
            filter_conditions: Box::new([AuthoredFilterCondition {
                owner: DeclarationId(0),
                document: DocumentId(0),
                form: FilterForm::View,
                span: Span::dummy(),
                shape: ExpressionEvalShape::Literal(EvaluatedValue::Integer(5)),
                predicate: FilterPredicate::Unsupported,
            }]),
            invocations: Box::new([]),
        }
    }

    /// One semantic storage slice with every typed membership role used by the exact generated
    /// specialization contracts. No parser text or declaration name participates in the test:
    /// applicability and branch selection consume the canonical declaration kind/owner facts.
    fn storage_with_membership_role_specializations() -> SemanticModelStorage {
        let id = |index| DeclarationId::from_index(index).expect("test declaration id");
        SemanticModelStorage {
            documents: Box::new([]),
            declarations: vec![
                declaration(DocumentId(0), None, None, DeclarationKind::Package),
                declaration(
                    DocumentId(0),
                    Some(id(0)),
                    None,
                    DeclarationKind::RequirementDefinition,
                ),
                declaration(DocumentId(0), Some(id(1)), None, DeclarationKind::Frame),
                declaration(
                    DocumentId(0),
                    Some(id(1)),
                    None,
                    DeclarationKind::AssumeConstraintUsage,
                ),
                declaration(
                    DocumentId(0),
                    Some(id(1)),
                    None,
                    DeclarationKind::RequireConstraintUsage,
                ),
                declaration(
                    DocumentId(0),
                    Some(id(1)),
                    None,
                    DeclarationKind::RequirementActor,
                ),
                declaration(
                    DocumentId(0),
                    Some(id(0)),
                    None,
                    DeclarationKind::CaseDefinition,
                ),
                declaration(DocumentId(0), Some(id(6)), None, DeclarationKind::CaseActor),
                declaration(
                    DocumentId(0),
                    Some(id(1)),
                    None,
                    DeclarationKind::StakeholderUsage,
                ),
                declaration(DocumentId(0), Some(id(1)), None, DeclarationKind::PartUsage),
                declaration(
                    DocumentId(0),
                    Some(id(1)),
                    None,
                    DeclarationKind::VerifyRequirement,
                ),
            ]
            .into_boxed_slice(),
            declaration_facts: vec![DeclarationFacts::none(); 11].into_boxed_slice(),
            memberships: Box::new([]),
            references: Box::new([]),
            documentation: Box::new([]),
            feature_values: Box::new([]),
            unsupported: Box::new([]),
            recovery: Box::new([]),
            symbols: SymbolTableBuilder::default().freeze(),
            paths: SymbolPathArenaBuilder::default().freeze(),
            evaluation_facts: Box::new([]),
            unit_tokens: Box::new([]),
            filter_conditions: Box::new([]),
            invocations: Box::new([]),
        }
    }

    /// Canonical fact slice for the three exact `AcceptActionUsage` predicates. It separates the
    /// action's metaclass, its explicit trigger membership fact, and the independently owned
    /// composite/owner facts used by `isSubactionUsage()`.
    fn storage_with_accept_action_specializations() -> SemanticModelStorage {
        let id = |index| DeclarationId::from_index(index).expect("test declaration id");
        let non_trigger = |composite| DeclarationFacts {
            modifiers: DeclarationModifiers {
                composite,
                ..DeclarationModifiers::default()
            },
            is_trigger_action: Some(false),
            ..DeclarationFacts::none()
        };
        SemanticModelStorage {
            documents: Box::new([]),
            declarations: vec![
                declaration(DocumentId(0), None, None, DeclarationKind::Package),
                declaration(
                    DocumentId(0),
                    Some(id(0)),
                    None,
                    DeclarationKind::AcceptActionUsage,
                ),
                declaration(
                    DocumentId(0),
                    Some(id(0)),
                    None,
                    DeclarationKind::ActionDefinition,
                ),
                declaration(
                    DocumentId(0),
                    Some(id(2)),
                    None,
                    DeclarationKind::AcceptActionUsage,
                ),
                declaration(
                    DocumentId(0),
                    Some(id(0)),
                    None,
                    DeclarationKind::Transition,
                ),
                declaration(
                    DocumentId(0),
                    Some(id(4)),
                    None,
                    DeclarationKind::AcceptActionUsage,
                ),
            ]
            .into_boxed_slice(),
            declaration_facts: vec![
                DeclarationFacts::none(),
                non_trigger(true),
                DeclarationFacts::none(),
                non_trigger(true),
                DeclarationFacts::none(),
                DeclarationFacts {
                    modifiers: DeclarationModifiers {
                        composite: true,
                        ..DeclarationModifiers::default()
                    },
                    is_trigger_action: Some(true),
                    ..DeclarationFacts::none()
                },
            ]
            .into_boxed_slice(),
            memberships: Box::new([]),
            references: Box::new([]),
            documentation: Box::new([]),
            feature_values: Box::new([]),
            unsupported: Box::new([]),
            recovery: Box::new([]),
            symbols: SymbolTableBuilder::default().freeze(),
            paths: SymbolPathArenaBuilder::default().freeze(),
            evaluation_facts: Box::new([]),
            unit_tokens: Box::new([]),
            filter_conditions: Box::new([]),
            invocations: Box::new([]),
        }
    }

    fn resolution_with_status(status: SolverStatus) -> ResolutionResults {
        ResolutionResults {
            outcomes: Box::new([]),
            ambiguous_candidates: Box::new([]),
            inherited_names: NameIndex::build(Vec::new()).unwrap(),
            solver_status: status,
            implied_relationships: Box::new([]),
            library_specialization_anchors: LibrarySpecializationAnchorFacts::default(),
            work: ResolutionWork::default(),
        }
    }

    /// A converged publication settles every authored condition.
    #[test]
    fn every_authored_filter_condition_settles_when_resolution_converges() {
        let storage = storage_with_one_filter();
        let settled = compute_evaluation(
            &storage,
            &resolution_with_status(SolverStatus::Converged),
            EvaluationPolicy::Evaluate,
        );
        let SettledEvaluation::Settled { filters, .. } = settled else {
            panic!("a converged publication settles its filter conditions");
        };
        assert_eq!(filters.len(), storage.filter_conditions.len());
        assert_eq!(
            filters[0].state,
            EvaluationState::Literal(crate::evaluation::EvaluatedScalar::Integer(5))
        );
    }

    /// A publication whose resolution did not converge has no outcomes to publish, and saying so
    /// is not the same as failing to build.
    ///
    /// The evaluation pass and the filter table used to be joined by index, so the branch that
    /// could not produce an outcome per condition published none and the join rejected the whole
    /// publication -- turning an explicitly supported incomplete state into a construction error
    /// for any model that authored a `filter`.
    #[test]
    fn a_non_converged_publication_settles_nothing_rather_than_failing() {
        let storage = storage_with_one_filter();
        let settled = compute_evaluation(
            &storage,
            &resolution_with_status(SolverStatus::NonConverged),
            EvaluationPolicy::Evaluate,
        );
        assert!(
            matches!(settled, SettledEvaluation::Vacuous),
            "a non-converged publication must settle no expression outcome"
        );
    }

    fn reference(
        source: DeclarationId,
        kind: ReferenceKind,
        path: SymbolPathId,
        wildcard: bool,
    ) -> TestReference {
        TestReference {
            source,
            kind,
            path,
            flags: RelationshipFlags {
                wildcard,
                ..RelationshipFlags::default()
            },
        }
    }

    struct ResolverFixture {
        declarations: Box<[Declaration]>,
        memberships: Box<[MembershipRecord]>,
        paths: SymbolPathArena,
        references: Box<[TestReference]>,
    }

    fn memberships_for(
        declarations: &[Declaration],
        public_imports: &[DeclarationId],
    ) -> Box<[MembershipRecord]> {
        declarations
            .iter()
            .enumerate()
            .map(|(index, declaration)| {
                let member = DeclarationId::from_index(index).unwrap();
                MembershipRecord {
                    member,
                    kind: if declaration.kind == DeclarationKind::Import {
                        MembershipKind::Import
                    } else {
                        MembershipKind::Owning
                    },
                    visibility: if public_imports.contains(&member) {
                        Visibility::Public
                    } else {
                        Visibility::Default
                    },
                    span: Span::dummy(),
                }
            })
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn cross_file_fixture(duplicate_vehicle: bool) -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let definitions_name = symbols.intern("Definitions").unwrap();
        let usage_name = symbols.intern("Usage").unwrap();
        let vehicle_name = symbols.intern("Vehicle").unwrap();
        let v_name = symbols.intern("v").unwrap();

        let mut paths = SymbolPathArenaBuilder::default();
        let definitions_path = paths.push(&[definitions_name], false).unwrap();
        let vehicle_path = paths.push(&[vehicle_name], false).unwrap();

        let definition_document = DocumentId(0);
        let usage_document = DocumentId(1);
        let definitions = DeclarationId(0);
        let usage = DeclarationId(2);
        let import = DeclarationId(3);
        let v = DeclarationId(4);
        let mut declarations = vec![
            declaration(
                definition_document,
                None,
                Some(definitions_name),
                DeclarationKind::Package,
            ),
            declaration(
                definition_document,
                Some(definitions),
                Some(vehicle_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                usage_document,
                None,
                Some(usage_name),
                DeclarationKind::Package,
            ),
            declaration(usage_document, Some(usage), None, DeclarationKind::Import),
            declaration(
                usage_document,
                Some(usage),
                Some(v_name),
                DeclarationKind::PartUsage,
            ),
        ];
        if duplicate_vehicle {
            declarations.push(declaration(
                definition_document,
                Some(definitions),
                Some(vehicle_name),
                DeclarationKind::PartDefinition,
            ));
        }

        let references = vec![
            reference(
                import,
                ReferenceKind::NamespaceImport,
                definitions_path,
                true,
            ),
            reference(v, ReferenceKind::FeatureTyping, vehicle_path, false),
        ];
        let _symbols = symbols.freeze();
        let memberships = memberships_for(&declarations, &[]);
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    fn resolve_fixture(
        fixture: &ResolverFixture,
    ) -> (NameIndex, NameIndex, MembershipIndex, ResolutionResults) {
        resolve_dense(
            &fixture.declarations,
            &fixture.memberships,
            &fixture.paths,
            &fixture.references,
            None,
        )
        .unwrap()
    }

    fn transitive_import_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let a_name = symbols.intern("A").unwrap();
        let b_name = symbols.intern("B").unwrap();
        let c_name = symbols.intern("C").unwrap();
        let use_name = symbols.intern("Use").unwrap();
        let thing_name = symbols.intern("Thing").unwrap();
        let v_name = symbols.intern("v").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let a_path = paths.push(&[a_name], false).unwrap();
        let b_path = paths.push(&[b_name], false).unwrap();
        let c_path = paths.push(&[c_name], false).unwrap();
        let thing_path = paths.push(&[thing_name], false).unwrap();

        let declarations = vec![
            declaration(DocumentId(0), None, Some(a_name), DeclarationKind::Package),
            declaration(
                DocumentId(0),
                Some(DeclarationId(0)),
                Some(thing_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(DocumentId(1), None, Some(b_name), DeclarationKind::Package),
            declaration(
                DocumentId(1),
                Some(DeclarationId(2)),
                None,
                DeclarationKind::Import,
            ),
            declaration(DocumentId(2), None, Some(c_name), DeclarationKind::Package),
            declaration(
                DocumentId(2),
                Some(DeclarationId(4)),
                None,
                DeclarationKind::Import,
            ),
            declaration(
                DocumentId(3),
                None,
                Some(use_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(3),
                Some(DeclarationId(6)),
                None,
                DeclarationKind::Import,
            ),
            declaration(
                DocumentId(3),
                Some(DeclarationId(6)),
                Some(v_name),
                DeclarationKind::PartUsage,
            ),
        ];
        let references = vec![
            reference(
                DeclarationId(3),
                ReferenceKind::NamespaceImport,
                a_path,
                true,
            ),
            reference(
                DeclarationId(5),
                ReferenceKind::NamespaceImport,
                b_path,
                true,
            ),
            reference(
                DeclarationId(7),
                ReferenceKind::NamespaceImport,
                c_path,
                true,
            ),
            reference(
                DeclarationId(8),
                ReferenceKind::FeatureTyping,
                thing_path,
                false,
            ),
        ];
        let _symbols = symbols.freeze();
        let memberships = memberships_for(&declarations, &[DeclarationId(3), DeclarationId(5)]);
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    fn imported_target_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let a_name = symbols.intern("A").unwrap();
        let nested_name = symbols.intern("Nested").unwrap();
        let thing_name = symbols.intern("Thing").unwrap();
        let b_name = symbols.intern("B").unwrap();
        let v_name = symbols.intern("v").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let a_path = paths.push(&[a_name], false).unwrap();
        let nested_path = paths.push(&[nested_name], false).unwrap();
        let thing_path = paths.push(&[thing_name], false).unwrap();

        let declarations = vec![
            declaration(DocumentId(0), None, Some(a_name), DeclarationKind::Package),
            declaration(
                DocumentId(0),
                Some(DeclarationId(0)),
                Some(nested_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(0),
                Some(DeclarationId(1)),
                Some(thing_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(DocumentId(1), None, Some(b_name), DeclarationKind::Package),
            declaration(
                DocumentId(1),
                Some(DeclarationId(3)),
                None,
                DeclarationKind::Import,
            ),
            declaration(
                DocumentId(1),
                Some(DeclarationId(3)),
                None,
                DeclarationKind::Import,
            ),
            declaration(
                DocumentId(1),
                Some(DeclarationId(3)),
                Some(v_name),
                DeclarationKind::PartUsage,
            ),
        ];
        let references = vec![
            reference(
                DeclarationId(4),
                ReferenceKind::NamespaceImport,
                a_path,
                true,
            ),
            reference(
                DeclarationId(5),
                ReferenceKind::NamespaceImport,
                nested_path,
                true,
            ),
            reference(
                DeclarationId(6),
                ReferenceKind::FeatureTyping,
                thing_path,
                false,
            ),
        ];
        let _symbols = symbols.freeze();
        let memberships = memberships_for(&declarations, &[]);
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    fn cyclic_import_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let a_name = symbols.intern("A").unwrap();
        let b_name = symbols.intern("B").unwrap();
        let type_a_name = symbols.intern("TypeA").unwrap();
        let type_b_name = symbols.intern("TypeB").unwrap();
        let usage_name = symbols.intern("value").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let b_path = paths.push(&[b_name], false).unwrap();
        let a_path = paths.push(&[a_name], false).unwrap();
        let type_b_path = paths.push(&[type_b_name], false).unwrap();

        let declarations = vec![
            declaration(DocumentId(0), None, Some(a_name), DeclarationKind::Package),
            declaration(
                DocumentId(0),
                Some(DeclarationId(0)),
                Some(type_a_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(DeclarationId(0)),
                None,
                DeclarationKind::Import,
            ),
            declaration(
                DocumentId(0),
                Some(DeclarationId(0)),
                Some(usage_name),
                DeclarationKind::PartUsage,
            ),
            declaration(DocumentId(1), None, Some(b_name), DeclarationKind::Package),
            declaration(
                DocumentId(1),
                Some(DeclarationId(4)),
                Some(type_b_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentId(1),
                Some(DeclarationId(4)),
                None,
                DeclarationKind::Import,
            ),
        ];
        let references = vec![
            reference(
                DeclarationId(2),
                ReferenceKind::NamespaceImport,
                b_path,
                true,
            ),
            reference(
                DeclarationId(6),
                ReferenceKind::NamespaceImport,
                a_path,
                true,
            ),
            reference(
                DeclarationId(3),
                ReferenceKind::FeatureTyping,
                type_b_path,
                false,
            ),
        ];
        let _symbols = symbols.freeze();
        let memberships = memberships_for(&declarations, &[DeclarationId(2), DeclarationId(6)]);
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    fn qualified_import_target_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let a_name = symbols.intern("A").unwrap();
        let nested_name = symbols.intern("Nested").unwrap();
        let thing_name = symbols.intern("Thing").unwrap();
        let b_name = symbols.intern("B").unwrap();
        let use_name = symbols.intern("Use").unwrap();
        let v_name = symbols.intern("v").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let a_path = paths.push(&[a_name], false).unwrap();
        let qualified_nested_path = paths.push(&[b_name, nested_name], false).unwrap();
        let thing_path = paths.push(&[thing_name], false).unwrap();

        let declarations = vec![
            declaration(DocumentId(0), None, Some(a_name), DeclarationKind::Package),
            declaration(
                DocumentId(0),
                Some(DeclarationId(0)),
                Some(nested_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(0),
                Some(DeclarationId(1)),
                Some(thing_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(DocumentId(1), None, Some(b_name), DeclarationKind::Package),
            declaration(
                DocumentId(1),
                Some(DeclarationId(3)),
                None,
                DeclarationKind::Import,
            ),
            declaration(
                DocumentId(2),
                None,
                Some(use_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(2),
                Some(DeclarationId(5)),
                None,
                DeclarationKind::Import,
            ),
            declaration(
                DocumentId(2),
                Some(DeclarationId(5)),
                Some(v_name),
                DeclarationKind::PartUsage,
            ),
        ];
        let references = vec![
            reference(
                DeclarationId(4),
                ReferenceKind::NamespaceImport,
                a_path,
                true,
            ),
            reference(
                DeclarationId(6),
                ReferenceKind::NamespaceImport,
                qualified_nested_path,
                true,
            ),
            reference(
                DeclarationId(7),
                ReferenceKind::FeatureTyping,
                thing_path,
                false,
            ),
        ];
        let _symbols = symbols.freeze();
        let memberships = memberships_for(&declarations, &[DeclarationId(4)]);
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    fn redefinition_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let p_name = symbols.intern("P").unwrap();
        let base_name = symbols.intern("Base").unwrap();
        let child_name = symbols.intern("Child").unwrap();
        let mass_name = symbols.intern("mass").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let base_path = paths.push(&[base_name], false).unwrap();

        let package = DeclarationId(0);
        let base = DeclarationId(1);
        let child = DeclarationId(3);
        let declarations = vec![
            declaration(DocumentId(0), None, Some(p_name), DeclarationKind::Package),
            declaration(
                DocumentId(0),
                Some(package),
                Some(base_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(base),
                Some(mass_name),
                DeclarationKind::AttributeUsage,
            ),
            declaration(
                DocumentId(0),
                Some(package),
                Some(child_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(child),
                Some(mass_name),
                DeclarationKind::AttributeUsage,
            ),
        ];
        let memberships = declarations
            .iter()
            .enumerate()
            .map(|(index, declaration)| {
                let member = DeclarationId::from_index(index).unwrap();
                let kind = if matches!(
                    declaration.kind,
                    DeclarationKind::AttributeUsage | DeclarationKind::PartUsage
                ) {
                    MembershipKind::Feature
                } else {
                    MembershipKind::Owning
                };
                MembershipRecord {
                    member,
                    kind,
                    visibility: Visibility::Default,
                    span: Span::dummy(),
                }
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let references = vec![reference(
            child,
            ReferenceKind::Subclassification,
            base_path,
            false,
        )];
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn same_name_direct_parent_feature_synthesizes_implied_redefinition() {
        let fixture = redefinition_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.implied_relationships.as_ref(),
            &[ImpliedRelationship {
                kind: ReferenceKind::Redefinition,
                source: DeclarationId(4),
                target: DeclarationId(2),
            }]
        );
    }

    #[test]
    fn explicit_redefinition_suppresses_implied_duplicate() {
        let mut fixture = redefinition_fixture();
        let mut references = fixture.references.into_vec();
        references.push(reference(
            DeclarationId(4),
            ReferenceKind::Redefinition,
            references[0].path,
            false,
        ));
        fixture.references = references.into_boxed_slice();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert!(resolution.implied_relationships.is_empty());
    }

    #[test]
    fn ambiguous_immediate_parent_candidates_leave_no_implied_fact() {
        let mut fixture = redefinition_fixture();
        let mut declarations = fixture.declarations.into_vec();
        // A second directly owned `mass` feature on Base makes the immediate-parent same-name
        // lookup ambiguous; the synthesis must not guess a target.
        declarations.push(declaration(
            DocumentId(0),
            Some(DeclarationId(1)),
            Some(declarations[2].name.unwrap()),
            DeclarationKind::AttributeUsage,
        ));
        fixture.declarations = declarations.into_boxed_slice();
        let mut memberships = fixture.memberships.into_vec();
        memberships.push(MembershipRecord {
            member: DeclarationId(5),
            kind: MembershipKind::Feature,
            visibility: Visibility::Default,
            span: Span::dummy(),
        });
        fixture.memberships = memberships.into_boxed_slice();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert!(resolution.implied_relationships.is_empty());
    }

    /// Like `redefinition_fixture`, but exposes an unqualified single-segment `mass` symbol path
    /// (in addition to the `Base` path used for the Subclassification reference) so callers can
    /// author an unqualified `Subsetting`/`Redefinition` reference to the inherited `mass`
    /// feature, resolved through lexical/ancestor lookup rather than a qualified path. `Child`'s
    /// own redefining/subsetting attribute (`DeclarationId(4)`) is deliberately left unnamed --
    /// matching an authored `attribute :>> mass = ...;`/`attribute :> mass;`, whose usage has no
    /// name of its own -- so it is never itself indexed under the `mass` name and cannot shadow
    /// the inherited `Base::mass` target it is trying to reach.
    fn redefinition_fixture_with_mass_path() -> (ResolverFixture, SymbolPathId) {
        let mut symbols = SymbolTableBuilder::default();
        let p_name = symbols.intern("P").unwrap();
        let base_name = symbols.intern("Base").unwrap();
        let child_name = symbols.intern("Child").unwrap();
        let mass_name = symbols.intern("mass").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let base_path = paths.push(&[base_name], false).unwrap();
        let mass_path = paths.push(&[mass_name], false).unwrap();

        let package = DeclarationId(0);
        let base = DeclarationId(1);
        let child = DeclarationId(3);
        let declarations = vec![
            declaration(DocumentId(0), None, Some(p_name), DeclarationKind::Package),
            declaration(
                DocumentId(0),
                Some(package),
                Some(base_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(base),
                Some(mass_name),
                DeclarationKind::AttributeUsage,
            ),
            declaration(
                DocumentId(0),
                Some(package),
                Some(child_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(child),
                None,
                DeclarationKind::AttributeUsage,
            ),
        ];
        let memberships = declarations
            .iter()
            .enumerate()
            .map(|(index, declaration)| {
                let member = DeclarationId::from_index(index).unwrap();
                let kind = if matches!(
                    declaration.kind,
                    DeclarationKind::AttributeUsage | DeclarationKind::PartUsage
                ) {
                    MembershipKind::Feature
                } else {
                    MembershipKind::Owning
                };
                MembershipRecord {
                    member,
                    kind,
                    visibility: Visibility::Default,
                    span: Span::dummy(),
                }
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let references = vec![reference(
            child,
            ReferenceKind::Subclassification,
            base_path,
            false,
        )];
        let _symbols = symbols.freeze();
        (
            ResolverFixture {
                declarations: declarations.into_boxed_slice(),
                memberships,
                paths: paths.freeze(),
                references: references.into_boxed_slice(),
            },
            mass_path,
        )
    }

    #[test]
    fn explicit_redefinition_resolves_through_inherited_ancestor_lookup() {
        // `attribute :>> mass = ...;` on `Child` (which specializes `Base`) must resolve its
        // authored `Redefinition` reference against `Base::mass`, an inherited member reachable
        // only through the ancestor-closure lookup built for Subclassification -- not just a
        // directly owned member of `Child` itself.
        let (mut fixture, mass_path) = redefinition_fixture_with_mass_path();
        let mut references = fixture.references.into_vec();
        let redefinition_index = u32::try_from(references.len()).unwrap();
        references.push(reference(
            DeclarationId(4),
            ReferenceKind::Redefinition,
            mass_path,
            false,
        ));
        fixture.references = references.into_boxed_slice();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(redefinition_index)),
            Some(ResolutionStatus::Resolved(DeclarationId(2)))
        );
    }

    #[test]
    fn subsetting_reference_resolves_to_an_inherited_feature_target() {
        // `attribute simpleMass :> mass;` subsets another *feature*, not a type/definition, so
        // `Subsetting` must resolve against `DeclarationDomain::Any` rather than the
        // Subclassification/FeatureTyping `Type` domain, and must reach the inherited `Base::mass`
        // feature through the same ancestor-scoped inherited lookup used by
        // `FeatureTyping`/`Redefinition`.
        let (mut fixture, mass_path) = redefinition_fixture_with_mass_path();
        let mut references = fixture.references.into_vec();
        let subsetting_index = u32::try_from(references.len()).unwrap();
        references.push(reference(
            DeclarationId(4),
            ReferenceKind::Subsetting,
            mass_path,
            false,
        ));
        fixture.references = references.into_boxed_slice();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(subsetting_index)),
            Some(ResolutionStatus::Resolved(DeclarationId(2)))
        );
    }

    #[test]
    fn redefinition_inside_a_usage_body_resolves_through_the_usages_feature_typing_target() {
        // `need : Need { attribute :>> status = ...; }` -- the redefining attribute is owned by a
        // *usage* (`need`), not a def/type, so it has no Subclassification ancestors of its own.
        // `status` is only reachable by first following `need`'s own `FeatureTyping` reference to
        // `Need`, then walking `Need`'s ancestor closure (`Need -> UserRequirement ->
        // ManagedRequirement`) to find `ManagedRequirement::status`. Mirrors
        // tests/snapshots/resolution/enum_status_redefinition.md.
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let managed_requirement_name = symbols.intern("ManagedRequirement").unwrap();
        let status_name = symbols.intern("status").unwrap();
        let user_requirement_name = symbols.intern("UserRequirement").unwrap();
        let need_def_name = symbols.intern("Need").unwrap();
        let need_usage_name = symbols.intern("need").unwrap();

        let mut paths = SymbolPathArenaBuilder::default();
        let managed_requirement_path = paths.push(&[managed_requirement_name], false).unwrap();
        let user_requirement_path = paths.push(&[user_requirement_name], false).unwrap();
        let need_def_path = paths.push(&[need_def_name], false).unwrap();
        let status_path = paths.push(&[status_name], false).unwrap();

        let demo = DeclarationId(0);
        let managed_requirement = DeclarationId(1);
        let status = DeclarationId(2);
        let user_requirement = DeclarationId(3);
        let need_def = DeclarationId(4);
        let need_usage = DeclarationId(5);
        let declarations = vec![
            declaration(
                DocumentId(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(managed_requirement_name),
                DeclarationKind::RequirementDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(managed_requirement),
                Some(status_name),
                DeclarationKind::AttributeUsage,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(user_requirement_name),
                DeclarationKind::RequirementDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(need_def_name),
                DeclarationKind::RequirementDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(need_usage_name),
                DeclarationKind::RequirementUsage,
            ),
            declaration(
                DocumentId(0),
                Some(need_usage),
                None,
                DeclarationKind::AttributeUsage,
            ),
        ];
        let memberships = declarations
            .iter()
            .enumerate()
            .map(|(index, declaration)| {
                let member = DeclarationId::from_index(index).unwrap();
                let kind = if matches!(
                    declaration.kind,
                    DeclarationKind::AttributeUsage | DeclarationKind::RequirementUsage
                ) {
                    MembershipKind::Feature
                } else {
                    MembershipKind::Owning
                };
                MembershipRecord {
                    member,
                    kind,
                    visibility: Visibility::Default,
                    span: Span::dummy(),
                }
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();

        let references = vec![
            reference(
                user_requirement,
                ReferenceKind::Subclassification,
                managed_requirement_path,
                false,
            ),
            reference(
                need_def,
                ReferenceKind::Subclassification,
                user_requirement_path,
                false,
            ),
            reference(
                need_usage,
                ReferenceKind::FeatureTyping,
                need_def_path,
                false,
            ),
            reference(
                DeclarationId(6),
                ReferenceKind::Redefinition,
                status_path,
                false,
            ),
        ];
        let redefinition_index = u32::try_from(references.len() - 1).unwrap();

        let _symbols = symbols.freeze();
        let fixture = ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        };

        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(redefinition_index)),
            Some(ResolutionStatus::Resolved(status))
        );
    }

    #[test]
    fn qualified_reference_resolves_to_an_enum_defs_owned_literal_member() {
        // `enum def StatusKind { enum approved; }` -- StatusKind::approved is looked up through
        // exactly the same generic multi-segment lexical lookup as any other owned member; no
        // enum-specific resolver code is needed once EnumerationDefinition/EnumerationLiteral are
        // lowered as ordinary owned declarations.
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let status_kind_name = symbols.intern("StatusKind").unwrap();
        let approved_name = symbols.intern("approved").unwrap();
        let alias_name = symbols.intern("aliasToApproved").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let qualified_path = paths
            .push(&[status_kind_name, approved_name], false)
            .unwrap();

        let demo = DeclarationId(0);
        let status_kind = DeclarationId(1);
        let approved = DeclarationId(2);
        let alias = DeclarationId(3);
        let declarations = vec![
            declaration(
                DocumentId(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(status_kind_name),
                DeclarationKind::EnumerationDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(status_kind),
                Some(approved_name),
                DeclarationKind::EnumerationLiteral,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(alias_name),
                DeclarationKind::Alias,
            ),
        ];
        let memberships = declarations
            .iter()
            .enumerate()
            .map(|(index, declaration)| {
                let member = DeclarationId::from_index(index).unwrap();
                let kind = match declaration.kind {
                    DeclarationKind::EnumerationLiteral => MembershipKind::Feature,
                    DeclarationKind::Alias => MembershipKind::Alias,
                    _ => MembershipKind::Owning,
                };
                // Interior/final segments of a multi-segment qualified name are looked up through
                // the exported-names index (`build_effective_import_indexes`'s sibling,
                // `build_direct_name_index(.., Some(&memberships))`), which only admits publicly
                // visible members -- the same rule every other owned-member kind is subject to, not
                // an enum-specific one. Publicize the literal explicitly here to exercise that
                // generic path rather than asserting anything enum-specific about visibility
                // defaults.
                let visibility = if declaration.kind == DeclarationKind::EnumerationLiteral {
                    Visibility::Public
                } else {
                    Visibility::Default
                };
                MembershipRecord {
                    member,
                    kind,
                    visibility,
                    span: Span::dummy(),
                }
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let references = vec![reference(
            alias,
            ReferenceKind::AliasBinding,
            qualified_path,
            false,
        )];
        let _symbols = symbols.freeze();
        let fixture = ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        };

        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(approved))
        );
    }

    /// Builds a `Demo { port def Base; port def Derived :> Base; }`-shaped fixture: `Derived`'s
    /// `:>` specialization reference is authored with `conjugated` set per `typing_conjugated`,
    /// exercising `port def`'s participation in the shared Subclassification/FeatureTyping
    /// lexical lookup fixed point (`DeclarationDomain::Type`) exactly like `part def`.
    fn port_def_specialization_fixture(typing_conjugated: bool) -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let base_name = symbols.intern("Base").unwrap();
        let derived_name = symbols.intern("Derived").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let base_path = paths.push(&[base_name], false).unwrap();

        let demo = DeclarationId(0);
        let derived = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentId(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(base_name),
                DeclarationKind::PortDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(derived_name),
                DeclarationKind::PortDefinition,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let references = vec![TestReference {
            source: derived,
            kind: ReferenceKind::Subclassification,
            path: base_path,
            flags: RelationshipFlags {
                conjugated: typing_conjugated,
                ..RelationshipFlags::default()
            },
        }];
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn port_def_specialization_resolves_through_the_ancestor_fixed_point() {
        let fixture = port_def_specialization_fixture(false);
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    /// Builds a `Demo { occurrence def Base; occurrence def Derived :> Base; }`-shaped fixture,
    /// exercising `occurrence def`'s participation in the shared Subclassification/FeatureTyping
    /// lexical lookup fixed point (`DeclarationDomain::Type`) exactly like `port def`/`state def`.
    fn occurrence_def_specialization_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let base_name = symbols.intern("Base").unwrap();
        let derived_name = symbols.intern("Derived").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let base_path = paths.push(&[base_name], false).unwrap();

        let demo = DeclarationId(0);
        let derived = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentId(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(base_name),
                DeclarationKind::OccurrenceDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(derived_name),
                DeclarationKind::OccurrenceDefinition,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let references = vec![TestReference {
            source: derived,
            kind: ReferenceKind::Subclassification,
            path: base_path,
            flags: RelationshipFlags::default(),
        }];
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn occurrence_def_specialization_resolves_through_the_ancestor_fixed_point() {
        let fixture = occurrence_def_specialization_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    /// Builds a `Demo { analysis def Base; analysis def Derived :> Base; }`-shaped fixture,
    /// exercising `analysis def`'s participation in the shared Subclassification/FeatureTyping
    /// lexical lookup fixed point (`DeclarationDomain::Type`) exactly like `occurrence def`.
    fn analysis_case_def_specialization_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let base_name = symbols.intern("Base").unwrap();
        let derived_name = symbols.intern("Derived").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let base_path = paths.push(&[base_name], false).unwrap();

        let demo = DeclarationId(0);
        let derived = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentId(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(base_name),
                DeclarationKind::AnalysisCaseDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(derived_name),
                DeclarationKind::AnalysisCaseDefinition,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let references = vec![TestReference {
            source: derived,
            kind: ReferenceKind::Subclassification,
            path: base_path,
            flags: RelationshipFlags::default(),
        }];
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn analysis_case_def_specialization_resolves_through_the_ancestor_fixed_point() {
        let fixture = analysis_case_def_specialization_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    /// Builds a `Demo { Base; Derived :> Base; }`-shaped fixture for the given case-family
    /// `DeclarationKind` (`CaseDefinition`/`VerificationCaseDefinition`/`UseCaseDefinition`),
    /// exercising its participation in the shared Subclassification/FeatureTyping lexical lookup
    /// fixed point (`DeclarationDomain::Type`) exactly like `analysis def`/`occurrence def`.
    fn case_family_def_specialization_fixture(kind: DeclarationKind) -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let base_name = symbols.intern("Base").unwrap();
        let derived_name = symbols.intern("Derived").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let base_path = paths.push(&[base_name], false).unwrap();

        let demo = DeclarationId(0);
        let derived = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentId(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(DocumentId(0), Some(demo), Some(base_name), kind),
            declaration(DocumentId(0), Some(demo), Some(derived_name), kind),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let references = vec![TestReference {
            source: derived,
            kind: ReferenceKind::Subclassification,
            path: base_path,
            flags: RelationshipFlags::default(),
        }];
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn case_def_specialization_resolves_through_the_ancestor_fixed_point() {
        let fixture = case_family_def_specialization_fixture(DeclarationKind::CaseDefinition);
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    #[test]
    fn verification_case_def_specialization_resolves_through_the_ancestor_fixed_point() {
        let fixture =
            case_family_def_specialization_fixture(DeclarationKind::VerificationCaseDefinition);
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    #[test]
    fn use_case_def_specialization_resolves_through_the_ancestor_fixed_point() {
        let fixture = case_family_def_specialization_fixture(DeclarationKind::UseCaseDefinition);
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    #[test]
    fn conjugated_port_typing_reference_resolves_to_the_correct_target_and_carries_the_flag() {
        // `port source : ~InputPort;` -- the conjugated `~` polarity must be visible as an
        // explicit fact on the authored reference, distinct from the (unconjugated) target
        // declaration itself, which the resolved outcome still names correctly.
        let fixture = port_def_specialization_fixture(true);
        assert!(fixture.references[0].flags.conjugated);
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    #[test]
    fn non_conjugated_port_typing_reference_does_not_carry_the_conjugated_flag() {
        // Regression guard: an ordinary (non-`~`) port typing/specialization reference must not
        // spuriously pick up the conjugated flag.
        let fixture = port_def_specialization_fixture(false);
        assert!(!fixture.references[0].flags.conjugated);
    }

    /// Builds a `Demo { item def Base; item def Derived :> Base; }`-shaped fixture: `Derived`'s
    /// `:>` specialization reference exercises `item def`'s participation in the shared
    /// Subclassification/FeatureTyping lexical lookup fixed point (`DeclarationDomain::Type`)
    /// exactly like `part def`/`port def`.
    fn item_def_specialization_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let base_name = symbols.intern("Base").unwrap();
        let derived_name = symbols.intern("Derived").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let base_path = paths.push(&[base_name], false).unwrap();

        let demo = DeclarationId(0);
        let derived = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentId(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(base_name),
                DeclarationKind::ItemDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(derived_name),
                DeclarationKind::ItemDefinition,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let references = vec![TestReference {
            source: derived,
            kind: ReferenceKind::Subclassification,
            path: base_path,
            flags: RelationshipFlags::default(),
        }];
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn item_def_specialization_resolves_through_the_ancestor_fixed_point() {
        let fixture = item_def_specialization_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    /// Builds a `Demo { class Base; class Derived :> Base; }`-shaped fixture: `Derived`'s `:>`
    /// specialization reference exercises KerML `class def`'s participation in the shared
    /// Subclassification/FeatureTyping lexical lookup fixed point (`DeclarationDomain::Type`)
    /// exactly like `item def`.
    fn class_def_specialization_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let base_name = symbols.intern("Base").unwrap();
        let derived_name = symbols.intern("Derived").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let base_path = paths.push(&[base_name], false).unwrap();

        let demo = DeclarationId(0);
        let derived = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentId(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(base_name),
                DeclarationKind::ClassDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(derived_name),
                DeclarationKind::ClassDefinition,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let references = vec![TestReference {
            source: derived,
            kind: ReferenceKind::Subclassification,
            path: base_path,
            flags: RelationshipFlags::default(),
        }];
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn class_def_specialization_resolves_through_the_ancestor_fixed_point() {
        let fixture = class_def_specialization_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    /// Builds a `Demo { action def Base; action def Derived :> Base; }`-shaped fixture: `Derived`'s
    /// `:>` specialization reference exercises `action def`'s participation in the shared
    /// Subclassification/FeatureTyping lexical lookup fixed point (`DeclarationDomain::Type`)
    /// exactly like `item def`/`part def`/`port def`.
    fn action_def_specialization_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let base_name = symbols.intern("Base").unwrap();
        let derived_name = symbols.intern("Derived").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let base_path = paths.push(&[base_name], false).unwrap();

        let demo = DeclarationId(0);
        let derived = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentId(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(base_name),
                DeclarationKind::ActionDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(derived_name),
                DeclarationKind::ActionDefinition,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let references = vec![TestReference {
            source: derived,
            kind: ReferenceKind::Subclassification,
            path: base_path,
            flags: RelationshipFlags::default(),
        }];
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn action_def_specialization_resolves_through_the_ancestor_fixed_point() {
        let fixture = action_def_specialization_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    /// Builds a `Demo { state def Base; state def Derived :> Base; }`-shaped fixture: `Derived`'s
    /// `:>` specialization reference exercises `state def`'s participation in the shared
    /// Subclassification/FeatureTyping lexical lookup fixed point (`DeclarationDomain::Type`)
    /// exactly like `action def`/`item def`/`part def`/`port def`.
    fn state_def_specialization_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let base_name = symbols.intern("Base").unwrap();
        let derived_name = symbols.intern("Derived").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let base_path = paths.push(&[base_name], false).unwrap();

        let demo = DeclarationId(0);
        let derived = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentId(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(base_name),
                DeclarationKind::StateDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(derived_name),
                DeclarationKind::StateDefinition,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let references = vec![TestReference {
            source: derived,
            kind: ReferenceKind::Subclassification,
            path: base_path,
            flags: RelationshipFlags::default(),
        }];
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn state_def_specialization_resolves_through_the_ancestor_fixed_point() {
        let fixture = state_def_specialization_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    /// Builds a `Demo { metadata def Base; metadata def Derived :> Base; }`-shaped fixture:
    /// `Derived`'s `:>` specialization reference exercises `metadata def`'s participation in the
    /// shared Subclassification/FeatureTyping lexical lookup fixed point
    /// (`DeclarationDomain::Type`) exactly like `item def`/`action def`/`part def`/`port def`.
    fn metadata_def_specialization_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let base_name = symbols.intern("Base").unwrap();
        let derived_name = symbols.intern("Derived").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let base_path = paths.push(&[base_name], false).unwrap();

        let demo = DeclarationId(0);
        let derived = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentId(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(base_name),
                DeclarationKind::MetadataDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(derived_name),
                DeclarationKind::MetadataDefinition,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let references = vec![TestReference {
            source: derived,
            kind: ReferenceKind::Subclassification,
            path: base_path,
            flags: RelationshipFlags::default(),
        }];
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn metadata_def_specialization_resolves_through_the_ancestor_fixed_point() {
        let fixture = metadata_def_specialization_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    /// Builds a `Demo { metadata def Safety; part seatBelt {@Safety;} }`-shaped fixture: the part
    /// usage's `@Safety` metadata annotation exercises `ReferenceKind::MetadataAnnotation`'s
    /// participation in the shared Subclassification/FeatureTyping lexical lookup fixed point
    /// (`DeclarationDomain::Type`), sourced directly at the part usage's own declaration (not an
    /// anonymous nested feature).
    fn metadata_annotation_fixture(target_name: &str) -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let safety_name = symbols.intern("Safety").unwrap();
        let seat_belt_name = symbols.intern("seatBelt").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let target_symbol = symbols.intern(target_name).unwrap();
        let target_path = paths.push(&[target_symbol], false).unwrap();

        let demo = DeclarationId(0);
        let seat_belt = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentId(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(safety_name),
                DeclarationKind::MetadataDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(seat_belt_name),
                DeclarationKind::PartUsage,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let references = vec![TestReference {
            source: seat_belt,
            kind: ReferenceKind::MetadataAnnotation,
            path: target_path,
            flags: RelationshipFlags::default(),
        }];
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn metadata_annotation_on_part_usage_resolves_to_metadata_def() {
        let fixture = metadata_annotation_fixture("Safety");
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    #[test]
    fn metadata_annotation_with_unresolvable_target_stays_unresolved() {
        let fixture = metadata_annotation_fixture("NoSuchMetadata");
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Unresolved)
        );
    }

    /// Builds a `Demo { connection def Base; connection def Derived :> Base; }`-shaped fixture:
    /// `Derived`'s `:>` specialization reference exercises `connection def`'s participation in the
    /// shared Subclassification/FeatureTyping lexical lookup fixed point
    /// (`DeclarationDomain::Type`) exactly like `item def`/`action def`/`part def`/`port def`.
    fn connection_def_specialization_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let base_name = symbols.intern("Base").unwrap();
        let derived_name = symbols.intern("Derived").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let base_path = paths.push(&[base_name], false).unwrap();

        let demo = DeclarationId(0);
        let derived = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentId(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(base_name),
                DeclarationKind::ConnectionDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(derived_name),
                DeclarationKind::ConnectionDefinition,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let references = vec![TestReference {
            source: derived,
            kind: ReferenceKind::Subclassification,
            path: base_path,
            flags: RelationshipFlags::default(),
        }];
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn connection_def_specialization_resolves_through_the_ancestor_fixed_point() {
        let fixture = connection_def_specialization_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    fn interface_def_specialization_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let base_name = symbols.intern("Base").unwrap();
        let derived_name = symbols.intern("Derived").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let base_path = paths.push(&[base_name], false).unwrap();

        let demo = DeclarationId(0);
        let derived = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentId(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(base_name),
                DeclarationKind::InterfaceDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(derived_name),
                DeclarationKind::InterfaceDefinition,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let references = vec![TestReference {
            source: derived,
            kind: ReferenceKind::Subclassification,
            path: base_path,
            flags: RelationshipFlags::default(),
        }];
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn interface_def_specialization_resolves_through_the_ancestor_fixed_point() {
        let fixture = interface_def_specialization_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    /// Builds a `Demo { part d1; connection bus connect d1 to d1; }`-shaped fixture: `bus`'s
    /// `ConnectorEnd` reference exercises the `DeclarationDomain::Any` resolution slot
    /// (`connector_end_slots`) exactly like `AliasBinding` -- a connector end can reference any
    /// feature, not just a Type, so it must not join the Subclassification/FeatureTyping `Type`
    /// domain passes.
    fn connector_end_reference_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let d1_name = symbols.intern("d1").unwrap();
        let bus_name = symbols.intern("bus").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let d1_path = paths.push(&[d1_name], false).unwrap();

        let demo = DeclarationId(0);
        let bus = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentId(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(d1_name),
                DeclarationKind::PartUsage,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(bus_name),
                DeclarationKind::ConnectionUsage,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let references = vec![TestReference {
            source: bus,
            kind: ReferenceKind::ConnectorEnd,
            path: d1_path,
            flags: RelationshipFlags::default(),
        }];
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn connector_end_reference_resolves_to_its_target() {
        let fixture = connector_end_reference_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    #[test]
    fn namespace_import_populates_index_used_by_feature_typing() {
        let fixture = cross_file_fixture(false);
        let (direct_names, effective_imports, _memberships, resolution) = resolve_fixture(&fixture);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(0)))
        );
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(1)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
        assert_eq!(
            direct_names.candidates(Some(DeclarationId(0)), SymbolId(2)),
            &[DeclarationId(1)]
        );
        assert_eq!(
            effective_imports.candidates(Some(DeclarationId(2)), SymbolId(2)),
            &[DeclarationId(1)]
        );
        assert_eq!(fixture.paths.paths.len(), 2);
        assert_eq!(
            fixture.paths.get(SymbolPathId(0)),
            Some((&[SymbolId(0)][..], false))
        );
        assert_eq!(resolution.outcomes.len(), fixture.references.len());
    }

    #[test]
    fn default_visibility_is_settled_once_from_membership_context() {
        let declarations = [
            declaration(
                DocumentId(0),
                None,
                Some(SymbolId(0)),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(0),
                Some(DeclarationId(0)),
                Some(SymbolId(1)),
                DeclarationKind::Namespace,
            ),
            declaration(
                DocumentId(0),
                Some(DeclarationId(1)),
                Some(SymbolId(2)),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(DeclarationId(0)),
                None,
                DeclarationKind::Import,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let index = MembershipIndex::build(&declarations, &memberships).unwrap();

        assert!(index.is_public(DeclarationId(0)));
        assert!(index.is_public(DeclarationId(1)));
        assert!(!index.is_public(DeclarationId(2)));
        assert!(!index.is_public(DeclarationId(3)));
    }

    #[test]
    fn namespace_import_excludes_explicitly_private_members() {
        let mut fixture = cross_file_fixture(false);
        fixture.memberships[1].visibility = Visibility::Private;
        let (_, effective_imports, _memberships, resolution) = resolve_fixture(&fixture);

        assert!(effective_imports
            .candidates(Some(DeclarationId(2)), SymbolId(2))
            .is_empty());
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(1)),
            Some(ResolutionStatus::Unresolved)
        );
    }

    #[test]
    fn duplicate_imported_type_is_canonically_ambiguous() {
        let fixture = cross_file_fixture(true);
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        let Some(ResolutionStatus::Ambiguous(range)) = resolution.outcome(AuthoredReferenceId(1))
        else {
            panic!("feature typing must retain ambiguity");
        };
        assert_eq!(
            resolution.ambiguous_candidates(range),
            &[DeclarationId(1), DeclarationId(5)]
        );
    }

    #[test]
    fn transitive_namespace_imports_converge_without_reference_scans() {
        let fixture = transitive_import_fixture();
        let (_, effective_imports, _memberships, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(resolution.work.passes, 4);
        assert_eq!(resolution.work.import_evaluations, 12);
        assert_eq!(resolution.work.downstream_evaluations, 1);
        assert_eq!(resolution.work.direct_index_entries, 6);
        assert_eq!(resolution.work.effective_index_entries, 3);
        assert_eq!(
            effective_imports.candidates(Some(DeclarationId(6)), SymbolId(4)),
            &[DeclarationId(1)]
        );
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(3)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    #[test]
    fn an_import_target_can_become_visible_through_an_earlier_import() {
        let fixture = imported_target_fixture();
        let (_, effective_imports, _memberships, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(1)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
        assert_eq!(
            effective_imports.candidates(Some(DeclarationId(3)), SymbolId(2)),
            &[DeclarationId(2)]
        );
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(2)),
            Some(ResolutionStatus::Resolved(DeclarationId(2)))
        );
    }

    #[test]
    fn cyclic_namespace_imports_reach_a_finite_canonical_closure() {
        let fixture = cyclic_import_fixture();
        let (_, effective_imports, _memberships, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(resolution.work.passes, 3);
        assert_eq!(
            effective_imports.candidates(Some(DeclarationId(0)), SymbolId(3)),
            &[DeclarationId(5)]
        );
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(2)),
            Some(ResolutionStatus::Resolved(DeclarationId(5)))
        );
    }

    #[test]
    fn later_qualified_segments_use_the_effective_import_index() {
        let fixture = qualified_import_target_fixture();
        let (_, effective_imports, _memberships, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(1)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
        assert_eq!(
            effective_imports.candidates(Some(DeclarationId(5)), SymbolId(2)),
            &[DeclarationId(2)]
        );
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(2)),
            Some(ResolutionStatus::Resolved(DeclarationId(2)))
        );
    }

    #[test]
    fn exhausted_bound_is_a_typed_non_converged_publication_state() {
        let fixture = cross_file_fixture(false);
        let (_, _, _memberships, resolution) = resolve_dense_with_limit(
            &fixture.declarations,
            &fixture.memberships,
            &fixture.paths,
            &fixture.references,
            1,
            None,
        )
        .unwrap();
        assert_eq!(resolution.solver_status, SolverStatus::NonConverged);
        assert_eq!(resolution.work.passes, 1);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::NonConverged)
        );
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(1)),
            Some(ResolutionStatus::NonConverged)
        );
    }

    #[test]
    fn missing_and_filtered_references_remain_explicit() {
        let mut fixture = cross_file_fixture(false);
        fixture.references[0].kind = ReferenceKind::FilterImport;
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Unsupported)
        );
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(1)),
            Some(ResolutionStatus::Unresolved)
        );
    }

    /// Builds `package Diamond { part def Base { part def Member; } part def Left :> Base;
    /// part def Right :> Base; part def Diamond :> Left, Right { part <feature_name> : <typed>; } }`.
    /// `feature_name`/`typed` let the ambiguous-diamond test override the leaf feature and its
    /// authored typing target while sharing the rest of the diamond shape.
    fn diamond_fixture(feature_name: &str, typed: &str) -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let diamond_pkg = symbols.intern("Diamond").unwrap();
        let base_name = symbols.intern("Base").unwrap();
        let member_name = symbols.intern("Member").unwrap();
        let left_name = symbols.intern("Left").unwrap();
        let right_name = symbols.intern("Right").unwrap();
        let diamond_name = symbols.intern("Diamond").unwrap();
        let feature = symbols.intern(feature_name).unwrap();
        let typed_name = symbols.intern(typed).unwrap();

        let mut paths = SymbolPathArenaBuilder::default();
        let base_path = paths.push(&[base_name], false).unwrap();
        let left_path = paths.push(&[left_name], false).unwrap();
        let right_path = paths.push(&[right_name], false).unwrap();
        let typed_path = paths.push(&[typed_name], false).unwrap();

        let package = DeclarationId(0);
        let base = DeclarationId(1);
        let left = DeclarationId(3);
        let right = DeclarationId(4);
        let diamond = DeclarationId(5);
        let declarations = vec![
            declaration(
                DocumentId(0),
                None,
                Some(diamond_pkg),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(0),
                Some(package),
                Some(base_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(base),
                Some(member_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(package),
                Some(left_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(package),
                Some(right_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(package),
                Some(diamond_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(diamond),
                Some(feature),
                DeclarationKind::PartUsage,
            ),
        ];
        let references = vec![
            reference(left, ReferenceKind::Subclassification, base_path, false),
            reference(right, ReferenceKind::Subclassification, base_path, false),
            reference(diamond, ReferenceKind::Subclassification, left_path, false),
            reference(diamond, ReferenceKind::Subclassification, right_path, false),
            reference(
                DeclarationId(6),
                ReferenceKind::FeatureTyping,
                typed_path,
                false,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn diamond_inherited_member_lookup_dedups_to_a_single_target() {
        let fixture = diamond_fixture("p", "Member");
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        // Member is owned only by Base (id 2), reached via both Left -> Base and Right -> Base;
        // the diamond must dedup to exactly one Resolved outcome rather than an Ambiguous one.
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(4)),
            Some(ResolutionStatus::Resolved(DeclarationId(2)))
        );
    }

    #[test]
    fn single_ancestor_inherited_lookup_resolves_through_one_specialization_hop() {
        // A minimal non-diamond case: Diamond specializes only Left (drop the Right edge by
        // reusing the diamond fixture's Left -> Base -> Member chain through a direct
        // single-parent shape) still exercises the same inherited-lookup path.
        let mut fixture = diamond_fixture("p", "Member");
        // Remove the `Diamond :> Right` edge (reference index 3) and the `Right :> Base` edge
        // (reference index 1) so only one specialization hop feeds the closure.
        let mut references = fixture.references.into_vec();
        references.remove(3);
        references.remove(1);
        fixture.references = references.into_boxed_slice();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(2)),
            Some(ResolutionStatus::Resolved(DeclarationId(2)))
        );
    }

    /// `package P { part def Left { part def Special; } part def Right { part def Special; }
    /// part def Diamond :> Left, Right { part q : Special; } }`. Left and Right each directly own
    /// their own distinct `Special` member (no `Base`), so the diamond conflict is genuine: two
    /// different ancestors reach two different same-named targets, not one target through two
    /// paths.
    fn diamond_conflict_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let package_name = symbols.intern("P").unwrap();
        let left_name = symbols.intern("Left").unwrap();
        let right_name = symbols.intern("Right").unwrap();
        let diamond_name = symbols.intern("Diamond").unwrap();
        let special_name = symbols.intern("Special").unwrap();
        let q_name = symbols.intern("q").unwrap();

        let mut paths = SymbolPathArenaBuilder::default();
        let left_path = paths.push(&[left_name], false).unwrap();
        let right_path = paths.push(&[right_name], false).unwrap();
        let special_path = paths.push(&[special_name], false).unwrap();

        let package = DeclarationId(0);
        let left = DeclarationId(1);
        let right = DeclarationId(3);
        let diamond = DeclarationId(5);
        let declarations = vec![
            declaration(
                DocumentId(0),
                None,
                Some(package_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(0),
                Some(package),
                Some(left_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(left),
                Some(special_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(package),
                Some(right_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(right),
                Some(special_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(package),
                Some(diamond_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(diamond),
                Some(q_name),
                DeclarationKind::PartUsage,
            ),
        ];
        let references = vec![
            reference(diamond, ReferenceKind::Subclassification, left_path, false),
            reference(diamond, ReferenceKind::Subclassification, right_path, false),
            reference(
                DeclarationId(6),
                ReferenceKind::FeatureTyping,
                special_path,
                false,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn diamond_with_conflicting_ancestor_members_publishes_an_explicit_ambiguous_outcome() {
        let fixture = diamond_conflict_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        match resolution.outcome(AuthoredReferenceId(2)) {
            Some(ResolutionStatus::Ambiguous(range)) => {
                let mut candidates = resolution.ambiguous_candidates(range).to_vec();
                candidates.sort_by_key(|id| id.0);
                assert_eq!(candidates, vec![DeclarationId(2), DeclarationId(4)]);
            }
            other => panic!("expected an explicit ambiguous outcome, got {other:?}"),
        }
    }

    fn cyclic_specialization_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let package_name = symbols.intern("P").unwrap();
        let a_name = symbols.intern("A").unwrap();
        let b_name = symbols.intern("B").unwrap();
        let f_name = symbols.intern("f").unwrap();
        let x_name = symbols.intern("X").unwrap();

        let mut paths = SymbolPathArenaBuilder::default();
        let a_path = paths.push(&[a_name], false).unwrap();
        let b_path = paths.push(&[b_name], false).unwrap();
        let x_path = paths.push(&[x_name], false).unwrap();

        let package = DeclarationId(0);
        let a = DeclarationId(1);
        let b = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentId(0),
                None,
                Some(package_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(0),
                Some(package),
                Some(a_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(package),
                Some(b_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(a),
                Some(f_name),
                DeclarationKind::PartUsage,
            ),
        ];
        let references = vec![
            reference(a, ReferenceKind::Subclassification, b_path, false),
            reference(b, ReferenceKind::Subclassification, a_path, false),
            reference(
                DeclarationId(3),
                ReferenceKind::FeatureTyping,
                x_path,
                false,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn cyclic_specialization_yields_a_typed_non_converged_typing_outcome_not_a_loop() {
        let fixture = cyclic_specialization_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        // The import/typing fixed point above this family still converges; only the
        // ancestor-closure-dependent FeatureTyping outcome for the cyclically-specialized owner
        // is explicitly NonConverged, rather than the solver looping forever or silently guessing
        // an inherited candidate through the self-referential closure.
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(2)),
            Some(ResolutionStatus::NonConverged)
        );
    }

    /// `package P { part def Device; alias DeviceAlias for Device; part device : DeviceAlias; }`
    /// — mirrors `tests/snapshots/resolution/alias_target_binding.md`.
    fn alias_binding_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let package_name = symbols.intern("P").unwrap();
        let device_name = symbols.intern("Device").unwrap();
        let alias_name = symbols.intern("DeviceAlias").unwrap();
        let device_usage_name = symbols.intern("device").unwrap();

        let mut paths = SymbolPathArenaBuilder::default();
        let device_path = paths.push(&[device_name], false).unwrap();
        let alias_path = paths.push(&[alias_name], false).unwrap();

        let package = DeclarationId(0);
        let alias = DeclarationId(2);
        let device_usage = DeclarationId(3);
        let declarations = vec![
            declaration(
                DocumentId(0),
                None,
                Some(package_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(0),
                Some(package),
                Some(device_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(package),
                Some(alias_name),
                DeclarationKind::Alias,
            ),
            declaration(
                DocumentId(0),
                Some(package),
                Some(device_usage_name),
                DeclarationKind::PartUsage,
            ),
        ];
        let references = vec![
            reference(alias, ReferenceKind::AliasBinding, device_path, false),
            reference(
                device_usage,
                ReferenceKind::FeatureTyping,
                alias_path,
                false,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn alias_binding_resolves_through_the_shared_lexical_lookup_fixed_point() {
        let fixture = alias_binding_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        // DeviceAlias's own authored `AliasBinding` reference resolves to Device (id 1), using
        // the same fixed point as every other authored reference kind rather than a separate
        // ad hoc path.
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    #[test]
    fn typing_through_an_alias_resolves_transitively_to_the_ultimate_target() {
        let fixture = alias_binding_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        // `device : DeviceAlias`'s own FeatureTyping outcome targets the alias declaration (id 2)
        // itself -- the alias's own authored fact is never weakened or bypassed.
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(1)),
            Some(ResolutionStatus::Resolved(DeclarationId(2)))
        );
        // Downstream typing is nonetheless transparent: an implied FeatureTyping fact chases the
        // alias chain to publish device -> Device directly, with implied provenance.
        assert_eq!(
            resolution.implied_relationships.as_ref(),
            &[ImpliedRelationship {
                kind: ReferenceKind::FeatureTyping,
                source: DeclarationId(3),
                target: DeclarationId(1),
            }],
        );
    }

    /// `package P { alias A for B; alias B for A; }` — a two-hop alias cycle, mirroring the
    /// specialization-cycle shape of `cyclic_specialization_fixture` above.
    fn cyclic_alias_binding_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let package_name = symbols.intern("P").unwrap();
        let a_name = symbols.intern("A").unwrap();
        let b_name = symbols.intern("B").unwrap();

        let mut paths = SymbolPathArenaBuilder::default();
        let a_path = paths.push(&[a_name], false).unwrap();
        let b_path = paths.push(&[b_name], false).unwrap();

        let package = DeclarationId(0);
        let a = DeclarationId(1);
        let b = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentId(0),
                None,
                Some(package_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(0),
                Some(package),
                Some(a_name),
                DeclarationKind::Alias,
            ),
            declaration(
                DocumentId(0),
                Some(package),
                Some(b_name),
                DeclarationKind::Alias,
            ),
        ];
        let references = vec![
            reference(a, ReferenceKind::AliasBinding, b_path, false),
            reference(b, ReferenceKind::AliasBinding, a_path, false),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn alias_cycle_yields_a_typed_non_converged_outcome_not_a_hang() {
        // Bounded by `detect_cyclic_alias_bindings`'s `declarations.len() + 1` hop limit: this
        // test would time out (rather than merely fail an assertion) if alias cycle detection
        // ever degenerated into an unbounded chase.
        let fixture = cyclic_alias_binding_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::NonConverged)
        );
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(1)),
            Some(ResolutionStatus::NonConverged)
        );
        assert!(resolution.implied_relationships.is_empty());
    }

    /// `package A { part def T; } package C { import A::*; part T; part p : T; }` — mirrors
    /// `tests/snapshots/resolution/lexical_inner_shadow.md`. `nested` controls whether the
    /// FeatureTyping reference lives directly on `C::p` (false) or one namespace level deeper, on
    /// a feature owned by an intermediate `Inner` namespace inside `C` (true), so the same local
    /// binding is reached by walking one extra step of the enclosing-namespace chain.
    fn local_shadow_fixture(nested: bool) -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let a_name = symbols.intern("A").unwrap();
        let t_name = symbols.intern("T").unwrap();
        let c_name = symbols.intern("C").unwrap();
        let p_name = symbols.intern("p").unwrap();
        let inner_name = symbols.intern("Inner").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let a_path = paths.push(&[a_name], false).unwrap();
        let t_path = paths.push(&[t_name], false).unwrap();

        let a = DeclarationId(0);
        let c = DeclarationId(2);
        let mut declarations = vec![
            declaration(DocumentId(0), None, Some(a_name), DeclarationKind::Package),
            declaration(
                DocumentId(0),
                Some(a),
                Some(t_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(DocumentId(0), None, Some(c_name), DeclarationKind::Package),
            declaration(DocumentId(0), Some(c), None, DeclarationKind::Import),
            declaration(
                DocumentId(0),
                Some(c),
                Some(t_name),
                DeclarationKind::PartUsage,
            ),
        ];
        let p_owner = if nested {
            let inner = DeclarationId(u32::try_from(declarations.len()).unwrap());
            declarations.push(declaration(
                DocumentId(0),
                Some(c),
                Some(inner_name),
                DeclarationKind::Namespace,
            ));
            inner
        } else {
            c
        };
        declarations.push(declaration(
            DocumentId(0),
            Some(p_owner),
            Some(p_name),
            DeclarationKind::PartUsage,
        ));
        let p = DeclarationId(u32::try_from(declarations.len() - 1).unwrap());
        let references = vec![
            reference(
                DeclarationId(3),
                ReferenceKind::NamespaceImport,
                a_path,
                true,
            ),
            reference(p, ReferenceKind::FeatureTyping, t_path, false),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn local_feature_shadows_an_incompatible_imported_type_of_the_same_name() {
        // C::T (a PartUsage feature) is domain-incompatible as a FeatureTyping target, but it is
        // still owned directly by C, the reference's enclosing namespace, so per
        // it must shadow the imported, domain-compatible
        // A::T rather than being silently discarded in favor of the import or left Unresolved.
        let fixture = local_shadow_fixture(false);
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(1)),
            Some(ResolutionStatus::Resolved(DeclarationId(4)))
        );
    }

    #[test]
    fn without_a_local_binding_lookup_still_falls_through_to_the_import() {
        // Regression guard: removing C::T (and its membership record) must not disturb the
        // fallback to the imported A::T once no local/inherited candidate exists at any tier.
        let mut fixture = local_shadow_fixture(false);
        let mut declarations = fixture.declarations.into_vec();
        declarations.remove(4);
        fixture.declarations = declarations.into_boxed_slice();
        // Rebuild memberships from scratch rather than splicing: `MembershipRecord::member` is a
        // `DeclarationId` into the just-mutated declarations array, so it must be recomputed
        // against the post-removal indices, not the pre-removal ones.
        fixture.memberships = memberships_for(&fixture.declarations, &[]);
        // The FeatureTyping reference source shifts down by one index after the removal.
        let mut references = fixture.references.into_vec();
        references[1].source = DeclarationId(4);
        fixture.references = references.into_boxed_slice();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(1)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    #[test]
    fn intermediate_namespace_local_binding_shadows_the_outer_import() {
        // p lives inside C::Inner, one level below C itself. The local C::T binding is neither
        // owned by nor inherited into Inner directly, so the walk must climb the enclosing-scope
        // chain to C, find C::T there, and shadow A::T at that outer tier before ever consulting
        // imports.
        let fixture = local_shadow_fixture(true);
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(1)),
            Some(ResolutionStatus::Resolved(DeclarationId(4)))
        );
    }

    #[test]
    fn candidate_ranges_are_canonical_regardless_of_input_order() {
        let index = NameIndex::build(vec![
            (
                NameKey {
                    owner: None,
                    name: SymbolId(0),
                },
                DeclarationId(2),
            ),
            (
                NameKey {
                    owner: None,
                    name: SymbolId(0),
                },
                DeclarationId(1),
            ),
        ])
        .unwrap();
        assert_eq!(
            index.candidates(None, SymbolId(0)),
            &[DeclarationId(1), DeclarationId(2)]
        );
    }

    #[test]
    fn packed_name_entry_order_matches_the_canonical_tuple_order() {
        let entries = vec![
            (
                NameKey {
                    owner: Some(DeclarationId(u32::MAX)),
                    name: SymbolId(u32::MAX),
                },
                DeclarationId(u32::MAX),
            ),
            (
                NameKey {
                    owner: Some(DeclarationId(0)),
                    name: SymbolId(u32::MAX),
                },
                DeclarationId(0),
            ),
            (
                NameKey {
                    owner: None,
                    name: SymbolId(u32::MAX),
                },
                DeclarationId(u32::MAX),
            ),
            (
                NameKey {
                    owner: Some(DeclarationId(0)),
                    name: SymbolId(0),
                },
                DeclarationId(u32::MAX),
            ),
            (
                NameKey {
                    owner: None,
                    name: SymbolId(0),
                },
                DeclarationId(0),
            ),
        ];
        let mut canonical = entries.clone();
        canonical.sort_unstable();
        let mut packed = entries;
        packed.sort_unstable_by_key(name_entry_sort_key);
        assert_eq!(packed, canonical);
    }
}
