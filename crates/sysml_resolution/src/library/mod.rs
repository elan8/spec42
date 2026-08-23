//! Library-closure resolution: which files under the configured library roots a workspace needs.
//!
//! Library files are never admitted by default. The workspace's imports, typing references,
//! declared packages and unit literals seed a transitive closure over a package index of the
//! roots; workspace declarations take precedence over library packages of the same name. Every
//! fact comes from the syntax authority's parsed trees, the index is memoised by the listing's
//! digests, and the files are read through the source authority — nothing here scans text.

use std::collections::{HashMap, HashSet, VecDeque};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use sysml_source::{SourceAuthority, SourceDocument, SourceError, SourceKind};

use crate::syntax::{ParsedSource, SyntaxAuthority, SyntaxClosureFacts};

/// Packages that must accompany any workspace that writes a value-with-unit literal.
const QUANTITY_UNIT_CLOSURE_PACKAGES: &[&str] = &[
    "Measurement",
    "ISQ",
    "ISQBase",
    "ISQSpaceTime",
    "ISQMechanics",
    "ISQElectromagnetism",
    "ISQThermodynamics",
    "SI",
    "SIPrefixes",
    "USCustomaryUnits",
];

/// One configured library root and the provenance its files carry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LibraryRoot {
    pub path: PathBuf,
    pub kind: SourceKind,
}

/// Options for [`LibraryClosureAuthority::resolve`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LibraryClosureOptions {
    /// When the workspace imports `sysml::*` (or `sysml`), admit every package under a
    /// standard-library root.
    pub bootstrap_sysml_namespace: bool,
    /// Seed the closure from typing references and specialisations, not only imports.
    pub bootstrap_typing_references: bool,
    /// Packages that seed the closure whether or not the workspace references them.
    pub seed_packages: Vec<String>,
}

impl Default for LibraryClosureOptions {
    fn default() -> Self {
        Self {
            bootstrap_sysml_namespace: true,
            bootstrap_typing_references: true,
            seed_packages: Vec::new(),
        }
    }
}

/// The resolved closure: the library documents a workspace needs, and the workspace facts that
/// produced it (a cache key for anything derived from the closure).
#[derive(Debug, Clone)]
pub struct LibraryClosure {
    pub documents: Vec<SourceDocument>,
    pub signature: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PackageKey(String);

#[derive(Debug, Clone)]
struct IndexedDocument {
    document: SourceDocument,
    parsed: ParsedSource,
}

#[derive(Debug, Default)]
struct PackageIndex {
    packages: HashMap<PackageKey, Vec<IndexedDocument>>,
    unit_catalogs: Vec<IndexedDocument>,
    standard_packages: HashSet<PackageKey>,
}

/// The authority over library closure. One per host; the package index is rebuilt only when the
/// listing under the roots changes.
#[derive(Debug)]
pub struct LibraryClosureAuthority {
    source: Arc<SourceAuthority>,
    syntax: Arc<SyntaxAuthority>,
    index: Mutex<Option<(blake3::Hash, Arc<PackageIndex>)>>,
}

impl LibraryClosureAuthority {
    pub fn new(source: Arc<SourceAuthority>, syntax: Arc<SyntaxAuthority>) -> Self {
        Self {
            source,
            syntax,
            index: Mutex::new(None),
        }
    }

    /// A stable signature of the workspace facts that seed closure resolution.
    pub fn seed_signature(
        &self,
        workspace: &[ParsedSource],
        options: &LibraryClosureOptions,
    ) -> Vec<String> {
        let facts = workspace
            .iter()
            .map(ParsedSource::closure_facts)
            .collect::<Vec<_>>();
        seed_signature(&facts, options)
    }

    /// The library documents `workspace` needs from `roots`.
    pub fn resolve(
        &self,
        workspace: &[ParsedSource],
        roots: &[LibraryRoot],
        options: &LibraryClosureOptions,
    ) -> Result<LibraryClosure, SourceError> {
        let facts = workspace
            .iter()
            .map(ParsedSource::closure_facts)
            .collect::<Vec<_>>();
        let signature = seed_signature(&facts, options);
        if roots.is_empty() {
            return Ok(LibraryClosure {
                documents: Vec::new(),
                signature,
            });
        }
        let index = self.package_index(roots)?;
        let documents = resolve_closure(&index, &facts, options);
        Ok(LibraryClosure {
            documents,
            signature,
        })
    }

    fn package_index(&self, roots: &[LibraryRoot]) -> Result<Arc<PackageIndex>, SourceError> {
        let mut listed = Vec::new();
        for root in roots {
            let report = self
                .source
                .list(std::slice::from_ref(&root.path), root.kind)?;
            listed.extend(report.documents);
        }
        listed.sort_by(|left, right| left.uri().as_str().cmp(right.uri().as_str()));
        let key = listing_key(&listed);
        if let Some((cached_key, index)) = self.index.lock().unwrap().as_ref() {
            if *cached_key == key {
                return Ok(Arc::clone(index));
            }
        }
        let index = Arc::new(self.build_index(listed));
        *self.index.lock().unwrap() = Some((key, Arc::clone(&index)));
        Ok(index)
    }

    fn build_index(&self, listed: Vec<SourceDocument>) -> PackageIndex {
        use rayon::prelude::*;
        let syntax = Arc::clone(&self.syntax);
        let parsed = listed
            .into_par_iter()
            .map(|document| {
                let parsed = syntax.parse(&document);
                IndexedDocument { document, parsed }
            })
            .collect::<Vec<_>>();
        let mut index = PackageIndex::default();
        for entry in parsed {
            let facts = entry.parsed.closure_facts();
            let path_hint = entry.document.path_hint().unwrap_or("");
            if is_unit_catalog_path(entry.document.uri().path(), path_hint)
                || facts.declares_unit_definitions
            {
                index.unit_catalogs.push(entry.clone());
            }
            for package in facts.declared_packages {
                let key = PackageKey(package);
                if entry.document.kind() == SourceKind::StandardLibrary {
                    index.standard_packages.insert(key.clone());
                }
                index.packages.entry(key).or_default().push(entry.clone());
            }
        }
        index
    }
}

fn listing_key(listed: &[SourceDocument]) -> blake3::Hash {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"spec42-library-listing-v1\0");
    for document in listed {
        let uri = document.uri().as_str().as_bytes();
        hasher.update(&(uri.len() as u64).to_le_bytes());
        hasher.update(uri);
        hasher.update(&[document.kind() as u8]);
        hasher.update(document.digest().as_bytes());
    }
    hasher.finalize()
}

fn references_sysml_namespace(facts: &SyntaxClosureFacts) -> bool {
    facts
        .import_targets
        .iter()
        .chain(facts.type_reference_targets.iter())
        .any(|target| target == "SysML" || target.starts_with("SysML::"))
}

fn seed_signature(facts: &[SyntaxClosureFacts], options: &LibraryClosureOptions) -> Vec<String> {
    let mut signature = Vec::new();
    signature.push(format!(
        "option:bootstrap-sysml={}",
        options.bootstrap_sysml_namespace
    ));
    signature.push(format!(
        "option:bootstrap-typing={}",
        options.bootstrap_typing_references
    ));
    signature.extend(
        options
            .seed_packages
            .iter()
            .map(|package| format!("seed:{package}")),
    );
    for facts in facts {
        if options.bootstrap_sysml_namespace && references_sysml_namespace(facts) {
            signature.push("workspace:sysml-namespace".to_string());
        }
        signature.extend(
            facts
                .import_targets
                .iter()
                .map(|target| format!("import:{target}")),
        );
        if options.bootstrap_typing_references {
            signature.extend(
                facts
                    .type_reference_targets
                    .iter()
                    .map(|target| format!("type:{target}")),
            );
        }
        signature.extend(
            facts
                .declared_packages
                .iter()
                .map(|package| format!("workspace-package:{package}")),
        );
        if facts.uses_unit_literals {
            signature.push("workspace:unit-literal".to_string());
        }
    }
    signature.sort_unstable();
    signature.dedup();
    signature
}

fn resolve_closure(
    index: &PackageIndex,
    facts: &[SyntaxClosureFacts],
    options: &LibraryClosureOptions,
) -> Vec<SourceDocument> {
    let workspace_packages: HashSet<PackageKey> = facts
        .iter()
        .flat_map(|facts| facts.declared_packages.iter().cloned().map(PackageKey))
        .collect();

    let mut seeds = HashSet::<PackageKey>::new();
    seeds.extend(options.seed_packages.iter().cloned().map(PackageKey));
    let mut wants_sysml_bootstrap = false;
    for facts in facts {
        if options.bootstrap_sysml_namespace && references_sysml_namespace(facts) {
            seeds.insert(PackageKey("SysML".to_string()));
        }
        for target in &facts.import_targets {
            if options.bootstrap_sysml_namespace
                && (target == "sysml" || target.starts_with("sysml::"))
            {
                wants_sysml_bootstrap = true;
            }
            seeds.extend(package_keys_for_import_target(target).map(PackageKey));
        }
        if options.bootstrap_typing_references {
            for target in &facts.type_reference_targets {
                seeds.extend(package_keys_for_import_target(target).map(PackageKey));
            }
        }
        if facts.uses_unit_literals {
            seeds.extend(
                QUANTITY_UNIT_CLOSURE_PACKAGES
                    .iter()
                    .map(|package| PackageKey((*package).to_string())),
            );
        }
    }
    if wants_sysml_bootstrap {
        seeds.extend(index.standard_packages.iter().cloned());
    }
    // The packages the resolver's generated library rules anchor into (`Parts::parts`,
    // `Items::Item`, `Views::views`, ...). No workspace imports them, yet every implied
    // specialization resolves against them, so they are admitted whenever a standard-library
    // root provides them — including when a workspace package shares the name, because the
    // resolver looks anchors up by standard-library role, never by bare name.
    let anchor_packages: HashSet<PackageKey> = crate::resolve::implied::library_anchor_packages()
        .into_iter()
        .map(|package| PackageKey(package.to_string()))
        .filter(|package| index.standard_packages.contains(package))
        .collect();
    seeds.extend(anchor_packages.iter().cloned());

    let mut queue: VecDeque<PackageKey> = seeds.into_iter().collect();
    // Every workspace package's own imports and references are followed as well: an import
    // satisfied by a workspace package still pulls in what that package needs.
    for package in &workspace_packages {
        enqueue_workspace_package(facts, package, options, &mut queue);
    }

    let mut visited = HashSet::<PackageKey>::new();
    let mut admitted = HashSet::<String>::new();
    let mut documents = Vec::<SourceDocument>::new();
    while let Some(package) = queue.pop_front() {
        if !visited.insert(package.clone()) {
            continue;
        }
        let shadowed = workspace_packages.contains(&package);
        if shadowed {
            // Satisfied by a workspace package: follow its imports; the library's same-named
            // package is admitted only when it is a standard-library anchor package.
            enqueue_workspace_package(facts, &package, options, &mut queue);
            if !anchor_packages.contains(&package) {
                continue;
            }
        }
        let Some(entries) = index.packages.get(&package) else {
            continue;
        };
        for entry in entries {
            if shadowed && entry.document.kind() != SourceKind::StandardLibrary {
                continue;
            }
            if !admitted.insert(entry.document.uri().to_string()) {
                continue;
            }
            let facts = entry.parsed.closure_facts();
            for target in &facts.import_targets {
                queue.extend(package_keys_for_import_target(target).map(PackageKey));
            }
            if options.bootstrap_typing_references {
                for target in &facts.type_reference_targets {
                    queue.extend(package_keys_for_import_target(target).map(PackageKey));
                }
            }
            documents.push(entry.document.clone());
        }
    }
    for unit in &index.unit_catalogs {
        if admitted.insert(unit.document.uri().to_string()) {
            documents.push(unit.document.clone());
        }
    }
    documents.sort_by(|left, right| left.uri().as_str().cmp(right.uri().as_str()));
    documents
}

fn enqueue_workspace_package(
    facts: &[SyntaxClosureFacts],
    package: &PackageKey,
    options: &LibraryClosureOptions,
    queue: &mut VecDeque<PackageKey>,
) {
    for facts in facts {
        for targets in facts
            .packages
            .iter()
            .filter(|targets| targets.qualified_name == package.0)
        {
            for target in &targets.import_targets {
                queue.extend(package_keys_for_import_target(target).map(PackageKey));
            }
            if options.bootstrap_typing_references {
                for target in &targets.type_reference_targets {
                    queue.extend(package_keys_for_import_target(target).map(PackageKey));
                }
            }
        }
    }
}

/// Every namespace prefix an import target names: `A::B::C::*` seeds `A`, `A::B`, `A::B::C`.
fn package_keys_for_import_target(target: &str) -> impl Iterator<Item = String> + '_ {
    let target = target
        .trim()
        .trim_end_matches("::*")
        .trim_end_matches("::**");
    let parts: Vec<&str> = if target.is_empty() {
        Vec::new()
    } else {
        target.split("::").collect()
    };
    (0..parts.len()).map(move |end| parts[..=end].join("::"))
}

/// Library layout convention: unit catalogues live under `Quantities and Units` (or a
/// `QUDV` / `SI` file) in the OMG distributions and their KPAR repackagings.
fn is_unit_catalog_path(uri_path: &str, relative_path: &str) -> bool {
    let full = uri_path.to_ascii_lowercase();
    let relative = relative_path.replace('\\', "/").to_ascii_lowercase();
    full.ends_with("units.sysml")
        || relative.contains("quantities and units/")
        || relative.contains("quantities%20and%20units/")
        || relative.contains("quantities_and_units")
        || relative.contains("qudv")
        || relative.ends_with("/si.sysml")
        || relative == "si.sysml"
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    fn authority() -> LibraryClosureAuthority {
        LibraryClosureAuthority::new(
            Arc::new(SourceAuthority::new()),
            Arc::new(SyntaxAuthority::new()),
        )
    }

    fn workspace(sources: &[(&str, &str)]) -> Vec<ParsedSource> {
        let syntax = SyntaxAuthority::new();
        sources
            .iter()
            .map(|(_, content)| syntax.parse_text(content))
            .collect()
    }

    fn library(path: &Path) -> Vec<LibraryRoot> {
        vec![LibraryRoot {
            path: path.to_path_buf(),
            kind: SourceKind::Library,
        }]
    }

    fn standard(path: &Path) -> Vec<LibraryRoot> {
        vec![LibraryRoot {
            path: path.to_path_buf(),
            kind: SourceKind::StandardLibrary,
        }]
    }

    fn paths(closure: &LibraryClosure) -> Vec<String> {
        closure
            .documents
            .iter()
            .map(|document| document.uri().path().to_owned())
            .collect()
    }

    #[test]
    fn closure_seed_signature_tracks_workspace_imports() {
        let authority = authority();
        let without = workspace(&[(
            "Model.sysml",
            "package Model { private import ScalarValues::*; attribute value : Real; }",
        )]);
        let with = workspace(&[(
            "Model.sysml",
            "package Model { private import ScalarValues::*; private import ModelingMetadata::*; attribute value : Real; }",
        )]);
        let options = LibraryClosureOptions::default();
        let first = authority.seed_signature(&without, &options);
        let second = authority.seed_signature(&with, &options);
        assert_ne!(first, second);
        assert!(second
            .iter()
            .any(|seed| seed == "import:ModelingMetadata::*"));
    }

    #[test]
    fn closure_loads_transitive_library_package_and_omits_unreferenced_files() {
        let temp = tempfile::tempdir().unwrap();
        let lib = temp.path().join("lib");
        fs::create_dir_all(&lib).unwrap();
        fs::write(
            lib.join("Base.sysml"),
            "package Base { attribute def Name; }",
        )
        .unwrap();
        fs::write(
            lib.join("Consumer.sysml"),
            "package Demo { import Base::*; part def P { attribute n : Name; } }",
        )
        .unwrap();
        fs::write(lib.join("Unused.sysml"), "package Unused { part def X; }").unwrap();
        let closure = authority()
            .resolve(
                &workspace(&[(
                    "model.sysml",
                    "package App { import Demo::*; part def AppPart; }",
                )]),
                &library(&lib),
                &LibraryClosureOptions::default(),
            )
            .unwrap();
        let paths = paths(&closure);
        assert!(paths.iter().any(|p| p.ends_with("Base.sysml")), "{paths:?}");
        assert!(
            paths.iter().any(|p| p.ends_with("Consumer.sysml")),
            "{paths:?}"
        );
        assert!(
            !paths.iter().any(|p| p.ends_with("Unused.sysml")),
            "{paths:?}"
        );
        assert!(closure
            .documents
            .iter()
            .all(|document| document.kind() == SourceKind::Library));
    }

    #[test]
    fn closure_loads_qualified_type_reference_and_specialization_packages() {
        let temp = tempfile::tempdir().unwrap();
        let lib = temp.path().join("lib");
        fs::create_dir_all(&lib).unwrap();
        fs::write(
            lib.join("OtherPkg.sysml"),
            "package OtherPkg { part def Base { attribute x; } }",
        )
        .unwrap();
        fs::write(
            lib.join("Domain.sysml"),
            "package Domain { part def Robot :> OtherPkg::Base { part motor; } }",
        )
        .unwrap();
        let ws = workspace(&[("model.sysml", "package App { part app : Domain::Robot; }")]);
        let closure = authority()
            .resolve(&ws, &library(&lib), &LibraryClosureOptions::default())
            .unwrap();
        let paths = paths(&closure);
        assert!(
            paths.iter().any(|p| p.ends_with("Domain.sysml")),
            "{paths:?}"
        );
        assert!(
            paths.iter().any(|p| p.ends_with("OtherPkg.sysml")),
            "{paths:?}"
        );

        let without_typing = authority()
            .resolve(
                &ws,
                &library(&lib),
                &LibraryClosureOptions {
                    bootstrap_typing_references: false,
                    ..LibraryClosureOptions::default()
                },
            )
            .unwrap();
        assert!(without_typing.documents.is_empty());
    }

    #[test]
    fn closure_loads_sysml_package_when_workspace_references_sysml_qualified_names() {
        let temp = tempfile::tempdir().unwrap();
        let lib = temp.path().join("sysml.library");
        fs::create_dir_all(&lib).unwrap();
        fs::write(
            lib.join("SysML.sysml"),
            "standard library package SysML { package Systems { metadata def RequirementUsage; metadata def Usage; } }",
        )
        .unwrap();
        fs::write(
            lib.join("ScalarValues.sysml"),
            "standard library package ScalarValues { attribute def Real; }",
        )
        .unwrap();
        let ws = workspace(&[(
            "RequirementMetadata.sysml",
            "package RequirementMetadata { metadata def RequirementRole { :> annotatedElement : SysML::RequirementUsage; } }",
        )]);
        let closure = authority()
            .resolve(&ws, &standard(&lib), &LibraryClosureOptions::default())
            .unwrap();
        let paths = paths(&closure);
        assert!(
            paths.iter().any(|p| p.ends_with("SysML.sysml")),
            "{paths:?}"
        );
        assert!(
            !paths.iter().any(|p| p.ends_with("ScalarValues.sysml")),
            "{paths:?}"
        );
    }

    #[test]
    fn importing_the_sysml_namespace_admits_every_standard_library_package() {
        let temp = tempfile::tempdir().unwrap();
        let lib = temp.path().join("sysml.library");
        fs::create_dir_all(&lib).unwrap();
        fs::write(lib.join("A.sysml"), "standard library package A;").unwrap();
        fs::write(lib.join("B.sysml"), "standard library package B;").unwrap();
        let ws = workspace(&[("m.sysml", "package M { import sysml::*; }")]);
        let closure = authority()
            .resolve(&ws, &standard(&lib), &LibraryClosureOptions::default())
            .unwrap();
        assert_eq!(closure.documents.len(), 2);
    }

    #[test]
    fn closure_skips_library_package_shadowed_by_workspace() {
        let temp = tempfile::tempdir().unwrap();
        let lib = temp.path().join("lib");
        fs::create_dir_all(&lib).unwrap();
        fs::write(
            lib.join("WebShopExample.sysml"),
            "package WebShopExample { part def LibraryOnlyPart; }",
        )
        .unwrap();
        fs::write(
            lib.join("ScalarValues.sysml"),
            "standard library package ScalarValues { attribute def Real; }",
        )
        .unwrap();
        let ws = workspace(&[
            (
                "webshop.sysml",
                "package WebShopExample { private import ScalarValues::Real; part def WorkspaceOnlyPart; }",
            ),
            (
                "Views.sysml",
                "package Views { import WebShopExample::*; view structure { expose WebShopExample::WorkspaceOnlyPart; } }",
            ),
        ]);
        let closure = authority()
            .resolve(&ws, &library(&lib), &LibraryClosureOptions::default())
            .unwrap();
        let paths = paths(&closure);
        assert!(
            !paths.iter().any(|p| p.ends_with("WebShopExample.sysml")),
            "{paths:?}"
        );
        assert!(
            paths.iter().any(|p| p.ends_with("ScalarValues.sysml")),
            "{paths:?}"
        );
    }

    #[test]
    fn closure_admits_standard_library_anchor_packages_even_when_shadowed() {
        let temp = tempfile::tempdir().unwrap();
        let lib = temp.path().join("lib");
        fs::create_dir_all(&lib).unwrap();
        fs::write(
            lib.join("Parts.sysml"),
            "standard library package Parts { part parts; }",
        )
        .unwrap();
        fs::write(
            lib.join("Views.sysml"),
            "standard library package Views { view views; }",
        )
        .unwrap();
        fs::write(
            lib.join("Unrelated.sysml"),
            "standard library package Unrelated { part def Nothing; }",
        )
        .unwrap();
        let ws = workspace(&[(
            "Views.sysml",
            "package Views { part timer; view structure { expose timer; } }",
        )]);
        let closure = authority()
            .resolve(&ws, &standard(&lib), &LibraryClosureOptions::default())
            .unwrap();
        let admitted = paths(&closure);
        assert!(
            admitted.iter().any(|p| p.ends_with("Parts.sysml")),
            "{admitted:?}"
        );
        assert!(
            admitted.iter().any(|p| p.ends_with("Views.sysml")),
            "{admitted:?}"
        );
        assert!(
            !admitted.iter().any(|p| p.ends_with("Unrelated.sysml")),
            "{admitted:?}"
        );

        // A dependency root that merely reuses an anchor name is not a standard library.
        let closure = authority()
            .resolve(&ws, &library(&lib), &LibraryClosureOptions::default())
            .unwrap();
        assert!(paths(&closure).is_empty(), "{:?}", paths(&closure));
    }

    #[test]
    fn closure_loads_nested_library_package_beside_shared_workspace_namespace() {
        let temp = tempfile::tempdir().unwrap();
        let lib = temp.path().join("lib");
        fs::create_dir_all(&lib).unwrap();
        fs::write(
            lib.join("Method.sysml"),
            "package Elan8 { package Method { package Core { part def ProjectInfo; } } }",
        )
        .unwrap();
        let ws = workspace(&[(
            "Photonics.sysml",
            "package Elan8 { package Photonics { item def OpticalSignal; } } package App { private import Elan8::Method::Core::*; part project : ProjectInfo; }",
        )]);
        let closure = authority()
            .resolve(&ws, &library(&lib), &LibraryClosureOptions::default())
            .unwrap();
        assert!(paths(&closure).iter().any(|p| p.ends_with("Method.sysml")));
    }

    #[test]
    fn unit_catalogs_load_by_path_or_by_content_independent_of_imports() {
        let temp = tempfile::tempdir().unwrap();
        let lib = temp.path().join("lib");
        let units_dir = lib.join("Quantities and Units");
        fs::create_dir_all(&units_dir).unwrap();
        fs::write(lib.join("Base.sysml"), "package Base { part def Y; }").unwrap();
        fs::write(
            units_dir.join("units.sysml"),
            "package Units { attribute <kg> kilogram : MassUnit; }",
        )
        .unwrap();
        fs::write(
            lib.join("Measurements.sysml"),
            "package Measurements { attribute <widget> widget : WidgetUnit; }",
        )
        .unwrap();
        fs::write(
            lib.join("Plain.sysml"),
            "package Plain { attribute plain : Thing; }",
        )
        .unwrap();
        let ws = workspace(&[("model.sysml", "package App { import Base::*; }")]);
        let closure = authority()
            .resolve(&ws, &library(&lib), &LibraryClosureOptions::default())
            .unwrap();
        let paths = paths(&closure);
        assert!(
            paths.iter().any(|p| p.ends_with("units.sysml")),
            "{paths:?}"
        );
        assert!(
            paths.iter().any(|p| p.ends_with("Measurements.sysml")),
            "{paths:?}"
        );
        assert!(
            !paths.iter().any(|p| p.ends_with("Plain.sysml")),
            "{paths:?}"
        );
    }

    #[test]
    fn a_unit_literal_seeds_the_quantity_packages() {
        let temp = tempfile::tempdir().unwrap();
        let lib = temp.path().join("lib");
        fs::create_dir_all(&lib).unwrap();
        fs::write(lib.join("SI.sysml"), "package SI { attribute def Length; }").unwrap();
        let ws = workspace(&[("m.sysml", "package M { attribute length = 10 [m]; }")]);
        let closure = authority()
            .resolve(&ws, &library(&lib), &LibraryClosureOptions::default())
            .unwrap();
        assert!(paths(&closure).iter().any(|p| p.ends_with("SI.sysml")));
    }

    #[test]
    fn the_package_index_is_reused_while_the_listing_is_unchanged() {
        let temp = tempfile::tempdir().unwrap();
        let lib = temp.path().join("lib");
        fs::create_dir_all(&lib).unwrap();
        fs::write(lib.join("Base.sysml"), "package Base { part def Y; }").unwrap();
        let authority = authority();
        let ws = workspace(&[("m.sysml", "package App { import Base::*; }")]);
        authority
            .resolve(&ws, &library(&lib), &LibraryClosureOptions::default())
            .unwrap();
        let first = Arc::as_ptr(&authority.index.lock().unwrap().as_ref().unwrap().1);
        authority
            .resolve(&ws, &library(&lib), &LibraryClosureOptions::default())
            .unwrap();
        let second = Arc::as_ptr(&authority.index.lock().unwrap().as_ref().unwrap().1);
        assert_eq!(first, second, "same listing, same index");
        fs::write(lib.join("More.sysml"), "package More;").unwrap();
        authority
            .resolve(&ws, &library(&lib), &LibraryClosureOptions::default())
            .unwrap();
        let third = Arc::as_ptr(&authority.index.lock().unwrap().as_ref().unwrap().1);
        assert_ne!(first, third, "a changed listing rebuilds the index");
        assert_eq!(
            authority.syntax.memo_len(),
            2,
            "both library trees sit in the authority's memo after the rebuild"
        );
    }

    #[test]
    fn closure_parsing_does_not_inherit_the_callers_small_stack() {
        let temp = tempfile::tempdir().unwrap();
        let lib = temp.path().join("lib");
        fs::create_dir_all(&lib).unwrap();
        fs::write(
            lib.join("ScalarValues.sysml"),
            "package ScalarValues { attribute def Real; }",
        )
        .unwrap();
        let mut source =
            String::from("package ArchitectureCommon { private import ScalarValues::*;\n");
        for index in 0..100 {
            source.push_str(&format!(
                "item def Telemetry{index} {{ attribute value : Real; }}\n"
            ));
        }
        source.push_str("}\n");
        let roots = library(&lib);
        let worker = std::thread::Builder::new()
            .name("small-stack-library-caller".into())
            .stack_size(256 * 1024)
            .spawn(move || {
                let ws = workspace(&[("ArchitectureCommon.sysml", &source)]);
                authority().resolve(&ws, &roots, &LibraryClosureOptions::default())
            })
            .unwrap();
        let closure = worker.join().unwrap().unwrap();
        assert!(paths(&closure)
            .iter()
            .any(|p| p.ends_with("ScalarValues.sysml")));
    }
}
