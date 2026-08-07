//! Produces no artifacts. Exercises the empty ArtifactSet and `--check` on an empty tree.

use spec42_generator_sdk::{export, Artifact, Guest};

struct Empty;

impl Guest for Empty {
    fn generate(_args: Vec<String>) -> Result<Vec<Artifact>, String> {
        Ok(Vec::new())
    }
}

export!(Empty);
