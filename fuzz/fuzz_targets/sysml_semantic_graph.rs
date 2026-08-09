//! Recovery-mode semantic construction must handle every UTF-8 document safely.
//!
//! This uses Spec42's owning workspace pipeline rather than recreating semantic
//! facts in the fuzz target.
#![no_main]

use libfuzzer_sys::fuzz_target;
use sysml_model::{build_semantic_graph_from_documents, SysmlDocument, SysmlDocumentSourceKind};

fuzz_target!(|data: &[u8]| {
    let Ok(source) = std::str::from_utf8(data) else {
        return;
    };
    let document = SysmlDocument::from_memory_path(
        "fuzz",
        "input.sysml",
        source.to_owned(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("fixed memory URI must be valid");

    let (_graph, parsed) = build_semantic_graph_from_documents(&[document])
        .expect("in-memory document construction must not fail");
    assert_eq!(
        parsed.len(),
        1,
        "one input document must publish one parse result"
    );
    assert_eq!(
        parsed[0].content, source,
        "recovery must retain authored source"
    );
});
