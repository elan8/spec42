//! Workspace vs library URI classification.

use tower_lsp::lsp_types::Url;

/// Returns true if `uri` is under any of the library path roots (path prefix check).
pub fn uri_under_any_library(uri: &Url, library_paths: &[Url]) -> bool {
    let uri_path = match uri.to_file_path() {
        Ok(p) => p,
        Err(_) => return false,
    };
    for lib in library_paths {
        if let Ok(lib_path) = lib.to_file_path() {
            if uri_path.starts_with(&lib_path) {
                return true;
            }
        }
    }
    false
}
