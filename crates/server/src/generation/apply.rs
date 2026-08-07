//! The filesystem executor: applies a plan transactionally.
//!
//! Every mutation goes through the [`FileSystem`] trait, so tests can fail operation 1, then
//! operation 2, and so on through every point at which the transaction touches disk. That is
//! far more reliable than trying to induce real I/O failures with permissions or timing, and
//! it is the only practical way to reach the rollback paths at all.
//!
//! Planning is [`super::plan`]; nothing here decides policy.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Every filesystem mutation the transaction performs.
///
/// Reads are deliberately excluded: the planner works from an [`Observation`] gathered before
/// any mutation, so a failure part-way through cannot be caused by a read.
///
/// [`Observation`]: super::plan::Observation
pub trait FileSystem {
    fn create_dir_all(&self, path: &Path) -> io::Result<()>;
    fn write(&self, path: &Path, contents: &[u8]) -> io::Result<()>;
    fn rename(&self, from: &Path, to: &Path) -> io::Result<()>;
    fn remove_dir_all(&self, path: &Path) -> io::Result<()>;
    fn copy_tree(&self, source: &Path, destination: &Path) -> io::Result<()>;
}

/// The real filesystem.
pub struct RealFileSystem;

impl FileSystem for RealFileSystem {
    fn create_dir_all(&self, path: &Path) -> io::Result<()> {
        fs::create_dir_all(path)
    }

    fn write(&self, path: &Path, contents: &[u8]) -> io::Result<()> {
        fs::write(path, contents)
    }

    fn rename(&self, from: &Path, to: &Path) -> io::Result<()> {
        fs::rename(from, to)
    }

    fn remove_dir_all(&self, path: &Path) -> io::Result<()> {
        fs::remove_dir_all(path)
    }

    fn copy_tree(&self, source: &Path, destination: &Path) -> io::Result<()> {
        super::copy_tree(source, destination).map_err(io::Error::other)
    }
}

/// Where the previous output went when a swap failed and rollback could not restore it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoverableBackup(pub PathBuf);

#[derive(Debug)]
pub enum ApplyError {
    /// Failed before anything was displaced; the output tree is untouched.
    BeforeDisplacement(String),
    /// The swap failed and the previous tree was restored.
    RolledBack(String),
    /// The swap failed and rollback failed too. The data is at this path.
    Unrecovered {
        message: String,
        backup: RecoverableBackup,
    },
}

impl std::fmt::Display for ApplyError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BeforeDisplacement(message) => write!(formatter, "{message}"),
            Self::RolledBack(message) => {
                write!(formatter, "{message}; previous output restored")
            }
            Self::Unrecovered { message, backup } => write!(
                formatter,
                "{message}, and rollback failed; previous output remains at {}",
                backup.0.display()
            ),
        }
    }
}

/// Swaps `staged` into `output`, keeping the displaced tree until the swap succeeds.
///
/// The backup is deliberately not a `TempDir`: if rollback fails, the caller is told where
/// the previous output is, and that path has to outlive this function.
pub fn install(
    filesystem: &dyn FileSystem,
    output: &Path,
    staged: &Path,
    backup_root: &Path,
    output_exists: bool,
) -> Result<(), ApplyError> {
    if !output_exists {
        return filesystem.rename(staged, output).map_err(|error| {
            let _ = filesystem.remove_dir_all(staged);
            ApplyError::BeforeDisplacement(format!("failed to install generated output: {error}"))
        });
    }

    if let Err(error) = filesystem.create_dir_all(backup_root) {
        let _ = filesystem.remove_dir_all(staged);
        return Err(ApplyError::BeforeDisplacement(format!(
            "failed to create transactional backup: {error}"
        )));
    }
    let previous = backup_root.join("previous");

    if let Err(error) = filesystem.rename(output, &previous) {
        // Nothing was displaced, so the backup holds nothing worth keeping.
        let _ = filesystem.remove_dir_all(backup_root);
        let _ = filesystem.remove_dir_all(staged);
        return Err(ApplyError::BeforeDisplacement(format!(
            "failed to move existing output into transaction backup: {error}"
        )));
    }

    if let Err(error) = filesystem.rename(staged, output) {
        let message = format!("failed to install generated output: {error}");
        return Err(match filesystem.rename(&previous, output) {
            Ok(()) => {
                let _ = filesystem.remove_dir_all(backup_root);
                let _ = filesystem.remove_dir_all(staged);
                ApplyError::RolledBack(message)
            }
            Err(_) => ApplyError::Unrecovered {
                message,
                backup: RecoverableBackup(previous),
            },
        });
    }

    let _ = filesystem.remove_dir_all(backup_root);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::BTreeSet;

    /// Records every mutation and fails the nth one.
    struct Failing {
        inner: RealFileSystem,
        fail_at: usize,
        seen: RefCell<usize>,
        log: RefCell<Vec<String>>,
    }

    impl Failing {
        fn new(fail_at: usize) -> Self {
            Self {
                inner: RealFileSystem,
                fail_at,
                seen: RefCell::new(0),
                log: RefCell::new(Vec::new()),
            }
        }

        /// `true` when this call is the one to fail.
        fn should_fail(&self, what: &str) -> bool {
            let mut seen = self.seen.borrow_mut();
            *seen += 1;
            self.log.borrow_mut().push(format!("{}: {what}", *seen));
            *seen == self.fail_at
        }

        fn mutations(&self) -> usize {
            *self.seen.borrow()
        }
    }

    impl FileSystem for Failing {
        fn create_dir_all(&self, path: &Path) -> io::Result<()> {
            if self.should_fail("create_dir_all") {
                return Err(io::Error::other("injected"));
            }
            self.inner.create_dir_all(path)
        }

        fn write(&self, path: &Path, contents: &[u8]) -> io::Result<()> {
            if self.should_fail("write") {
                return Err(io::Error::other("injected"));
            }
            self.inner.write(path, contents)
        }

        fn rename(&self, from: &Path, to: &Path) -> io::Result<()> {
            if self.should_fail("rename") {
                return Err(io::Error::other("injected"));
            }
            self.inner.rename(from, to)
        }

        fn remove_dir_all(&self, path: &Path) -> io::Result<()> {
            if self.should_fail("remove_dir_all") {
                return Err(io::Error::other("injected"));
            }
            self.inner.remove_dir_all(path)
        }

        fn copy_tree(&self, source: &Path, destination: &Path) -> io::Result<()> {
            if self.should_fail("copy_tree") {
                return Err(io::Error::other("injected"));
            }
            self.inner.copy_tree(source, destination)
        }
    }

    struct Scene {
        _temp: tempfile::TempDir,
        root: PathBuf,
        output: PathBuf,
        staged: PathBuf,
        backup: PathBuf,
    }

    impl Scene {
        /// An existing output tree with one owned and one hand-written file, plus a staged
        /// replacement.
        fn new() -> Self {
            let temp = tempfile::tempdir().unwrap();
            let root = temp.path().to_path_buf();
            let output = root.join("generated");
            let staged = root.join(".spec42-stage-test");
            fs::create_dir_all(&output).unwrap();
            fs::write(output.join("owned.txt"), b"original").unwrap();
            fs::write(output.join("handwritten.txt"), b"mine").unwrap();
            fs::create_dir_all(&staged).unwrap();
            fs::write(staged.join("owned.txt"), b"regenerated").unwrap();
            fs::write(staged.join("handwritten.txt"), b"mine").unwrap();
            Self {
                _temp: temp,
                root: root.clone(),
                output,
                staged,
                backup: root.join(".spec42-backup-test"),
            }
        }

        fn original_is_intact(&self) -> bool {
            fs::read(self.output.join("owned.txt")).is_ok_and(|bytes| bytes == b"original")
                && fs::read(self.output.join("handwritten.txt")).is_ok_and(|bytes| bytes == b"mine")
        }

        /// Anything left beside the output root that should not be there.
        fn debris(&self) -> BTreeSet<String> {
            fs::read_dir(&self.root)
                .unwrap()
                .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
                .filter(|name| name != "generated")
                .collect()
        }

        /// Staged copies must never survive: unlike a backup they hold nothing recoverable,
        /// so leaving one behind is pure litter in the user's tree.
        fn stage_debris(&self) -> BTreeSet<String> {
            self.debris()
                .into_iter()
                .filter(|name| name.starts_with(".spec42-stage-"))
                .collect()
        }

        /// A surviving backup is acceptable only if it still holds the displaced tree.
        fn surviving_backup_is_recoverable(&self) -> bool {
            let previous = self.backup.join("previous");
            !self.backup.exists()
                || (fs::read(previous.join("owned.txt")).is_ok_and(|bytes| bytes == b"original")
                    && fs::read(previous.join("handwritten.txt"))
                        .is_ok_and(|bytes| bytes == b"mine"))
        }
    }

    /// Fails each mutation point in turn and checks the invariants after every one.
    ///
    /// The count is discovered rather than asserted, so adding a mutation to `install`
    /// extends the sweep automatically instead of silently leaving the new point uncovered.
    #[test]
    fn every_injected_failure_leaves_a_consistent_tree() {
        let mutation_points = {
            let scene = Scene::new();
            let filesystem = Failing::new(usize::MAX);
            let _ = install(
                &filesystem,
                &scene.output,
                &scene.staged,
                &scene.backup,
                true,
            );
            filesystem.mutations()
        };
        assert!(mutation_points >= 3, "expected several mutation points");

        for failure in 1..=mutation_points {
            let scene = Scene::new();
            let filesystem = Failing::new(failure);
            let result = install(
                &filesystem,
                &scene.output,
                &scene.staged,
                &scene.backup,
                true,
            );

            match result {
                Ok(()) => {
                    // The mutations after the swap are cleanup only, so failing one still
                    // leaves a correct output tree. What it must not do is lose the new
                    // content or strand a staged copy; a backup that survives its own failed
                    // removal is acceptable precisely because it is still recoverable.
                    assert_eq!(
                        fs::read(scene.output.join("owned.txt")).unwrap(),
                        b"regenerated",
                        "failure {failure} succeeded without installing the new content"
                    );
                    assert!(
                        scene.stage_debris().is_empty(),
                        "failure {failure} stranded a staged copy: {:?}",
                        scene.stage_debris()
                    );
                    assert!(
                        scene.surviving_backup_is_recoverable(),
                        "failure {failure} left a backup that no longer holds the old tree"
                    );
                }
                Err(ApplyError::BeforeDisplacement(_)) => {
                    assert!(
                        scene.original_is_intact(),
                        "failure {failure} damaged the output before displacing it"
                    );
                    assert!(
                        scene.debris().is_empty(),
                        "failure {failure} left {:?}",
                        scene.debris()
                    );
                }
                Err(ApplyError::RolledBack(_)) => {
                    assert!(
                        scene.original_is_intact(),
                        "failure {failure} rolled back but the original is not intact"
                    );
                    assert!(
                        scene.stage_debris().is_empty(),
                        "failure {failure} rolled back but stranded a staged copy"
                    );
                }
                Err(ApplyError::Unrecovered { backup, .. }) => {
                    // Rollback failed: the data must still be where the message says.
                    assert!(
                        backup.0.is_dir(),
                        "failure {failure} reported a backup that does not exist"
                    );
                    assert_eq!(
                        fs::read(backup.0.join("owned.txt")).unwrap(),
                        b"original",
                        "failure {failure} lost the previous output"
                    );
                    assert_eq!(
                        fs::read(backup.0.join("handwritten.txt")).unwrap(),
                        b"mine",
                        "failure {failure} lost an unowned file"
                    );
                }
            }
        }
    }

    #[test]
    fn a_clean_install_replaces_the_tree_and_removes_the_backup() {
        let scene = Scene::new();
        install(
            &RealFileSystem,
            &scene.output,
            &scene.staged,
            &scene.backup,
            true,
        )
        .expect("a clean install should succeed");

        assert_eq!(
            fs::read(scene.output.join("owned.txt")).unwrap(),
            b"regenerated"
        );
        assert!(scene.debris().is_empty(), "left {:?}", scene.debris());
    }

    #[test]
    fn installing_into_an_absent_output_needs_no_backup() {
        let scene = Scene::new();
        fs::remove_dir_all(&scene.output).unwrap();
        install(
            &RealFileSystem,
            &scene.output,
            &scene.staged,
            &scene.backup,
            false,
        )
        .expect("installing into an absent root should succeed");
        assert_eq!(
            fs::read(scene.output.join("owned.txt")).unwrap(),
            b"regenerated"
        );
        assert!(scene.debris().is_empty());
    }
}
