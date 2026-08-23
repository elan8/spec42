use std::path::PathBuf;

use sysml_query::library::{LibraryClosureOptions, LibraryRoot};
use sysml_query::source::{SourceDocument, SourceKind};
use sysml_query::syntax::ParsedSource;
use sysml_query::Services;
use tower_lsp::lsp_types::Url;

use crate::common::util;

/// The library documents in the import closure of the workspace sources (not full library trees).
///
/// Roots under `standard_library_paths` carry standard-library provenance; the rest are generic
/// libraries. Resolution runs through the host's services so every document returned is already
/// a memo hit for the publication that admits it.
pub(crate) fn load_library_closure_documents(
    workspace: &[ParsedSource],
    library_paths: &[Url],
    standard_library_paths: &[Url],
    services: &Services,
) -> Result<Vec<SourceDocument>, String> {
    let roots = library_paths
        .iter()
        .filter_map(|uri| {
            let path: PathBuf = uri.to_file_path().ok()?;
            let kind = if standard_library_paths.contains(uri) {
                SourceKind::StandardLibrary
            } else {
                SourceKind::Library
            };
            Some(LibraryRoot { path, kind })
        })
        .collect::<Vec<_>>();
    if roots.is_empty() {
        return Ok(Vec::new());
    }
    services
        .library
        .resolve(workspace, &roots, &LibraryClosureOptions::default())
        .map(|closure| closure.documents)
        .map_err(|error| error.to_string())
}

pub(crate) fn library_full_scan_enabled() -> bool {
    util::env_flag_enabled("SPEC42_LIBRARY_FULL_SCAN", false)
}
