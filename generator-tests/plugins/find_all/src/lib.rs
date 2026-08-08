//! Issues a single `find(None)` and reports the count.
//!
//! Isolates the cost of the host's element-exposure path: one query, one large response,
//! no per-element follow-up work.

use spec42_generator_sdk::{export, model, Artifact, Guest};

struct FindAll;

impl Guest for FindAll {
    fn generate(_args: Vec<String>) -> Result<Vec<Artifact>, String> {
        let all = model::find(None)?;
        Ok(vec![Artifact {
            file_path: "count.txt".to_owned(),
            contents: format!("{}\n", all.len()).into_bytes(),
        }])
    }
}

export!(FindAll);
