//! The parse authority: one parsed tree per source revision, memoised by content digest.
//!
//! [`SyntaxAuthority::parse`] is the only call site of the parser for admitted documents. It
//! returns a [`ParsedSource`], an `Arc` handle that the editor's syntax queries and the semantic
//! build share, so a revision is parsed once however many consumers ask about it. The memo is
//! invisible to callers: a hit and a miss return the same kind of value.

use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::{Arc, Mutex};

use hashbrown::{HashMap, HashSet};
use source_identity::ContentDigest;
use sysml_source::SourceDocument;
use sysml_v2_parser::{ParseError, ParsedDocument};

use super::{SyntaxDiagnostic, SyntaxDiagnosticCategory, SyntaxDiagnosticSeverity};

struct Inner {
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
/// Equality is identity: two handles are equal when they parse the same bytes. The tree is never
/// exposed; `pub(crate)` accessors serve the authority's own traversals and the semantic build.
#[derive(Clone)]
pub struct ParsedSource(Arc<Inner>);

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
        Arc::ptr_eq(&self.0, &other.0) || self.0.digest == other.0.digest
    }
}

impl Eq for ParsedSource {}

impl ParsedSource {
    /// Parse `content` outside any memo. Parser panics are captured and reported as a
    /// [`SyntaxDiagnosticCategory::ParserFailure`] diagnostic over an empty tree, so a host never
    /// has to special-case a document that failed to parse at all.
    pub(crate) fn parse_text(content: String, digest: ContentDigest) -> Self {
        let outcome = catch_unwind(AssertUnwindSafe(|| {
            sysml_v2_parser::parse_for_editor_owned(content)
        }));
        match outcome {
            Ok(result) => Self::from_result(result, digest, false),
            Err(_) => {
                let empty = sysml_v2_parser::parse_for_editor_owned(String::new());
                let mut source = Self::from_result(empty, digest, true);
                let inner = Arc::get_mut(&mut source.0).expect("freshly built handle is unique");
                inner.diagnostics = Box::new([SyntaxDiagnostic {
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
        let diagnostics = result
            .errors
            .iter()
            .map(SyntaxDiagnostic::from_parse_error)
            .collect();
        Self(Arc::new(Inner {
            document: Arc::new(result.document),
            errors: result.errors,
            diagnostics,
            digest,
            parser_failed,
        }))
    }

    /// The source this document was parsed from, after BOM stripping.
    pub fn source(&self) -> &str {
        self.0.document.source.as_str()
    }

    /// The digest of the admitted content this tree was parsed from.
    pub fn digest(&self) -> ContentDigest {
        self.0.digest
    }

    /// Every parser diagnostic, in source order.
    pub fn diagnostics(&self) -> &[SyntaxDiagnostic] {
        &self.0.diagnostics
    }

    /// Whether the parse produced no diagnostic at all — the strict-parse notion of success.
    pub fn is_clean(&self) -> bool {
        self.0.diagnostics.is_empty()
    }

    /// The first diagnostic, for callers that treat any diagnostic as rejection.
    pub fn first_error(&self) -> Option<&SyntaxDiagnostic> {
        self.0.diagnostics.first()
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
        self.0.document.normalize_for_test_comparison()
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
    map: HashMap<ContentDigest, ParsedSource>,
    /// Digests currently being parsed outside the lock. Waiters for the same digest sleep until
    /// the sole parser publishes its result; unrelated digests continue in parallel.
    in_flight: HashSet<ContentDigest>,
    /// Digests looked up or inserted since the last sweep. A revision the host parsed but has
    /// not yet admitted survives the next sweep because of this.
    touched: HashSet<ContentDigest>,
}

/// The only place the parser is called for admitted documents.
///
/// One lock guards the memo and it is never held across a parse. Per-digest single-flight ensures
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
        self.parse_with_digest(document.digest(), document.content())
    }

    /// Parse text that is not an admitted document (an editor's candidate reformatting, a
    /// stateless caller). Digested here so identical text still hits the memo.
    pub fn parse_text(&self, text: &str) -> ParsedSource {
        self.parse_with_digest(ContentDigest::of_bytes(text.as_bytes()), text)
    }

    fn parse_with_digest(&self, digest: ContentDigest, content: &str) -> ParsedSource {
        loop {
            let mut state = self.lock();
            state.touched.insert(digest);
            if let Some(hit) = state.map.get(&digest) {
                return hit.clone();
            }
            if state.in_flight.insert(digest) {
                break;
            }
            drop(
                self.parsed
                    .wait(state)
                    .unwrap_or_else(std::sync::PoisonError::into_inner),
            );
        }
        let parsed = ParsedSource::parse_text(content.to_owned(), digest);
        let mut state = self.lock();
        state.map.insert(digest, parsed.clone());
        state.in_flight.remove(&digest);
        self.parsed.notify_all();
        parsed
    }

    /// Keep only the revisions in `keep` plus everything touched since the last sweep.
    ///
    /// The host passes the digests of every handle it still holds, admitted or not. Anything else
    /// is a superseded revision and is dropped; the sweep and the clearing of the touched set are
    /// one operation under the lock, so a parse racing the sweep is never lost.
    pub fn retain(&self, keep: impl IntoIterator<Item = ContentDigest>) {
        let mut state = self.lock();
        let mut survivors: HashSet<ContentDigest> = keep.into_iter().collect();
        survivors.extend(state.touched.drain());
        state.map.retain(|digest, _| survivors.contains(digest));
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
        assert_send_sync::<SyntaxAuthority>();
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
            2,
            "admitted plus the in-flight revision"
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
        // The search-only file was neither kept nor touched and is gone; the host's own handle
        // keeps its tree alive regardless.
        assert!(!Arc::ptr_eq(&authority.parse(&search_only).0, &held.0));
    }
}
