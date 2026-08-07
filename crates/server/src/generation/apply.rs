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

/// Stages the artifacts and the manifest, then swaps them into place.
///
/// The whole operation goes through `filesystem`, so a fault sweep reaches the staging writes
/// and the tree copy as well as the swap. Previously staging used `std::fs` directly and only
/// the swap was injectable, leaving the larger half of the transaction uncovered.
#[allow(clippy::too_many_arguments)]
pub fn stage_and_install(
    filesystem: &dyn FileSystem,
    output: &Path,
    staged: &Path,
    backup_root: &Path,
    output_exists: bool,
    artifacts: &[(String, Vec<u8>)],
    manifest_name: &str,
    manifest: &[u8],
) -> Result<(), ApplyError> {
    let staging = |message: String| ApplyError::BeforeDisplacement(message);

    if let Err(error) = filesystem.create_dir_all(staged) {
        return Err(staging(format!(
            "failed to create private output staging directory: {error}"
        )));
    }
    if output_exists {
        if let Err(error) = filesystem.copy_tree(output, staged) {
            let _ = filesystem.remove_dir_all(staged);
            return Err(staging(error.to_string()));
        }
    }
    for (path, content) in artifacts {
        let target = staged.join(path);
        if let Some(parent) = target.parent() {
            if let Err(error) = filesystem.create_dir_all(parent) {
                let _ = filesystem.remove_dir_all(staged);
                return Err(staging(format!(
                    "failed to create staging directory {}: {error}",
                    parent.display()
                )));
            }
        }
        if let Err(error) = filesystem.write(&target, content) {
            let _ = filesystem.remove_dir_all(staged);
            return Err(staging(format!("failed to stage {path}: {error}")));
        }
    }
    if let Err(error) = filesystem.write(&staged.join(manifest_name), manifest) {
        let _ = filesystem.remove_dir_all(staged);
        return Err(staging(format!(
            "failed to stage generation manifest: {error}"
        )));
    }

    install(filesystem, output, staged, backup_root, output_exists)
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

    /// Records every mutation and fails the ones in its schedule.
    ///
    /// A schedule rather than a single index: reaching `Unrecovered` needs *two* failures --
    /// the install must fail and the rollback that follows must fail too -- so a one-shot
    /// injector could never reach that branch, and the assertion for it was dead.
    struct Failing {
        inner: RealFileSystem,
        schedule: BTreeSet<usize>,
        seen: RefCell<usize>,
        trace: RefCell<Vec<String>>,
        /// Operations the schedule actually failed, so the sweep can tell a leak from a
        /// cleanup that was itself sabotaged.
        failed: RefCell<Vec<String>>,
    }

    impl Failing {
        fn with_schedule(schedule: impl IntoIterator<Item = usize>) -> Self {
            Self {
                inner: RealFileSystem,
                schedule: schedule.into_iter().collect(),
                seen: RefCell::new(0),
                trace: RefCell::new(Vec::new()),
                failed: RefCell::new(Vec::new()),
            }
        }

        fn should_fail(&self, what: &str) -> bool {
            let mut seen = self.seen.borrow_mut();
            *seen += 1;
            self.trace.borrow_mut().push(format!("{}:{what}", *seen));
            let failing = self.schedule.contains(&*seen);
            if failing {
                self.failed.borrow_mut().push(what.to_owned());
            }
            failing
        }

        /// Whether the schedule sabotaged a cleanup. If it did, the thing that cleanup would
        /// have removed is expected to survive -- that is the injected failure, not a leak.
        fn sabotaged_cleanup(&self) -> bool {
            self.failed
                .borrow()
                .iter()
                .any(|operation| operation == "remove_dir_all")
        }

        /// How many mutations this run actually performed.
        ///
        /// The count depends on the schedule: failing early exposes cleanup and rollback
        /// operations that a successful run never performs, which is why mutation points
        /// cannot be discovered from the success trace alone.
        fn mutations(&self) -> usize {
            *self.seen.borrow()
        }

        fn trace(&self) -> Vec<String> {
            self.trace.borrow().clone()
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
            Self {
                _temp: temp,
                root: root.clone(),
                output,
                staged,
                backup: root.join(".spec42-backup-test"),
            }
        }

        /// Runs the full transaction: stage the artifacts and manifest, then swap.
        fn run(&self, filesystem: &dyn FileSystem) -> Result<(), ApplyError> {
            stage_and_install(
                filesystem,
                &self.output,
                &self.staged,
                &self.backup,
                true,
                &[("owned.txt".to_owned(), b"regenerated".to_vec())],
                ".spec42-generator-manifest.json",
                b"{}",
            )
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

    /// Explores failure schedules, checking the invariants after every one.
    ///
    /// Single failures are not enough. Reaching `Unrecovered` requires the install to fail
    /// *and* the rollback after it to fail, so a one-shot injector left that branch dead
    /// while an assertion for it sat in the sweep looking like coverage.
    ///
    /// Mutation points also cannot be discovered from a successful run: failing early exposes
    /// cleanup and rollback operations that a clean install never performs. So the search is
    /// recursive -- run a schedule, observe how many mutations that trace reached, and extend
    /// the schedule into the newly exposed positions.
    #[test]
    fn every_failure_schedule_leaves_a_consistent_tree() {
        let mut queue: Vec<BTreeSet<usize>> = vec![BTreeSet::new()];
        let mut explored: BTreeSet<Vec<usize>> = BTreeSet::new();
        let mut unrecovered_seen = false;
        let mut rolled_back_seen = false;
        let mut before_displacement_seen = false;

        while let Some(schedule) = queue.pop() {
            let key: Vec<usize> = schedule.iter().copied().collect();
            if !explored.insert(key.clone()) {
                continue;
            }
            // Two simultaneous failures is enough to reach every branch here, and keeps the
            // search finite; deeper schedules would only repeat the same outcomes.
            if key.len() > 2 {
                continue;
            }

            let scene = Scene::new();
            let filesystem = Failing::with_schedule(schedule.iter().copied());
            let result = scene.run(&filesystem);
            let reached = filesystem.mutations();
            let trace = filesystem.trace();

            match &result {
                Ok(()) => {
                    assert_eq!(
                        fs::read(scene.output.join("owned.txt")).unwrap(),
                        b"regenerated",
                        "schedule {key:?} succeeded without installing the new content\n{trace:?}"
                    );
                    assert!(
                        scene.stage_debris().is_empty() || filesystem.sabotaged_cleanup(),
                        "schedule {key:?} stranded a staged copy: {:?}\n{trace:?}",
                        scene.stage_debris()
                    );
                    assert!(
                        scene.surviving_backup_is_recoverable(),
                        "schedule {key:?} left an unrecoverable backup\n{trace:?}"
                    );
                }
                Err(ApplyError::BeforeDisplacement(_)) => {
                    before_displacement_seen = true;
                    assert!(
                        scene.original_is_intact(),
                        "schedule {key:?} damaged the output before displacing it\n{trace:?}"
                    );
                    assert!(
                        scene.stage_debris().is_empty() || filesystem.sabotaged_cleanup(),
                        "schedule {key:?} left {:?}\n{trace:?}",
                        scene.stage_debris()
                    );
                }
                Err(ApplyError::RolledBack(_)) => {
                    rolled_back_seen = true;
                    assert!(
                        scene.original_is_intact(),
                        "schedule {key:?} rolled back but the original is not intact\n{trace:?}"
                    );
                    assert!(
                        scene.stage_debris().is_empty() || filesystem.sabotaged_cleanup(),
                        "schedule {key:?} rolled back but stranded a staged copy\n{trace:?}"
                    );
                }
                Err(ApplyError::Unrecovered { backup, .. }) => {
                    unrecovered_seen = true;
                    assert!(
                        backup.0.is_dir(),
                        "schedule {key:?} reported a backup that does not exist\n{trace:?}"
                    );
                    assert_eq!(
                        fs::read(backup.0.join("owned.txt")).unwrap(),
                        b"original",
                        "schedule {key:?} lost the previous output\n{trace:?}"
                    );
                    assert_eq!(
                        fs::read(backup.0.join("handwritten.txt")).unwrap(),
                        b"mine",
                        "schedule {key:?} lost an unowned file\n{trace:?}"
                    );
                }
            }

            // Extend into every position this trace reached, including ones only a failing
            // run performs.
            for position in 1..=reached {
                if schedule.contains(&position) {
                    continue;
                }
                let mut next = schedule.clone();
                next.insert(position);
                queue.push(next);
            }
        }

        assert!(explored.len() > 10, "the search barely explored anything");
        assert!(
            before_displacement_seen,
            "no schedule failed before displacement"
        );
        assert!(rolled_back_seen, "no schedule exercised rollback");
        assert!(
            unrecovered_seen,
            "no schedule reached an unrecovered rollback failure, so that branch is untested"
        );
    }

    #[test]
    fn a_clean_install_replaces_the_tree_and_removes_the_backup() {
        let scene = Scene::new();
        scene
            .run(&RealFileSystem)
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
        stage_and_install(
            &RealFileSystem,
            &scene.output,
            &scene.staged,
            &scene.backup,
            false,
            &[("owned.txt".to_owned(), b"regenerated".to_vec())],
            ".spec42-generator-manifest.json",
            b"{}",
        )
        .expect("installing into an absent root should succeed");
        assert_eq!(
            fs::read(scene.output.join("owned.txt")).unwrap(),
            b"regenerated"
        );
        assert!(scene.debris().is_empty());
    }
}
