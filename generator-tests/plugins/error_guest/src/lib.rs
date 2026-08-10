//! Returns a generator error, which must fail the run without writing anything.

use spec42_generator_sdk::{export, Artifact, Guest};

struct ErrorGuest;

impl Guest for ErrorGuest {
    fn generate(args: Vec<String>) -> Result<Vec<Artifact>, String> {
        Err(format!("deliberate failure: {}", args.join(" ")))
    }
}

export!(ErrorGuest);
