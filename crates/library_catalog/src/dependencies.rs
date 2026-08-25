//! Resolution of KPAR project usages against projects already admitted by a host.
//!
//! This module deliberately does not fetch `ProjectUsage::resource`. A host must supply the
//! authoritative resource identities of locally installed or bundled projects. In particular,
//! display names, archive filenames, and install paths are not treated as resource identities.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use kpar::{Project, ProjectUsage};
use semver::{Version, VersionReq};
use serde::Serialize;

/// One installed project that may satisfy a `.project.json` usage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProjectDependencyCandidate {
    /// The exact resource identity by which projects declare this dependency.
    pub resource: String,
    pub project_name: String,
    pub version: String,
    pub package_roots: Vec<PathBuf>,
}

/// Stable, explicit result for every authored project usage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "status", rename_all = "kebab-case")]
pub enum ProjectDependencyResolution {
    Satisfied {
        resource: String,
        version_constraint: Option<String>,
        project_name: String,
        selected_version: String,
        package_roots: Vec<PathBuf>,
    },
    Unresolved {
        resource: String,
        version_constraint: Option<String>,
    },
    VersionMismatch {
        resource: String,
        version_constraint: String,
        available_versions: Vec<String>,
    },
    InvalidVersionConstraint {
        resource: String,
        version_constraint: String,
        reason: String,
    },
    Ambiguous {
        resource: String,
        version_constraint: Option<String>,
        matching_versions: Vec<String>,
    },
}

/// Resolve every authored usage without network access or implicit substitution.
///
/// A bare semantic version such as `1.0.0` means that exact version. Comparator syntax accepted
/// by `semver` (for example `>=1.0.0, <2.0.0`) is also supported. If multiple candidates with the
/// highest compatible version claim the same resource identity, resolution is ambiguous rather
/// than dependent on catalog insertion order.
pub fn resolve_project_dependencies(
    project: &Project,
    candidates: &[ProjectDependencyCandidate],
) -> Vec<ProjectDependencyResolution> {
    let mut by_resource: BTreeMap<&str, Vec<&ProjectDependencyCandidate>> = BTreeMap::new();
    for candidate in candidates {
        by_resource
            .entry(candidate.resource.as_str())
            .or_default()
            .push(candidate);
    }
    project
        .usage
        .iter()
        .map(|usage| resolve_usage(usage, by_resource.get(usage.resource.as_str())))
        .collect()
}

/// Read one project manifest and resolve its authored usages against admitted candidates.
///
/// KPAR schema decoding remains in the catalog boundary rather than leaking archive DTOs into
/// workspace hosts. This operation is local-only and never fetches a usage resource.
pub fn resolve_project_manifest_dependencies(
    manifest_path: &Path,
    candidates: &[ProjectDependencyCandidate],
) -> Result<Vec<ProjectDependencyResolution>, String> {
    let bytes = fs::read(manifest_path).map_err(|error| {
        format!(
            "Could not read project manifest {}: {error}",
            manifest_path.display()
        )
    })?;
    let project: Project = serde_json::from_slice(&bytes).map_err(|error| {
        format!(
            "Invalid project manifest {}: {error}",
            manifest_path.display()
        )
    })?;
    Ok(resolve_project_dependencies(&project, candidates))
}

fn resolve_usage(
    usage: &ProjectUsage,
    candidates: Option<&Vec<&ProjectDependencyCandidate>>,
) -> ProjectDependencyResolution {
    let constraint = usage.version_constraint.clone();
    let Some(candidates) = candidates else {
        return ProjectDependencyResolution::Unresolved {
            resource: usage.resource.clone(),
            version_constraint: constraint,
        };
    };

    let requirement = match usage.version_constraint.as_deref() {
        None => None,
        Some(raw) => match parse_requirement(raw) {
            Ok(requirement) => Some(requirement),
            Err(reason) => {
                return ProjectDependencyResolution::InvalidVersionConstraint {
                    resource: usage.resource.clone(),
                    version_constraint: raw.to_string(),
                    reason,
                }
            }
        },
    };
    let mut parsed = Vec::new();
    for candidate in candidates {
        if let Ok(version) = Version::parse(&candidate.version) {
            if requirement
                .as_ref()
                .is_none_or(|requirement| requirement.matches(&version))
            {
                parsed.push((version, *candidate));
            }
        }
    }
    if parsed.is_empty() {
        let mut available_versions: Vec<_> = candidates
            .iter()
            .map(|candidate| candidate.version.clone())
            .collect();
        available_versions.sort();
        available_versions.dedup();
        return ProjectDependencyResolution::VersionMismatch {
            resource: usage.resource.clone(),
            version_constraint: constraint.unwrap_or_else(|| "valid semantic version".into()),
            available_versions,
        };
    }
    parsed.sort_by(|left, right| right.0.cmp(&left.0));
    let selected_version = &parsed[0].0;
    let selected: Vec<_> = parsed
        .iter()
        .take_while(|(version, _)| version == selected_version)
        .collect();
    if selected.len() != 1 {
        return ProjectDependencyResolution::Ambiguous {
            resource: usage.resource.clone(),
            version_constraint: constraint,
            matching_versions: selected
                .iter()
                .map(|(_, candidate)| candidate.version.clone())
                .collect(),
        };
    }
    let candidate = selected[0].1;
    ProjectDependencyResolution::Satisfied {
        resource: usage.resource.clone(),
        version_constraint: constraint,
        project_name: candidate.project_name.clone(),
        selected_version: candidate.version.clone(),
        package_roots: candidate.package_roots.clone(),
    }
}

fn parse_requirement(raw: &str) -> Result<VersionReq, String> {
    let trimmed = raw.trim();
    if Version::parse(trimmed).is_ok() {
        VersionReq::parse(&format!("={trimmed}"))
    } else {
        VersionReq::parse(trimmed)
    }
    .map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn project(usage: Vec<ProjectUsage>) -> Project {
        Project {
            name: "model".into(),
            version: "1.0.0".into(),
            description: None,
            license: None,
            publisher: None,
            maintainer: vec![],
            website: None,
            topic: vec![],
            usage,
        }
    }

    fn usage(resource: &str, constraint: Option<&str>) -> ProjectUsage {
        ProjectUsage {
            resource: resource.into(),
            version_constraint: constraint.map(str::to_owned),
        }
    }

    fn candidate(resource: &str, version: &str) -> ProjectDependencyCandidate {
        ProjectDependencyCandidate {
            resource: resource.into(),
            project_name: "library".into(),
            version: version.into(),
            package_roots: vec![PathBuf::from("library-root")],
        }
    }

    #[test]
    fn bare_version_is_exact_and_compatible_range_selects_highest() {
        let candidates = [candidate("library", "1.0.0"), candidate("library", "1.2.0")];
        let exact = resolve_project_dependencies(
            &project(vec![usage("library", Some("1.0.0"))]),
            &candidates,
        );
        assert!(
            matches!(&exact[0], ProjectDependencyResolution::Satisfied { selected_version, .. } if selected_version == "1.0.0")
        );
        let range = resolve_project_dependencies(
            &project(vec![usage("library", Some(">=1.0.0, <2.0.0"))]),
            &candidates,
        );
        assert!(
            matches!(&range[0], ProjectDependencyResolution::Satisfied { selected_version, .. } if selected_version == "1.2.0")
        );
    }

    #[test]
    fn identity_is_exact_and_failures_remain_explicit() {
        let candidates = [candidate("https://example.test/library.kpar", "2.0.0")];
        let resolutions = resolve_project_dependencies(
            &project(vec![
                usage("library.kpar", Some("2.0.0")),
                usage("https://example.test/library.kpar", Some("1.0.0")),
                usage("https://example.test/library.kpar", Some("not a version")),
            ]),
            &candidates,
        );
        assert!(matches!(
            resolutions[0],
            ProjectDependencyResolution::Unresolved { .. }
        ));
        assert!(matches!(
            resolutions[1],
            ProjectDependencyResolution::VersionMismatch { .. }
        ));
        assert!(matches!(
            resolutions[2],
            ProjectDependencyResolution::InvalidVersionConstraint { .. }
        ));
    }

    #[test]
    fn duplicate_claim_at_selected_version_is_ambiguous() {
        let candidates = [candidate("library", "1.0.0"), candidate("library", "1.0.0")];
        let resolutions =
            resolve_project_dependencies(&project(vec![usage("library", None)]), &candidates);
        assert!(matches!(
            resolutions[0],
            ProjectDependencyResolution::Ambiguous { .. }
        ));
    }
}
