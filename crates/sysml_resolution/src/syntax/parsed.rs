//! The parse authority: one parsed tree per content digest and source language.
//!
//! [`SyntaxAuthority::parse`] is the only call site of the parser for admitted documents. It
//! returns a [`ParsedSource`], an `Arc` handle that the editor's syntax queries and the semantic
//! build share, so a revision is parsed once however many consumers ask about it. The memo is
//! invisible to callers: a hit and a miss return the same kind of value.

use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::{Arc, Mutex};

use hashbrown::{HashMap, HashSet};
use source_identity::{ContentDigest, SourceLanguage};
use sysml_source::SourceDocument;
use sysml_v2_parser::{ParseError, ParsedDocument};

use super::{SyntaxDiagnostic, SyntaxDiagnosticCategory, SyntaxDiagnosticSeverity};

pub(crate) const fn parser_name_language(
    language: SourceLanguage,
) -> sysml_v2_parser::ast::NameLanguage {
    match language {
        SourceLanguage::SysML => sysml_v2_parser::ast::NameLanguage::Sysml,
        SourceLanguage::KerML => sysml_v2_parser::ast::NameLanguage::Kerml,
    }
}

type ParseIdentity = (SourceLanguage, ContentDigest);

struct Inner {
    positions: crate::positions::LineIndex,
    /// Shared with the semantic build's admitted documents and the library stratum: admitting a
    /// parsed source is two reference-count bumps, never a tree clone.
    document: Arc<ParsedDocument>,
    errors: Vec<ParseError>,
    diagnostics: Box<[SyntaxDiagnostic]>,
    digest: ContentDigest,
    parser_failed: bool,
}

/// One parsed source revision. Cloning is a reference-count bump.
///
/// Equality is identity: two handles are equal when they parse the same bytes in the same language. The tree is never
/// exposed; `pub(crate)` accessors serve the authority's own traversals and the semantic build.
#[derive(Clone)]
pub struct ParsedSource(Arc<Inner>);

/// An immutable batch of source revisions and their matching syntax leases.
///
/// Preparation may run away from a host's admission barrier. Owning the leases, rather than
/// relying on a recent memo touch, keeps every tree alive across other projects' sweeps. This
/// batch is a work product, not another cache; dropping it releases unadmitted revisions.
#[derive(Debug)]
pub struct PreparedSyntax {
    sources: Vec<(SourceDocument, ParsedSource)>,
}

impl PreparedSyntax {
    /// Borrow the paired revisions without transferring their leases.
    pub fn sources(&self) -> impl ExactSizeIterator<Item = (&SourceDocument, &ParsedSource)> {
        self.sources
            .iter()
            .map(|(document, syntax)| (document, syntax))
    }
    /// The complete source set whose identity a host must validate before admission.
    pub fn documents(&self) -> impl ExactSizeIterator<Item = &SourceDocument> {
        self.sources.iter().map(|(document, _)| document)
    }

    /// Transfer already prepared leases at the admission barrier. This never parses or looks
    /// up the memo, and preserves the pairing and order supplied during preparation.
    pub fn into_sources(self) -> impl ExactSizeIterator<Item = (SourceDocument, ParsedSource)> {
        self.sources.into_iter()
    }
}

impl std::fmt::Debug for ParsedSource {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ParsedSource")
            .field("digest", &self.0.digest)
            .field("diagnostics", &self.0.diagnostics.len())
            .field("parser_failed", &self.0.parser_failed)
            .finish()
    }
}

impl PartialEq for ParsedSource {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
            || (self.0.digest == other.0.digest && self.language() == other.language())
    }
}

impl Eq for ParsedSource {}

impl ParsedSource {
    /// Parse `content` outside any memo. Parser panics are captured and reported as a
    /// [`SyntaxDiagnosticCategory::ParserFailure`] diagnostic over an empty tree, so a host never
    /// has to special-case a document that failed to parse at all.
    pub(crate) fn parse_text(
        content: String,
        digest: ContentDigest,
        language: SourceLanguage,
    ) -> Self {
        let outcome = catch_unwind(AssertUnwindSafe(|| {
            sysml_v2_parser::parse_for_editor_owned_with_name_language(
                content,
                parser_name_language(language),
            )
        }));
        match outcome {
            Ok(result) => Self::from_result(result, digest, false),
            Err(_) => {
                let empty = sysml_v2_parser::parse_for_editor_owned_with_name_language(
                    String::new(),
                    parser_name_language(language),
                );
                let mut source = Self::from_result(empty, digest, true);
                let inner = Arc::get_mut(&mut source.0).expect("freshly built handle is unique");
                inner.diagnostics = Box::new([SyntaxDiagnostic {
                    source_range: None,
                    severity: SyntaxDiagnosticSeverity::Error,
                    category: SyntaxDiagnosticCategory::ParserFailure,
                    message: "the parser panicked on this document".to_owned(),
                    code: Some("parser_failure".to_owned()),
                    line: None,
                    column: None,
                    offset: None,
                    length: None,
                    expected: None,
                    found: None,
                    suggestion: None,
                    is_cascade: None,
                }]);
                source
            }
        }
    }

    fn from_result(
        result: sysml_v2_parser::ParseResult,
        digest: ContentDigest,
        parser_failed: bool,
    ) -> Self {
        let positions = crate::positions::LineIndex::build(result.document.source.as_str());
        let diagnostics = result
            .errors
            .iter()
            .map(|error| {
                let mut diagnostic = SyntaxDiagnostic::from_parse_error(error);
                diagnostic.source_range = match error.offset {
                    Some(offset) => positions.position(offset).and_then(|start| {
                        let end =
                            positions.position(offset.checked_add(error.length.unwrap_or(0))?)?;
                        Some(super::SyntaxRange {
                            start_line: start.line,
                            start_character: start.character,
                            end_line: end.line,
                            end_character: end.character,
                        })
                    }),
                    None => diagnostic
                        .source_range
                        .and_then(|range| positions.syntax_range(range)),
                };
                diagnostic
            })
            .collect();
        Self(Arc::new(Inner {
            positions,
            document: Arc::new(result.document),
            errors: result.errors,
            diagnostics,
            digest,
            parser_failed,
        }))
    }

    pub(crate) fn positions(&self) -> &crate::positions::LineIndex {
        &self.0.positions
    }

    /// The source this document was parsed from, after BOM stripping.
    pub fn source(&self) -> &str {
        self.0.document.source.as_str()
    }

    /// The source language under which this handle was parsed.
    pub fn language(&self) -> SourceLanguage {
        match self.0.document.name_language {
            sysml_v2_parser::ast::NameLanguage::Sysml => SourceLanguage::SysML,
            sysml_v2_parser::ast::NameLanguage::Kerml => SourceLanguage::KerML,
        }
    }

    /// The digest of the admitted content this tree was parsed from.
    pub fn digest(&self) -> ContentDigest {
        self.0.digest
    }

    /// Every parser diagnostic, in source order.
    pub fn diagnostics(&self) -> &[SyntaxDiagnostic] {
        &self.0.diagnostics
    }

    /// Whether strict parsing succeeded. Non-fatal warnings remain available in diagnostics.
    pub fn is_clean(&self) -> bool {
        !self.0.parser_failed
            && !self
                .0
                .errors
                .iter()
                .any(ParseError::prevents_strict_parsing)
    }

    /// The first diagnostic preventing strict parsing. Fully parsed syntax warnings do not reject it.
    pub fn first_error(&self) -> Option<&SyntaxDiagnostic> {
        if self.0.parser_failed {
            return self.0.diagnostics.first();
        }
        self.0
            .errors
            .iter()
            .position(ParseError::prevents_strict_parsing)
            .and_then(|index| self.0.diagnostics.get(index))
    }

    /// Whether the parser failed outright (panicked) rather than recovering.
    pub fn parser_failed(&self) -> bool {
        self.0.parser_failed
    }

    /// Whether the document has any root element at all.
    pub fn has_root_elements(&self) -> bool {
        !self.0.document.elements.is_empty()
    }

    /// Whether two sources parse to the same tree modulo span movement.
    pub fn same_tree_as(&self, other: &ParsedSource) -> bool {
        self.language() == other.language()
            && self.0.document.normalize_for_test_comparison()
                == other.0.document.normalize_for_test_comparison()
    }

    pub(crate) fn inner(&self) -> &ParsedDocument {
        &self.0.document
    }

    pub(crate) fn errors(&self) -> &[ParseError] {
        &self.0.errors
    }

    /// What the semantic build admits: the shared tree and the parser's own errors.
    pub(crate) fn admission_parts(&self) -> (Arc<ParsedDocument>, Vec<ParseError>) {
        (Arc::clone(&self.0.document), self.0.errors.clone())
    }
}

#[derive(Default)]
struct MemoState {
    map: HashMap<ParseIdentity, ParsedSource>,
    /// Content/language identities currently being parsed outside the lock. Waiters for the same identity sleep until
    /// the sole parser publishes its result; unrelated digests continue in parallel.
    in_flight: HashSet<ParseIdentity>,
    /// Content/language identities looked up or inserted since the last sweep. A revision the host parsed but has
    /// not yet admitted survives the next sweep because of this.
    touched: HashSet<ParseIdentity>,
}

/// The only place the parser is called for admitted documents.
///
/// One lock guards the memo and it is never held across a parse. Per-content/language single-flight ensures
/// concurrent misses share the one parse while unrelated revisions may parse in parallel.
pub struct SyntaxAuthority {
    state: Mutex<MemoState>,
    parsed: std::sync::Condvar,
}

impl Default for SyntaxAuthority {
    fn default() -> Self {
        Self {
            state: Mutex::new(MemoState::default()),
            parsed: std::sync::Condvar::new(),
        }
    }
}

impl std::fmt::Debug for SyntaxAuthority {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SyntaxAuthority")
            .field("memo_len", &self.memo_len())
            .finish()
    }
}

impl SyntaxAuthority {
    pub fn new() -> Self {
        Self::default()
    }

    /// The parsed tree for an admitted document: a memo hit, or one parse.
    pub fn parse(&self, document: &SourceDocument) -> ParsedSource {
        self.parse_with_digest(document.language(), document.digest(), document.content())
    }

    /// Prepare and retain matching source/syntax pairs until their owner admits or drops them.
    /// The caller may bound or cooperatively stop the iterator before accepting the batch.
    pub fn prepare<'a>(
        &self,
        documents: impl IntoIterator<Item = &'a SourceDocument>,
    ) -> PreparedSyntax {
        PreparedSyntax {
            sources: documents
                .into_iter()
                .map(|document| (document.clone(), self.parse(document)))
                .collect(),
        }
    }

    /// Parse text that is not an admitted document (an editor's candidate reformatting, a
    /// stateless caller). Digested here so identical text still hits the memo.
    pub fn parse_text(&self, text: &str) -> ParsedSource {
        self.parse_text_with_language(text, SourceLanguage::SysML)
    }

    /// Parse a candidate in an explicitly selected source language.
    pub fn parse_text_with_language(&self, text: &str, language: SourceLanguage) -> ParsedSource {
        self.parse_with_digest(language, ContentDigest::of_bytes(text.as_bytes()), text)
    }

    fn parse_with_digest(
        &self,
        language: SourceLanguage,
        digest: ContentDigest,
        content: &str,
    ) -> ParsedSource {
        let identity = (language, digest);
        loop {
            let mut state = self.lock();
            state.touched.insert(identity);
            if let Some(hit) = state.map.get(&identity) {
                return hit.clone();
            }
            if state.in_flight.insert(identity) {
                break;
            }
            drop(
                self.parsed
                    .wait(state)
                    .unwrap_or_else(std::sync::PoisonError::into_inner),
            );
        }
        let parsed = ParsedSource::parse_text(content.to_owned(), digest, language);
        let mut state = self.lock();
        state.map.insert(identity, parsed.clone());
        state.in_flight.remove(&identity);
        self.parsed.notify_all();
        parsed
    }

    /// Keep explicitly retained revisions, live source/tree leases and recent touches.
    ///
    /// The host passes the digests of every handle it still holds, admitted or not. Anything else
    /// is a superseded revision and is dropped. A different project's sweep must not evict a
    /// still-live handle or a tree leased by an active build/library stratum and cause a second
    /// parse of the same revision. The sweep and touch clearing are one operation under the lock.
    pub fn retain(&self, keep: impl IntoIterator<Item = ContentDigest>) {
        let mut state = self.lock();
        let survivors: HashSet<ContentDigest> = keep.into_iter().collect();
        let touched = state.touched.drain().collect::<HashSet<_>>();
        state.map.retain(|identity, parsed| {
            survivors.contains(&identity.1)
                || touched.contains(identity)
                || Arc::strong_count(&parsed.0) > 1
                || Arc::strong_count(&parsed.0.document) > 1
        });
    }

    pub fn memo_len(&self) -> usize {
        self.lock().map.len()
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, MemoState> {
        self.state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sysml_source::{SourceAuthority, SourceKind};

    fn document(uri: &str, content: &str) -> SourceDocument {
        SourceAuthority::new()
            .admit(uri, content, SourceKind::Workspace)
            .unwrap()
    }

    #[test]
    fn handles_are_send_and_sync_without_unsafe_impls() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<ParsedSource>();
        assert_send_sync::<PreparedSyntax>();
        assert_send_sync::<SyntaxAuthority>();
    }

    #[test]
    fn prepared_batch_keeps_exact_leases_across_sweeps_until_transfer() {
        let authority = SyntaxAuthority::new();
        let sources = [
            document("memory://prepared/first.sysml", "package First;"),
            document("memory://prepared/second.sysml", "package Second;"),
        ];
        let batch = authority.prepare(&sources);
        let leases = batch
            .sources
            .iter()
            .map(|(_, parsed)| Arc::downgrade(&parsed.0))
            .collect::<Vec<_>>();
        for _ in 0..4 {
            authority.retain([]);
        }
        assert_eq!(authority.memo_len(), 2);
        assert_eq!(
            batch.documents().collect::<Vec<_>>(),
            sources.iter().collect::<Vec<_>>()
        );
        for ((source, parsed), original) in batch.into_sources().zip(&leases) {
            assert_eq!(source.digest(), parsed.digest());
            assert_eq!(source.language(), parsed.language());
            assert!(Arc::ptr_eq(&original.upgrade().unwrap(), &parsed.0));
            assert!(Arc::ptr_eq(&authority.parse(&source).0, &parsed.0));
        }
        authority.retain([]);
        authority.retain([]);
        assert_eq!(authority.memo_len(), 0);
        assert!(leases.iter().all(|lease| lease.upgrade().is_none()));
    }

    #[test]
    fn dropping_an_unadmitted_batch_releases_its_revisions() {
        let authority = SyntaxAuthority::new();
        let source = document("memory://prepared/cancelled.sysml", "package Cancelled;");
        let batch = authority.prepare([&source]);
        let lease = Arc::downgrade(&batch.sources[0].1 .0);
        authority.retain([]);
        authority.retain([]);
        assert!(lease.upgrade().is_some());
        drop(batch);
        authority.retain([]);
        assert_eq!(authority.memo_len(), 0);
        assert!(lease.upgrade().is_none());
    }

    #[test]
    fn prepared_batch_preserves_document_order_and_language_qualified_identity() {
        let authority = SyntaxAuthority::new();
        let sources = [
            document("memory://prepared/first.sysml", "package Same;"),
            document("memory://prepared/second.kerml", "package Same;"),
            document("memory://prepared/third.sysml", "package Same;"),
        ];
        let batch = authority.prepare(&sources);
        assert_eq!(
            batch.documents().collect::<Vec<_>>(),
            sources.iter().collect::<Vec<_>>()
        );
        assert!(Arc::ptr_eq(&batch.sources[0].1 .0, &batch.sources[2].1 .0));
        assert!(!Arc::ptr_eq(&batch.sources[0].1 .0, &batch.sources[1].1 .0));
        for (source, parsed) in batch.into_sources() {
            assert_eq!(source.language(), parsed.language());
            assert_eq!(source.digest(), parsed.digest());
        }
    }

    #[test]
    fn unrelated_sweeps_keep_live_source_handles_without_reparsing() {
        let authority = SyntaxAuthority::new();
        let source = document("memory://leases/model.sysml", "package P;");
        let held = authority.parse(&source);
        authority.retain([]);
        authority.retain([]);
        let again = authority.parse(&source);
        assert!(Arc::ptr_eq(&held.0, &again.0));
        drop((held, again));
        authority.retain([]);
        authority.retain([]);
        assert_eq!(authority.memo_len(), 0);
    }

    #[test]
    fn admitted_tree_leases_survive_an_unrelated_project_sweep() {
        let authority = SyntaxAuthority::new();
        let source = document("memory://leases/model.sysml", "package P;");
        let held = authority.parse(&source);
        let (tree, _) = held.admission_parts();
        drop(held);
        authority.retain([]);
        authority.retain([]);
        let again = authority.parse(&source);
        assert!(Arc::ptr_eq(&again.0.document, &tree));
        drop((tree, again));
        authority.retain([]);
        authority.retain([]);
        assert_eq!(authority.memo_len(), 0);
    }

    #[test]
    fn continuous_edits_bound_history_while_the_previous_publication_stays_live() {
        let authority = SyntaxAuthority::new();
        let published =
            authority.parse(&document("memory://edits/model.sysml", "package Initial;"));
        let mut desired = published.clone();
        for revision in 0..300 {
            let replacement = authority.parse(&document(
                "memory://edits/model.sysml",
                &format!("package P{revision};"),
            ));
            drop(std::mem::replace(&mut desired, replacement));
            authority.retain([published.digest(), desired.digest()]);
            assert!(authority.memo_len() <= 2);
        }
        assert_ne!(published.digest(), desired.digest());
    }

    #[test]
    fn equal_content_yields_the_same_allocation() {
        let authority = SyntaxAuthority::new();
        let first = authority.parse(&document("memory://t/a.sysml", "package A;"));
        let second = authority.parse(&document("memory://t/b.sysml", "package A;"));
        assert!(Arc::ptr_eq(&first.0, &second.0));
        assert_eq!(authority.memo_len(), 1);
    }

    #[test]
    fn equal_content_in_different_languages_has_distinct_memo_identity() {
        let authority = SyntaxAuthority::new();
        let source = document("memory://t/model.sysml", "package P;");
        let sysml = authority.parse(&source);
        let kerml = authority.parse(&source.with_language(SourceLanguage::KerML));
        assert_eq!(sysml.digest(), kerml.digest());
        assert_eq!(sysml.language(), SourceLanguage::SysML);
        assert_eq!(kerml.language(), SourceLanguage::KerML);
        assert_ne!(sysml, kerml);
        assert!(!sysml.same_tree_as(&kerml));
        assert!(!Arc::ptr_eq(&sysml.0, &kerml.0));
        assert!(Arc::ptr_eq(
            &kerml.0,
            &authority
                .parse(&source.with_language(SourceLanguage::KerML))
                .0
        ));
        assert!(Arc::ptr_eq(&sysml.0, &authority.parse(&source).0));
        assert_eq!(authority.memo_len(), 2);
        authority.retain([]);
        authority.retain([source.digest()]);
        assert_eq!(authority.memo_len(), 2);
        authority.retain([]);
        assert_eq!(
            authority.memo_len(),
            2,
            "live language-specific handles remain leased"
        );
        drop((sysml, kerml));
        authority.retain([]);
        assert_eq!(authority.memo_len(), 0);
    }

    #[test]
    fn concurrent_misses_share_one_parse_allocation() {
        let authority = Arc::new(SyntaxAuthority::new());
        let barrier = Arc::new(std::sync::Barrier::new(8));
        let content = format!(
            "package Shared {{ {} }}",
            (0..2_000)
                .map(|index| format!("part p{index};"))
                .collect::<String>()
        );
        let handles = (0..8)
            .map(|index| {
                let authority = Arc::clone(&authority);
                let barrier = Arc::clone(&barrier);
                let content = content.clone();
                std::thread::spawn(move || {
                    let document = document(&format!("memory://t/{index}.sysml"), &content);
                    barrier.wait();
                    authority.parse(&document)
                })
            })
            .collect::<Vec<_>>();
        let parsed = handles
            .into_iter()
            .map(|handle| handle.join().expect("parse worker"))
            .collect::<Vec<_>>();
        assert!(parsed
            .windows(2)
            .all(|pair| Arc::ptr_eq(&pair[0].0, &pair[1].0)));
        assert_eq!(authority.memo_len(), 1);
    }

    #[test]
    fn admission_parts_share_the_tree() {
        let parsed = SyntaxAuthority::new().parse(&document("memory://t/a.sysml", "package A;"));
        let (tree, _) = parsed.admission_parts();
        assert!(Arc::ptr_eq(&tree, &parsed.0.document));
    }

    #[test]
    fn a_parser_failure_is_a_diagnostic_over_an_empty_tree() {
        let digest = ContentDigest::of_bytes(b"x");
        // There is no known panicking input at the pinned revision; exercise the failure path
        // directly so its shape is pinned.
        let empty = sysml_v2_parser::parse_for_editor_owned(String::new());
        let mut failed = ParsedSource::from_result(empty, digest, true);
        Arc::get_mut(&mut failed.0).unwrap().diagnostics = Box::new([SyntaxDiagnostic {
            source_range: None,
            severity: SyntaxDiagnosticSeverity::Error,
            category: SyntaxDiagnosticCategory::ParserFailure,
            message: String::new(),
            code: None,
            line: None,
            column: None,
            offset: None,
            length: None,
            expected: None,
            found: None,
            suggestion: None,
            is_cascade: None,
        }]);
        assert!(failed.parser_failed());
        assert!(!failed.is_clean());
        assert!(!failed.has_root_elements());
        assert_eq!(
            failed.first_error().map(|d| d.category),
            Some(SyntaxDiagnosticCategory::ParserFailure)
        );
    }

    #[test]
    fn retain_keeps_held_and_touched_revisions_and_drops_the_rest() {
        let authority = SyntaxAuthority::new();
        let search_only = document("memory://lib/l.sysml", "library package L;");
        let held = authority.parse(&search_only);
        for revision in 0..100 {
            authority.parse(&document(
                "memory://t/edit.sysml",
                &format!("package Edit {{ part p{revision}; }}"),
            ));
        }
        let admitted = document("memory://t/edit.sysml", "package Edit { part p99; }");
        let admitted_parsed = authority.parse(&admitted);

        // Everything parsed since the previous sweep survives this one: a revision the host
        // parsed but has not admitted yet must not be evicted underneath it.
        authority.retain([admitted.digest(), held.digest()]);
        assert_eq!(authority.memo_len(), 101);
        // Nothing touched since; only what the host still holds survives.
        authority.retain([admitted.digest(), held.digest()]);
        assert_eq!(
            authority.memo_len(),
            2,
            "admitted plus the held search-only file"
        );

        let in_flight = authority.parse(&document(
            "memory://t/edit.sysml",
            "package Edit { part p100; }",
        ));
        authority.retain([admitted.digest()]);
        assert_eq!(
            authority.memo_len(),
            3,
            "admitted, in-flight, and the live search-only handle"
        );
        assert!(Arc::ptr_eq(
            &authority
                .parse(&document(
                    "memory://t/edit.sysml",
                    "package Edit { part p100; }"
                ))
                .0,
            &in_flight.0
        ));
        assert!(Arc::ptr_eq(
            &authority.parse(&admitted).0,
            &admitted_parsed.0
        ));
        // An independent owner's sweep cannot evict a still-leased revision and reparse it.
        assert!(Arc::ptr_eq(&authority.parse(&search_only).0, &held.0));
        drop((held, in_flight, admitted_parsed));
        authority.retain([]);
        authority.retain([]);
        assert_eq!(authority.memo_len(), 0);
    }
}
