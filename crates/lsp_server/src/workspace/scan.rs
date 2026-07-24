use ignore::WalkBuilder;
use tower_lsp::lsp_types::Url;

use crate::workspace::state::ScanSummary;

/// Walks a workspace root for `.sysml`/`.kerml` files, honoring the same ignore rules a
/// developer already relies on for everything else in the project: `.gitignore`, `.ignore`,
/// global gitignore, and `.git/info/exclude` (via the `ignore` crate -- the same one ripgrep and
/// rust-analyzer use for this). Without this, a project's own `target/`, `node_modules/`, or
/// similar build/scratch directory -- already declared not-part-of-the-project in `.gitignore` --
/// gets scanned like any other source file: duplicate/near-duplicate copies of real model files
/// (e.g. test fixtures, generated diagram exports) get parsed and merged into the live workspace
/// model right alongside the real ones, both bloating startup time and producing duplicate/
/// conflicting views for anything named the same across copies.
fn walk_root(path: &std::path::Path) -> impl Iterator<Item = std::path::PathBuf> {
    WalkBuilder::new(path)
        .follow_links(false)
        // Honor `.gitignore`/`.git/info/exclude` even if `path` isn't recognized as sitting
        // inside an actual git working tree (the crate's default `require_git(true)` would
        // otherwise silently skip them in that case) -- a workspace root handed to the LSP isn't
        // guaranteed to itself contain `.git`, and `.gitignore`-style excludes are worth honoring
        // either way.
        .require_git(false)
        .build()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_some_and(|ft| ft.is_file()))
        .map(|entry| entry.into_path())
}

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
        for entry in walk_root(&path) {
            let ext = entry.extension().and_then(|ext| ext.to_str());
            if ext != Some("sysml") && ext != Some("kerml") {
                continue;
            }
            summary.candidate_files += 1;
            match std::fs::read_to_string(&entry) {
                Ok(raw) => match Url::from_file_path(&entry) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn skips_gitignored_directories_like_target() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();

        fs::write(root.join(".gitignore"), "target/*\n").expect("write .gitignore");
        fs::write(root.join("Real.sysml"), "package Real;").expect("write real file");

        let target_model = root.join("target/scratch/model");
        fs::create_dir_all(&target_model).expect("mkdir target/scratch/model");
        fs::write(target_model.join("Copy.sysml"), "package Copy;").expect("write ignored file");

        let root_url = Url::from_file_path(root).expect("file url");
        let (entries, summary) = scan_sysml_files(vec![root_url]);

        assert_eq!(
            entries.len(),
            1,
            "expected only the non-ignored file, got: {:?}",
            entries
                .iter()
                .map(|(uri, _)| uri.as_str())
                .collect::<Vec<_>>()
        );
        assert!(entries[0].0.as_str().ends_with("Real.sysml"));
        assert_eq!(summary.candidate_files, 1);
    }

    #[test]
    fn still_scans_normally_with_no_gitignore_present() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        fs::write(root.join("A.sysml"), "package A;").expect("write A");
        fs::create_dir_all(root.join("nested")).expect("mkdir nested");
        fs::write(root.join("nested/B.sysml"), "package B;").expect("write B");

        let root_url = Url::from_file_path(root).expect("file url");
        let (entries, summary) = scan_sysml_files(vec![root_url]);

        assert_eq!(entries.len(), 2);
        assert_eq!(summary.candidate_files, 2);
    }
}
