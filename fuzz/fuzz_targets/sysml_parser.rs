//! The pinned parser must classify every UTF-8 document without panicking.
//!
//! `sysml-v2-parser` exposes text APIs, so arbitrary bytes are deliberately
//! rejected at this boundary rather than being lossy-decoded into different source.
#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(source) = std::str::from_utf8(data) else {
        return;
    };

    let recovered = sysml_v2_parser::parse_for_editor(source);
    let strict = sysml_v2_parser::parse(source);

    // The pinned parser contracts its strict and editor entry points to agree
    // for a clean document. Recovered documents deliberately retain errors.
    if recovered.is_ok() {
        assert!(
            strict.is_ok(),
            "clean editor parse must pass strict parsing"
        );
    }
});
