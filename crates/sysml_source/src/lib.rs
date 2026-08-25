//! The source authority.
//!
//! This crate is the only place that reads SysML text from disk, admits text as an identified
//! document, normalises a URI or a line ending, or computes a content digest. Everything above it
//! works with [`SourceDocument`] handles obtained from a [`SourceAuthority`]; nothing above it can
//! construct a document from raw parts, so every document in the system carries the identity and
//! normalisation this crate decided.
//!
//! The crate knows nothing about SysML grammar. Its single dependant is the semantic authority,
//! which re-exports these types for the query facade.

use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

/// The conventional KerML/SysML project manifest name.
///
/// This is a source-boundary convention, not a parsed KPAR contract: source discovery only needs
/// to know whether the marker exists. KPAR remains the authority for its schema and contents.
pub const PROJECT_MANIFEST_FILE: &str = ".project.json";

/// The filesystem boundary owning a source file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectBoundary {
    root: PathBuf,
    manifest: Option<PathBuf>,
}

impl ProjectBoundary {
    pub fn project_root(&self) -> &Path {
        &self.root
    }

    pub fn project_manifest(&self) -> Option<&Path> {
        self.manifest.as_deref()
    }

    pub fn is_manifestless(&self) -> bool {
        self.manifest.is_none()
    }
}

/// Finds the nearest project manifest owning `source`, without walking above `ceiling`.
///
/// The ceiling is the host's admission boundary (an editor workspace folder or an explicit batch
/// workspace root). If no manifest is found, the ceiling is returned as the manifestless fallback
/// project. A nested manifest therefore owns its subtree instead of being absorbed by a parent.
pub fn discover_project_boundary(source: &Path, ceiling: &Path) -> Option<ProjectBoundary> {
    let source = canonicalize_or_self(source);
    let ceiling = canonicalize_or_self(ceiling);
    let mut current = if source.is_dir() {
        source
    } else {
        source.parent()?.to_path_buf()
    };
    if !current.starts_with(&ceiling) {
        return None;
    }
    loop {
        let manifest = current.join(PROJECT_MANIFEST_FILE);
        if manifest.is_file() {
            return Some(ProjectBoundary {
                root: current,
                manifest: Some(manifest),
            });
        }
        if current == ceiling {
            return Some(ProjectBoundary {
                root: ceiling,
                manifest: None,
            });
        }
        current = current.parent()?.to_path_buf();
    }
}

fn canonicalize_or_self(path: &Path) -> PathBuf {
    fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
}
use std::sync::Arc;

pub use source_identity;
pub use source_identity::{ContentDigest, RootDigest};
pub use url::Url;

/// Where a document comes from, which decides how the semantic authority treats it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SourceKind {
    Workspace,
    StandardLibrary,
    Library,
    External,
}

impl SourceKind {
    pub fn is_library(self) -> bool {
        matches!(self, SourceKind::StandardLibrary | SourceKind::Library)
    }
}

/// An admitted source document: normalised URI, provenance, content digest, and the text itself.
///
/// Fields are private. A document can only be produced by a [`SourceAuthority`], which is what
/// guarantees that its digest is of exactly the text it carries after the one line-ending policy
/// has been applied, and that its URI is in the one normalised form the rest of the system keys
/// on. Cloning is a reference-count bump: hosts hold one of these per open file and clone it into
/// every publication request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceDocument {
    uri: Url,
    kind: SourceKind,
    digest: ContentDigest,
    content: Arc<str>,
    path_hint: Option<Box<str>>,
    library_root_slot: Option<u32>,
    library_relative_path: Option<Box<str>>,
}

impl SourceDocument {
    pub fn uri(&self) -> &Url {
        &self.uri
    }

    pub fn kind(&self) -> SourceKind {
        self.kind
    }

    /// The BLAKE3 digest of [`Self::content`].
    pub fn digest(&self) -> ContentDigest {
        self.digest
    }

    /// The admitted text, after line-ending normalisation.
    pub fn content(&self) -> &str {
        &self.content
    }

    /// The admitted text as a shared allocation, for consumers that need to hold it.
    pub fn content_arc(&self) -> Arc<str> {
        Arc::clone(&self.content)
    }

    pub fn byte_len(&self) -> usize {
        self.content.len()
    }

    /// A logical, root-relative path for hosts that address documents by path rather than URI.
    pub fn path_hint(&self) -> Option<&str> {
        self.path_hint.as_deref()
    }

    /// Configured precedence slot and root-relative path for a library document.
    ///
    /// These are semantic identity inputs, unlike `path_hint`, and are assigned by the library
    /// closure authority that knows the ordered root configuration.
    pub fn library_location(&self) -> Option<(u32, &str)> {
        Some((
            self.library_root_slot?,
            self.library_relative_path.as_deref()?,
        ))
    }

    /// The same document under a different provenance. Identity (URI and digest) is unchanged,
    /// so a memoised parse of this document still applies.
    pub fn with_kind(&self, kind: SourceKind) -> Self {
        Self {
            kind,
            ..self.clone()
        }
    }

    /// The same document with a logical path attached.
    pub fn with_path_hint(&self, path_hint: impl Into<Box<str>>) -> Self {
        Self {
            path_hint: Some(path_hint.into()),
            ..self.clone()
        }
    }

    /// Attach the configured library-root identity owned by the closure authority.
    pub fn with_library_location(
        &self,
        root_slot: u32,
        relative_path: impl Into<Box<str>>,
    ) -> Self {
        Self {
            library_root_slot: Some(root_slot),
            library_relative_path: Some(relative_path.into()),
            ..self.clone()
        }
    }
}

/// Why admission or loading failed. Every variant names what was being admitted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceError {
    InvalidUri {
        uri: String,
        reason: String,
    },
    EmptyIdentity,
    Read {
        path: PathBuf,
        reason: String,
    },
    NotUtf8 {
        path: PathBuf,
        reason: String,
    },
    PathNotFound {
        path: PathBuf,
    },
    NoSourcesFound,
    /// A provider-specific failure (cancellation, a remote fetch, …) with its own message.
    Provider(String),
}

impl fmt::Display for SourceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SourceError::InvalidUri { uri, reason } => {
                write!(formatter, "failed to parse source URI '{uri}': {reason}")
            }
            SourceError::EmptyIdentity => formatter.write_str("source identity must not be empty"),
            SourceError::Read { path, reason } => {
                write!(formatter, "failed to read {}: {reason}", path.display())
            }
            SourceError::NotUtf8 { path, reason } => {
                write!(
                    formatter,
                    "failed to decode {} as UTF-8: {reason}",
                    path.display()
                )
            }
            SourceError::PathNotFound { path } => {
                write!(formatter, "Path does not exist: {}", path.display())
            }
            SourceError::NoSourcesFound => formatter
                .write_str("No .sysml or .kerml files were found under the requested path."),
            SourceError::Provider(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for SourceError {}

/// Whether a path names a SysML or KerML source file.
pub fn is_sysml_like(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| {
            extension.eq_ignore_ascii_case("sysml") || extension.eq_ignore_ascii_case("kerml")
        })
}

/// The one line-ending policy: every admitted document is LF-only.
///
/// Editors send LF text regardless of what is on disk, and identity must not depend on which
/// side a document arrived from. Applied before the digest is taken, so the digest describes
/// exactly the text the document carries.
pub fn normalize_line_endings(text: &str) -> std::borrow::Cow<'_, str> {
    if text.contains('\r') {
        std::borrow::Cow::Owned(text.replace("\r\n", "\n").replace('\r', "\n"))
    } else {
        std::borrow::Cow::Borrowed(text)
    }
}

/// The one URI normalisation policy.
///
/// URI hosts are lowercased because DNS host names are case-insensitive, including for custom
/// in-memory schemes whose parsers may preserve authored case. Existing file paths are
/// canonicalised so alternate filesystem spellings (for example macOS's `/var` symlink to
/// `/private/var`) cannot create distinct document identities. Nonexistent paths keep their
/// lexical identity, which unsaved editor buffers require. Windows drive letters are lowercased
/// in either case.
pub fn normalize_uri(uri: &Url) -> Url {
    let mut uri = uri.clone();
    if let Some(host) = uri.host_str() {
        let normalized = host.to_ascii_lowercase();
        if normalized != host {
            // The existing parsed host proves this replacement is syntactically valid.
            uri.set_host(Some(&normalized))
                .expect("lowercasing a parsed URI host preserves validity");
        }
    }
    if uri.scheme() != "file" {
        return uri;
    }
    let Ok(path) = uri.to_file_path() else {
        return uri;
    };
    let path = fs::canonicalize(&path).unwrap_or(path);
    let Ok(normalized) = Url::from_file_path(path) else {
        return uri.clone();
    };
    lowercase_drive_letter(normalized)
}

/// Whether `candidate` lies under any of `roots` by URL authority and path segments.
///
/// This is lexical rather than filesystem-based so it also works for unsaved documents beneath
/// a root that does not exist yet. A root matches itself and descendants, but never a sibling
/// whose last common path segment merely starts with the same characters.
pub fn uri_under_any(candidate: &Url, roots: &[Url]) -> bool {
    roots.iter().any(|root| {
        if candidate.scheme() != root.scheme()
            || candidate.username() != root.username()
            || candidate.password() != root.password()
            || candidate.host_str() != root.host_str()
            || candidate.port_or_known_default() != root.port_or_known_default()
        {
            return false;
        }

        let root_path = root.path().trim_end_matches('/');
        candidate.path() == root_path
            || candidate
                .path()
                .strip_prefix(root_path)
                .is_some_and(|suffix| suffix.starts_with('/'))
    })
}

/// A filesystem path as a normalised `file://` URL. Directories get a trailing slash so they can
/// serve as prefixes for [`uri_under_any`].
pub fn path_to_file_url(path: &Path) -> Result<Url, SourceError> {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|error| SourceError::Read {
                path: path.to_path_buf(),
                reason: format!("failed to resolve current directory: {error}"),
            })?
            .join(path)
    };
    let canonical = fs::canonicalize(&absolute).unwrap_or(absolute);
    let url = if canonical.is_dir() {
        Url::from_directory_path(&canonical)
    } else {
        Url::from_file_path(&canonical)
    }
    .map_err(|_| SourceError::InvalidUri {
        uri: canonical.display().to_string(),
        reason: "failed to convert path to file URI".to_owned(),
    })?;
    Ok(lowercase_drive_letter(url))
}

fn lowercase_drive_letter(url: Url) -> Url {
    let path = url.path();
    let bytes = path.as_bytes();
    if bytes.len() >= 3 && bytes[0] == b'/' && bytes[1].is_ascii_uppercase() && bytes[2] == b':' {
        let lowered = format!("/{}{}", (bytes[1] as char).to_ascii_lowercase(), &path[2..]);
        if let Ok(normalized) = Url::parse(&format!("file://{lowered}")) {
            return normalized;
        }
    }
    url
}

/// A file the provider could not admit, with the reason, so a host can report it without the
/// whole load failing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkippedSource {
    pub path: PathBuf,
    pub error: SourceError,
}

/// What a provider produced: the admitted documents plus what it saw and could not admit.
#[derive(Debug, Clone, Default)]
pub struct SourceLoadReport {
    pub documents: Vec<SourceDocument>,
    pub skipped: Vec<SkippedSource>,
    pub roots_scanned: usize,
    pub roots_skipped: usize,
    pub candidate_files: usize,
}

impl SourceLoadReport {
    /// The documents, or the first skip as an error. For hosts where a partial load is wrong.
    pub fn require_complete(self) -> Result<Vec<SourceDocument>, SourceError> {
        match self.skipped.into_iter().next() {
            Some(skipped) => Err(skipped.error),
            None => Ok(self.documents),
        }
    }
}

/// Something that yields source documents through the authority.
pub trait SourceProvider {
    fn load(&self, authority: &SourceAuthority) -> Result<SourceLoadReport, SourceError>;
}

/// Documents already admitted, handed back as-is.
#[derive(Debug, Clone, Default)]
pub struct InMemoryProvider {
    documents: Vec<SourceDocument>,
}

impl InMemoryProvider {
    pub fn new(documents: Vec<SourceDocument>) -> Self {
        Self { documents }
    }

    pub fn documents(&self) -> &[SourceDocument] {
        &self.documents
    }
}

impl SourceProvider for InMemoryProvider {
    fn load(&self, _authority: &SourceAuthority) -> Result<SourceLoadReport, SourceError> {
        Ok(SourceLoadReport {
            documents: self.documents.clone(),
            ..SourceLoadReport::default()
        })
    }
}

/// Walks filesystem roots for SysML sources.
///
/// Ignore rules (`.gitignore`, `.ignore`, global gitignore, `.git/info/exclude`) are honoured by
/// default, so a project's own `target/` or `node_modules/` is not admitted as model source. Files
/// that cannot be read or decoded are reported as skipped rather than failing the load; a host
/// that needs all-or-nothing calls [`SourceLoadReport::require_complete`].
#[derive(Debug, Clone)]
pub struct FilesystemProvider {
    roots: Vec<PathBuf>,
    kind: SourceKind,
    honor_ignore_rules: bool,
    max_files_per_root: Option<usize>,
    respect_project_boundary: bool,
}

impl FilesystemProvider {
    pub fn new(roots: Vec<PathBuf>, kind: SourceKind) -> Self {
        Self {
            roots,
            kind,
            honor_ignore_rules: true,
            max_files_per_root: None,
            respect_project_boundary: false,
        }
    }

    /// Whether to honour `.gitignore`-style rules. Library roots are not projects and are walked
    /// in full.
    pub fn with_ignore_rules(mut self, honor: bool) -> Self {
        self.honor_ignore_rules = honor;
        self
    }

    /// Cap the number of files admitted per root; files beyond it are counted as skipped.
    pub fn with_max_files_per_root(mut self, limit: Option<usize>) -> Self {
        self.max_files_per_root = limit;
        self
    }

    /// Excludes sources owned by nested `.project.json` projects below each scan root.
    pub fn within_project_boundary(mut self, enabled: bool) -> Self {
        self.respect_project_boundary = enabled;
        self
    }

    fn walk(&self, root: &Path) -> Vec<PathBuf> {
        let mut builder = ignore::WalkBuilder::new(root);
        builder.follow_links(false).require_git(false);
        if !self.honor_ignore_rules {
            builder
                .git_ignore(false)
                .git_global(false)
                .git_exclude(false)
                .ignore(false)
                .hidden(false);
        }
        let mut paths = builder
            .build()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_some_and(|kind| kind.is_file()))
            .map(ignore::DirEntry::into_path)
            .filter(|path| is_sysml_like(path))
            .collect::<Vec<_>>();
        paths.sort();
        paths
    }
}

impl SourceProvider for FilesystemProvider {
    fn load(&self, authority: &SourceAuthority) -> Result<SourceLoadReport, SourceError> {
        let mut report = SourceLoadReport::default();
        for root in &self.roots {
            let root = fs::canonicalize(root).unwrap_or_else(|_| root.clone());
            if !root.is_dir() {
                report.roots_skipped += 1;
                continue;
            }
            report.roots_scanned += 1;
            let mut admitted_here = 0usize;
            for path in self.walk(&root) {
                if self.respect_project_boundary
                    && discover_project_boundary(&path, &root)
                        .is_some_and(|boundary| boundary.project_root() != root)
                {
                    continue;
                }
                report.candidate_files += 1;
                if self
                    .max_files_per_root
                    .is_some_and(|limit| admitted_here >= limit)
                {
                    report.skipped.push(SkippedSource {
                        path,
                        error: SourceError::Read {
                            path: root.clone(),
                            reason: "per-root file limit reached".to_owned(),
                        },
                    });
                    continue;
                }
                let hint = path
                    .strip_prefix(&root)
                    .map(|relative| relative.to_string_lossy().replace('\\', "/"))
                    .unwrap_or_else(|_| path.display().to_string());
                match authority.admit_path(&path, self.kind) {
                    Ok(document) => {
                        report.documents.push(document.with_path_hint(hint));
                        admitted_here += 1;
                    }
                    Err(error) => report.skipped.push(SkippedSource { path, error }),
                }
            }
        }
        Ok(report)
    }
}

/// The authority: every document enters the system through one of these methods.
///
/// It holds no state; it exists so that admission is a named capability a host receives rather
/// than a constructor anyone can call, and so the facade can hand out one handle for it.
#[derive(Debug, Clone, Default)]
pub struct SourceAuthority;

impl SourceAuthority {
    pub fn new() -> Self {
        Self
    }

    /// Admit text under a URI. The URI is normalised, the text gets the line-ending policy, and
    /// the digest is of the resulting text.
    pub fn admit(
        &self,
        uri: &str,
        content: impl AsRef<str>,
        kind: SourceKind,
    ) -> Result<SourceDocument, SourceError> {
        if uri.is_empty() {
            return Err(SourceError::EmptyIdentity);
        }
        let uri = Url::parse(uri).map_err(|error| SourceError::InvalidUri {
            uri: uri.to_owned(),
            reason: error.to_string(),
        })?;
        Ok(self.admit_url(uri, content.as_ref(), kind))
    }

    /// Admit text under an already-parsed URL.
    pub fn admit_url(&self, uri: Url, content: &str, kind: SourceKind) -> SourceDocument {
        let content: Arc<str> = Arc::from(normalize_line_endings(content).as_ref());
        SourceDocument {
            uri: normalize_uri(&uri),
            kind,
            digest: ContentDigest::of_bytes(content.as_bytes()),
            content,
            path_hint: None,
            library_root_slot: None,
            library_relative_path: None,
        }
    }

    /// Admit text under a `memory://{scope}/{path}` identity, for in-memory corpora.
    pub fn admit_memory(
        &self,
        scope: &str,
        path: &str,
        content: impl AsRef<str>,
        kind: SourceKind,
    ) -> Result<SourceDocument, SourceError> {
        let normalized = path.trim_start_matches('/').replace('\\', "/");
        if scope.is_empty() || normalized.is_empty() {
            return Err(SourceError::EmptyIdentity);
        }
        let document = self.admit(&format!("memory://{scope}/{normalized}"), content, kind)?;
        Ok(document.with_path_hint(path))
    }

    /// Read one file's text, for a host that admits it under an identity of its own (a memory
    /// scope for a fixture corpus). The read and the UTF-8 decision stay with the authority.
    pub fn read_text(&self, path: &Path) -> Result<String, SourceError> {
        let bytes = fs::read(path).map_err(|error| SourceError::Read {
            path: path.to_path_buf(),
            reason: error.to_string(),
        })?;
        String::from_utf8(bytes).map_err(|error| SourceError::NotUtf8 {
            path: path.to_path_buf(),
            reason: error.to_string(),
        })
    }

    /// Admit one file from disk.
    pub fn admit_path(&self, path: &Path, kind: SourceKind) -> Result<SourceDocument, SourceError> {
        let bytes = fs::read(path).map_err(|error| SourceError::Read {
            path: path.to_path_buf(),
            reason: error.to_string(),
        })?;
        let text = String::from_utf8(bytes).map_err(|error| SourceError::NotUtf8 {
            path: path.to_path_buf(),
            reason: error.to_string(),
        })?;
        let uri = path_to_file_url(path)?;
        Ok(self.admit_url(uri, &text, kind))
    }

    /// Load everything a provider yields.
    pub fn load(&self, provider: &dyn SourceProvider) -> Result<SourceLoadReport, SourceError> {
        provider.load(self)
    }

    /// Every SysML source under `roots`, walked in full (no ignore rules), under one provenance.
    pub fn list(
        &self,
        roots: &[PathBuf],
        kind: SourceKind,
    ) -> Result<SourceLoadReport, SourceError> {
        FilesystemProvider::new(roots.to_vec(), kind)
            .with_ignore_rules(false)
            .load(self)
    }

    /// The SysML source files named by `targets`: a file is itself, a directory is walked in full.
    /// Paths only; no admission. Errors if a target does not exist or nothing is found.
    pub fn discover(&self, targets: &[PathBuf]) -> Result<Vec<PathBuf>, SourceError> {
        let mut files = std::collections::BTreeSet::new();
        for target in targets {
            if !target.exists() {
                return Err(SourceError::PathNotFound {
                    path: target.clone(),
                });
            }
            let path = fs::canonicalize(target).unwrap_or_else(|_| target.clone());
            if path.is_file() {
                if is_sysml_like(&path) {
                    files.insert(path);
                }
                continue;
            }
            let walker = FilesystemProvider::new(vec![path.clone()], SourceKind::Workspace)
                .with_ignore_rules(false);
            files.extend(walker.walk(&path));
        }
        if files.is_empty() {
            return Err(SourceError::NoSourcesFound);
        }
        Ok(files.into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom_uri_schemes_are_preserved() {
        let document = SourceAuthority::new()
            .admit(
                "surreal://org/project/document/Architecture.sysml",
                "package Architecture {}",
                SourceKind::External,
            )
            .expect("custom URI");
        assert_eq!(document.uri().scheme(), "surreal");
    }

    #[test]
    fn custom_uri_hosts_have_one_case_insensitive_identity() {
        let authority = SourceAuthority::new();
        let authored = authority
            .admit(
                "memory://WORKSPACE/model.sysml",
                "package P;",
                SourceKind::Workspace,
            )
            .unwrap();
        let canonical = authority
            .admit(
                "memory://workspace/model.sysml",
                "package P;",
                SourceKind::Workspace,
            )
            .unwrap();

        assert_eq!(authored.uri(), canonical.uri());
        assert_eq!(authored.digest(), canonical.digest());
    }

    #[test]
    fn line_endings_are_normalised_before_the_digest_is_taken() {
        let authority = SourceAuthority::new();
        let lf = authority
            .admit(
                "memory://t/a.sysml",
                "package A;\npart def B;\n",
                SourceKind::Workspace,
            )
            .unwrap();
        let crlf = authority
            .admit(
                "memory://t/a.sysml",
                "package A;\r\npart def B;\r\n",
                SourceKind::Workspace,
            )
            .unwrap();
        assert_eq!(lf.digest(), crlf.digest());
        assert_eq!(lf.content(), crlf.content());
        assert!(!crlf.content().contains('\r'));
    }

    #[test]
    fn memory_paths_are_normalised_and_keep_a_hint() {
        let document = SourceAuthority::new()
            .admit_memory(
                "scope",
                "/nested\\Model.sysml",
                "package P;",
                SourceKind::Workspace,
            )
            .unwrap();
        assert_eq!(document.uri().as_str(), "memory://scope/nested/Model.sysml");
        assert_eq!(document.path_hint(), Some("/nested\\Model.sysml"));
        assert!(SourceAuthority::new()
            .admit_memory("", "x.sysml", "", SourceKind::Workspace)
            .is_err());
    }

    #[test]
    fn with_kind_keeps_identity() {
        let document = SourceAuthority::new()
            .admit("memory://t/a.sysml", "package A;", SourceKind::Workspace)
            .unwrap();
        let relabelled = document.with_kind(SourceKind::Library);
        assert_eq!(relabelled.digest(), document.digest());
        assert_eq!(relabelled.uri(), document.uri());
        assert_eq!(relabelled.kind(), SourceKind::Library);
    }

    #[test]
    fn filesystem_provider_honours_gitignore_and_reports_counts() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        fs::write(root.join(".gitignore"), "target/*\n").unwrap();
        fs::write(root.join("Real.sysml"), "package Real;").unwrap();
        fs::create_dir_all(root.join("target/scratch")).unwrap();
        fs::write(root.join("target/scratch/Copy.sysml"), "package Copy;").unwrap();

        let authority = SourceAuthority::new();
        let report = FilesystemProvider::new(vec![root.to_path_buf()], SourceKind::Workspace)
            .load(&authority)
            .unwrap();
        assert_eq!(report.documents.len(), 1);
        assert_eq!(report.candidate_files, 1);
        assert_eq!(report.documents[0].path_hint(), Some("Real.sysml"));

        let full = authority
            .list(&[root.to_path_buf()], SourceKind::Library)
            .unwrap();
        assert_eq!(full.documents.len(), 2, "list() walks without ignore rules");
        assert!(full
            .documents
            .iter()
            .all(|d| d.kind() == SourceKind::Library));
    }

    #[test]
    fn per_root_limit_is_reported_as_skipped() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        fs::write(root.join("A.sysml"), "package A;").unwrap();
        fs::write(root.join("B.sysml"), "package B;").unwrap();
        let report = FilesystemProvider::new(vec![root.to_path_buf()], SourceKind::Workspace)
            .with_max_files_per_root(Some(1))
            .load(&SourceAuthority::new())
            .unwrap();
        assert_eq!(report.documents.len(), 1);
        assert_eq!(report.skipped.len(), 1);
        assert!(report.clone().require_complete().is_err());
    }

    #[test]
    fn discover_accepts_files_and_directories_and_rejects_nothing_found() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        fs::write(root.join("A.sysml"), "package A;").unwrap();
        fs::write(root.join("notes.txt"), "no").unwrap();
        let authority = SourceAuthority::new();
        assert_eq!(authority.discover(&[root.to_path_buf()]).unwrap().len(), 1);
        assert_eq!(
            authority.discover(&[root.join("A.sysml")]).unwrap().len(),
            1
        );
        assert!(matches!(
            authority.discover(&[root.join("notes.txt")]),
            Err(SourceError::NoSourcesFound)
        ));
        assert!(matches!(
            authority.discover(&[root.join("missing")]),
            Err(SourceError::PathNotFound { .. })
        ));
    }

    #[test]
    fn nearest_manifest_owns_a_source_within_the_ceiling() {
        let temp = tempfile::tempdir().unwrap();
        let repo = temp.path();
        let parent = repo.join("model");
        let nested = parent.join("submodel");
        fs::create_dir_all(nested.join("src")).unwrap();
        fs::write(parent.join(PROJECT_MANIFEST_FILE), "{}").unwrap();
        fs::write(nested.join(PROJECT_MANIFEST_FILE), "{}").unwrap();
        let source = nested.join("src/Model.sysml");
        fs::write(&source, "package Model;").unwrap();

        let boundary = discover_project_boundary(&source, repo).unwrap();
        assert_eq!(boundary.project_root(), fs::canonicalize(&nested).unwrap());
        assert_eq!(
            boundary.project_manifest(),
            Some(
                fs::canonicalize(nested.join(PROJECT_MANIFEST_FILE))
                    .unwrap()
                    .as_path()
            )
        );
    }

    #[test]
    fn ceiling_is_the_manifestless_fallback_and_cannot_be_escaped() {
        let temp = tempfile::tempdir().unwrap();
        let repo = temp.path();
        let workspace = repo.join("workspace");
        fs::create_dir_all(workspace.join("scratch")).unwrap();
        fs::write(repo.join(PROJECT_MANIFEST_FILE), "{}").unwrap();
        let source = workspace.join("scratch/Model.sysml");
        fs::write(&source, "package Model;").unwrap();

        let boundary = discover_project_boundary(&source, &workspace).unwrap();
        assert!(boundary.is_manifestless());
        assert_eq!(
            boundary.project_root(),
            fs::canonicalize(&workspace).unwrap()
        );
        assert!(discover_project_boundary(&source, &repo.join("sibling")).is_none());
    }

    #[test]
    fn project_scan_excludes_sources_owned_by_nested_manifests() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        fs::write(root.join("Root.sysml"), "package Root;").unwrap();
        let nested = root.join("nested");
        fs::create_dir(&nested).unwrap();
        fs::write(nested.join(PROJECT_MANIFEST_FILE), "{}").unwrap();
        fs::write(nested.join("Nested.sysml"), "package Nested;").unwrap();

        let report = FilesystemProvider::new(vec![root.to_path_buf()], SourceKind::Workspace)
            .within_project_boundary(true)
            .load(&SourceAuthority::new())
            .unwrap();
        assert_eq!(report.documents.len(), 1);
        assert!(report.documents[0].uri().path().ends_with("Root.sysml"));
    }

    #[cfg(unix)]
    #[test]
    fn existing_file_uris_use_the_canonical_filesystem_identity() {
        use std::os::unix::fs::symlink;
        let temp = tempfile::tempdir().expect("tempdir");
        let real_dir = temp.path().join("real");
        let alias_dir = temp.path().join("alias");
        fs::create_dir(&real_dir).unwrap();
        symlink(&real_dir, &alias_dir).unwrap();
        let real_file = real_dir.join("Model.sysml");
        fs::write(&real_file, "package Model;").unwrap();
        let aliased = Url::from_file_path(alias_dir.join("Model.sysml")).unwrap();
        let canonical = Url::from_file_path(&real_file).unwrap();
        assert_eq!(normalize_uri(&aliased), normalize_uri(&canonical));
    }

    #[test]
    fn nonexistent_file_uri_keeps_its_lexical_identity() {
        let temp = tempfile::tempdir().expect("tempdir");
        let uri = Url::from_file_path(temp.path().join("Unsaved.sysml")).unwrap();
        assert_eq!(normalize_uri(&uri), uri);
    }

    #[test]
    fn uri_containment_uses_path_segment_boundaries() {
        let root = Url::parse("file:///workspace/library").unwrap();
        let child = Url::parse("file:///workspace/library/nested/Model.sysml").unwrap();
        let sibling_prefix = Url::parse("file:///workspace/library-copy/Model.sysml").unwrap();

        assert!(uri_under_any(&root, std::slice::from_ref(&root)));
        assert!(uri_under_any(&child, std::slice::from_ref(&root)));
        assert!(!uri_under_any(&sibling_prefix, &[root]));
    }

    #[test]
    fn uri_containment_does_not_require_the_root_to_exist() {
        let temp = tempfile::tempdir().expect("tempdir");
        let missing_root = temp.path().join("future-library");
        let root = path_to_file_url(&missing_root).unwrap();
        let child = Url::from_file_path(missing_root.join("nested/Model.sysml")).unwrap();
        let sibling =
            Url::from_file_path(temp.path().join("future-library-old/Model.sysml")).unwrap();

        assert!(uri_under_any(&child, std::slice::from_ref(&root)));
        assert!(!uri_under_any(&sibling, &[root]));
    }
}
