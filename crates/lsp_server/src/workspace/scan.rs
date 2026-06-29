use tower_lsp::lsp_types::Url;
use walkdir::WalkDir;

use crate::workspace::state::ScanSummary;

pub(crate) fn scan_sysml_files(roots: Vec<Url>) -> (Vec<(Url, String)>, ScanSummary) {
    let mut out = Vec::new();
    let mut summary = ScanSummary::default();
    for root in roots {
        summary.roots_scanned += 1;
        let path = match root.to_file_path() {
            Ok(path) => path,
            Err(_) => {
                summary.roots_skipped_non_file += 1;
                continue;
            }
        };
        for entry in WalkDir::new(path)
            .follow_links(false)
            .into_iter()
            .filter_map(|entry| entry.ok())
        {
            if !entry.file_type().is_file() {
                continue;
            }
            let ext = entry.path().extension().and_then(|ext| ext.to_str());
            if ext != Some("sysml") && ext != Some("kerml") {
                continue;
            }
            summary.candidate_files += 1;
            match std::fs::read_to_string(entry.path()) {
                Ok(raw) => match Url::from_file_path(entry.path()) {
                    Ok(uri) => {
                        summary.files_loaded += 1;
                        // Normalize CRLF → LF so disk content matches what VS Code
                        // sends in textDocument/didOpen (which always uses LF).
                        // Without this, files with CRLF line endings always appear
                        // "changed" in did_open, triggering unnecessary relinking.
                        let content = if raw.contains('\r') {
                            raw.replace("\r\n", "\n").replace('\r', "\n")
                        } else {
                            raw
                        };
                        out.push((uri, content));
                    }
                    Err(_) => summary.uri_failures += 1,
                },
                Err(_) => summary.read_failures += 1,
            }
        }
    }
    (out, summary)
}
