//! Object envelope encode/decode (plan §7.2).
//!
//! Envelope v1 defines exactly one payload format: a canonical postcard record compressed with
//! zstd. Encoding/decoding is manual (not serde-derived) so that every field can be validated,
//! in order, before any bytes are trusted enough to decompress or decode.
//!
//! Wire layout:
//!
//! ```text
//! [magic:              4 bytes  "S42C"]
//! [envelope_version:   2 bytes  u16 LE]
//! [artifact_kind:      1 byte   u8]
//! [artifact_schema:    4 bytes  u32 LE]
//! [expected_key:      32 bytes]
//! [compressed_len:     8 bytes  u64 LE]
//! [uncompressed_len:   8 bytes  u64 LE]
//! [payload_digest:    32 bytes  BLAKE3 of the uncompressed payload]
//! [payload:  compressed_len bytes, zstd-compressed canonical postcard record]
//! ```

use super::api::{ArtifactKind, CacheMissReason};
use super::config::CacheLimits;
use source_identity::ArtifactKey;

pub const MAGIC: &[u8; 4] = b"S42C";
pub const ENVELOPE_VERSION: u16 = 1;

pub(super) const HEADER_LEN: usize = 4 + 2 + 1 + 4 + 32 + 8 + 8 + 32;

/// Byte offset of the artifact-kind field within the fixed header.
pub(super) const KIND_OFFSET: usize = 4 + 2;

/// The largest object worth reading at all: an object whose file is longer than the header plus
/// the compressed-payload limit cannot pass [`decode`], so it is rejected before it is read into
/// memory rather than after (plan §7.2, prerequisite B10).
pub(super) fn max_object_file_bytes(limits: &CacheLimits) -> u64 {
    HEADER_LEN as u64 + limits.max_compressed_object_bytes
}

/// Encodes `plain_payload` (an already-postcard-serialized artifact record) into a complete
/// envelope: zstd-compresses it, computes its BLAKE3 digest, and writes the fixed header.
///
/// Returns `None` only if the uncompressed payload already exceeds the configured limit; the
/// caller should surface that as a write failure rather than publishing an oversized object.
pub fn encode(
    kind: ArtifactKind,
    schema_version: u32,
    key: ArtifactKey,
    plain_payload: &[u8],
    limits: &CacheLimits,
) -> Result<Vec<u8>, String> {
    let uncompressed_len = plain_payload.len() as u64;
    if uncompressed_len > limits.max_uncompressed_object_bytes {
        return Err(format!(
            "uncompressed payload {uncompressed_len} bytes exceeds limit {}",
            limits.max_uncompressed_object_bytes
        ));
    }
    let payload_digest = blake3::hash(plain_payload);
    let compressed = zstd::stream::encode_all(plain_payload, 0)
        .map_err(|e| format!("zstd compression failed: {e}"))?;
    let compressed_len = compressed.len() as u64;
    if compressed_len > limits.max_compressed_object_bytes {
        return Err(format!(
            "compressed payload {compressed_len} bytes exceeds limit {}",
            limits.max_compressed_object_bytes
        ));
    }

    let mut out = Vec::with_capacity(HEADER_LEN + compressed.len());
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&ENVELOPE_VERSION.to_le_bytes());
    out.push(kind.to_u8());
    out.extend_from_slice(&schema_version.to_le_bytes());
    out.extend_from_slice(key.as_bytes());
    out.extend_from_slice(&compressed_len.to_le_bytes());
    out.extend_from_slice(&uncompressed_len.to_le_bytes());
    out.extend_from_slice(payload_digest.as_bytes());
    out.extend_from_slice(&compressed);
    Ok(out)
}

/// A successfully validated and decoded envelope: the raw uncompressed payload bytes plus the
/// lengths that were verified. Callers still need to postcard-decode `payload` into `T` and run
/// `T::validate_invariants`.
#[derive(Debug)]
pub struct DecodedEnvelope {
    pub payload: Vec<u8>,
    pub compressed_len: u64,
    pub uncompressed_len: u64,
}

/// Validates magic, versions, expected key, lengths, and resource limits, then streams
/// decompression into a bounded buffer and verifies the payload digest — all before any
/// artifact-specific decode is attempted (plan §7.2).
pub fn decode(
    bytes: &[u8],
    expected_kind: ArtifactKind,
    expected_schema_version: u32,
    expected_key: ArtifactKey,
    limits: &CacheLimits,
) -> Result<DecodedEnvelope, CacheMissReason> {
    if bytes.len() < HEADER_LEN {
        return Err(CacheMissReason::Truncated);
    }
    let mut cursor = 0usize;
    let magic = &bytes[cursor..cursor + 4];
    cursor += 4;
    if magic != MAGIC {
        return Err(CacheMissReason::DecodeFailure {
            detail: "bad magic".to_string(),
        });
    }

    let envelope_version = u16::from_le_bytes(bytes[cursor..cursor + 2].try_into().unwrap());
    cursor += 2;
    if envelope_version != ENVELOPE_VERSION {
        return Err(CacheMissReason::IncompatibleVersion {
            detail: format!("envelope version {envelope_version}, expected {ENVELOPE_VERSION}"),
        });
    }

    let kind_byte = bytes[cursor];
    cursor += 1;
    let kind = ArtifactKind::from_u8(kind_byte).ok_or_else(|| CacheMissReason::DecodeFailure {
        detail: format!("unknown artifact kind byte {kind_byte}"),
    })?;
    if kind != expected_kind {
        return Err(CacheMissReason::DecodeFailure {
            detail: format!("artifact kind {kind:?} does not match expected {expected_kind:?}"),
        });
    }

    let schema_version = u32::from_le_bytes(bytes[cursor..cursor + 4].try_into().unwrap());
    cursor += 4;
    if schema_version != expected_schema_version {
        return Err(CacheMissReason::IncompatibleVersion {
            detail: format!(
                "artifact schema version {schema_version}, expected {expected_schema_version}"
            ),
        });
    }

    let key_bytes: [u8; 32] = bytes[cursor..cursor + 32].try_into().unwrap();
    cursor += 32;
    if key_bytes != *expected_key.as_bytes() {
        return Err(CacheMissReason::KeyMismatch);
    }

    let compressed_len = u64::from_le_bytes(bytes[cursor..cursor + 8].try_into().unwrap());
    cursor += 8;
    let uncompressed_len = u64::from_le_bytes(bytes[cursor..cursor + 8].try_into().unwrap());
    cursor += 8;

    let payload_digest: [u8; 32] = bytes[cursor..cursor + 32].try_into().unwrap();
    cursor += 32;

    if uncompressed_len > limits.max_uncompressed_object_bytes {
        return Err(CacheMissReason::ResourceLimit {
            detail: format!(
                "declared uncompressed length {uncompressed_len} exceeds limit {}",
                limits.max_uncompressed_object_bytes
            ),
        });
    }
    if compressed_len > limits.max_compressed_object_bytes {
        return Err(CacheMissReason::ResourceLimit {
            detail: format!(
                "declared compressed length {compressed_len} exceeds limit {}",
                limits.max_compressed_object_bytes
            ),
        });
    }

    let remaining = bytes.len() - cursor;
    if (remaining as u64) < compressed_len {
        return Err(CacheMissReason::Truncated);
    }
    if (remaining as u64) > compressed_len {
        // Trailing garbage past the declared compressed length: reject rather than silently
        // ignoring extra bytes.
        return Err(CacheMissReason::DecodeFailure {
            detail: "trailing bytes past declared compressed length".to_string(),
        });
    }

    let compressed = &bytes[cursor..cursor + compressed_len as usize];

    // Bound decompression by the declared uncompressed length so a decompression bomb (a small
    // compressed stream that expands far past what was declared) is rejected rather than
    // exhausting memory. `zstd::bulk::decompress` errors if the true decompressed size would
    // exceed the given capacity.
    let capacity = uncompressed_len as usize;
    let payload = zstd::bulk::decompress(compressed, capacity).map_err(|e| {
        CacheMissReason::ResourceLimit {
            detail: format!("decompression failed or bomb: {e}"),
        }
    })?;

    if payload.len() as u64 != uncompressed_len {
        return Err(CacheMissReason::Truncated);
    }

    let actual_digest = blake3::hash(&payload);
    if actual_digest.as_bytes() != &payload_digest {
        return Err(CacheMissReason::ChecksumFailure);
    }

    Ok(DecodedEnvelope {
        payload,
        compressed_len,
        uncompressed_len,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn limits() -> CacheLimits {
        CacheLimits::default()
    }

    #[test]
    fn round_trip() {
        let key = ArtifactKey::of_bytes(b"test-artifact");
        let payload = b"hello, this is a small test payload".to_vec();
        let encoded = encode(ArtifactKind::ParseOutcome, 1, key, &payload, &limits()).unwrap();
        let decoded = decode(&encoded, ArtifactKind::ParseOutcome, 1, key, &limits()).unwrap();
        assert_eq!(decoded.payload, payload);
    }

    #[test]
    fn rejects_bad_magic() {
        let key = ArtifactKey::of_bytes(b"k");
        let mut encoded =
            encode(ArtifactKind::ParseOutcome, 1, key, b"payload", &limits()).unwrap();
        encoded[0] = b'X';
        let err = decode(&encoded, ArtifactKind::ParseOutcome, 1, key, &limits()).unwrap_err();
        assert!(matches!(err, CacheMissReason::DecodeFailure { .. }));
    }

    #[test]
    fn rejects_incompatible_envelope_version() {
        let key = ArtifactKey::of_bytes(b"k");
        let mut encoded =
            encode(ArtifactKind::ParseOutcome, 1, key, b"payload", &limits()).unwrap();
        encoded[4] = 9;
        encoded[5] = 0;
        let err = decode(&encoded, ArtifactKind::ParseOutcome, 1, key, &limits()).unwrap_err();
        assert!(matches!(err, CacheMissReason::IncompatibleVersion { .. }));
    }

    #[test]
    fn rejects_incompatible_schema_version() {
        let key = ArtifactKey::of_bytes(b"k");
        let encoded = encode(ArtifactKind::ParseOutcome, 1, key, b"payload", &limits()).unwrap();
        let err = decode(&encoded, ArtifactKind::ParseOutcome, 2, key, &limits()).unwrap_err();
        assert!(matches!(err, CacheMissReason::IncompatibleVersion { .. }));
    }

    #[test]
    fn rejects_key_mismatch() {
        let key = ArtifactKey::of_bytes(b"k1");
        let other_key = ArtifactKey::of_bytes(b"k2");
        let encoded = encode(ArtifactKind::ParseOutcome, 1, key, b"payload", &limits()).unwrap();
        let err = decode(
            &encoded,
            ArtifactKind::ParseOutcome,
            1,
            other_key,
            &limits(),
        )
        .unwrap_err();
        assert!(matches!(err, CacheMissReason::KeyMismatch));
    }

    #[test]
    fn rejects_truncation() {
        let key = ArtifactKey::of_bytes(b"k");
        let encoded = encode(
            ArtifactKind::ParseOutcome,
            1,
            key,
            b"a reasonably long payload here",
            &limits(),
        )
        .unwrap();
        let truncated = &encoded[..encoded.len() - 5];
        let err = decode(truncated, ArtifactKind::ParseOutcome, 1, key, &limits()).unwrap_err();
        assert!(matches!(err, CacheMissReason::Truncated));
    }

    #[test]
    fn rejects_bit_flip_in_payload() {
        let key = ArtifactKey::of_bytes(b"k");
        let mut encoded = encode(
            ArtifactKind::ParseOutcome,
            1,
            key,
            b"payload bytes for bit flip test",
            &limits(),
        )
        .unwrap();
        let last = encoded.len() - 1;
        encoded[last] ^= 0xFF;
        let err = decode(&encoded, ArtifactKind::ParseOutcome, 1, key, &limits());
        // A bit flip either corrupts the zstd frame (decode failure/resource limit) or, if it
        // still decompresses, must fail the checksum. Either is an acceptable typed miss; a
        // "successful" decode of corrupted bytes is not.
        assert!(err.is_err());
    }

    #[test]
    fn rejects_oversized_declared_uncompressed_length() {
        let key = ArtifactKey::of_bytes(b"k");
        let mut tight_limits = limits();
        tight_limits.max_uncompressed_object_bytes = 4;
        let encoded = encode(
            ArtifactKind::ParseOutcome,
            1,
            key,
            b"payload",
            &tight_limits,
        );
        assert!(
            encoded.is_err(),
            "encode should reject payload exceeding limits at write time"
        );

        // Also verify decode-time rejection using object encoded under generous limits but
        // decoded under tight limits (simulating a reconfigured/attacker-supplied limit).
        let encoded = encode(ArtifactKind::ParseOutcome, 1, key, b"payload", &limits()).unwrap();
        let err = decode(&encoded, ArtifactKind::ParseOutcome, 1, key, &tight_limits).unwrap_err();
        assert!(matches!(err, CacheMissReason::ResourceLimit { .. }));
    }

    #[test]
    fn rejects_decompression_bomb() {
        // Craft an envelope whose declared uncompressed_len is small but whose actual zstd
        // stream expands far past it.
        let key = ArtifactKey::of_bytes(b"k");
        let huge_payload = vec![0u8; 10 * 1024 * 1024];
        let compressed = zstd::stream::encode_all(huge_payload.as_slice(), 0).unwrap();

        let mut out = Vec::new();
        out.extend_from_slice(MAGIC);
        out.extend_from_slice(&ENVELOPE_VERSION.to_le_bytes());
        out.push(ArtifactKind::ParseOutcome.to_u8());
        out.extend_from_slice(&1u32.to_le_bytes());
        out.extend_from_slice(key.as_bytes());
        out.extend_from_slice(&(compressed.len() as u64).to_le_bytes());
        // Lie about the uncompressed length: claim it's tiny.
        out.extend_from_slice(&64u64.to_le_bytes());
        out.extend_from_slice(blake3::hash(&huge_payload).as_bytes());
        out.extend_from_slice(&compressed);

        let err = decode(&out, ArtifactKind::ParseOutcome, 1, key, &limits()).unwrap_err();
        // Rejected either as a resource limit (bulk decompress capacity exceeded) or truncation
        // (declared vs actual mismatch) — both are safe typed misses that never allocate
        // unbounded memory.
        assert!(matches!(
            err,
            CacheMissReason::ResourceLimit { .. } | CacheMissReason::Truncated
        ));
    }
}
