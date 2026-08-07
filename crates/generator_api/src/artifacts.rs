use std::collections::BTreeMap;
use std::path::{Component, Path};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use unicode_normalization::UnicodeNormalization;

pub const MAX_ARTIFACT_PATH_BYTES: usize = 4 * 1024;

/// The host writes its ownership manifest here; a generator must not also claim it.
pub const RESERVED_MANIFEST_NAME: &str = ".spec42-generator-manifest.json";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactLimits {
    pub max_files: usize,
    pub max_file_bytes: usize,
    pub max_total_bytes: usize,
}

impl Default for ArtifactLimits {
    fn default() -> Self {
        Self {
            max_files: 1_000,
            max_file_bytes: 16 * 1024 * 1024,
            max_total_bytes: 128 * 1024 * 1024,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Artifact {
    pub path: String,
    pub content: Vec<u8>,
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ArtifactError {
    #[error("artifact path must not be empty")]
    EmptyPath,
    #[error("artifact path `{0}` is absolute or has a platform prefix")]
    AbsolutePath(String),
    #[error("artifact path `{0}` contains a forbidden component")]
    ForbiddenComponent(String),
    #[error("artifact path `{0}` must use `/` as its separator")]
    InvalidSeparator(String),
    #[error("artifact path contains NUL")]
    Nul,
    #[error("artifact path is {actual} bytes; the path limit is {limit}")]
    PathTooLong { actual: usize, limit: usize },
    #[error("artifact `{0}` was returned more than once")]
    Duplicate(String),
    #[error(
        "artifact `{path}` collides with `{existing}` on case- or normalization-insensitive \
         filesystems; they would be the same file on disk"
    )]
    CollidingPaths { path: String, existing: String },
    #[error("artifact path `{0}` is reserved by Spec42")]
    ReservedPath(String),
    #[error("artifact `{path}` is {actual} bytes; the per-file limit is {limit}")]
    FileTooLarge {
        path: String,
        actual: usize,
        limit: usize,
    },
    #[error("generator returned {actual} files; the limit is {limit}")]
    TooManyFiles { actual: usize, limit: usize },
    #[error("generator returned {actual} bytes; the total-output limit is {limit}")]
    TotalTooLarge { actual: usize, limit: usize },
}

#[derive(Debug, Clone)]
pub struct ArtifactSet {
    limits: ArtifactLimits,
    total_bytes: usize,
    files: BTreeMap<String, Vec<u8>>,
    /// Case- and normalization-folded paths, to catch collisions the `files` key cannot.
    folded: BTreeMap<String, String>,
}

impl ArtifactSet {
    pub fn new(limits: ArtifactLimits) -> Self {
        Self {
            limits,
            total_bytes: 0,
            files: BTreeMap::new(),
            folded: BTreeMap::new(),
        }
    }

    pub fn emit(&mut self, path: &str, content: Vec<u8>) -> Result<(), ArtifactError> {
        let normalized = normalize_artifact_path(path)?;
        if self.files.contains_key(&normalized) {
            return Err(ArtifactError::Duplicate(normalized));
        }
        // Byte inequality is not enough: macOS and Windows filesystems fold case, and APFS
        // also folds Unicode normalization, so `README.md` and `readme.md` are one file on
        // disk. Two such artifacts would write over each other, and the survivor would then
        // conflict on every subsequent run -- a directory that never converges and that
        // `--force` cannot repair. Refuse the set instead.
        let folded = fold_artifact_path(&normalized);
        if let Some(existing) = self.folded.get(&folded) {
            return Err(ArtifactError::CollidingPaths {
                path: normalized,
                existing: existing.clone(),
            });
        }
        if folded == fold_artifact_path(RESERVED_MANIFEST_NAME) {
            return Err(ArtifactError::ReservedPath(normalized));
        }
        if content.len() > self.limits.max_file_bytes {
            return Err(ArtifactError::FileTooLarge {
                path: normalized,
                actual: content.len(),
                limit: self.limits.max_file_bytes,
            });
        }
        let next_count = self.files.len().saturating_add(1);
        if next_count > self.limits.max_files {
            return Err(ArtifactError::TooManyFiles {
                actual: next_count,
                limit: self.limits.max_files,
            });
        }
        let next_total = self.total_bytes.saturating_add(content.len());
        if next_total > self.limits.max_total_bytes {
            return Err(ArtifactError::TotalTooLarge {
                actual: next_total,
                limit: self.limits.max_total_bytes,
            });
        }
        self.total_bytes = next_total;
        self.folded.insert(folded, normalized.clone());
        self.files.insert(normalized, content);
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.files.len()
    }

    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }

    pub fn total_bytes(&self) -> usize {
        self.total_bytes
    }

    pub fn iter(&self) -> impl Iterator<Item = Artifact> + '_ {
        self.files.iter().map(|(path, content)| Artifact {
            path: path.clone(),
            content: content.clone(),
        })
    }

    pub fn get(&self, path: &str) -> Option<&[u8]> {
        self.files.get(path).map(Vec::as_slice)
    }
}

pub fn normalize_artifact_path(path: &str) -> Result<String, ArtifactError> {
    if path.is_empty() {
        return Err(ArtifactError::EmptyPath);
    }
    if path.len() > MAX_ARTIFACT_PATH_BYTES {
        return Err(ArtifactError::PathTooLong {
            actual: path.len(),
            limit: MAX_ARTIFACT_PATH_BYTES,
        });
    }
    if path.contains('\0') {
        return Err(ArtifactError::Nul);
    }
    if path.contains('\\') {
        return Err(ArtifactError::InvalidSeparator(path.to_owned()));
    }
    if path.starts_with('/') || has_windows_prefix(path) {
        return Err(ArtifactError::AbsolutePath(path.to_owned()));
    }

    let parsed = Path::new(path);
    let mut segments = Vec::new();
    for component in parsed.components() {
        match component {
            Component::Normal(value) => {
                let value = value
                    .to_str()
                    .ok_or_else(|| ArtifactError::ForbiddenComponent(path.to_owned()))?;
                if value.is_empty() || value == "." || value == ".." {
                    return Err(ArtifactError::ForbiddenComponent(path.to_owned()));
                }
                if is_windows_alias(value) {
                    return Err(ArtifactError::ReservedPath(path.to_owned()));
                }
                segments.push(value);
            }
            _ => return Err(ArtifactError::ForbiddenComponent(path.to_owned())),
        }
    }
    if segments.is_empty()
        || path
            .split('/')
            .any(|part| part.is_empty() || part == "." || part == "..")
    {
        return Err(ArtifactError::ForbiddenComponent(path.to_owned()));
    }
    Ok(segments.join("/"))
}

/// Folds a path the way a case- and normalization-insensitive filesystem would.
///
/// APFS and NTFS both compare case-insensitively, and APFS additionally treats composed and
/// decomposed forms of the same character as one name. Decomposing to NFD before lowercasing
/// makes `café` and `cafe\u{301}` fold together, which byte comparison cannot see.
///
/// This is a conservative approximation, not the exact algorithm of any one filesystem: it
/// may report a collision where a given filesystem would allow both names. Refusing an
/// unusual pair of paths is a much better outcome than an output directory that can never
/// converge, which is what a missed collision produces.
fn fold_artifact_path(path: &str) -> String {
    path.nfd().flat_map(char::to_lowercase).collect()
}

/// Names Windows reserves regardless of extension: `NUL.txt` is still the null device.
///
/// `COM` and `LPT` are reserved for the superscript digits as well as the ASCII ones --
/// Windows treats `COM\u{b9}`, `COM\u{b2}` and `COM\u{b3}` as `COM1`-`COM3`. Per
/// <https://learn.microsoft.com/en-us/windows/win32/fileio/naming-a-file#naming-conventions>.
const WINDOWS_DEVICE_NAMES: &[&str] = &[
    "con",
    "prn",
    "aux",
    "nul",
    "conin$",
    "conout$",
    "com1",
    "com2",
    "com3",
    "com4",
    "com5",
    "com6",
    "com7",
    "com8",
    "com9",
    "com\u{b9}",
    "com\u{b2}",
    "com\u{b3}",
    "lpt1",
    "lpt2",
    "lpt3",
    "lpt4",
    "lpt5",
    "lpt6",
    "lpt7",
    "lpt8",
    "lpt9",
    "lpt\u{b9}",
    "lpt\u{b2}",
    "lpt\u{b3}",
];

/// Characters Windows forbids in a filename.
///
/// `:` is the sharpest of these: on NTFS it introduces an alternate data stream, so
/// `report.txt:hidden` and `manifest.json::$DATA` both address a *different* stream of an
/// existing file. The latter aliases a file's default stream, which is how a generator could
/// otherwise reach the reserved manifest past a name comparison.
const WINDOWS_FORBIDDEN_CHARACTERS: &[char] = &['<', '>', ':', '"', '|', '?', '*'];

/// Whether a path component would alias another name, or be rejected outright, on Windows.
///
/// Trailing dots and spaces are silently stripped when creating a file, so `report.` and
/// `report` are the same file; device names are intercepted before they reach the filesystem
/// at all; and the reserved characters above either name a different stream or are refused.
/// All are rejected on every platform so a generator's output set does not depend on where it
/// runs, and so validation cannot be sidestepped by generating on one platform for another.
///
/// Rules per Microsoft's file naming documentation:
/// <https://learn.microsoft.com/en-us/windows/win32/fileio/naming-a-file>
fn is_windows_alias(segment: &str) -> bool {
    if segment.ends_with('.') || segment.ends_with(' ') {
        return true;
    }
    if segment.contains(WINDOWS_FORBIDDEN_CHARACTERS) {
        return true;
    }
    // Control characters are invalid in a filename and are invisible in any report of what
    // was written, so a path containing one cannot be reviewed.
    if segment.chars().any(|character| character.is_control()) {
        return true;
    }
    let stem = segment.split('.').next().unwrap_or(segment);
    WINDOWS_DEVICE_NAMES
        .iter()
        .any(|device| stem.eq_ignore_ascii_case(device))
}

fn has_windows_prefix(path: &str) -> bool {
    let bytes = path.as_bytes();
    bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_escape_and_platform_paths() {
        for path in [
            "../escape",
            "a/../../escape",
            "/absolute",
            "C:/drive",
            "a\\b",
            "a//b",
        ] {
            assert!(normalize_artifact_path(path).is_err(), "accepted {path}");
        }
    }

    #[test]
    fn rejects_overlong_paths() {
        let path = "a".repeat(MAX_ARTIFACT_PATH_BYTES + 1);
        assert_eq!(
            normalize_artifact_path(&path),
            Err(ArtifactError::PathTooLong {
                actual: MAX_ARTIFACT_PATH_BYTES + 1,
                limit: MAX_ARTIFACT_PATH_BYTES,
            })
        );
    }

    #[test]
    fn stages_binary_artifacts_in_path_order() {
        let mut set = ArtifactSet::new(ArtifactLimits::default());
        set.emit("z.bin", vec![0, 255]).unwrap();
        set.emit("a/file.txt", b"hello".to_vec()).unwrap();
        let paths = set.iter().map(|file| file.path).collect::<Vec<_>>();
        assert_eq!(paths, ["a/file.txt", "z.bin"]);
        assert_eq!(set.total_bytes(), 7);
    }

    #[test]
    fn rejects_paths_that_collide_on_a_case_folding_filesystem() {
        let mut set = ArtifactSet::new(ArtifactLimits::default());
        set.emit("README.md", b"a".to_vec()).unwrap();
        assert!(matches!(
            set.emit("readme.md", b"b".to_vec()),
            Err(ArtifactError::CollidingPaths { .. })
        ));
        // Same file, composed versus decomposed.
        set.emit("caf\u{e9}.txt", b"a".to_vec()).unwrap();
        assert!(matches!(
            set.emit("cafe\u{301}.txt", b"b".to_vec()),
            Err(ArtifactError::CollidingPaths { .. })
        ));
    }

    #[test]
    fn rejects_the_reserved_manifest_path() {
        let mut set = ArtifactSet::new(ArtifactLimits::default());
        assert!(matches!(
            set.emit(RESERVED_MANIFEST_NAME, b"{}".to_vec()),
            Err(ArtifactError::ReservedPath(_))
        ));
    }

    #[test]
    fn distinct_paths_that_only_look_similar_are_still_allowed() {
        let mut set = ArtifactSet::new(ArtifactLimits::default());
        set.emit("a/report.txt", b"a".to_vec()).unwrap();
        set.emit("b/report.txt", b"b".to_vec()).unwrap();
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn rejects_case_variants_of_the_reserved_manifest() {
        for variant in [
            ".SPEC42-GENERATOR-MANIFEST.JSON",
            ".Spec42-Generator-Manifest.json",
        ] {
            let mut set = ArtifactSet::new(ArtifactLimits::default());
            assert!(
                matches!(
                    set.emit(variant, b"{}".to_vec()),
                    Err(ArtifactError::ReservedPath(_))
                ),
                "accepted `{variant}`, which aliases the manifest on a case-folding filesystem"
            );
        }
    }

    #[test]
    fn rejects_windows_aliasing_names() {
        for path in [
            "report.",
            "report ",
            "nested/report.",
            "NUL",
            "nul.txt",
            "com1",
            "dir/AUX.log",
            // Windows reads superscripts as digits in COM# and LPT# device names.
            "COM\u{b9}.txt",
            "LPT\u{b2}.log",
            "com\u{b3}",
            "CONIN$",
            // NTFS alternate data streams. The second addresses the default stream of the
            // reserved manifest, so a name comparison alone would not catch it.
            "report.txt:hidden",
            ".spec42-generator-manifest.json::$DATA",
            // Remaining reserved characters and a control character.
            "a<b",
            "a>b",
            "a\"b",
            "a|b",
            "a?b",
            "a*b",
            "bell\u{7}",
        ] {
            let mut set = ArtifactSet::new(ArtifactLimits::default());
            assert!(
                set.emit(path, b"x".to_vec()).is_err(),
                "accepted `{path}`, which aliases another name on Windows"
            );
        }
    }

    #[test]
    fn rejects_duplicates_and_limits() {
        let mut set = ArtifactSet::new(ArtifactLimits {
            max_files: 1,
            max_file_bytes: 2,
            max_total_bytes: 2,
        });
        set.emit("a", vec![1, 2]).unwrap();
        assert!(matches!(
            set.emit("a", vec![]),
            Err(ArtifactError::Duplicate(_))
        ));
        assert!(matches!(
            set.emit("b", vec![]),
            Err(ArtifactError::TooManyFiles { .. })
        ));
    }
}
