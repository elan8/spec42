use std::path::PathBuf;

use sysml_model::{resolve_library_closure, LibraryClosureOptions, WorkspaceSource};
use tower_lsp::lsp_types::Url;

use crate::common::util;

/// Load library files in the import closure of workspace sources (not full library trees).
pub(crate) fn load_library_closure_scan_entries(
    workspace_sources: &[WorkspaceSource<'_>],
    library_paths: &[Url],
) -> Result<Vec<(Url, String)>, String> {
    let roots = library_paths
        .iter()
        .filter_map(|uri| {
            uri.to_file_path()
                .ok()
                .map(|path| path.to_string_lossy().replace('\\', "/"))
        })
        .collect::<Vec<_>>();
    if roots.is_empty() {
        return Ok(Vec::new());
    }
    let loaded =
        resolve_library_closure(workspace_sources, &roots, &LibraryClosureOptions::default())?;
    let mut entries = Vec::with_capacity(loaded.len());
    for file in loaded {
        let path = PathBuf::from(&file.root).join(&file.path);
        let uri = Url::from_file_path(&path)
            .map_err(|_| format!("library file path is not a file URL: {}", path.display()))?;
        entries.push((util::normalize_file_uri(&uri), file.content));
    }
    Ok(entries)
}

pub(crate) fn library_full_scan_enabled() -> bool {
    util::env_flag_enabled("SPEC42_LIBRARY_FULL_SCAN", false)
}
