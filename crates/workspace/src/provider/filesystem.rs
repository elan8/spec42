use std::fs;
use std::path::{Path, PathBuf};

use ignore::WalkBuilder;
use url::Url;

use crate::library::{resolve_library_closure, LibraryClosureOptions, WorkspaceSource};
use source_identity::ContentDigest;
use sysml_source::{SysmlDocument, SysmlDocumentProvider, SysmlDocumentSourceKind};

/// Reads `path` as bytes exactly once, computes its BLAKE3 content digest from that single
/// buffer, and decodes it as UTF-8 from the same buffer (plan §5.1). A read failure or a UTF-8
/// decode failure is a provider error; it is never swallowed or hidden behind a cache hit.
fn read_source_exactly_once(path: &Path) -> Result<(String, ContentDigest, i64), String> {
    let bytes =
        fs::read(path).map_err(|err| format!("failed to read {}: {err}", path.display()))?;
    let digest = ContentDigest::of_bytes(&bytes);
    let byte_len = bytes.len() as i64;
    let content = String::from_utf8(bytes)
        .map_err(|err| format!("failed to decode {} as UTF-8: {err}", path.display()))?;
    Ok((content, digest, byte_len))
}

#[derive(Debug, Clone)]
pub struct FileSystemDocumentProvider {
    target: PathBuf,
    workspace_root: Option<PathBuf>,
    library_paths: Vec<PathBuf>,
    standard_library_paths: Vec<PathBuf>,
    full_library_scan: bool,
    library_seed_packages: Vec<String>,
}

pub type HostFilesystemProvider = FileSystemDocumentProvider;

impl FileSystemDocumentProvider {
    pub fn new(
        target: PathBuf,
        workspace_root: Option<PathBuf>,
        library_paths: Vec<PathBuf>,
    ) -> Self {
        Self {
            target,
            workspace_root,
            library_paths,
            standard_library_paths: Vec::new(),
            full_library_scan: false,
            library_seed_packages: Vec::new(),
        }
    }

    pub fn from_paths(
        target: &Path,
        workspace_root: Option<&Path>,
        library_paths: &[PathBuf],
    ) -> Self {
        Self::new(
            target.to_path_buf(),
            workspace_root.map(Path::to_path_buf),
            library_paths.to_vec(),
        )
    }

    pub fn from_paths_with_standard_library(
        target: &Path,
        workspace_root: Option<&Path>,
        library_paths: &[PathBuf],
        standard_library_paths: &[PathBuf],
    ) -> Self {
        Self::from_paths(target, workspace_root, library_paths)
            .with_standard_library_paths(standard_library_paths.to_vec())
    }

    /// When enabled, every file under each library root is loaded wholesale
    /// instead of only the files reachable from the workspace's import closure.
    pub fn with_full_library_scan(mut self, enabled: bool) -> Self {
        self.full_library_scan = enabled;
        self
    }

    /// Adds package names that seed the otherwise reference-scoped library closure.
    pub fn with_library_seed_packages(mut self, packages: Vec<String>) -> Self {
        self.library_seed_packages = packages;
        self
    }

    /// Marks which configured library roots are the canonical SysML standard library. Other
    /// library roots remain dependencies and cannot satisfy universal implied relationships.
    pub fn with_standard_library_paths(mut self, paths: Vec<PathBuf>) -> Self {
        self.standard_library_paths = paths;
        self
    }
}

impl SysmlDocumentProvider for FileSystemDocumentProvider {
    fn load_documents(&self) -> Result<Vec<SysmlDocument>, String> {
        let workspace_root = resolve_workspace_root(&self.target, self.workspace_root.as_deref());
        let workspace_root = canonicalize_or_self(&workspace_root);
        let standard_library_paths = self
            .standard_library_paths
            .iter()
            .map(|path| canonicalize_or_self(path))
            .collect::<Vec<_>>();

        let mut documents = Vec::new();
        let mut workspace_file_contents = Vec::new();
        let mut workspace_path_hints = Vec::new();
        let mut workspace_digests = Vec::new();
        let mut workspace_byte_sizes = Vec::new();

        if workspace_root.exists() {
            for path in collect_sysml_files(&workspace_root)? {
                let (content, digest, byte_len) = read_source_exactly_once(&path)?;
                let path_hint = path
                    .strip_prefix(&workspace_root)
                    .ok()
                    .map(|relative| relative.to_string_lossy().replace('\\', "/"))
                    .unwrap_or_else(|| path.display().to_string());
                workspace_path_hints.push(path_hint);
                workspace_file_contents.push(content);
                workspace_digests.push(digest);
                workspace_byte_sizes.push(byte_len);
            }
        }

        for ((path_hint, content), (digest, byte_len)) in workspace_path_hints
            .iter()
            .zip(workspace_file_contents.iter())
            .zip(workspace_digests.iter().zip(workspace_byte_sizes.iter()))
        {
            let path = workspace_root.join(path_hint);
            let uri = path_to_url(&path)?;
            documents.push(SysmlDocument {
                uri,
                content: content.clone(),
                path_hint: Some(path_hint.clone()),
                source_kind: SysmlDocumentSourceKind::Workspace,
                content_digest: Some(*digest),
                byte_size: Some(*byte_len),
            });
        }

        if self.full_library_scan {
            for library_path in &self.library_paths {
                let library_root = canonicalize_or_self(library_path);
                let source_kind = library_source_kind(&library_root, &standard_library_paths);
                if !library_root.exists() {
                    continue;
                }
                for path in collect_sysml_files(&library_root)? {
                    let (content, digest, byte_len) = read_source_exactly_once(&path)?;
                    let path_hint = path
                        .strip_prefix(&library_root)
                        .ok()
                        .map(|relative| relative.to_string_lossy().replace('\\', "/"))
                        .unwrap_or_else(|| path.display().to_string());
                    let uri = path_to_url(&path)?;
                    documents.push(SysmlDocument {
                        uri,
                        content,
                        path_hint: Some(path_hint),
                        source_kind,
                        content_digest: Some(digest),
                        byte_size: Some(byte_len),
                    });
                }
            }
        } else {
            let library_roots: Vec<String> = self
                .library_paths
                .iter()
                .map(|path| {
                    canonicalize_or_self(path)
                        .to_string_lossy()
                        .replace('\\', "/")
                })
                .collect();
            if !library_roots.is_empty() && !workspace_file_contents.is_empty() {
                let workspace_sources: Vec<WorkspaceSource<'_>> = workspace_path_hints
                    .iter()
                    .zip(workspace_file_contents.iter())
                    .map(|(path_hint, content)| WorkspaceSource {
                        path: path_hint.as_str(),
                        content: content.as_str(),
                    })
                    .collect();
                let options = LibraryClosureOptions {
                    seed_packages: self.library_seed_packages.clone(),
                    ..LibraryClosureOptions::default()
                };
                let loaded = resolve_library_closure(&workspace_sources, &library_roots, &options)?;
                for file in loaded {
                    let path = PathBuf::from(&file.root).join(&file.path);
                    let uri = path_to_url(&path)?;
                    // `resolve_library_closure` already performed the single admitting read and
                    // UTF-8 decode of this file. Since that decode succeeded, re-encoding the
                    // resulting `String` via `as_bytes()` reconstructs the exact original byte
                    // sequence (UTF-8 decode is bijective for valid input), so hashing it here is
                    // equivalent to hashing the original read buffer directly.
                    let digest = ContentDigest::of_bytes(file.content.as_bytes());
                    let byte_size = file.content.len() as i64;
                    documents.push(SysmlDocument {
                        uri,
                        content: file.content,
                        path_hint: Some(file.path.replace('\\', "/")),
                        source_kind: library_source_kind(
                            &canonicalize_or_self(&PathBuf::from(&file.root)),
                            &standard_library_paths,
                        ),
                        content_digest: Some(digest),
                        byte_size: Some(byte_size),
                    });
                }
            }
        }

        Ok(documents)
    }
}

fn library_source_kind(root: &Path, standard_library_paths: &[PathBuf]) -> SysmlDocumentSourceKind {
    if standard_library_paths.iter().any(|path| path == root) {
        SysmlDocumentSourceKind::StandardLibrary
    } else {
        SysmlDocumentSourceKind::Library
    }
}

fn collect_sysml_files(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut paths = Vec::new();
    for entry in WalkBuilder::new(root)
        .follow_links(false)
        .require_git(false)
        .build()
        .filter_map(Result::ok)
    {
        if !entry
            .file_type()
            .is_some_and(|file_type| file_type.is_file())
        {
            continue;
        }
        let path = entry.path();
        if path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| {
                ext.eq_ignore_ascii_case("sysml") || ext.eq_ignore_ascii_case("kerml")
            })
        {
            paths.push(path.to_path_buf());
        }
    }
    paths.sort();
    Ok(paths)
}

fn canonicalize_or_self(path: &Path) -> PathBuf {
    std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
}

fn path_to_url(path: &Path) -> Result<Url, String> {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|err| format!("failed to resolve current directory: {err}"))?
            .join(path)
    };
    let canonical = canonicalize_or_self(&absolute);
    let url = Url::from_file_path(&canonical).map_err(|_| {
        format!(
            "failed to convert path to file URI: {}",
            canonical.display()
        )
    })?;
    Ok(normalize_file_url_drive_letter(url))
}

/// Lowercases the Windows drive letter in a `file://` URL so all paths use a
/// consistent form (`file:///c:/...` not `file:///C:/...`). This matches the
/// normalisation applied by the kernel/LSP layer and ensures graph node URIs
/// are comparable to the target URLs used in workspace lookups.
fn normalize_file_url_drive_letter(url: Url) -> Url {
    if url.scheme() != "file" {
        return url;
    }
    let path = url.path();
    // Windows path: /C:/... — lowercase the drive letter at index 1.
    if path.len() >= 3 {
        let bytes = path.as_bytes();
        if bytes[0] == b'/' && bytes[1].is_ascii_uppercase() && bytes[2] == b':' {
            let new_path = format!("/{}{}", (bytes[1] as char).to_ascii_lowercase(), &path[2..]);
            if let Ok(normalized) = Url::parse(&format!("file://{new_path}")) {
                return normalized;
            }
        }
    }
    url
}

fn resolve_workspace_root(target: &Path, workspace_root: Option<&Path>) -> PathBuf {
    workspace_root.map(Path::to_path_buf).unwrap_or_else(|| {
        if target.is_dir() {
            target.to_path_buf()
        } else {
            target
                .parent()
                .map(Path::to_path_buf)
                .unwrap_or_else(|| PathBuf::from("."))
        }
    })
}
