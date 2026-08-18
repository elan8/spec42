//! Shared path resolution for the vendored sysml-robot-vacuum-cleaner fixture.
//!
//! Resolution order:
//! 1. `SYSML_ROBOT_VACUUM_DIR` environment override
//! 2. `third_party/sysml-robot-vacuum-cleaner` under the Spec42 repo root

use std::path::{Path, PathBuf};

/// Spec42 repository root (directory containing `config/robot-vacuum-cleaner.json`).
pub fn spec42_repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .find(|p| p.join("config").join("robot-vacuum-cleaner.json").is_file())
        .expect("spec42 repository root")
        .to_path_buf()
}

/// Root of the robot vacuum showcase checkout (directory containing `model/`).
pub fn robot_vacuum_fixture_root() -> PathBuf {
    if let Ok(override_dir) = std::env::var("SYSML_ROBOT_VACUUM_DIR") {
        return PathBuf::from(override_dir);
    }
    spec42_repo_root().join("third_party/sysml-robot-vacuum-cleaner")
}

/// Returns `(repo_root, model_dir)` or panics with fetch instructions.
pub fn require_robot_vacuum_fixture() -> (PathBuf, PathBuf) {
    let root = robot_vacuum_fixture_root();
    let model_dir = root.join("model");
    if !model_dir.is_dir() {
        panic!(
            "robot vacuum fixture missing at {} — run: bash scripts/fetch-robot-vacuum-cleaner.sh",
            root.display()
        );
    }
    (root, model_dir)
}
