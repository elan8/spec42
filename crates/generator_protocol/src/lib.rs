//! Shared data types for Spec42's core WebAssembly generator ABI.
//!
//! The executable ABI is intentionally small: one Postcard query import plus
//! dedicated artifact and diagnostic imports. No custom metadata is required.

use serde::{Deserialize, Serialize};

pub const ABI_VERSION: u32 = 1;
pub const IMPORT_MODULE: &str = "spec42";

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelInfo {
    pub model_digest: String,
    pub spec42_version: String,
    pub semantic_api_version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ElementSummary {
    pub handle: String,
    pub semantic_id: String,
    pub metaclass: String,
    pub name: Option<String>,
    pub qualified_name: String,
    pub library_element: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceRange {
    pub start_line: u32,
    pub start_character: u32,
    pub end_line: u32,
    pub end_character: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Multiplicity {
    pub lower: Option<String>,
    pub upper: Option<String>,
    pub ordered: bool,
    pub unique: Option<bool>,
    pub implied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Relationship {
    pub kind: String,
    pub source: ElementSummary,
    pub target: ElementSummary,
    pub implied: bool,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    Debug = 0,
    Info = 1,
    Warning = 2,
    Error = 3,
}
