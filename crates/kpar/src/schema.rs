use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// KerML interchange project metadata (`.project.json`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub name: String,
    pub version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub maintainer: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topic: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub usage: Vec<ProjectUsage>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProjectUsage {
    pub resource: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_constraint: Option<String>,
}

/// KerML interchange archive metadata (`.meta.json`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    /// Maps logical source path to archive entry path (identical for Spec42 packs).
    pub index: HashMap<String, String>,
    pub created: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metamodel: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub includes_derived: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub includes_implied: Option<bool>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub checksum: HashMap<String, ChecksumEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChecksumEntry {
    pub value: String,
    pub algorithm: String,
}

pub const PROJECT_FILE: &str = ".project.json";
pub const META_FILE: &str = ".meta.json";

pub const SOURCE_EXTENSIONS: &[&str] = &[".sysml", ".kerml"];
