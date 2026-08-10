//! Decoding guest-supplied Postcard payloads must not panic on arbitrary bytes.
//!
//! The host treats everything a guest returns as untrusted input, so a malformed payload has
//! to be an error rather than an abort.
#![no_main]

use libfuzzer_sys::fuzz_target;
use spec42_generator_protocol::{Artifact, ElementDetail, ElementSummary, ModelInfo, Relationship};

fuzz_target!(|data: &[u8]| {
    // The entrypoint result, and each response type a query can carry.
    let _ = postcard::from_bytes::<Result<Vec<Artifact>, String>>(data);
    let _ = postcard::from_bytes::<Result<ModelInfo, String>>(data);
    let _ = postcard::from_bytes::<Result<Vec<ElementSummary>, String>>(data);
    let _ = postcard::from_bytes::<Result<ElementDetail, String>>(data);
    let _ = postcard::from_bytes::<Result<Vec<Relationship>, String>>(data);
    let _ = postcard::from_bytes::<Vec<String>>(data);
});
