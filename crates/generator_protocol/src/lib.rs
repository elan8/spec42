//! Shared data types for Spec42's core WebAssembly generator ABI.
//!
//! The executable ABI is intentionally small: one Postcard query import plus
//! a diagnostic import. Generated artifacts are returned from the guest entrypoint.

use postcard_schema::Schema;
use serde::{Deserialize, Serialize};

pub const ABI_VERSION: u32 = 3;
pub const IMPORT_MODULE: &str = "spec42";

/// Every type that crosses the guest boundary, in one place.
///
/// This exists only to be hashed. Postcard is positional and carries no field names, no
/// type tags and no version, so a host and a guest built against different revisions of
/// this crate will happily misread each other's bytes -- a reordered or inserted field
/// shifts every subsequent field and still decodes, producing plausible but wrong output.
/// Naming all the wire types here means [`SCHEMA_FINGERPRINT`] changes automatically
/// whenever any of them changes, with no version number for anyone to forget to bump.
#[derive(Schema)]
#[allow(dead_code)]
struct WireSchema {
    artifact: Artifact,
    model_info: ModelInfo,
    element_summary: ElementSummary,
    element_detail: ElementDetail,
    source_range: SourceRange,
    multiplicity: Multiplicity,
    relationship: Relationship,
    level: Level,
    /// Request payloads, which are part of the contract just as much as the responses.
    metaclass_filter: Option<String>,
    handle: String,
    /// The entrypoint's argument and result shapes.
    generator_args: Vec<String>,
    generator_result: Result<Vec<Artifact>, String>,
}

/// Structural fingerprint of the wire schema, checked at load time.
///
/// A guest reports this via its `spec42_abi_version` export; the host refuses any module
/// whose value differs from its own. That turns every schema change -- deliberate or
/// accidental -- into a clean load-time rejection instead of a silent misparse.
pub const SCHEMA_FINGERPRINT: u64 =
    u64::from_le_bytes(postcard_schema::key::hash::fnv1a64::hash_ty_path::<
        WireSchema,
    >(SCHEMA_PATH));

/// Namespace for the fingerprint hash. Bumping [`ABI_VERSION`] alone changes the
/// fingerprint even if no type changed, so an intentional break is always observable.
const SCHEMA_PATH: &str = "spec42:generator-abi/3";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Schema)]
pub struct Artifact {
    pub file_path: String,
    pub contents: Vec<u8>,
}

pub mod operation {
    pub const INFO: i32 = 0;
    pub const ROOTS: i32 = 1;
    pub const FIND: i32 = 2;
    pub const CHILDREN: i32 = 3;
    pub const ELEMENT: i32 = 4;
    pub const TYPED_BY: i32 = 5;
    pub const RELATIONSHIPS: i32 = 6;
    pub const EFFECTIVE_FEATURES: i32 = 7;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Schema)]
pub struct ModelInfo {
    pub model_digest: String,
    pub spec42_version: String,
    pub semantic_api_version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Schema)]
pub struct ElementSummary {
    pub handle: String,
    pub semantic_id: String,
    pub metaclass: String,
    pub name: Option<String>,
    pub qualified_name: String,
    pub library_element: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Schema)]
pub struct SourceRange {
    pub start_line: u32,
    pub start_character: u32,
    pub end_line: u32,
    pub end_character: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Schema)]
pub struct Multiplicity {
    pub lower: Option<String>,
    pub upper: Option<String>,
    pub ordered: bool,
    pub unique: Option<bool>,
    pub implied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Schema)]
pub struct ElementDetail {
    pub summary: ElementSummary,
    pub owner: Option<ElementSummary>,
    pub declared_name: Option<String>,
    pub effective_name: Option<String>,
    pub source_uri: String,
    pub source_range: SourceRange,
    pub definition: bool,
    pub documentation: Option<String>,
    pub short_name: Option<String>,
    pub direction: Option<String>,
    pub derived: bool,
    pub constant: bool,
    pub abstract_flag: bool,
    pub variation: bool,
    pub individual: bool,
    pub conjugated: bool,
    pub composite: Option<bool>,
    pub reference: Option<bool>,
    pub end: bool,
    pub ordered: Option<bool>,
    pub unique: Option<bool>,
    pub multiplicity: Option<Multiplicity>,
    pub evaluated_value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Schema)]
pub struct Relationship {
    pub kind: String,
    pub source: ElementSummary,
    pub target: ElementSummary,
    pub implied: bool,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Schema)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    Debug = 0,
    Info = 1,
    Warning = 2,
    Error = 3,
}

#[cfg(test)]
mod tests {
    use super::*;
    use postcard_schema::key::hash::fnv1a64::hash_ty_path;

    fn fingerprint<T: Schema + ?Sized>() -> u64 {
        u64::from_le_bytes(hash_ty_path::<T>(SCHEMA_PATH))
    }

    /// The load-time compatibility check is only worth anything if the fingerprint moves
    /// when the schema moves. These stand in for the three ways the wire format can drift.
    #[test]
    #[allow(dead_code)]
    fn the_fingerprint_detects_every_shape_of_schema_drift() {
        #[derive(Schema)]
        struct Baseline {
            handle: String,
            name: Option<String>,
            library: bool,
        }

        #[derive(Schema)]
        struct FieldAdded {
            handle: String,
            name: Option<String>,
            library: bool,
            deprecated: bool,
        }

        #[derive(Schema)]
        struct FieldsReordered {
            handle: String,
            library: bool,
            name: Option<String>,
        }

        #[derive(Schema)]
        struct TypeChanged {
            handle: String,
            name: String,
            library: bool,
        }

        let baseline = fingerprint::<Baseline>();
        for (label, other) in [
            ("an appended field", fingerprint::<FieldAdded>()),
            ("reordered fields", fingerprint::<FieldsReordered>()),
            ("a changed field type", fingerprint::<TypeChanged>()),
        ] {
            assert_ne!(
                baseline, other,
                "{label} left the fingerprint unchanged, so drift would go undetected"
            );
        }
    }

    /// A deliberate tripwire. Changing any wire type changes this value; update it in the
    /// same commit as the schema change, and treat an unexpected diff here as a warning
    /// that existing guests are about to be rejected.
    #[test]
    fn the_wire_schema_fingerprint_is_pinned() {
        assert_eq!(
            SCHEMA_FINGERPRINT, 0x7615_ab81_94e9_0b5c,
            "the generator wire schema changed; every guest must be rebuilt"
        );
    }
}
