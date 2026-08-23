//! Phase 6: per-document position and span lookup.

use crate::diagnose::declaration_identifier_range;
use crate::diagnose::document_range;
use crate::diagnose::identifier_range;
use crate::lower::storage::ParsedSources;
use crate::lower::storage::SemanticModelStorage;
use crate::model::query::range_contains;
use crate::model::AuthoredReferenceId;
use crate::model::DeclarationId;
use crate::model::DocumentId;
use crate::model::SymbolId;
use crate::resolve::results::ResolutionError;
use crate::TextPosition;
use crate::TextRange;
use hashbrown::HashTable;
use std::collections::hash_map::RandomState;
use std::hash::BuildHasher;

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
pub(crate) struct DocumentPositions {
    /// Authored reference ranges, ordered by start.
    pub(crate) references: Box<[(TextRange, AuthoredReferenceId)]>,
    /// Declaration identifier ranges, ordered by start. Only named declarations appear.
    pub(crate) identifiers: Box<[(TextRange, DeclarationId)]>,
    /// Every declaration's full span, as a containment tree.
    pub(crate) spans: SpanTree,
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
pub(crate) struct SpanTree {
    pub(crate) entries: Box<[(TextRange, DeclarationId)]>,
    pub(crate) subtree_end: Box<[u32]>,
}

impl SpanTree {
    pub(crate) fn build(mut entries: Vec<(TextRange, DeclarationId)>) -> Self {
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
    pub(crate) fn innermost_containing(&self, position: TextPosition) -> Option<DeclarationId> {
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
    pub(crate) fn iter(&self) -> impl Iterator<Item = &(TextRange, DeclarationId)> {
        self.entries.iter()
    }
}

/// Document lookup by identity, plus each document's position tables.
#[derive(Debug)]
pub(crate) struct DocumentIndex {
    pub(crate) by_identity: HashTable<DocumentId>,
    pub(crate) hash_builder: RandomState,
    pub(crate) positions: Box<[DocumentPositions]>,
    /// Per-document ranges into [`Self::declaration_order`]: the declarations each document
    /// authored.
    ///
    /// CSR rather than a `Vec<Vec<_>>` rebuilt per caller, and prebuilt because the alternative is
    /// a scan of every declaration in the publication per document-scoped query -- which makes one
    /// workspace projection cost the size of the bundled standard library.
    pub(crate) declarations: Box<[(u32, u32)]>,
    pub(crate) declaration_order: Box<[DeclarationId]>,
    /// Each declaration's settled identifier range, dense by `DeclarationId`.
    ///
    /// `design.md`: a sealed publication answers a location query from a settled fact, never by
    /// re-reading source text. Locating the declared name inside a declaration's span is the one
    /// answer that needed the text, so it is settled here, at the barrier, while the parse product
    /// is still in scope. `None` for a declaration with no authored name, or one whose header the
    /// parser could not recover.
    pub(crate) declaration_identifiers: Box<[Option<TextRange>]>,
    /// Per-reference ranges into [`Self::reference_identifier_entries`].
    pub(crate) reference_identifiers: Box<[(u32, u32)]>,
    /// For each authored reference, the range of each distinct name its path spells.
    ///
    /// A find-all-references result points at the segment naming the target, not at the whole
    /// path, and which segment that is depends on which declaration the caller asked about -- so
    /// the settled fact is per segment name, and the query picks the one it needs.
    pub(crate) reference_identifier_entries: Box<[(SymbolId, TextRange)]>,
}

impl DocumentIndex {
    pub(crate) fn build(
        storage: &SemanticModelStorage,
        sources: &ParsedSources,
    ) -> Result<Self, ResolutionError> {
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
        // Counting CSR rather than a `Vec` per document: the ranges below index the payload
        // directly, so building the document->declaration view costs two allocations for the whole
        // publication instead of one per document.
        let mut counts = vec![0u32; storage.documents.len()];
        for declaration in storage.declarations.iter() {
            if let Some(slot) = counts.get_mut(declaration.document.index()) {
                *slot = slot.checked_add(1).ok_or(ResolutionError::Capacity)?;
            }
        }
        let mut declarations: Vec<(u32, u32)> = Vec::with_capacity(storage.documents.len());
        let mut cursor = 0u32;
        for count in &counts {
            declarations.push((cursor, cursor + count));
            cursor += count;
        }
        let mut fill = declarations
            .iter()
            .map(|(start, _)| *start)
            .collect::<Vec<_>>();
        let mut declaration_identifiers: Vec<Option<TextRange>> =
            vec![None; storage.declarations.len()];
        let mut declaration_order: Vec<DeclarationId> = match storage.declarations.len() {
            0 => Vec::new(),
            length => {
                let placeholder =
                    DeclarationId::from_index(0).map_err(|_| ResolutionError::Capacity)?;
                vec![placeholder; length]
            }
        };

        for (index, (declaration, settled_identifier)) in storage
            .declarations
            .iter()
            .zip(declaration_identifiers.iter_mut())
            .enumerate()
        {
            let Ok(id) = DeclarationId::from_index(index) else {
                continue;
            };
            if let Some(slot) = fill.get_mut(declaration.document.index()) {
                let position = *slot as usize;
                if let Some(entry) = declaration_order.get_mut(position) {
                    *entry = id;
                }
                *slot += 1;
            }
            if let Ok(range) = document_range(storage, declaration.document, &declaration.span) {
                spans[declaration.document.index()].push((range, id));
            }
            let Some(name) = declaration.name.and_then(|name| storage.symbol(name)) else {
                continue;
            };
            if let Ok(range) = declaration_identifier_range(
                storage,
                sources,
                declaration.document,
                &declaration.span,
                name,
            ) {
                *settled_identifier = Some(range);
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

        // The reference-segment ranges, settled with the same text the declaration ranges used.
        let mut reference_identifiers: Vec<(u32, u32)> =
            Vec::with_capacity(storage.references.len());
        let mut reference_identifier_entries: Vec<(SymbolId, TextRange)> = Vec::new();
        for reference in storage.references.iter() {
            let start = u32::try_from(reference_identifier_entries.len())
                .map_err(|_| ResolutionError::Capacity)?;
            let Some(source) = storage.declaration(reference.source) else {
                reference_identifiers.push((start, start));
                continue;
            };
            if let Some((segments, _)) = storage.paths.get(reference.path) {
                for segment in segments {
                    if reference_identifier_entries[start as usize..]
                        .iter()
                        .any(|(name, _)| name == segment)
                    {
                        continue;
                    }
                    let Some(text) = storage.symbol(*segment) else {
                        continue;
                    };
                    if let Ok(range) =
                        identifier_range(storage, sources, source.document, &reference.span, text)
                    {
                        reference_identifier_entries.push((*segment, range));
                    }
                }
            }
            let end = u32::try_from(reference_identifier_entries.len())
                .map_err(|_| ResolutionError::Capacity)?;
            reference_identifiers.push((start, end));
        }

        Ok(Self {
            by_identity,
            hash_builder,
            positions,
            declarations: declarations.into_boxed_slice(),
            declaration_order: declaration_order.into_boxed_slice(),
            declaration_identifiers: declaration_identifiers.into_boxed_slice(),
            reference_identifiers: reference_identifiers.into_boxed_slice(),
            reference_identifier_entries: reference_identifier_entries.into_boxed_slice(),
        })
    }

    /// The settled identifier range of one declaration.
    pub(crate) fn declaration_identifier(&self, id: DeclarationId) -> Option<TextRange> {
        self.declaration_identifiers
            .get(id.index())
            .copied()
            .flatten()
    }

    /// The settled range of the segment spelling `name` inside one authored reference.
    pub(crate) fn reference_identifier(
        &self,
        id: AuthoredReferenceId,
        name: SymbolId,
    ) -> Option<TextRange> {
        let (start, end) = *self.reference_identifiers.get(id.index())?;
        self.reference_identifier_entries[start as usize..end as usize]
            .iter()
            .find(|(segment, _)| *segment == name)
            .map(|(_, range)| *range)
    }

    /// The declarations one document authored, as the settled slice rather than a corpus scan.
    pub(crate) fn document_declarations(&self, document: DocumentId) -> &[DeclarationId] {
        match self.declarations.get(document.index()) {
            Some((start, end)) => &self.declaration_order[*start as usize..*end as usize],
            None => &[],
        }
    }

    pub(crate) fn document(
        &self,
        storage: &SemanticModelStorage,
        identity: &str,
    ) -> Option<DocumentId> {
        let hash = self.hash_builder.hash_one(identity);
        self.by_identity
            .find(hash, |candidate| {
                storage.documents[candidate.index()].identity.as_ref() == identity
            })
            .copied()
    }

    pub(crate) fn positions(&self, document: DocumentId) -> Option<&DocumentPositions> {
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
pub(crate) fn record_visited_index_entries(entries: usize) {
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
pub(crate) fn leaf_ranges_containing<T: Copy>(
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
