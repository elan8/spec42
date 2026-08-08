//! Artifact path parsing must be total: every input is classified, none panics.
//!
//! Findings belong in crates/generator_api/tests/path_fuzz_corpus.rs as minimal deterministic
//! cases; this target is exploratory and does not gate anything.
#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else {
        return;
    };
    if let Ok(parsed) = generator_api::ArtifactPath::parse(text) {
        // An accepted path must survive a round trip through its own normalization, or the
        // set's collision key and the path it writes to could disagree.
        let reparsed = generator_api::ArtifactPath::parse(parsed.as_str())
            .expect("a normalized path must re-parse");
        assert_eq!(parsed, reparsed);
        // Folding must be idempotent, since it is the collision key.
        assert_eq!(parsed.folded(), reparsed.folded());
    }
});
