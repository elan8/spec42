//! Validated artifact paths.
//!
//! [`ArtifactPath`] can only be built through [`ArtifactPath::parse`], which is the single
//! place that parses, validates and normalizes. Everything downstream — the artifact set, the
//! transaction planner, the filesystem executor — takes an `ArtifactPath` and can assume it
//! is already safe, rather than each re-deriving what "safe" means.
//!
//! That matters because the rejections here are not stylistic. A generator that returns
//! `../escape`, `.spec42-generator-manifest.json::$DATA` or `COM¹.txt` is trying, deliberately
//! or not, to write somewhere the ownership model does not cover.

use serde::{Deserialize, Serialize};
use thiserror::Error;
use unicode_normalization::UnicodeNormalization;

pub const MAX_ARTIFACT_PATH_BYTES: usize = 4 * 1024;

/// The host writes its ownership manifest here; a generator must not also claim it.
pub const RESERVED_MANIFEST_NAME: &str = ".spec42-generator-manifest.json";

/// Characters Windows forbids in a filename.
///
/// `:` is the sharpest: on NTFS it opens an alternate data stream, so `report.txt:hidden` and
/// `manifest.json::$DATA` address a *different* stream of an existing file.
const WINDOWS_FORBIDDEN: &[char] = &['<', '>', ':', '"', '|', '?', '*'];

/// Digits Windows accepts in `COM#` and `LPT#` device names, including the superscript forms.
const DEVICE_DIGITS: &[char] = &['1', '2', '3', '4', '5', '6', '7', '8', '9', '¹', '²', '³'];

/// Single-name devices, reserved in every directory regardless of extension.
const DEVICE_NAMES: &[&str] = &["con", "prn", "aux", "nul", "conin$", "conout$"];

/// Device families taking a trailing digit.
const DEVICE_FAMILIES: &[&str] = &["com", "lpt"];

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ArtifactPathError {
    #[error("artifact path must not be empty")]
    Empty,
    #[error("artifact path `{0}` is absolute or has a platform prefix")]
    Absolute(String),
    #[error("artifact path `{0}` contains a forbidden component")]
    ForbiddenComponent(String),
    #[error("artifact path `{0}` must use `/` as its separator")]
    InvalidSeparator(String),
    #[error("artifact path contains NUL")]
    Nul,
    #[error("artifact path is {actual} bytes; the path limit is {limit}")]
    TooLong { actual: usize, limit: usize },
    #[error("artifact path `{0}` is reserved by Spec42 or by the filesystem")]
    Reserved(String),
}

/// A relative, `/`-separated path that is safe to join beneath the output root.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct ArtifactPath(String);

impl ArtifactPath {
    /// The only constructor. Everything else in the crate takes the result.
    pub fn parse(raw: &str) -> Result<Self, ArtifactPathError> {
        if raw.is_empty() {
            return Err(ArtifactPathError::Empty);
        }
        if raw.len() > MAX_ARTIFACT_PATH_BYTES {
            return Err(ArtifactPathError::TooLong {
                actual: raw.len(),
                limit: MAX_ARTIFACT_PATH_BYTES,
            });
        }
        if raw.contains('\0') {
            return Err(ArtifactPathError::Nul);
        }
        if raw.contains('\\') {
            return Err(ArtifactPathError::InvalidSeparator(raw.to_owned()));
        }
        if raw.starts_with('/') || has_drive_prefix(raw) {
            return Err(ArtifactPathError::Absolute(raw.to_owned()));
        }

        // Split on `/` only. Deliberately not `Path::components`, which is
        // platform-dependent: on Windows it would also split on `\`, so a path accepted on
        // Linux could mean something different on Windows.
        let segments: Vec<&str> = raw.split('/').collect();
        for segment in &segments {
            if segment.is_empty() || *segment == "." || *segment == ".." {
                return Err(ArtifactPathError::ForbiddenComponent(raw.to_owned()));
            }
            if is_reserved_segment(segment) {
                return Err(ArtifactPathError::Reserved(raw.to_owned()));
            }
        }

        let normalized = segments.join("/");
        if fold(&normalized) == fold(RESERVED_MANIFEST_NAME) {
            return Err(ArtifactPathError::Reserved(raw.to_owned()));
        }
        Ok(Self(normalized))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn segments(&self) -> impl Iterator<Item = &str> {
        self.0.split('/')
    }

    /// The path as a case- and normalization-insensitive filesystem would see it.
    ///
    /// Two artifacts whose folded forms are equal would be one file on disk: the second write
    /// would win and every later run would conflict with no way to converge.
    pub fn folded(&self) -> String {
        fold(&self.0)
    }
}

impl std::fmt::Display for ArtifactPath {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl TryFrom<String> for ArtifactPath {
    type Error = ArtifactPathError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(&value)
    }
}

impl From<ArtifactPath> for String {
    fn from(value: ArtifactPath) -> Self {
        value.0
    }
}

fn has_drive_prefix(path: &str) -> bool {
    let bytes = path.as_bytes();
    bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':'
}

/// Whether a segment names something the filesystem would treat as other than a plain file.
///
/// Expressed as rules rather than an enumeration of spellings, so `COM` + any accepted digit
/// is covered without a list to keep complete. Applied on every platform: an output set must
/// not depend on where the generator ran.
///
/// Rules per <https://learn.microsoft.com/en-us/windows/win32/fileio/naming-a-file>.
fn is_reserved_segment(segment: &str) -> bool {
    if segment.contains(WINDOWS_FORBIDDEN) {
        return true;
    }
    if segment.chars().any(char::is_control) {
        return true;
    }
    // Windows strips these when creating a file, so `report.` and `report` are one name.
    if segment.ends_with('.') || segment.ends_with(' ') {
        return true;
    }
    is_device_name(segment.split('.').next().unwrap_or(segment))
}

/// Whether `stem` names a DOS device.
fn is_device_name(stem: &str) -> bool {
    let lower = stem.to_lowercase();
    if DEVICE_NAMES.contains(&lower.as_str()) {
        return true;
    }
    DEVICE_FAMILIES.iter().any(|family| {
        lower
            .strip_prefix(family)
            .and_then(|rest| {
                let mut characters = rest.chars();
                let digit = characters.next()?;
                characters.next().is_none().then_some(digit)
            })
            .is_some_and(|digit| DEVICE_DIGITS.contains(&digit))
    })
}

/// Folds a path the way a case- and normalization-insensitive filesystem would.
///
/// Decomposing to NFD before lowercasing makes `café` and `cafe\u{301}` fold together, which
/// byte comparison cannot see. Conservative by design: it may report a collision a given
/// filesystem would allow, which is a far better failure than a missed one, because a missed
/// collision produces an output directory that can never converge.
fn fold(path: &str) -> String {
    path.nfd().flat_map(char::to_lowercase).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn accepted(raw: &str) -> bool {
        ArtifactPath::parse(raw).is_ok()
    }

    /// Every ASCII byte, in each structural position of a segment.
    ///
    /// Finite and exhaustive rather than randomized: 128 codepoints in four positions is 512
    /// cases, so there is no reason to sample.
    #[test]
    fn every_ascii_byte_is_classified_in_every_position() {
        for byte in 0u8..=127 {
            let character = byte as char;
            let structural = matches!(character, '/' | '\\' | '\0');
            let reserved = WINDOWS_FORBIDDEN.contains(&character) || character.is_control();

            // Leading, middle and trailing within a single segment, plus inside a nested one.
            for (label, path) in [
                ("leading", format!("{character}ok.txt")),
                ("middle", format!("ok{character}file.txt")),
                ("trailing", format!("okfile{character}")),
                ("nested", format!("dir/{character}file")),
            ] {
                let ok = accepted(&path);
                if reserved {
                    assert!(!ok, "accepted reserved {byte:#04x} in {label} position");
                } else if !structural {
                    // Trailing `.` and ` ` alias another name; everything else is fine.
                    let aliases = label == "trailing" && matches!(character, '.' | ' ');
                    assert_eq!(
                        ok, !aliases,
                        "byte {byte:#04x} in {label} position classified unexpectedly"
                    );
                }
            }
        }
    }

    /// Every device family, digit, case and extension.
    #[test]
    fn every_device_name_variant_is_reserved() {
        for family in DEVICE_FAMILIES {
            for digit in DEVICE_DIGITS {
                let base = format!("{family}{digit}");
                for spelling in [base.clone(), base.to_uppercase()] {
                    for candidate in [
                        spelling.clone(),
                        format!("{spelling}.txt"),
                        format!("{spelling}.tar.gz"),
                        format!("nested/dir/{spelling}"),
                    ] {
                        assert!(!accepted(&candidate), "accepted device name `{candidate}`");
                    }
                }
            }
            // The family without a digit is an ordinary name.
            assert!(
                accepted(family),
                "rejected `{family}`, which is not a device"
            );
            // And with two digits, or a non-digit.
            assert!(accepted(&format!("{family}10")));
            assert!(accepted(&format!("{family}x")));
        }
        for name in DEVICE_NAMES {
            for candidate in [
                name.to_string(),
                name.to_uppercase(),
                format!("{name}.log"),
                format!("a/b/{name}"),
            ] {
                assert!(!accepted(&candidate), "accepted device name `{candidate}`");
            }
        }
    }

    #[test]
    fn length_boundaries_are_exact() {
        let longest = "a".repeat(MAX_ARTIFACT_PATH_BYTES);
        assert!(accepted(&longest), "rejected a path at exactly the limit");
        assert_eq!(
            ArtifactPath::parse(&"a".repeat(MAX_ARTIFACT_PATH_BYTES + 1)),
            Err(ArtifactPathError::TooLong {
                actual: MAX_ARTIFACT_PATH_BYTES + 1,
                limit: MAX_ARTIFACT_PATH_BYTES,
            })
        );
        assert_eq!(ArtifactPath::parse(""), Err(ArtifactPathError::Empty));
    }

    /// Traversal and separator forms, in every component position.
    #[test]
    fn every_traversal_and_separator_form_is_rejected() {
        for form in [
            "..",
            ".",
            "../a",
            "a/..",
            "a/../b",
            "./a",
            "a/./b",
            "a//b",
            "/a",
            "a/",
            "//a",
            "a\\b",
            "\\a",
            "a\\",
            "C:/x",
            "c:x",
            "Z:/",
            "a/../../b",
        ] {
            assert!(
                !accepted(form),
                "accepted traversal or separator form `{form}`"
            );
        }
    }

    /// Canonical composed/decomposed pairs must fold together.
    #[test]
    fn unicode_collision_pairs_fold_to_the_same_key() {
        for (composed, decomposed) in [
            ("café.txt", "cafe\u{301}.txt"),
            ("Ångström", "A\u{30a}ngstro\u{308}m"),
            ("ñ", "n\u{303}"),
        ] {
            let left = ArtifactPath::parse(composed).expect(composed);
            let right = ArtifactPath::parse(decomposed).expect(decomposed);
            assert_eq!(
                left.folded(),
                right.folded(),
                "`{composed}` and `{decomposed}` should collide"
            );
        }
        // Case variants fold together too.
        assert_eq!(
            ArtifactPath::parse("README.md").unwrap().folded(),
            ArtifactPath::parse("readme.md").unwrap().folded()
        );
        // Genuinely different names must not.
        assert_ne!(
            ArtifactPath::parse("a/report.txt").unwrap().folded(),
            ArtifactPath::parse("b/report.txt").unwrap().folded()
        );
    }

    #[test]
    fn the_reserved_manifest_is_rejected_in_every_spelling() {
        for spelling in [
            RESERVED_MANIFEST_NAME,
            ".SPEC42-GENERATOR-MANIFEST.JSON",
            ".Spec42-Generator-Manifest.json",
        ] {
            assert!(
                !accepted(spelling),
                "accepted manifest spelling `{spelling}`"
            );
        }
        // Only at the root: the host writes its manifest there and nowhere else.
        assert!(accepted(&format!("nested/{RESERVED_MANIFEST_NAME}")));
    }

    #[test]
    fn ordinary_paths_survive_and_normalize_predictably() {
        for path in ["README.md", "a/b/c.txt", "dir/file.tar.gz", "_x", "a.b.c"] {
            let parsed =
                ArtifactPath::parse(path).unwrap_or_else(|error| panic!("{path}: {error}"));
            assert_eq!(parsed.as_str(), path, "parsing changed `{path}`");
        }
        assert_eq!(
            ArtifactPath::parse("a/b/c.txt")
                .unwrap()
                .segments()
                .collect::<Vec<_>>(),
            ["a", "b", "c.txt"]
        );
    }

    /// Windows is its own oracle: a path this validator accepts must produce a regular
    /// directory entry with exactly that name. Catches device and stream behaviour without
    /// restating the policy in a second table.
    #[cfg(windows)]
    #[test]
    fn accepted_paths_create_regular_files_on_windows() {
        let temp = tempfile::tempdir().unwrap();
        for representative in [
            "README.md",
            "nested/dir/file.txt",
            "com10",
            "comx",
            "a.b.c",
            "caf\u{e9}.txt",
        ] {
            let parsed = ArtifactPath::parse(representative)
                .unwrap_or_else(|error| panic!("{representative}: {error}"));
            let target = temp.path().join(parsed.as_str());
            std::fs::create_dir_all(target.parent().unwrap()).unwrap();
            std::fs::write(&target, b"x")
                .unwrap_or_else(|error| panic!("{representative} was not writable: {error}"));

            let metadata = std::fs::symlink_metadata(&target)
                .unwrap_or_else(|error| panic!("{representative} did not appear: {error}"));
            assert!(
                metadata.is_file(),
                "`{representative}` did not become a regular file"
            );
            let name = target.file_name().unwrap().to_string_lossy();
            let expected = parsed.segments().next_back().unwrap();
            assert_eq!(
                name, expected,
                "`{representative}` landed under a different name"
            );
        }
    }
}
