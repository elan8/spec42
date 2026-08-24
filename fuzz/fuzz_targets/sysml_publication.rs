//! Recovery-mode immutable publication must handle every UTF-8 document safely.
#![no_main]

use libfuzzer_sys::fuzz_target;
use sysml_query::source::SourceKind;
use sysml_query::Services;

fuzz_target!(|data: &[u8]| {
    let Ok(source) = std::str::from_utf8(data) else {
        return;
    };
    let services = Services::new();
    let document = services
        .source
        .admit_memory("fuzz", "input.sysml", source, SourceKind::Workspace)
        .expect("fixed memory URI must be valid");
    let model = services
        .publication
        .publish(&[document], std::iter::empty::<Box<str>>())
        .expect("in-memory immutable publication must not fail");
    std::hint::black_box(model.publication().completeness());
});
