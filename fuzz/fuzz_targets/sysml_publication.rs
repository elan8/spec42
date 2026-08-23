//! Recovery-mode immutable publication must handle every UTF-8 document safely.
#![no_main]

use libfuzzer_sys::fuzz_target;
use sysml_query::resolved_slice::{
    build, BuildRequest, ConstructionStrategy, AdmittedSource, SourceKind,
};

fuzz_target!(|data: &[u8]| {
    let Ok(source) = std::str::from_utf8(data) else {
        return;
    };
    let document = AdmittedSource::from_memory_path(
        "fuzz",
        "input.sysml",
        source.to_owned(),
        SourceKind::Workspace,
    )
    .expect("fixed memory URI must be valid");
    let request = BuildRequest::resolved(vec![document], ConstructionStrategy::Sequential)
        .expect("one source has a unique identity");
    let model = build(request).expect("in-memory immutable publication must not fail");
    std::hint::black_box(model.publication().completeness());
});
