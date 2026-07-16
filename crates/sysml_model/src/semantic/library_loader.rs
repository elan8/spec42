//! Import- and typing-scoped loading of SysML/KerML library files from configured roots.
//!
//! Library files are never merged into the semantic graph by default. They enter through:
//!
//! 1. Collect `import` targets and (optionally) type/specialization references from workspace sources.
//! 2. Walk the transitive closure over imports and typing references.
//! 3. When a package name is already declared in the workspace, that declaration
//!    satisfies the import and the walker continues through the workspace package body.
//! 4. Only otherwise-unresolved package names are loaded from library roots.
//!
//! Unit catalogs and optional `sysml` namespace bootstrap are deliberate exceptions to step 4.

use std::collections::{HashMap, HashSet, VecDeque};
use std::path::PathBuf;

use sysml_v2_parser::ast::{
    AttributeBody, AttributeBodyElement, AttributeDef, AttributeUsage, Identification, Import,
    ItemUsage, LibraryPackage, MetadataDef, MetadataUsage, Package, PackageBody,
    PackageBodyElement, PartDef, PartDefBody, PartDefBodyElement, PartUsage, PartUsageBody,
    PartUsageBodyElement, PortBody, PortBodyElement, PortDef, PortDefBody, PortDefBodyElement,
    PortUsage, RefDecl, RootElement,
};
use sysml_v2_parser::{Node, RootNamespace as ParsedRoot};
use walkdir::WalkDir;

/// Workspace file path and text used to seed the library import closure.
#[derive(Debug, Clone)]
pub struct WorkspaceSource<'a> {
    pub path: &'a str,
    pub content: &'a str,
}

/// Loaded library file (path relative to its root, UTF-8 content).
#[derive(Debug, Clone)]
pub struct LoadedLibraryFile {
    pub root: String,
    pub path: String,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct PackageKey(String);

#[derive(Debug, Clone)]
pub(crate) struct IndexedFile {
    pub(crate) root: String,
    pub(crate) path: String,
}

/// Options for [`resolve_library_closure`].
#[derive(Debug, Clone)]
pub struct LibraryClosureOptions {
    /// When workspace imports `sysml::*` (or `sysml`), load packages under `sysml.library` / `kerml` roots.
    pub bootstrap_sysml_namespace: bool,
    /// Seed closure from part/port/attribute type references and `:>` specializations in workspace text.
    pub bootstrap_typing_references: bool,
}

impl Default for LibraryClosureOptions {
    fn default() -> Self {
        Self {
            bootstrap_sysml_namespace: true,
            bootstrap_typing_references: true,
        }
    }
}

/// Build the set of library files required by `workspace` imports (transitive closure).
pub fn resolve_library_closure(
    workspace: &[WorkspaceSource<'_>],
    library_roots: &[String],
    options: &LibraryClosureOptions,
) -> Result<Vec<LoadedLibraryFile>, String> {
    if library_roots.is_empty() {
        return Ok(Vec::new());
    }
    let index = build_package_index(library_roots)?;
    let workspace_declared_packages = workspace_declared_packages(workspace);
    let mut seeds = HashSet::<PackageKey>::new();
    let mut wants_sysml_bootstrap = false;
    for source in workspace {
        if options.bootstrap_sysml_namespace && source.content.contains("SysML::") {
            seeds.insert(PackageKey("SysML".to_string()));
        }
        for target in collect_import_targets_from_content(source.content) {
            if options.bootstrap_sysml_namespace
                && (target == "sysml" || target.starts_with("sysml::"))
            {
                wants_sysml_bootstrap = true;
            }
            for key in package_keys_for_import_target(&target) {
                seeds.insert(PackageKey(key));
            }
        }
        if options.bootstrap_typing_references {
            for target in collect_type_reference_targets_from_content(source.content) {
                for key in package_keys_for_import_target(&target) {
                    seeds.insert(PackageKey(key));
                }
            }
        }
        if workspace_contains_unit_literal(source.content) {
            for pkg in QUANTITY_UNIT_CLOSURE_PACKAGES {
                seeds.insert(PackageKey(pkg.to_string()));
            }
        }
    }
    if wants_sysml_bootstrap {
        for (key, entries) in &index.packages {
            if entries
                .iter()
                .any(|entry| is_stdlib_slice_root(&entry.root))
            {
                seeds.insert(key.clone());
            }
        }
    }
    let mut loaded_paths = HashSet::<(String, String)>::new();
    let mut files = Vec::<LoadedLibraryFile>::new();
    let mut visited_packages = HashSet::<PackageKey>::new();
    let mut queue: VecDeque<PackageKey> = seeds.into_iter().collect();
    enqueue_imports_from_workspace_packages(
        workspace,
        &workspace_declared_packages,
        options,
        &mut queue,
    );
    while let Some(pkg) = queue.pop_front() {
        if !visited_packages.insert(pkg.clone()) {
            continue;
        }
        if workspace_declared_packages.contains(&pkg) {
            // Import target is satisfied by a workspace package: follow its imports only.
            enqueue_imports_from_workspace_package(workspace, &pkg, options, &mut queue);
            continue;
        }
        let Some(indexed) = index.packages.get(&pkg) else {
            continue;
        };
        for entry in indexed {
            let key = (entry.root.clone(), entry.path.clone());
            if !loaded_paths.insert(key.clone()) {
                continue;
            }
            let full_path = PathBuf::from(&entry.root).join(&entry.path);
            let content = std::fs::read_to_string(&full_path).map_err(|err| {
                format!("failed to read library file {}: {err}", full_path.display())
            })?;
            enqueue_closure_targets_from_content(&content, options, &mut queue);
            files.push(LoadedLibraryFile {
                root: entry.root.clone(),
                path: entry.path.clone(),
                content,
            });
        }
    }
    for unit in &index.unit_catalog_files {
        let key = (unit.root.clone(), unit.path.clone());
        if !loaded_paths.insert(key.clone()) {
            continue;
        }
        let full_path = PathBuf::from(&unit.root).join(&unit.path);
        let content = std::fs::read_to_string(&full_path).map_err(|err| {
            format!(
                "failed to read library unit catalog {}: {err}",
                full_path.display()
            )
        })?;
        files.push(LoadedLibraryFile {
            root: unit.root.clone(),
            path: unit.path.clone(),
            content,
        });
    }
    files.sort_by(|a, b| {
        (a.root.as_str(), a.path.as_str()).cmp(&(b.root.as_str(), b.path.as_str()))
    });
    Ok(files)
}

mod package_scan;
mod type_refs;
pub use package_scan::*;
pub(crate) use type_refs::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn closure_loads_transitive_library_package() {
        let temp = tempfile::tempdir().expect("tempdir");
        let lib = temp.path().join("lib");
        fs::create_dir_all(&lib).expect("lib dir");
        fs::write(
            lib.join("Base.sysml"),
            "package Base { attribute def Name; }",
        )
        .expect("base");
        fs::write(
            lib.join("Consumer.sysml"),
            "package Demo { import Base::*; part def P { attribute n : Name; } }",
        )
        .expect("consumer");
        let workspace = [WorkspaceSource {
            path: "model.sysml",
            content: "package App { import Demo::*; part def AppPart; }",
        }];
        let roots = vec![lib.to_string_lossy().replace('\\', "/")];
        let loaded = resolve_library_closure(&workspace, &roots, &LibraryClosureOptions::default())
            .expect("closure");
        let paths: Vec<_> = loaded.iter().map(|f| f.path.as_str()).collect();
        assert!(paths.iter().any(|p| p.contains("Base.sysml")), "{paths:?}");
        assert!(
            paths.iter().any(|p| p.contains("Consumer.sysml")),
            "{paths:?}"
        );
    }

    #[test]
    fn closure_indexes_standard_library_package_declarations() {
        let temp = tempfile::tempdir().expect("tempdir");
        let lib = temp.path().join("lib");
        fs::create_dir_all(&lib).expect("lib dir");
        fs::write(
            lib.join("ScalarValues.sysml"),
            "standard library package ScalarValues { attribute def Real; }",
        )
        .expect("scalar values");
        let workspace = [WorkspaceSource {
            path: "loose.sysml",
            content: "package P { private import ScalarValues::Real; attribute x : Real; }",
        }];
        let roots = vec![lib.to_string_lossy().replace('\\', "/")];
        let loaded = resolve_library_closure(&workspace, &roots, &LibraryClosureOptions::default())
            .expect("closure");
        assert!(
            loaded.iter().any(|f| f.path.contains("ScalarValues.sysml")),
            "expected ScalarValues.sysml in closure, got {:?}",
            loaded.iter().map(|f| &f.path).collect::<Vec<_>>()
        );
    }

    #[test]
    fn closure_loads_sysml_package_when_workspace_references_sysml_qualified_names() {
        let temp = tempfile::tempdir().expect("tempdir");
        let lib = temp.path().join("sysml.library");
        fs::create_dir_all(&lib).expect("lib dir");
        fs::write(
            lib.join("SysML.sysml"),
            r#"standard library package SysML {
  package Systems {
    metadata def RequirementUsage;
    metadata def Usage;
  }
}"#,
        )
        .expect("sysml package");
        fs::write(
            lib.join("ScalarValues.sysml"),
            "standard library package ScalarValues { attribute def Real; }",
        )
        .expect("scalar values");
        let workspace = [WorkspaceSource {
            path: "RequirementMetadata.sysml",
            content: r#"package RequirementMetadata {
  metadata def RequirementRole {
    :> annotatedElement : SysML::RequirementUsage;
  }
}"#,
        }];
        let roots = vec![lib.to_string_lossy().replace('\\', "/")];
        let loaded = resolve_library_closure(&workspace, &roots, &LibraryClosureOptions::default())
            .expect("closure");
        assert!(
            loaded.iter().any(|f| f.path.contains("SysML.sysml")),
            "expected SysML.sysml in closure for SysML:: references, got {:?}",
            loaded.iter().map(|f| &f.path).collect::<Vec<_>>()
        );
    }

    #[test]
    fn closure_skips_library_package_shadowed_by_workspace() {
        let temp = tempfile::tempdir().expect("tempdir");
        let lib = temp.path().join("lib");
        fs::create_dir_all(&lib).expect("lib dir");
        fs::write(
            lib.join("WebShopExample.sysml"),
            "package WebShopExample { part def LibraryOnlyPart; }",
        )
        .expect("library duplicate");
        fs::write(
            lib.join("ScalarValues.sysml"),
            "standard library package ScalarValues { attribute def Real; }",
        )
        .expect("scalar values");
        let workspace = [
            WorkspaceSource {
                path: "webshop.sysml",
                content: r#"
package WebShopExample {
    private import ScalarValues::Real;
    part def WorkspaceOnlyPart;
}
"#,
            },
            WorkspaceSource {
                path: "Views.sysml",
                content: r#"
package Views {
    import WebShopExample::*;
    view structure {
        expose WebShopExample::WorkspaceOnlyPart;
    }
}
"#,
            },
        ];
        let roots = vec![lib.to_string_lossy().replace('\\', "/")];
        let loaded = resolve_library_closure(&workspace, &roots, &LibraryClosureOptions::default())
            .expect("closure");
        let paths: Vec<_> = loaded.iter().map(|f| f.path.as_str()).collect();
        assert!(
            !paths.iter().any(|p| p.contains("WebShopExample.sysml")),
            "workspace-defined package must not load library duplicate, got {paths:?}"
        );
        assert!(
            paths.iter().any(|p| p.contains("ScalarValues.sysml")),
            "transitive workspace import should still load ScalarValues, got {paths:?}"
        );
    }

    #[test]
    fn closure_loads_unit_catalogs_independent_of_quantity_imports() {
        let temp = tempfile::tempdir().expect("tempdir");
        let lib = temp.path().join("lib");
        let units_dir = lib.join("Quantities and Units");
        fs::create_dir_all(&units_dir).expect("units dir");
        fs::write(lib.join("Base.sysml"), "package Base { part def Y; }").expect("base");
        fs::write(
            units_dir.join("units.sysml"),
            "package Units { attribute <kg> kilogram : MassUnit; }",
        )
        .expect("units catalog");
        let workspace = [WorkspaceSource {
            path: "model.sysml",
            content: "package App { import Base::*; }",
        }];
        let roots = vec![lib.to_string_lossy().replace('\\', "/")];
        let loaded = resolve_library_closure(&workspace, &roots, &LibraryClosureOptions::default())
            .expect("closure");
        assert!(
            loaded.iter().any(|f| f.path.contains("units.sysml")),
            "unit catalogs should load with the library closure, got {:?}",
            loaded.iter().map(|f| &f.path).collect::<Vec<_>>()
        );
    }

    #[test]
    fn closure_detects_unit_catalogs_by_content() {
        let temp = tempfile::tempdir().expect("tempdir");
        let lib = temp.path().join("lib");
        fs::create_dir_all(&lib).expect("lib dir");
        fs::write(lib.join("Base.sysml"), "package Base { part def Y; }").expect("base");
        fs::write(
            lib.join("Measurements.sysml"),
            "package Measurements { attribute <widget> widget : WidgetUnit; }",
        )
        .expect("measurements catalog");
        let workspace = [WorkspaceSource {
            path: "model.sysml",
            content: "package App { import Base::*; }",
        }];
        let roots = vec![lib.to_string_lossy().replace('\\', "/")];
        let loaded = resolve_library_closure(&workspace, &roots, &LibraryClosureOptions::default())
            .expect("closure");
        assert!(
            loaded.iter().any(|f| f.path.contains("Measurements.sysml")),
            "unit catalog should be detected by unit definitions, got {:?}",
            loaded.iter().map(|f| &f.path).collect::<Vec<_>>()
        );
    }

    #[test]
    fn unit_catalog_path_hint_recognizes_kpar_quantity_roots() {
        assert!(is_unit_catalog_path_hint(
            "c:/data/stdlib/quantities_and_units_library-1.0.0/si.sysml",
            "Quantities_and_Units_Library-1.0.0/SI.sysml"
        ));
        assert!(is_unit_catalog_path_hint(
            "c:/data/stdlib/qudv-1.0.0/uscustomaryunits.sysml",
            "QUDV-1.0.0/USCustomaryUnits.sysml"
        ));
    }

    #[test]
    fn closure_omits_unreferenced_library_file() {
        let temp = tempfile::tempdir().expect("tempdir");
        let lib = temp.path().join("lib");
        fs::create_dir_all(&lib).expect("lib dir");
        fs::write(lib.join("Unused.sysml"), "package Unused { part def X; }").expect("unused");
        fs::write(lib.join("Base.sysml"), "package Base { part def Y; }").expect("base");
        let workspace = [WorkspaceSource {
            path: "model.sysml",
            content: "package App { import Base::*; }",
        }];
        let roots = vec![lib.to_string_lossy().replace('\\', "/")];
        let loaded = resolve_library_closure(&workspace, &roots, &LibraryClosureOptions::default())
            .expect("closure");
        assert!(
            !loaded.iter().any(|f| f.path.contains("Unused.sysml")),
            "unused library file should not load"
        );
    }

    #[test]
    fn closure_loads_library_package_for_qualified_part_type_without_import() {
        let temp = tempfile::tempdir().expect("tempdir");
        let lib = temp.path().join("lib");
        fs::create_dir_all(&lib).expect("lib dir");
        fs::write(
            lib.join("Domain.sysml"),
            "package Domain { part def Robot { part motor; } }",
        )
        .expect("domain");
        let workspace = [WorkspaceSource {
            path: "model.sysml",
            content: "package App { part app : Domain::Robot; }",
        }];
        let roots = vec![lib.to_string_lossy().replace('\\', "/")];
        let loaded = resolve_library_closure(&workspace, &roots, &LibraryClosureOptions::default())
            .expect("closure");
        assert!(
            loaded.iter().any(|f| f.path.contains("Domain.sysml")),
            "expected Domain.sysml for qualified part type, got {:?}",
            loaded.iter().map(|f| &f.path).collect::<Vec<_>>()
        );
    }

    #[test]
    fn closure_loads_transitive_specialization_packages() {
        let temp = tempfile::tempdir().expect("tempdir");
        let lib = temp.path().join("lib");
        fs::create_dir_all(&lib).expect("lib dir");
        fs::write(
            lib.join("OtherPkg.sysml"),
            "package OtherPkg { part def Base { attribute x; } }",
        )
        .expect("other");
        fs::write(
            lib.join("Domain.sysml"),
            "package Domain { part def Robot :> OtherPkg::Base { part motor; } }",
        )
        .expect("domain");
        let workspace = [WorkspaceSource {
            path: "model.sysml",
            content: "package App { part app : Domain::Robot; }",
        }];
        let roots = vec![lib.to_string_lossy().replace('\\', "/")];
        let loaded = resolve_library_closure(&workspace, &roots, &LibraryClosureOptions::default())
            .expect("closure");
        let paths: Vec<_> = loaded.iter().map(|f| f.path.as_str()).collect();
        assert!(
            paths.iter().any(|p| p.contains("Domain.sysml")),
            "expected Domain.sysml, got {paths:?}"
        );
        assert!(
            paths.iter().any(|p| p.contains("OtherPkg.sysml")),
            "expected OtherPkg.sysml via specializes closure, got {paths:?}"
        );
    }

    #[test]
    fn closure_typing_reference_bootstrap_can_be_disabled() {
        let temp = tempfile::tempdir().expect("tempdir");
        let lib = temp.path().join("lib");
        fs::create_dir_all(&lib).expect("lib dir");
        fs::write(
            lib.join("Domain.sysml"),
            "package Domain { part def Robot { part motor; } }",
        )
        .expect("domain");
        let workspace = [WorkspaceSource {
            path: "model.sysml",
            content: "package App { part app : Domain::Robot; }",
        }];
        let roots = vec![lib.to_string_lossy().replace('\\', "/")];
        let options = LibraryClosureOptions {
            bootstrap_sysml_namespace: true,
            bootstrap_typing_references: false,
        };
        let loaded = resolve_library_closure(&workspace, &roots, &options).expect("closure");
        assert!(
            !loaded.iter().any(|f| f.path.contains("Domain.sysml")),
            "typing bootstrap disabled should not load Domain, got {:?}",
            loaded.iter().map(|f| &f.path).collect::<Vec<_>>()
        );
    }

    #[test]
    fn closure_loads_metaobjects_when_workspace_imports_semantic_metadata() {
        let temp = tempfile::tempdir().expect("tempdir");
        let lib = temp.path().join("sysml.library");
        fs::create_dir_all(&lib).expect("lib dir");
        fs::write(
            lib.join("Metaobjects.kerml"),
            r#"standard library package Metaobjects {
  abstract metaclass SemanticMetadata {
    feature baseType;
  }
}"#,
        )
        .expect("metaobjects");
        fs::write(
            lib.join("SysML.sysml"),
            r#"standard library package SysML {
  package Systems {
    metadata def Usage;
  }
}"#,
        )
        .expect("sysml");
        let workspace = [WorkspaceSource {
            path: "Profile.sysml",
            content: r#"package Profile {
  private import Metaobjects::SemanticMetadata;
  metadata def Role :> SemanticMetadata {
    :>> baseType = checks meta SysML::Usage;
  }
}"#,
        }];
        let roots = vec![lib.to_string_lossy().replace('\\', "/")];
        let loaded = resolve_library_closure(&workspace, &roots, &LibraryClosureOptions::default())
            .expect("closure");
        assert!(
            loaded.iter().any(|f| f.path.contains("Metaobjects")),
            "expected Metaobjects in closure, got {:?}",
            loaded.iter().map(|f| &f.path).collect::<Vec<_>>()
        );
        assert!(
            loaded.iter().any(|f| f.path.contains("SysML.sysml")),
            "expected SysML.sysml in closure for SysML::Usage, got {:?}",
            loaded.iter().map(|f| &f.path).collect::<Vec<_>>()
        );
    }
}
