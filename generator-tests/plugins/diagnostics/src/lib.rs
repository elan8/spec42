//! Emits diagnostics at every level, with a valid handle, an invalid handle and none.

use spec42_generator_sdk::{diagnostics, export, model, Artifact, Guest};

struct Diagnostics;

impl Guest for Diagnostics {
    fn generate(_args: Vec<String>) -> Result<Vec<Artifact>, String> {
        let first = model::find(None)?.into_iter().next();

        diagnostics::log(diagnostics::Level::Debug, "debug, unscoped");
        diagnostics::log(diagnostics::Level::Info, "info, unscoped");
        diagnostics::log(diagnostics::Level::Warning, "warning, unscoped");
        diagnostics::log(diagnostics::Level::Error, "error, unscoped");

        if let Some(element) = &first {
            diagnostics::report(
                diagnostics::Level::Warning,
                "warning, scoped to a real element",
                Some(&element.handle),
            );
        }
        // An unresolvable handle must degrade to an unscoped diagnostic, not fail the run.
        diagnostics::report(
            diagnostics::Level::Info,
            "info, scoped to a handle that does not resolve",
            Some("h:not-a-real-handle"),
        );

        Ok(vec![Artifact {
            file_path: "diagnostics.txt".to_owned(),
            contents: b"see the report\n".to_vec(),
        }])
    }
}

export!(Diagnostics);
