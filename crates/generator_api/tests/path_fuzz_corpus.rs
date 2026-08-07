//! Deterministic regressions distilled from exploratory runs.
//!
//! Fuzzing and mutation testing belong off required CI: they are not reproducible from the
//! same inputs every time, so they cannot gate a PR. What they produce that *is* durable is a
//! minimal input, and those live here as ordinary tests.
//!
//! Add a case whenever an exploratory run finds something, noting where it came from.

use generator_api::ArtifactPath;

/// Parsing is total: every input is classified, none panics.
#[test]
fn known_awkward_inputs_are_classified_without_panicking() {
    let nested = "a/".repeat(500);
    let long_segment = "x".repeat(4095);
    let name_length_segment = "x".repeat(255);

    let cases: Vec<&str> = vec![
        // Alternate data streams, reduced to their minimal forms.
        ".spec42-generator-manifest.json::$DATA",
        "a:b",
        // Superscript device digits.
        "COM\u{b9}.txt",
        "LPT\u{b3}",
        // Combining marks alone and applied.
        "\u{301}",
        "a\u{301}",
        "\u{fffd}",
        &name_length_segment,
        &long_segment,
        nested.trim_end_matches('/'),
        // Separator-only and traversal forms.
        "/",
        "//",
        "./",
        "..",
        "a\\b/c",
        // A name that is only an extension.
        ".gitignore",
        ".",
    ];

    for case in cases {
        let _ = ArtifactPath::parse(case);
    }
}

/// Cases whose classification is the point, not merely that parsing terminates.
#[test]
fn awkward_inputs_are_classified_correctly() {
    assert!(ArtifactPath::parse(".gitignore").is_ok());
    assert!(
        ArtifactPath::parse("\u{301}").is_ok(),
        "a combining mark alone is an unusual but legal name"
    );
    assert!(ArtifactPath::parse("a:b").is_err(), "alternate data stream");
    assert!(ArtifactPath::parse("COM\u{b9}.txt").is_err(), "device name");
    assert!(
        ArtifactPath::parse("a\\b/c").is_err(),
        "backslash separator"
    );
}
