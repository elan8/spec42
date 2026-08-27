use std::fs;
use std::path::{Path, PathBuf};

use language_service::{format_document_text, FormatOptions};
use std::sync::Arc;

use sysml_query::resolved_slice::PublishedModel;
use sysml_query::source::SourceKind;
use sysml_query::Services;

const FORMAT_OPTIONS: [FormatOptions; 2] = [
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
fn checked_in_sysml_seeds_satisfy_the_language_fuzz_contracts() {
    let corpus = Path::new(env!("CARGO_MANIFEST_DIR")).join("corpus/sysml");
    let seeds = seed_paths(&corpus);
    assert!(
        !seeds.is_empty(),
        "{} must contain at least one checked-in seed",
        corpus.display()
    );

    for path in seeds {
        let bytes = fs::read(&path)
            .unwrap_or_else(|error| panic!("{}: seed read failed: {error}", path.display()));
        assert!(
            !bytes.is_empty(),
            "{}: seed must not be empty",
            path.display()
        );
        let source = std::str::from_utf8(&bytes)
            .unwrap_or_else(|error| panic!("{}: seed is not UTF-8: {error}", path.display()));

        let first = publish_seed(&path, source);
        let second = publish_seed(&path, source);
        assert_eq!(
            first.publication().identity(),
            second.publication().identity(),
            "{}: identical input must have a stable publication identity",
            path.display()
        );
        assert_eq!(
            first.publication().completeness(),
            second.publication().completeness(),
            "{}: identical input must have stable completeness",
            path.display()
        );

        for options in FORMAT_OPTIONS {
            let formatted = format_document_text(source, options);
            assert_eq!(
                format_document_text(&formatted, options),
                formatted,
                "{}: formatter must converge after one pass",
                path.display()
            );
        }
    }
}

fn seed_paths(corpus: &Path) -> Vec<PathBuf> {
    let mut paths = fs::read_dir(corpus)
        .unwrap_or_else(|error| panic!("{}: corpus read failed: {error}", corpus.display()))
        .map(|entry| entry.expect("seed directory entry").path())
        .filter(|path| path.is_file())
        .collect::<Vec<_>>();
    paths.sort();
    paths
}

fn publish_seed(path: &Path, source: &str) -> Arc<PublishedModel> {
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .expect("seed filename is UTF-8");
    let services = Services::new();
    let document = services
        .source
        .admit_memory("fuzz-seed", name, source, SourceKind::Workspace)
        .unwrap_or_else(|error| panic!("{}: admission failed: {error}", path.display()));
    services
        .publication
        .publish(&[document], std::iter::empty::<Box<str>>())
        .unwrap_or_else(|error| panic!("{}: publication failed: {error}", path.display()))
}
