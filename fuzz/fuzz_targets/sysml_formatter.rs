//! Formatting any UTF-8 document must converge in one pass.
//!
//! Inputs are not lossy-decoded: the formatter's API accepts authored UTF-8
//! text, while arbitrary non-UTF-8 bytes are outside that contract.
#![no_main]

use language_service::{format_document_text, FormatOptions};
use libfuzzer_sys::fuzz_target;

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

fuzz_target!(|data: &[u8]| {
    let Ok(source) = std::str::from_utf8(data) else {
        return;
    };

    for options in OPTIONS {
        let formatted = format_document_text(source, options);
        assert_eq!(
            format_document_text(&formatted, options),
            formatted,
            "formatter must converge after one pass"
        );
    }
});
