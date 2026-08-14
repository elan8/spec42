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
            if !is_portable_path_component(value) {
                return Err(KparError::InvalidArchive(format!(
                    "project metadata field '{field}' must be a non-empty portable path component"
                )));
            }
        }
        Ok(())
    }
}

fn is_portable_path_component(value: &str) -> bool {
    if value.trim().is_empty()
        || value.ends_with(['.', ' '])
        || value.chars().any(|character| {
            character.is_control()
                || matches!(
                    character,
                    '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*'
                )
        })
        || matches!(value, "." | "..")
    {
        return false;
    }

    let base = value.split('.').next().unwrap_or_default();
    let upper = base.to_ascii_uppercase();
    !matches!(upper.as_str(), "CON" | "PRN" | "AUX" | "NUL")
        && !matches!(
            upper.as_str(),
            "COM1"
                | "COM2"
                | "COM3"
                | "COM4"
                | "COM5"
                | "COM6"
                | "COM7"
                | "COM8"
                | "COM9"
                | "LPT1"
                | "LPT2"
                | "LPT3"
                | "LPT4"
                | "LPT5"
                | "LPT6"
                | "LPT7"
                | "LPT8"
                | "LPT9"
        )
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
    fn project_identity_rejects_nonportable_components() {
        for (name, version) in [
            ("", "1.0.0"),
            ("Example", ""),
            ("../Example", "1.0.0"),
            ("Example", "release/1"),
            (".", "1.0.0"),
            ("Example", ".."),
            ("A:B", "1.0.0"),
            ("Example", "A?B"),
            ("Example\0", "1.0.0"),
            ("Example.", "1.0.0"),
            ("Example", "1.0.0 "),
            ("CON", "1.0.0"),
            ("Example", "LPT1"),
        ] {
            assert!(project(name, version).validate_identity().is_err());
        }
        assert!(project("Example Library", "1.0.0 rc 1")
            .validate_identity()
            .is_ok());
    }
}
