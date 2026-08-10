//! Emits whatever paths its arguments name, so path validation can be driven from a case
//! file rather than needing one plugin per rejected path.
//!
//! Usage: `-- path=<value> [path=<value> ...]`

use spec42_generator_sdk::{export, Artifact, Guest};

struct ArtifactPaths;

impl Guest for ArtifactPaths {
    fn generate(args: Vec<String>) -> Result<Vec<Artifact>, String> {
        let paths = args
            .iter()
            .filter_map(|arg| arg.strip_prefix("path="))
            .collect::<Vec<_>>();
        if paths.is_empty() {
            return Err("expected at least one path=<value> argument".to_owned());
        }
        Ok(paths
            .into_iter()
            .enumerate()
            .map(|(index, path)| Artifact {
                file_path: path.to_owned(),
                contents: format!("artifact {index}\n").into_bytes(),
            })
            .collect())
    }
}

export!(ArtifactPaths);
