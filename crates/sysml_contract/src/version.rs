//! The version of the vocabulary itself.
//!
//! Every publication records the contract version it was produced under, and hashes it into its
//! identity, so a model built under one vocabulary can never be mistaken for a model built under
//! another. The constant therefore belongs to the crate that *defines* the vocabulary, not to the
//! authority that implements it: `sysml_resolution` cannot bump the version its own answers are
//! recorded under without editing the contract.

use core::fmt;

/// The semantic contract version every resolved publication is recorded under.
///
/// A newtype rather than a bare `&str` so that it cannot be confused with any of the other
/// version strings a publication carries -- the host artefact schema versions, the source digest,
/// the parser revision -- and so that widening it later (to a structured major/minor, say) is a
/// change to this type rather than to every site that compares a string.
///
/// `Copy` and `'static`: it names a compile-time property of the build, never per-publication
/// state.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SemanticContractVersion(&'static str);

impl SemanticContractVersion {
    /// The version as it is written into a publication identity and into serialised models.
    ///
    /// This is the serialisation boundary, and the only place the value becomes a bare string.
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

impl fmt::Display for SemanticContractVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

impl fmt::Debug for SemanticContractVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SemanticContractVersion({:?})", self.0)
    }
}

impl PartialEq<str> for SemanticContractVersion {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<SemanticContractVersion> for str {
    fn eq(&self, other: &SemanticContractVersion) -> bool {
        self == other.0
    }
}

/// The version this build of the vocabulary is.
///
/// Bumping it invalidates every publication identity and every serialised model, which is the
/// intended effect of a change in what a contract type or a derivation means. `tests/version.rs`
/// asserts the literal, so a bump is a visible diff in a test rather than a silent widening of
/// what an old artefact appears to be compatible with.
pub const SEMANTIC_CONTRACT_VERSION: SemanticContractVersion =
    SemanticContractVersion("operator-expression-arguments-v7");
