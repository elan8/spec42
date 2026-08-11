//! Typed BLAKE3 identities for the unified cache (plan §5.1).
//!
//! `Blake3Digest` is the raw 32-byte digest. `ContentDigest`, `RootDigest`, and `ArtifactKey`
//! are distinct newtypes over it so that digests computed in one identity domain can never be
//! silently substituted for another, even when the underlying bytes are identical. Domain
//! separation is achieved by mixing a stable domain tag into every hash via
//! [`CanonicalEncoder`], never by string concatenation of the input fields themselves.

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Length of the stable text form's hex payload (32 bytes -> 64 hex characters).
const HEX_LEN: usize = 64;
const TEXT_PREFIX: &str = "blake3:";

/// A raw 32-byte BLAKE3 digest with a stable `blake3:<64 lowercase hex>` text form.
///
/// Binary (non-human-readable) serde formats store the 32 bytes directly. Human-readable
/// formats (JSON, TOML, etc.) use the prefixed text form. No unprefixed hex string and no
/// SHA-256 digest is ever accepted here; callers that hold legacy SHA-256 identities must
/// convert them at a boundary this module does not provide.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Blake3Digest([u8; 32]);

/// Error returned when parsing a digest's stable text form fails.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DigestParseError {
    #[error("digest is missing the required \"blake3:\" prefix")]
    MissingPrefix,
    #[error("digest hex payload must be exactly {HEX_LEN} lowercase hex characters, got {0}")]
    WrongLength(usize),
    #[error("digest hex payload contains non-lowercase-hex characters")]
    InvalidHex,
}

impl Blake3Digest {
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn to_bytes(self) -> [u8; 32] {
        self.0
    }

    /// Renders the stable `blake3:<64 lowercase hex>` text form.
    pub fn to_text(self) -> String {
        format!("{TEXT_PREFIX}{}", hex_encode(&self.0))
    }
}

impl fmt::Debug for Blake3Digest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Blake3Digest({})", self.to_text())
    }
}

impl fmt::Display for Blake3Digest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_text())
    }
}

impl FromStr for Blake3Digest {
    type Err = DigestParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex_part = s
            .strip_prefix(TEXT_PREFIX)
            .ok_or(DigestParseError::MissingPrefix)?;
        if hex_part.len() != HEX_LEN {
            return Err(DigestParseError::WrongLength(hex_part.len()));
        }
        let mut bytes = [0u8; 32];
        hex_decode(hex_part, &mut bytes).ok_or(DigestParseError::InvalidHex)?;
        Ok(Self(bytes))
    }
}

impl Serialize for Blake3Digest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() {
            serializer.serialize_str(&self.to_text())
        } else {
            serializer.serialize_bytes(&self.0)
        }
    }
}

struct Blake3BytesVisitor;

impl<'de> serde::de::Visitor<'de> for Blake3BytesVisitor {
    type Value = Blake3Digest;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("32 raw bytes of a BLAKE3 digest")
    }

    fn visit_bytes<E: serde::de::Error>(self, v: &[u8]) -> Result<Self::Value, E> {
        let arr: [u8; 32] = v
            .try_into()
            .map_err(|_| E::custom("blake3 digest must be 32 bytes"))?;
        Ok(Blake3Digest(arr))
    }

    fn visit_byte_buf<E: serde::de::Error>(self, v: Vec<u8>) -> Result<Self::Value, E> {
        self.visit_bytes(&v)
    }
}

impl<'de> Deserialize<'de> for Blake3Digest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error;
        if deserializer.is_human_readable() {
            let s = String::deserialize(deserializer)?;
            Blake3Digest::from_str(&s).map_err(D::Error::custom)
        } else {
            deserializer.deserialize_bytes(Blake3BytesVisitor)
        }
    }
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push(HEX[(b >> 4) as usize] as char);
        out.push(HEX[(b & 0x0f) as usize] as char);
    }
    out
}

fn hex_decode(s: &str, out: &mut [u8; 32]) -> Option<()> {
    if !s.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    // Reject uppercase explicitly: the stable text form is lowercase-only.
    if s.bytes().any(|b| b.is_ascii_uppercase()) {
        return None;
    }
    let bytes = s.as_bytes();
    for i in 0..32 {
        let hi = hex_val(bytes[i * 2])?;
        let lo = hex_val(bytes[i * 2 + 1])?;
        out[i] = (hi << 4) | lo;
    }
    Some(())
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        _ => None,
    }
}

/// Domain-separated, length-prefixed canonical encoder used to build inputs to
/// [`RootDigest`]/[`ArtifactKey`] hashing. Never build a digest input by string concatenation:
/// every field is length-prefixed so that no combination of field boundaries can collide.
pub struct CanonicalEncoder {
    hasher: blake3::Hasher,
}

impl CanonicalEncoder {
    /// Starts a new canonical encoding scoped to `domain`. Distinct domains applied to
    /// identical field sequences always produce distinct digests.
    pub fn new(domain: &str) -> Self {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&(domain.len() as u64).to_le_bytes());
        hasher.update(domain.as_bytes());
        Self { hasher }
    }

    /// Appends one length-prefixed field to the canonical encoding.
    pub fn field(&mut self, bytes: &[u8]) -> &mut Self {
        self.hasher.update(&(bytes.len() as u64).to_le_bytes());
        self.hasher.update(bytes);
        self
    }

    /// Appends a length-prefixed `u64` field in a fixed little-endian encoding.
    pub fn field_u64(&mut self, value: u64) -> &mut Self {
        self.field(&value.to_le_bytes())
    }

    pub fn finish(&self) -> Blake3Digest {
        Blake3Digest(*self.hasher.finalize().as_bytes())
    }
}

macro_rules! digest_newtype {
    ($name:ident, $domain:literal) => {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(Blake3Digest);

        impl $name {
            /// Domain tag mixed into every digest of this type. Exposed so identity
            /// construction code can build a [`CanonicalEncoder`] with the same domain.
            pub const DOMAIN: &'static str = $domain;

            pub const fn from_digest(digest: Blake3Digest) -> Self {
                Self(digest)
            }

            pub const fn digest(&self) -> Blake3Digest {
                self.0
            }

            pub const fn as_bytes(&self) -> &[u8; 32] {
                self.0.as_bytes()
            }

            /// Hashes `bytes` as the sole field of this type's domain. Suitable for simple
            /// whole-content identities (e.g. raw source bytes).
            pub fn of_bytes(bytes: &[u8]) -> Self {
                let mut enc = CanonicalEncoder::new(Self::DOMAIN);
                enc.field(bytes);
                Self(enc.finish())
            }

            /// Builds this digest from a caller-populated [`CanonicalEncoder`] that was
            /// created with `Self::DOMAIN`. Use this for structured, multi-field identities.
            pub fn from_encoder(encoder: &CanonicalEncoder) -> Self {
                Self(encoder.finish())
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}({})", stringify!($name), self.0.to_text())
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(&self.0, f)
            }
        }

        impl FromStr for $name {
            type Err = DigestParseError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Blake3Digest::from_str(s).map(Self)
            }
        }
    };
}

digest_newtype!(ContentDigest, "spec42.cache.content.v1");
digest_newtype!(RootDigest, "spec42.cache.root.v1");
digest_newtype!(ArtifactKey, "spec42.cache.artifact_key.v1");

impl ArtifactKey {
    /// Two lowercase hex bytes each, used for the two-level object directory sharding
    /// (plan §7.1): `objects/<hex[0..2]>/<hex[2..4]>/<key>.s42c`.
    pub fn shard(&self) -> (String, String) {
        let hex = hex_encode(self.as_bytes());
        (hex[0..2].to_string(), hex[2..4].to_string())
    }

    /// Full lowercase hex representation of the key bytes, used as the object file stem.
    pub fn hex(&self) -> String {
        hex_encode(self.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blake3_known_vector_empty_string() {
        // Known BLAKE3 test vector: hash of the empty input.
        let digest = ContentDigest::of_bytes(b"");
        // This is a domain-separated digest (not the raw BLAKE3("") vector), so just check
        // it is stable/deterministic and matches a directly recomputed value.
        let again = ContentDigest::of_bytes(b"");
        assert_eq!(digest, again);

        // Cross-check the *raw* BLAKE3 empty-input vector against the plain hasher, to prove
        // we are linked against a spec-conformant blake3 implementation.
        let raw = blake3::hash(b"");
        assert_eq!(
            raw.to_hex().as_str(),
            "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262"
        );
    }

    #[test]
    fn text_form_round_trips() {
        let key = ArtifactKey::of_bytes(b"hello world");
        let text = key.to_string();
        assert!(text.starts_with("blake3:"));
        assert_eq!(text.len(), "blake3:".len() + 64);
        let parsed: ArtifactKey = text.parse().unwrap();
        assert_eq!(key, parsed);
    }

    #[test]
    fn rejects_unprefixed_hex() {
        let raw_hex = "a".repeat(64);
        assert_eq!(
            ArtifactKey::from_str(&raw_hex),
            Err(DigestParseError::MissingPrefix)
        );
    }

    #[test]
    fn rejects_sha256_length() {
        // SHA-256 hex is 64 chars too, but without the blake3 prefix it's still rejected;
        // a prefixed value with the wrong length (e.g. from a different digest family) is
        // rejected explicitly.
        let wrong_len = format!("blake3:{}", "a".repeat(63));
        assert_eq!(
            ArtifactKey::from_str(&wrong_len),
            Err(DigestParseError::WrongLength(63))
        );
    }

    #[test]
    fn rejects_uppercase_hex() {
        let upper = format!("blake3:{}", "A".repeat(64));
        assert_eq!(
            ArtifactKey::from_str(&upper),
            Err(DigestParseError::InvalidHex)
        );
    }

    #[test]
    fn domain_separation_across_digest_types() {
        let bytes = b"identical input bytes";
        let content = ContentDigest::of_bytes(bytes);
        let root = RootDigest::of_bytes(bytes);
        let key = ArtifactKey::of_bytes(bytes);
        assert_ne!(content.digest(), root.digest());
        assert_ne!(root.digest(), key.digest());
        assert_ne!(content.digest(), key.digest());
    }

    #[test]
    fn canonical_encoder_length_prefixing_prevents_field_boundary_collision() {
        // Without length prefixing, ("ab", "c") and ("a", "bc") would concatenate to the same
        // bytes. The canonical encoder must keep them distinct.
        let mut a = CanonicalEncoder::new(ArtifactKey::DOMAIN);
        a.field(b"ab").field(b"c");
        let mut b = CanonicalEncoder::new(ArtifactKey::DOMAIN);
        b.field(b"a").field(b"bc");
        assert_ne!(a.finish(), b.finish());
    }

    #[test]
    fn shard_uses_first_four_hex_chars() {
        let key = ArtifactKey::of_bytes(b"shard-test");
        let hex = key.hex();
        let (a, b) = key.shard();
        assert_eq!(format!("{a}{b}"), &hex[0..4]);
    }
}
