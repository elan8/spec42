//! Deterministic regression coverage for the checked-in SysML fuzz seeds.
//!
//! Fuzzing is exploratory. Any finding that is minimized into this corpus is
//! exercised here by the same public APIs so it remains a normal test failure.

use std::fs;
use std::path::{Path, PathBuf};

use language_service::{format_document_text, FormatOptions};
use sysml_query::resolved_slice::{build, BuildRequest, ConstructionStrategy, SourceDocument, SourceKind};

const CORPUS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/corpus/sysml");
const SEED_COUNT: usize = 6;
const OPTIONS: [FormatOptions; 2] = [
    FormatOptions {
        tab_size: 4,
        insert_spaces: true,
    },
    FormatOptions {
        tab_size: 1,
        insert_spaces: false,
    },
];

#[test]
fn sysml_seed_corpus_exercises_public_language_apis() {
    let seeds = seed_paths(Path::new(CORPUS));
    assert_eq!(
        seeds.len(),
        SEED_COUNT,
        "update the seed count when intentionally changing the corpus"
    );

    for seed in seeds {
        let bytes = fs::read(&seed).expect("read seed");
        let source = std::str::from_utf8(&bytes).expect("seed must be valid UTF-8 for text APIs");

        let recovered = sysml_v2_parser::parse_for_editor(source);
        let strict = sysml_v2_parser::parse(source);
        if recovered.is_ok() {
            assert!(
                strict.is_ok(),
                "{}: clean editor parse must pass strict parsing",
                seed.display()
            );
        }

        let document = SourceDocument::from_memory_path(
            "fuzz-seed",
            seed.file_name()
                .and_then(|name| name.to_str())
                .expect("UTF-8 seed file name"),
            source.to_owned(),
            SourceKind::Workspace,
        )
        .expect("fixed memory URI must be valid");
        let request = BuildRequest::resolved(vec![document], ConstructionStrategy::Sequential)
            .expect("one source has a unique identity");
        let model = build(request).expect("recovery-mode immutable publication construction");
        std::hint::black_box(model.publication().completeness());

        for options in OPTIONS {
            let formatted = format_document_text(source, options);
            assert_eq!(
                format_document_text(&formatted, options),
                formatted,
                "{}: formatter must converge after one pass",
                seed.display()
            );
        }
    }
}

fn seed_paths(root: &Path) -> Vec<PathBuf> {
    let mut paths = fs::read_dir(root)
        .expect("read seed corpus")
        .map(|entry| entry.expect("seed directory entry").path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "sysml")
        })
        .collect::<Vec<_>>();
    paths.sort();
    paths
}
