//! The syntax service: the one parser call, its memo, and syntax-fidelity queries over a parsed
//! tree.
//!
//! Consumers parse through a [`SyntaxService`] and keep the [`ParsedSource`] handle it returns;
//! outline, folding, token roles, package declarations and closure facts are methods on the
//! handle. No function here takes source text except [`SyntaxService::parse_text`], which exists
//! for text that is not an admitted document (a formatter's candidate output).

use std::sync::Arc;

pub use sysml_resolution::source::ContentDigest;
pub use sysml_resolution::syntax::{
    is_reserved_keyword, reserved_keywords, ImportScope, PackageTargets, ParsedSource,
    SyntaxClosureFacts, SyntaxImport,
    SyntaxDiagnostic, SyntaxDiagnosticCategory, SyntaxDiagnosticSeverity, SyntaxFoldingKind,
    SyntaxFoldingRegion, SyntaxOutlineKind, SyntaxOutlineNode, SyntaxRange, SyntaxRole,
    RESERVED_KEYWORDS,
};

use sysml_resolution::syntax::SyntaxAuthority;

/// Handle on the syntax authority. Cheap to clone; all clones share one memo.
#[derive(Debug, Clone, Default)]
pub struct SyntaxService {
    inner: Arc<SyntaxAuthority>,
}

impl SyntaxService {
    pub fn new() -> Self {
        Self::default()
    }

    /// The parsed tree for an admitted document: a memo hit, or one parse.
    pub fn parse(&self, document: &crate::source::SourceDocument) -> ParsedSource {
        self.inner.parse(document)
    }

    /// Parse text that is not an admitted document.
    pub fn parse_text(&self, text: &str) -> ParsedSource {
        self.inner.parse_text(text)
    }

    /// Keep only the revisions in `keep` plus everything parsed since the last call. Hosts pass
    /// the digest of every handle they still hold.
    pub fn retain(&self, keep: impl IntoIterator<Item = ContentDigest>) {
        self.inner.retain(keep)
    }

    /// Whether reformatting `source` into `candidate` provably preserves what the parser sees.
    pub fn reformatting_preserves_meaning(&self, source: &ParsedSource, candidate: &str) -> bool {
        sysml_resolution::syntax::reformatting_preserves_meaning_of(
            source,
            &self.inner.parse_text(candidate),
        )
    }

    /// The distinct namespaces the SysML sources under `path` reach into.
    ///
    /// A directory is walked and a file is read alone; `file_budget` caps how many files are
    /// parsed, so a caller deciding one coarse question about a large tree stays bounded. The
    /// budget is the caller's policy, which is why it is a parameter rather than a constant here.
    ///
    /// Files are admitted through the source service and answered from their parsed trees: the
    /// question is "what does this source name", and a substring search over the bytes answered
    /// yes for a name in a comment, in a string, or inside a longer identifier.
    pub fn referenced_namespace_roots(
        &self,
        source: &crate::source::SourceService,
        path: &std::path::Path,
        file_budget: usize,
    ) -> std::collections::BTreeSet<String> {
        let mut roots = std::collections::BTreeSet::new();
        let Ok(files) = source.discover(&[path.to_path_buf()]) else {
            return roots;
        };
        for file in files.into_iter().take(file_budget) {
            let Ok(document) = source.admit_path(&file, crate::source::SourceKind::Workspace)
            else {
                continue;
            };
            roots.extend(self.parse(&document).referenced_namespace_roots());
        }
        roots
    }

    pub fn reserved_keywords(&self) -> &'static [&'static str] {
        reserved_keywords()
    }

    pub(crate) fn authority(&self) -> &Arc<SyntaxAuthority> {
        &self.inner
    }
}
