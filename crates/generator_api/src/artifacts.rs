use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::path::{ArtifactPath, ArtifactPathError};

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
    pub path: ArtifactPath,
    pub content: Vec<u8>,
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ArtifactError {
    #[error(transparent)]
    Path(#[from] ArtifactPathError),
    #[error("artifact `{0}` was returned more than once")]
    Duplicate(String),
    #[error(
        "artifact `{path}` collides with `{existing}` on case- or normalization-insensitive \
         filesystems; they would be the same file on disk"
    )]
    CollidingPaths { path: String, existing: String },
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
    files: BTreeMap<ArtifactPath, Vec<u8>>,
    /// Folded paths, to catch collisions the `files` key cannot see.
    folded: BTreeMap<String, ArtifactPath>,
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

    /// Adds an artifact, enforcing the set-level policy.
    ///
    /// Path *validity* is not checked here: [`ArtifactPath::parse`] is the only way to obtain
    /// the argument, so by this point the path is already known to be relative, separator-safe
    /// and free of reserved names. What remains is policy that depends on the rest of the set:
    /// duplicates, collisions, and the size limits.
    pub fn emit(&mut self, path: &str, content: Vec<u8>) -> Result<(), ArtifactError> {
        self.insert(ArtifactPath::parse(path)?, content)
    }

    pub fn insert(&mut self, path: ArtifactPath, content: Vec<u8>) -> Result<(), ArtifactError> {
        if self.files.contains_key(&path) {
            return Err(ArtifactError::Duplicate(path.to_string()));
        }
        let folded = path.folded();
        if let Some(existing) = self.folded.get(&folded) {
            return Err(ArtifactError::CollidingPaths {
                path: path.to_string(),
                existing: existing.to_string(),
            });
        }
        if content.len() > self.limits.max_file_bytes {
            return Err(ArtifactError::FileTooLarge {
                path: path.to_string(),
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
        self.folded.insert(folded, path.clone());
        self.files.insert(path, content);
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

    /// Borrowing iteration. Preferred over [`Self::iter`]: cloning every artifact's contents
    /// costs a full copy of the generated output, and both the planner and the executor walk
    /// the set.
    pub fn entries(&self) -> impl Iterator<Item = (&ArtifactPath, &[u8])> + '_ {
        self.files
            .iter()
            .map(|(path, content)| (path, content.as_slice()))
    }

    pub fn iter(&self) -> impl Iterator<Item = Artifact> + '_ {
        self.files.iter().map(|(path, content)| Artifact {
            path: path.clone(),
            content: content.clone(),
        })
    }

    pub fn get(&self, path: &ArtifactPath) -> Option<&[u8]> {
        self.files.get(path).map(Vec::as_slice)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Path validity is [`ArtifactPath`]'s job and is tested there. These cover what only the
    /// set can know: how one artifact relates to the others already in it.
    #[test]
    fn stages_binary_artifacts_in_path_order() {
        let mut set = ArtifactSet::new(ArtifactLimits::default());
        set.emit("z.bin", vec![0, 255]).unwrap();
        set.emit("a/file.txt", b"hello".to_vec()).unwrap();
        let paths = set
            .entries()
            .map(|(path, _)| path.to_string())
            .collect::<Vec<_>>();
        assert_eq!(paths, ["a/file.txt", "z.bin"]);
        assert_eq!(set.total_bytes(), 7);
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

    #[test]
    fn rejects_paths_that_collide_on_a_case_folding_filesystem() {
        let mut set = ArtifactSet::new(ArtifactLimits::default());
        set.emit("README.md", b"a".to_vec()).unwrap();
        assert!(matches!(
            set.emit("readme.md", b"b".to_vec()),
            Err(ArtifactError::CollidingPaths { .. })
        ));
        set.emit("caf\u{e9}.txt", b"a".to_vec()).unwrap();
        assert!(matches!(
            set.emit("cafe\u{301}.txt", b"b".to_vec()),
            Err(ArtifactError::CollidingPaths { .. })
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
    fn an_invalid_path_is_reported_as_a_path_error() {
        let mut set = ArtifactSet::new(ArtifactLimits::default());
        assert!(matches!(
            set.emit("../escape", b"x".to_vec()),
            Err(ArtifactError::Path(_))
        ));
    }
}
