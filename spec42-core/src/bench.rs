//! Public wrappers intended for Criterion benchmarks.
//!
//! The LSP server’s workspace/index pipeline is intentionally `pub(crate)`. Benches live in
//! `benches/` and compile as an external crate, so they need a narrow public surface to call into
//! the real implementation without exposing the entire workspace module as public API.

use tower_lsp::lsp_types::Url;

/// Scan roots for `.sysml` / `.kerml` files and read their contents.
///
/// This mirrors the discovery+read portion of Spec42’s workspace scan.
pub fn scan_sysml_files(roots: Vec<Url>) -> Vec<(Url, String)> {
    crate::workspace::scan_sysml_files(roots).0
}

/// Parse a batch of scanned `(Url, String)` entries using Spec42’s editor-oriented parse path.
///
/// Returns the number of parsed entries (one per input file).
pub fn parse_scanned_entries(entries: Vec<(Url, String)>, parallel_enabled: bool) -> usize {
    crate::workspace::parse_scanned_entries(entries, parallel_enabled).len()
}

