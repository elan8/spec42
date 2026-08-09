use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::error::{KparError, Result};

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

impl Project {
    /// Validate the portable identity used for default archive and extraction paths.
    pub fn validate_identity(&self) -> Result<()> {
        for (field, value) in [("name", &self.name), ("version", &self.version)] {
            if value.trim().is_empty()
                || value.contains(['/', '\\'])
                || value == "."
                || value == ".."
            {
                return Err(KparError::InvalidArchive(format!(
                    "project metadata field '{field}' must be a non-empty portable path component"
                )));
            }
        }
        Ok(())
    }
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
    pub index: BTreeMap<String, String>,
    pub created: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metamodel: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub includes_derived: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub includes_implied: Option<bool>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub checksum: BTreeMap<String, ChecksumEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChecksumEntry {
    pub value: String,
    pub algorithm: String,
}

pub const PROJECT_FILE: &str = ".project.json";
pub const META_FILE: &str = ".meta.json";

pub const SOURCE_EXTENSIONS: &[&str] = &[".sysml", ".kerml"];

#[cfg(test)]
mod tests {
    use super::*;

    fn project(name: &str, version: &str) -> Project {
        Project {
            name: name.to_string(),
            version: version.to_string(),
            description: None,
            license: None,
            publisher: None,
            maintainer: Vec::new(),
            website: None,
            topic: Vec::new(),
            usage: Vec::new(),
        }
    }

    #[test]
    fn project_identity_rejects_blank_and_path_like_components() {
        for (name, version) in [
            ("", "1.0.0"),
            ("Example", ""),
            ("../Example", "1.0.0"),
            ("Example", "release/1"),
            (".", "1.0.0"),
            ("Example", ".."),
        ] {
            assert!(project(name, version).validate_identity().is_err());
        }
    }
}
