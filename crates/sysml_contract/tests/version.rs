//! The contract version is a value, and changing it is a visible diff.
//!
//! Every publication identity hashes this string and every serialised model records it, so a
//! change here makes every existing artefact incompatible -- correctly, but only if the change is
//! deliberate. Asserting the literal makes an accidental edit fail here, next to the constant,
//! rather than as unexplained snapshot churn in a crate that merely consumes it.

use sysml_contract::{SemanticContractVersion, SEMANTIC_CONTRACT_VERSION};

#[test]
fn the_semantic_contract_version_is_the_value_every_publication_records() {
    assert_eq!(
        SEMANTIC_CONTRACT_VERSION.as_str(),
        "standard-library-availability-v37",
        "changing the semantic contract version invalidates every publication identity and every \
         serialised model; if that is intended, update this assertion in the same commit"
    );
}

/// The newtype is transparent at the serialisation boundary and nowhere else.
#[test]
fn the_version_displays_and_compares_as_the_recorded_string() {
    assert_eq!(
        SEMANTIC_CONTRACT_VERSION.to_string(),
        SEMANTIC_CONTRACT_VERSION.as_str()
    );
    assert!(SEMANTIC_CONTRACT_VERSION == *"standard-library-availability-v37");
    let same: SemanticContractVersion = SEMANTIC_CONTRACT_VERSION;
    assert_eq!(same, SEMANTIC_CONTRACT_VERSION);
}
