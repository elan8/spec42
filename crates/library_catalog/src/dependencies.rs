//! Resolution of KPAR project usages against projects already admitted by a host.
//!
//! This module deliberately does not fetch `ProjectUsage::resource`. A host must supply the
//! authoritative resource identities of locally installed or bundled projects. In particular,
//! display names, archive filenames, and install paths are not treated as resource identities.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use kpar::{Project, ProjectUsage};
use semver::{Version, VersionReq};
use serde::Serialize;

use crate::LibraryCatalog;
use sysml_query::StandardLibraryAvailability;

/// Provenance of one locally configured dependency candidate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProjectDependencyCandidateKind {
    StandardLibrary,
    Project,
}

/// One installed project that may satisfy a `.project.json` usage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProjectDependencyCandidate {
    /// The exact resource identity by which projects declare this dependency.
    pub resource: String,
    pub project_name: String,
    pub version: String,
    pub package_roots: Vec<PathBuf>,
    pub kind: ProjectDependencyCandidateKind,
}

/// How a satisfied authored usage contributes to project admission.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProjectDependencyKind {
    StandardLibraryConstraint,
    Project,
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
        kind: ProjectDependencyKind,
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
    StandardLibraryUnavailable {
        resource: String,
        version_constraint: Option<String>,
        availability: StandardLibraryAvailability,
    },
    StandardLibraryVersionMismatch {
        resource: String,
        version_constraint: Option<String>,
        configured_versions: Vec<String>,
    },
}

/// Complete local library admission for one project boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectDependencyAdmission {
    pub manifest_present: bool,
    pub resolutions: Vec<ProjectDependencyResolution>,
    /// Complete library baseline admitted for this project: the mandatory standard libraries plus
    /// roots selected by authored project usages.
    pub library_roots: Vec<PathBuf>,
    /// The mandatory KerML/SysML library subset of [`Self::library_roots`].
    pub standard_library_roots: Vec<PathBuf>,
    pub standard_library_availability: StandardLibraryAvailability,
    pub candidate_roots: Vec<PathBuf>,
    /// Roots selected specifically by authored project usages. This excludes mandatory roots that
    /// are admitted independently of the manifest.
    pub selected_candidate_roots: Vec<PathBuf>,
}

/// Convert the resolved standard-library component into authored, exact-version usages.
///
/// This is used when promoting a manifestless model into a reproducible project. Every standard
/// library root must have KPAR resource/version provenance; otherwise a manifest could not pin the
/// component and creation fails instead of silently dropping that input. Other catalog libraries
/// remain ordinary dependencies and are not implicitly added to a new project.
pub fn manifest_usages_for_standard_library(
    catalog: &LibraryCatalog,
) -> Result<Vec<ProjectUsage>, String> {
    let canonical = |root: &PathBuf| root.canonicalize().unwrap_or_else(|_| root.clone());
    let standard_roots = catalog
        .stdlib
        .roots
        .iter()
        .map(canonical)
        .collect::<BTreeSet<_>>();
    let standard_candidates = catalog
        .dependency_candidates
        .iter()
        .filter(|candidate| candidate.kind == ProjectDependencyCandidateKind::StandardLibrary)
        .collect::<Vec<_>>();
    let identified_roots = standard_candidates
        .iter()
        .flat_map(|candidate| candidate.package_roots.iter())
        .map(canonical)
        .collect::<BTreeSet<_>>();
    let unidentified = standard_roots
        .iter()
        .filter(|root| !identified_roots.contains(*root))
        .cloned()
        .collect::<Vec<_>>();
    if !unidentified.is_empty() {
        return Err(format!(
            "Cannot create a reproducible project manifest because these admitted library roots have no KPAR resource identity: {}",
            unidentified
                .iter()
                .map(|root| root.display().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }

    let mut identities = BTreeMap::<(String, String), usize>::new();
    for candidate in standard_candidates {
        *identities
            .entry((candidate.resource.clone(), candidate.version.clone()))
            .or_default() += 1;
    }
    let duplicates = identities
        .iter()
        .filter(|(_, count)| **count > 1)
        .map(|((resource, version), _)| format!("{resource}@{version}"))
        .collect::<Vec<_>>();
    if !duplicates.is_empty() {
        return Err(format!(
            "Cannot create a reproducible project manifest because dependency candidates are ambiguous: {}",
            duplicates.join(", ")
        ));
    }

    Ok(identities
        .into_keys()
        .map(|(resource, version)| ProjectUsage {
            resource,
            version_constraint: Some(version),
        })
        .collect())
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

/// Resolve the exact library roots admitted for one filesystem project.
///
/// Every project admits the configured KerML/SysML standard-library baseline. Manifestless
/// projects additionally retain catalog defaults. Manifest projects additionally admit roots
/// selected by authored, satisfied usages; unidentified generic paths are not implicit
/// dependencies. Any unresolved authored usage fails explicitly and never falls back to an
/// undeclared project dependency.
pub fn resolve_project_dependency_admission(
    project_root: &Path,
    catalog: &LibraryCatalog,
) -> Result<ProjectDependencyAdmission, String> {
    let manifest_path = project_root.join(kpar::PROJECT_FILE);
    if !manifest_path.is_file() {
        return Ok(ProjectDependencyAdmission {
            manifest_present: false,
            resolutions: Vec::new(),
            library_roots: catalog.package_roots.clone(),
            standard_library_roots: catalog.stdlib.roots.clone(),
            standard_library_availability: catalog.stdlib.availability,
            candidate_roots: Vec::new(),
            selected_candidate_roots: Vec::new(),
        });
    }

    let bytes = fs::read(&manifest_path).map_err(|error| {
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
    let resolutions = resolve_project_dependencies_for_catalog(&project, catalog);
    let failures = resolutions
        .iter()
        .filter(|resolution| !matches!(resolution, ProjectDependencyResolution::Satisfied { .. }))
        .collect::<Vec<_>>();
    if !failures.is_empty() {
        let states = serde_json::to_string(&failures)
            .map_err(|error| format!("Could not serialize project dependency states: {error}"))?;
        return Err(format!(
            "Project dependencies from {} were not satisfied: {states}. Spec42 does not fetch dependency resources implicitly.",
            manifest_path.display()
        ));
    }

    let canonical = |root: &PathBuf| root.canonicalize().unwrap_or_else(|_| root.clone());
    let mut candidate_roots = catalog
        .dependency_candidates
        .iter()
        .flat_map(|candidate| candidate.package_roots.iter())
        .map(canonical)
        .collect::<Vec<_>>();
    candidate_roots.sort();
    candidate_roots.dedup();
    let mut selected_candidate_roots = resolutions
        .iter()
        .filter_map(|resolution| match resolution {
            ProjectDependencyResolution::Satisfied {
                package_roots,
                kind: ProjectDependencyKind::Project,
                ..
            } => Some(package_roots),
            _ => None,
        })
        .flatten()
        .map(canonical)
        .collect::<Vec<_>>();
    selected_candidate_roots.sort();
    selected_candidate_roots.dedup();

    let standard_library_roots = catalog.stdlib.roots.clone();
    let library_roots = mandatory_and_selected_roots(
        &catalog.package_roots,
        &standard_library_roots,
        &selected_candidate_roots,
    );

    Ok(ProjectDependencyAdmission {
        manifest_present: true,
        resolutions,
        library_roots,
        standard_library_roots,
        standard_library_availability: catalog.stdlib.availability,
        candidate_roots,
        selected_candidate_roots,
    })
}

fn resolve_project_dependencies_for_catalog(
    project: &Project,
    catalog: &LibraryCatalog,
) -> Vec<ProjectDependencyResolution> {
    let mut by_resource: BTreeMap<&str, Vec<&ProjectDependencyCandidate>> = BTreeMap::new();
    for candidate in &catalog.dependency_candidates {
        by_resource
            .entry(candidate.resource.as_str())
            .or_default()
            .push(candidate);
    }
    let configured_resources = catalog
        .standard_library
        .projects
        .iter()
        .flat_map(|project| project.resources.iter().map(String::as_str))
        .collect::<BTreeSet<_>>();
    project
        .usage
        .iter()
        .map(|usage| {
            if configured_resources.contains(usage.resource.as_str()) {
                return resolve_standard_library_usage(
                    usage,
                    by_resource.get(usage.resource.as_str()),
                    catalog.stdlib.availability,
                );
            }
            resolve_usage(usage, by_resource.get(usage.resource.as_str()))
        })
        .collect()
}

fn resolve_standard_library_usage(
    usage: &ProjectUsage,
    candidates: Option<&Vec<&ProjectDependencyCandidate>>,
    availability: StandardLibraryAvailability,
) -> ProjectDependencyResolution {
    if availability != StandardLibraryAvailability::Available {
        return ProjectDependencyResolution::StandardLibraryUnavailable {
            resource: usage.resource.clone(),
            version_constraint: usage.version_constraint.clone(),
            availability,
        };
    }
    let standard = candidates
        .map(|candidates| {
            candidates
                .iter()
                .copied()
                .filter(|candidate| {
                    candidate.kind == ProjectDependencyCandidateKind::StandardLibrary
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let resolution = resolve_usage(usage, Some(&standard));
    match resolution {
        ProjectDependencyResolution::Satisfied {
            resource,
            version_constraint,
            project_name,
            selected_version,
            package_roots,
            ..
        } => ProjectDependencyResolution::Satisfied {
            resource,
            version_constraint,
            project_name,
            selected_version,
            package_roots,
            kind: ProjectDependencyKind::StandardLibraryConstraint,
        },
        ProjectDependencyResolution::VersionMismatch {
            resource,
            version_constraint,
            available_versions,
        } => ProjectDependencyResolution::StandardLibraryVersionMismatch {
            resource,
            version_constraint: Some(version_constraint),
            configured_versions: available_versions,
        },
        ProjectDependencyResolution::Unresolved {
            resource,
            version_constraint,
        } => ProjectDependencyResolution::StandardLibraryVersionMismatch {
            resource,
            version_constraint,
            configured_versions: Vec::new(),
        },
        other => other,
    }
}

/// Preserve catalog precedence while restricting a manifest project to its language baseline and
/// explicitly selected project dependencies.
fn mandatory_and_selected_roots(
    catalog_roots: &[PathBuf],
    mandatory_roots: &[PathBuf],
    selected_roots: &[PathBuf],
) -> Vec<PathBuf> {
    let admitted = mandatory_roots
        .iter()
        .chain(selected_roots)
        .map(|root| root.canonicalize().unwrap_or_else(|_| root.clone()))
        .collect::<BTreeSet<_>>();
    let mut roots = catalog_roots
        .iter()
        .filter(|root| {
            let canonical = root.canonicalize().unwrap_or_else(|_| (*root).clone());
            admitted.contains(&canonical)
        })
        .cloned()
        .collect::<Vec<_>>();
    for root in admitted {
        if !roots.iter().any(|candidate| {
            candidate
                .canonicalize()
                .unwrap_or_else(|_| candidate.clone())
                == root
        }) {
            roots.push(root);
        }
    }
    roots
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
        kind: if candidate.kind == ProjectDependencyCandidateKind::StandardLibrary {
            ProjectDependencyKind::StandardLibraryConstraint
        } else {
            ProjectDependencyKind::Project
        },
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
            kind: ProjectDependencyCandidateKind::Project,
        }
    }

    fn catalog_with_standard_library(availability: StandardLibraryAvailability) -> LibraryCatalog {
        let resource = "https://example.test/stdlib";
        let standard_root = PathBuf::from("standard-root");
        LibraryCatalog {
            root_digest: sysml_query::source::identity::RootDigest::of_bytes(b"test"),
            package_roots: vec![standard_root.clone(), PathBuf::from("alternate-root")],
            stdlib: crate::StdlibComponent {
                path: Some(standard_root.clone()),
                roots: vec![standard_root.clone()],
                source: Some("test".into()),
                used_legacy_vscode_fallback: false,
                availability,
            },
            kpar_libraries: Vec::new(),
            dependency_candidates: vec![
                ProjectDependencyCandidate {
                    resource: resource.into(),
                    project_name: "Configured baseline".into(),
                    version: "1.0.0".into(),
                    package_roots: vec![standard_root],
                    kind: ProjectDependencyCandidateKind::StandardLibrary,
                },
                ProjectDependencyCandidate {
                    resource: resource.into(),
                    project_name: "Ordinary alternate".into(),
                    version: "9.0.0".into(),
                    package_roots: vec![PathBuf::from("alternate-root")],
                    kind: ProjectDependencyCandidateKind::Project,
                },
            ],
            standard_library: crate::StandardLibraryConfig {
                projects: vec![crate::StandardLibraryProjectConfig {
                    archive: "stdlib.kpar".into(),
                    name: "Configured baseline".into(),
                    version: "1.0.0".into(),
                    resources: vec![resource.into()],
                }],
                ..crate::StandardLibraryConfig::default()
            },
            standard_library_paths: crate::standard_library_paths_from_data_dir(PathBuf::from(
                "cache",
            )),
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

    #[test]
    fn standard_library_usage_constrains_only_the_configured_baseline() {
        let catalog = catalog_with_standard_library(StandardLibraryAvailability::Available);
        let exact = resolve_project_dependencies_for_catalog(
            &project(vec![usage("https://example.test/stdlib", Some("1.0.0"))]),
            &catalog,
        );
        assert!(matches!(
            &exact[0],
            ProjectDependencyResolution::Satisfied {
                selected_version,
                kind: ProjectDependencyKind::StandardLibraryConstraint,
                ..
            } if selected_version == "1.0.0"
        ));

        let mismatch = resolve_project_dependencies_for_catalog(
            &project(vec![usage("https://example.test/stdlib", Some("9.0.0"))]),
            &catalog,
        );
        assert_eq!(
            mismatch,
            vec![
                ProjectDependencyResolution::StandardLibraryVersionMismatch {
                    resource: "https://example.test/stdlib".into(),
                    version_constraint: Some("9.0.0".into()),
                    configured_versions: vec!["1.0.0".into()],
                }
            ]
        );
    }

    #[test]
    fn authored_standard_library_usage_cannot_enable_a_disabled_baseline() {
        let catalog = catalog_with_standard_library(StandardLibraryAvailability::Disabled);
        assert_eq!(
            resolve_project_dependencies_for_catalog(
                &project(vec![usage("https://example.test/stdlib", Some("1.0.0"))]),
                &catalog,
            ),
            vec![ProjectDependencyResolution::StandardLibraryUnavailable {
                resource: "https://example.test/stdlib".into(),
                version_constraint: Some("1.0.0".into()),
                availability: StandardLibraryAvailability::Disabled,
            }]
        );
    }

    #[test]
    fn mandatory_roots_survive_an_empty_manifest_selection() {
        let mandatory = vec![PathBuf::from("kernel"), PathBuf::from("sysml")];
        let catalog = vec![
            PathBuf::from("custom"),
            PathBuf::from("sysml"),
            PathBuf::from("kernel"),
        ];

        assert_eq!(
            mandatory_and_selected_roots(&catalog, &mandatory, &[]),
            vec![PathBuf::from("sysml"), PathBuf::from("kernel")]
        );
    }

    #[test]
    fn selected_projects_are_added_without_admitting_other_catalog_roots() {
        let mandatory = vec![PathBuf::from("kernel"), PathBuf::from("sysml")];
        let selected = vec![PathBuf::from("declared")];
        let catalog = vec![
            PathBuf::from("undeclared"),
            PathBuf::from("declared"),
            PathBuf::from("sysml"),
            PathBuf::from("kernel"),
        ];

        assert_eq!(
            mandatory_and_selected_roots(&catalog, &mandatory, &selected),
            vec![
                PathBuf::from("declared"),
                PathBuf::from("sysml"),
                PathBuf::from("kernel")
            ]
        );
    }
}
