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

/// Longest single path component.
///
/// Windows filesystems cap each component at 255 characters independently of the total path
/// length, so a 4 KiB single segment is within the path budget and still impossible to
/// create. Applied on every platform, conservatively in bytes rather than characters, so a
/// generator's output set does not depend on where it ran.
///
/// <https://learn.microsoft.com/en-us/windows/win32/fileio/filesystem-functionality-comparison#limits>
pub const MAX_ARTIFACT_SEGMENT_BYTES: usize = 255;

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
    #[error(
        "artifact path component `{segment}` is {actual} bytes; the component limit is {limit}"
    )]
    SegmentTooLong {
        segment: String,
        actual: usize,
        limit: usize,
    },
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
            if segment.len() > MAX_ARTIFACT_SEGMENT_BYTES {
                return Err(ArtifactPathError::SegmentTooLong {
                    segment: (*segment).to_owned(),
                    actual: segment.len(),
                    limit: MAX_ARTIFACT_SEGMENT_BYTES,
                });
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

    pub fn segments(&self) -> impl DoubleEndedIterator<Item = &str> {
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
        assert_eq!(ArtifactPath::parse(""), Err(ArtifactPathError::Empty));

        // Component boundary: 255 accepted, 256 refused. Independent of the path budget --
        // a single 4 KiB segment fits the path limit and is still impossible to create.
        let longest_segment = "a".repeat(MAX_ARTIFACT_SEGMENT_BYTES);
        assert!(
            accepted(&longest_segment),
            "rejected a component at the limit"
        );
        assert!(
            accepted(&format!("{longest_segment}/{longest_segment}")),
            "rejected two components that are each within the limit"
        );
        assert_eq!(
            ArtifactPath::parse(&"a".repeat(MAX_ARTIFACT_SEGMENT_BYTES + 1)),
            Err(ArtifactPathError::SegmentTooLong {
                segment: "a".repeat(MAX_ARTIFACT_SEGMENT_BYTES + 1),
                actual: MAX_ARTIFACT_SEGMENT_BYTES + 1,
                limit: MAX_ARTIFACT_SEGMENT_BYTES,
            })
        );

        // Total-path boundary, reached with components that are each legal.
        let component = "a".repeat(MAX_ARTIFACT_SEGMENT_BYTES);
        let mut within = Vec::new();
        while within.join("/").len() + 1 + component.len() <= MAX_ARTIFACT_PATH_BYTES {
            within.push(component.clone());
        }
        assert!(
            accepted(&within.join("/")),
            "rejected a path within the total limit"
        );
        within.push(component);
        assert!(
            matches!(
                ArtifactPath::parse(&within.join("/")),
                Err(ArtifactPathError::TooLong { .. })
            ),
            "accepted a path over the total limit"
        );
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

    /// Windows is its own oracle: what the validator accepts must be creatable, and what it
    /// rejects must be something Windows would not have stored faithfully anyway.
    ///
    /// The stored name is read back by *enumerating the directory*, not from the path that was
    /// requested. Asking `target.file_name()` would only echo the input and prove nothing --
    /// the whole risk is that Windows silently stores something other than what was asked for.
    #[cfg(windows)]
    #[test]
    fn accepted_paths_are_stored_under_exactly_the_requested_name() {
        let temp = tempfile::tempdir().unwrap();

        // Chosen to sit next to the rules rather than comfortably inside them: names that
        // resemble devices without being them, boundary lengths, and non-ASCII.
        let representatives = [
            "README.md",
            "nested/dir/file.txt",
            "com10",
            "comx",
            "com",
            "lpt0",
            "nulls.txt",
            "auxiliary",
            "a.b.c",
            "caf\u{e9}.txt",
            "\u{301}",
            &"a".repeat(MAX_ARTIFACT_SEGMENT_BYTES),
        ];

        for representative in representatives {
            let parsed = ArtifactPath::parse(representative)
                .unwrap_or_else(|error| panic!("{representative}: {error}"));
            let directory = temp.path().join("case").join(
                parsed
                    .segments()
                    .take(parsed.segments().count() - 1)
                    .collect::<Vec<_>>()
                    .join("/"),
            );
            std::fs::create_dir_all(&directory).unwrap();
            let expected = parsed.segments().next_back().unwrap();
            let target = directory.join(expected);
            std::fs::write(&target, b"x")
                .unwrap_or_else(|error| panic!("`{representative}` was not writable: {error}"));

            // Enumerate: this is what Windows actually stored, not what we asked for.
            let stored: Vec<String> = std::fs::read_dir(&directory)
                .unwrap()
                .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
                .collect();
            assert!(
                stored.iter().any(|name| name == expected),
                "`{representative}` was stored as {stored:?}, not `{expected}`"
            );
            let metadata = std::fs::symlink_metadata(&target).unwrap();
            assert!(
                metadata.is_file(),
                "`{representative}` is not a regular file"
            );
            std::fs::remove_file(&target).unwrap();
        }
    }

    /// Reserved DOS device names the validator rejects for portability, not because current
    /// Windows necessarily refuses them itself.
    ///
    /// Verified directly against a real filesystem (`CreateFileW` via `std::fs::write`, no
    /// test harness or `tempfile` crate involved, so this isn't a harness artifact): only a
    /// bare, extension-less `NUL` still redirects to the null device today. `CON`, `COM1`, and
    /// any reserved name *with* an extension -- including `nul.txt`, `COM¹.txt`, `LPT².log`,
    /// and even the console-specific `CONIN$`/`CONOUT$` -- write as ordinary files with their
    /// literal requested name. The rejection is still correct: other tools, older Windows
    /// versions, network shares, and archive formats can still choke on these names, and an
    /// output set should not depend on where it ran. But that policy isn't something today's
    /// Windows will independently confirm for us, so these are checked against the validator
    /// only, not against a live filesystem oracle.
    const RESERVED_DEVICE_NAMES_ONLY_THE_VALIDATOR_REJECTS: [&str; 5] = [
        "nul.txt",
        "COM1",
        "COM\u{b9}.txt",
        "LPT\u{b2}.log",
        "CONIN$",
    ];

    #[test]
    fn reserved_device_names_are_rejected_by_the_validator() {
        for candidate in RESERVED_DEVICE_NAMES_ONLY_THE_VALIDATOR_REJECTS {
            assert!(
                ArtifactPath::parse(candidate).is_err(),
                "validator accepted `{candidate}`"
            );
        }
    }

    /// The other direction: a rejected name must be one Windows would not have stored as
    /// asked. Independently written rather than derived from the validator's own rules, so a
    /// mistake in those rules cannot make this pass vacuously.
    ///
    /// Limited to candidates verified to still be genuinely refused or renamed by Windows
    /// today -- illegal characters, alternate-data-stream colons, trailing dot/space
    /// stripping, and the segment length ceiling. See
    /// `RESERVED_DEVICE_NAMES_ONLY_THE_VALIDATOR_REJECTS` above for the reserved-name
    /// candidates this deliberately excludes.
    #[cfg(windows)]
    #[test]
    fn rejected_paths_would_not_have_been_stored_faithfully() {
        let temp = tempfile::tempdir().unwrap();
        let dangerous = [
            "NUL",
            "report.",
            "report ",
            "a:b",
            ".spec42-generator-manifest.json::$DATA",
            "a<b",
            "a>b",
            "a|b",
            "a?b",
            "a*b",
            "bell\u{7}",
            &"a".repeat(MAX_ARTIFACT_SEGMENT_BYTES + 1),
        ];

        for candidate in dangerous {
            assert!(
                ArtifactPath::parse(candidate).is_err(),
                "validator accepted `{candidate}`"
            );

            // And confirm Windows agrees it is not an ordinary name: either the write fails,
            // or what lands on disk is not what was asked for.
            let target = temp.path().join(candidate);
            let before: std::collections::BTreeSet<String> = std::fs::read_dir(temp.path())
                .map(|entries| {
                    entries
                        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
                        .collect()
                })
                .unwrap_or_default();
            if std::fs::write(&target, b"x").is_ok() {
                let after: std::collections::BTreeSet<String> = std::fs::read_dir(temp.path())
                    .unwrap()
                    .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
                    .collect();
                let appeared: Vec<&String> = after.difference(&before).collect();
                assert!(
                    !appeared.iter().any(|name| name.as_str() == candidate),
                    "`{candidate}` was stored verbatim, so rejecting it may be too strict"
                );
                let _ = std::fs::remove_file(&target);
            }
        }
    }
}
