use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::Value;
use syn::visit::{self, Visit};
use syn::{Fields, ImplItem, Item, ReturnType, Signature, Type, UseTree, Visibility};

const DESIGNATED_CONSUMERS: &[&str] = &[
    "spec42-resolution-benchmark",
    "spec42-snapshot",
    "workspace_session",
];
/// Crates that still reach the legacy mutable semantic graph directly.
///
/// `sysml_query` is deliberately absent: the facade now depends only on `sysml_resolution`, which
/// is what `facade_depends_only_on_the_immutable_resolution_owner` pins. Every crate below is an
/// unmigrated consumer tracked in `PRODUCTION_CUTOVER.md`, and this list may only shrink.
const TRANSITIONAL_DIRECT_CONSUMERS: &[&str] = &[
    "language_service",
    "lsp_server",
    "server",
    "sysml_diagnostics",
    "workspace",
];

const FORBIDDEN_PUBLIC_TYPES: &[&str] = &[
    "SemanticGraph",
    "SemanticNode",
    "SemanticModel",
    "SemanticModelIdentity",
    "SemanticBuildRequest",
    "PreparedSemanticBuildRequest",
    "ImmutableSourceSnapshot",
    "SemanticQueryIndexes",
    "ResolutionState",
    "ResolutionFact",
    "ResolutionView",
    "ResolvedRelationship",
    "EvaluationState",
    "DeclaredSemanticFacts",
];

/// Forbidden names that the immutable publication also, independently, defines.
///
/// `sysml_model::semantic::publication::EvaluationState` is a crate-private storage struct;
/// `sysml_resolution::EvaluationState` is the published evaluation contract. They are two distinct
/// types that happen to share a spelling, and the identifier-based ban above cannot tell them
/// apart. The exemption is keyed on the *root* of the use path, so the publication's type reaches
/// the facade without the storage type's ban losing an identifier.
const PUBLISHED_RESOLUTION_TYPES: &[&str] = &["EvaluationState"];

/// The exemption must stay pinned to a real collision, not become a general escape hatch: if the
/// mutable model's type is renamed or removed, the name should go back to being plainly forbidden.
#[test]
fn every_publication_exemption_names_a_real_collision() {
    let storage = fs::read_to_string(
        repository_root().join("crates/sysml_model/src/semantic/publication.rs"),
    )
    .expect("read publication storage");
    let published =
        fs::read_to_string(repository_root().join("crates/sysml_resolution/src/evaluation.rs"))
            .expect("read published evaluation contract");
    for name in PUBLISHED_RESOLUTION_TYPES {
        assert!(
            FORBIDDEN_PUBLIC_TYPES.contains(name),
            "{name} is exempted from a ban it is not subject to"
        );
        assert!(
            storage.contains(&format!("struct {name}")),
            "{name} no longer collides with a mutable-model storage type; drop the exemption"
        );
        assert!(
            published.contains(&format!("pub enum {name}")),
            "{name} is not a published contract type; drop the exemption"
        );
    }
}

#[test]
fn designated_consumers_use_the_query_facade_and_direct_model_dependencies_do_not_expand() {
    let root = repository_root();
    let output = Command::new(env!("CARGO"))
        .args(["metadata", "--format-version", "1", "--no-deps"])
        .current_dir(&root)
        .output()
        .expect("run cargo metadata");
    assert!(
        output.status.success(),
        "cargo metadata failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let metadata: Value = serde_json::from_slice(&output.stdout).expect("parse cargo metadata");
    let packages = metadata["packages"].as_array().expect("packages array");
    let mut direct_model_dependencies = BTreeSet::new();
    for package in packages {
        let name = package["name"].as_str().expect("package name");
        let dependencies = package["dependencies"]
            .as_array()
            .expect("package dependencies");
        let dependency_names = dependencies
            .iter()
            .filter_map(|dependency| dependency["name"].as_str())
            .collect::<BTreeSet<_>>();
        if dependency_names.contains("sysml_model") {
            direct_model_dependencies.insert(name);
        }
        if DESIGNATED_CONSUMERS.contains(&name) {
            assert!(
                dependency_names.contains("sysml_query"),
                "designated semantic consumer {name} must depend on sysml_query"
            );
            assert!(
                !dependency_names.contains("sysml_model"),
                "designated semantic consumer {name} must not depend directly on sysml_model"
            );
            assert!(
                !dependency_names.contains("sysml_diagnostics"),
                "designated semantic consumer {name} must use the facade diagnostic service"
            );
        }
    }

    let known = TRANSITIONAL_DIRECT_CONSUMERS
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    assert_eq!(
        direct_model_dependencies, known,
        "direct sysml_model dependency inventory changed; additions bypass the query boundary, \
         while removals must shrink TRANSITIONAL_DIRECT_CONSUMERS in the same change"
    );
}

#[test]
fn facade_tests_do_not_duplicate_semantic_pipeline_snapshots() {
    let tests = repository_root().join("crates/sysml_query/tests");
    let mut violations = Vec::new();
    for entry in fs::read_dir(&tests).expect("read sysml_query tests") {
        let path = entry.expect("test entry").path();
        if path.extension().is_none_or(|extension| extension != "rs")
            || path
                .file_name()
                .is_some_and(|name| name == "architecture.rs")
        {
            continue;
        }
        let source = fs::read_to_string(&path).expect("read facade test");
        if source.contains("BuildRequest")
            || source.contains("SourceDocument")
            || source.contains("target_at(")
            || source.contains("visible_members(")
            || source.contains("prepare_rename(")
        {
            violations.push(path);
        }
    }
    assert!(
        violations.is_empty(),
        "sysml_query facade tests must not reconstruct models to duplicate semantic behavior; \
         add an owner-defined projection and a standalone snapshot fixture instead: {violations:?}"
    );
}

#[test]
fn immutable_snapshot_runner_has_an_exact_graph_free_dependency_boundary() {
    let root = repository_root();
    let output = Command::new(env!("CARGO"))
        .args(["metadata", "--format-version", "1", "--no-deps"])
        .current_dir(&root)
        .output()
        .expect("run cargo metadata");
    assert!(
        output.status.success(),
        "cargo metadata failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let metadata: Value = serde_json::from_slice(&output.stdout).expect("parse cargo metadata");
    let packages = metadata["packages"].as_array().expect("packages array");
    let snapshot = packages
        .iter()
        .find(|package| package["name"] == "spec42-snapshot")
        .expect("snapshot package");
    snapshot["dependencies"]
        .as_array()
        .expect("snapshot dependencies")
        .iter()
        .find(|dependency| dependency["name"] == "sysml_query")
        .expect("snapshot query dependency");

    let resolution = packages
        .iter()
        .find(|package| package["name"] == "sysml_resolution")
        .expect("resolution package");
    let actual_dependencies = resolution["dependencies"]
        .as_array()
        .expect("resolution dependencies")
        .iter()
        .map(|dependency| {
            dependency["rename"]
                .as_str()
                .or_else(|| dependency["name"].as_str())
                .expect("dependency name")
                .to_owned()
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(
        actual_dependencies,
        BTreeSet::from([
            "hashbrown".to_owned(),
            "rayon".to_owned(),
            "source_identity".to_owned(),
            "sysml-v2-parser-next".to_owned(),
        ]),
        "the immutable resolution owner dependency boundary changed"
    );

    let tree = Command::new(env!("CARGO"))
        .args(["tree", "-p", "spec42-snapshot", "-e", "normal"])
        .current_dir(&root)
        .output()
        .expect("run cargo tree");
    assert!(
        tree.status.success(),
        "cargo tree failed: {}",
        String::from_utf8_lossy(&tree.stderr)
    );
    let tree = String::from_utf8(tree.stdout).expect("utf-8 cargo tree");
    assert!(tree.contains("sysml_query"));
    assert!(tree.contains("sysml_resolution"));
    assert!(
        !tree.contains("sysml_model"),
        "legacy model reached snapshot runner:\n{tree}"
    );
    assert!(
        !tree.contains("sysml_diagnostics"),
        "legacy diagnostics reached snapshot runner:\n{tree}"
    );

    assert!(
        !root
            .join("crates/sysml_model/src/semantic/semantic_model_builder.rs")
            .exists(),
        "the parser-owned semantic builder must have exactly one owner"
    );
    assert!(root.join("crates/sysml_resolution/src/model.rs").exists());
}

/// The facade's whole dependency surface, stated exactly.
///
/// This replaces the feature selection consumers used to carry. While `sysml_query` had a
/// `legacy-model` feature, a consumer had to opt out of the graph and the guardrail could only
/// check that it had; now there is nothing to opt out of, and the boundary is a property of the
/// facade itself. A feature reintroducing an optional dependency would fail here, because an
/// always-off feature is still a way to grow this set later without review.
#[test]
fn facade_depends_only_on_the_immutable_resolution_owner() {
    let root = repository_root();
    let output = Command::new(env!("CARGO"))
        .args(["metadata", "--format-version", "1", "--no-deps"])
        .current_dir(&root)
        .output()
        .expect("run cargo metadata");
    assert!(
        output.status.success(),
        "cargo metadata failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let metadata: Value = serde_json::from_slice(&output.stdout).expect("parse cargo metadata");
    let query = metadata["packages"]
        .as_array()
        .expect("packages array")
        .iter()
        .find(|package| package["name"] == "sysml_query")
        .expect("query package");

    let dependencies = query["dependencies"]
        .as_array()
        .expect("query dependencies")
        .iter()
        .filter(|dependency| dependency["kind"].is_null())
        .map(|dependency| {
            dependency["name"]
                .as_str()
                .expect("dependency name")
                .to_owned()
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(
        dependencies,
        BTreeSet::from(["sysml_resolution".to_owned()]),
        "the query facade's normal dependency boundary changed"
    );
    assert!(
        query["features"]
            .as_object()
            .expect("query features")
            .is_empty(),
        "the facade must not offer a feature that can admit another dependency"
    );
}

/// `PRODUCTION_CUTOVER.md` claims its family table is the complete inventory of what
/// `sysml_diagnostics` still owns. A claim of completeness has to be checked, not trusted.
///
/// The risk this guards is specific: a consumer pointed at `PublishedModel::diagnostics()` before
/// its families are published stops reporting those codes, and nothing else fails. Keeping the
/// table exhaustive is what makes that decision reviewable.
#[test]
fn the_cutover_inventory_names_every_legacy_diagnostic_check_family() {
    let root = repository_root();
    let modules = fs::read_to_string(root.join("crates/sysml_diagnostics/src/checks/mod.rs"))
        .expect("read diagnostic check modules");
    let inventory = fs::read_to_string(root.join("crates/sysml_query/PRODUCTION_CUTOVER.md"))
        .expect("read cutover inventory");

    let mut missing = Vec::new();
    for line in modules.lines() {
        let Some(module) = line
            .trim()
            .strip_prefix("pub(super) mod ")
            .and_then(|rest| rest.strip_suffix(';'))
        else {
            continue;
        };
        // The table names families in prose ("kind compatibility"), not by module path, because it
        // is read by people deciding whether a slice is safe to migrate.
        let prose = module.replace('_', " ");
        if !inventory.contains(module) && !inventory.contains(&prose) {
            missing.push(module.to_owned());
        }
    }
    assert!(
        missing.is_empty(),
        "diagnostic check families absent from PRODUCTION_CUTOVER.md: {missing:?}. \
         A family that is not in the table can be dropped by a consumer migration without review."
    );
}

/// The feature-conformance families are owned by the immutable publication, and this keeps a
/// graph-based version from coming back.
///
/// Deleting the two modules is not self-enforcing: a new check module, or a few functions added to
/// a surviving one, would report the same codes over `&SemanticGraph` again and nothing would fail.
/// The guard is therefore two-sided -- the modules must stay gone, and no `sysml_diagnostics`
/// source may name a code the publication now owns.
#[test]
fn the_migrated_feature_conformance_families_cannot_return_to_the_graph() {
    let root = repository_root();
    let checks = root.join("crates/sysml_diagnostics/src/checks");
    for module in ["kind_compatibility.rs", "structural_feature_conformance.rs"] {
        assert!(
            !checks.join(module).exists(),
            "{module} is owned by sysml_resolution; it must not return to sysml_diagnostics"
        );
    }

    // Every code the publication settles for these families. A `sysml_diagnostics` source
    // mentioning one is either reporting it over the mutable graph or suppressing the immutable
    // one; both are the dual-reporting this migration removed.
    let migrated = [
        "incompatible_type_kind",
        "incompatible_specializes_kind",
        "incompatible_subset_redefine_kind",
        "specialization_cycle",
        "redefinition_multiplicity_widened",
        "redefinition_type_incompatible",
        "subsetting_type_incompatible",
        "single_type_relationship_operand",
        "flow_payload_type_not_occurrence",
        "incomplete_connection_like_end_pair",
        "invalid_binary_connection_like_end_count",
        "end_feature_invalid_restrictions",
        "invalid_variation_member_kind",
        "redefinition_featuring_type_incompatible",
        "redefinition_end_mismatch",
        "redefinition_direction_mismatch",
        "subsetting_uniqueness_mismatch",
    ];
    let mut sources = Vec::new();
    rust_sources(&root.join("crates/sysml_diagnostics/src"), &mut sources);
    let mut violations = Vec::new();
    for file in sources {
        let source = fs::read_to_string(&file).expect("read diagnostic source");
        for code in migrated {
            if source.contains(code) {
                violations.push(format!("{}: reintroduces {code}", file.display()));
            }
        }
    }
    assert!(
        violations.is_empty(),
        "these codes are settled by the immutable publication:\n{}",
        violations.join("\n")
    );

    // The tables those checks asked for are gone too. A caller-supplied allowlist is the shape the
    // migration replaced with an exhaustive metaclass-family relation, so a returning helper would
    // be the old design even under a new name.
    let kinds = fs::read_to_string(root.join("crates/sysml_model/src/semantic/kinds.rs"))
        .expect("read kind tables");
    for helper in [
        "allowed_subset_redefine_target_kinds",
        "allowed_specializes_target_kinds",
        "is_compatible_specializes_target",
        "is_compatible_typing_target",
        "expected_typing_definition_label",
        "is_flow_payload_occurrence_type",
    ] {
        assert!(
            !kinds.contains(helper),
            "{helper} served only the deleted kind checks and must not return"
        );
    }
}

#[test]
fn query_facade_public_api_contains_no_raw_semantic_storage() {
    assert_source_tree_has_no_raw_semantic_storage(
        &repository_root().join("crates/sysml_query/src"),
    );
}

#[test]
fn workspace_publication_owner_contains_no_raw_semantic_storage() {
    assert_source_tree_has_no_raw_semantic_storage(
        &repository_root().join("crates/workspace_session/src"),
    );
}

#[test]
fn workspace_cannot_restore_the_retired_semantic_publication_wrapper() {
    let root = repository_root();
    let files = [
        root.join("crates/workspace/src/lib.rs"),
        root.join("crates/workspace/src/semantic/mod.rs"),
    ];
    let retired_types = BTreeSet::from([
        "AuthoredReferenceId",
        "ConstructionStrategy",
        "EvaluationPolicy",
        "ImmutableSourceSnapshot",
        "ReferenceKind",
        "ResolutionOutcome",
        "ResolutionProvenance",
        "SemanticBuildFailure",
        "SemanticBuildRequest",
        "SemanticModelCompleteness",
        "SemanticConfiguration",
        "SemanticModel",
        "SemanticModelIdentity",
        "SemanticModelPhase",
    ]);
    let mut violations = Vec::new();
    for file in files {
        let source = fs::read_to_string(&file).expect("read workspace facade");
        let syntax = syn::parse_file(&source).expect("parse workspace facade");
        for item in syntax.items {
            match item {
                Item::Fn(function)
                    if is_public(&function.vis)
                        && function.sig.ident == "build_semantic_model_from_documents" =>
                {
                    violations.push(format!(
                        "{} restores build_semantic_model_from_documents",
                        file.display()
                    ));
                }
                Item::Use(item_use) if is_public(&item_use.vis) => {
                    let mut identifiers = BTreeSet::new();
                    use_identifiers(&item_use.tree, &mut identifiers);
                    for retired in &retired_types {
                        if identifiers.contains(*retired) {
                            violations.push(format!(
                                "{} reexports retired publication type {retired}",
                                file.display()
                            ));
                        }
                    }
                }
                _ => {}
            }
        }
    }

    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

fn assert_source_tree_has_no_raw_semantic_storage(source_root: &Path) {
    let mut files = Vec::new();
    rust_sources(source_root, &mut files);
    let mut violations = Vec::new();
    for file in files {
        let source = fs::read_to_string(&file).expect("read Rust source");
        let syntax = syn::parse_file(&source).expect("parse Rust source");
        PublicApiVisitor {
            file: &file,
            violations: &mut violations,
        }
        .visit_file(&syntax);
    }
    assert!(
        violations.is_empty(),
        "{} exposes forbidden implementation types:\n{}",
        source_root.display(),
        violations.join("\n")
    );
}

#[test]
fn model_publication_has_no_raw_model_or_index_escape_hatch() {
    let file = repository_root().join("crates/sysml_model/src/semantic/publication.rs");
    let source = fs::read_to_string(&file).expect("read publication source");
    let syntax = syn::parse_file(&source).expect("parse publication source");
    let forbidden_methods = BTreeSet::from([
        "facts",
        "evaluation_facts",
        "graph",
        "indexes",
        "node",
        "raw_graph",
        "resolution",
        "structural_graph",
        "view",
    ]);
    let mut violations = Vec::new();
    for item in syntax.items {
        match item {
            Item::Struct(item)
                if matches!(
                    item.ident.to_string().as_str(),
                    "ResolutionFact"
                        | "ResolutionState"
                        | "ResolutionView"
                        | "ResolvedRelationship"
                        | "EvaluationState"
                        | "SemanticQueryIndexes"
                ) && is_public(&item.vis) =>
            {
                violations.push(format!("raw type {} is public", item.ident));
            }
            Item::Impl(item) => {
                let owner = type_last_identifier(&item.self_ty);
                if !matches!(owner.as_deref(), Some("SemanticModel" | "ResolutionView")) {
                    continue;
                }
                for member in item.items {
                    let ImplItem::Fn(function) = member else {
                        continue;
                    };
                    if is_public(&function.vis)
                        && forbidden_methods.contains(function.sig.ident.to_string().as_str())
                    {
                        violations.push(format!(
                            "public {}::{} is a forbidden storage escape hatch",
                            owner.as_deref().unwrap_or("unknown"),
                            function.sig.ident
                        ));
                    }
                }
            }
            _ => {}
        }
    }
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

struct PublicApiVisitor<'a> {
    file: &'a Path,
    violations: &'a mut Vec<String>,
}

impl PublicApiVisitor<'_> {
    fn check_signature(&mut self, signature: &Signature) {
        for input in &signature.inputs {
            if let syn::FnArg::Typed(input) = input {
                self.check_type(&input.ty, &signature.ident.to_string());
            }
        }
        if let ReturnType::Type(_, output) = &signature.output {
            self.check_type(output, &signature.ident.to_string());
        }
    }

    fn check_fields(&mut self, fields: &Fields, context: &str, containing_public: bool) {
        for field in fields {
            if containing_public && (is_public(&field.vis) || matches!(fields, Fields::Unnamed(_)))
            {
                self.check_type(&field.ty, context);
            }
        }
    }

    fn check_type(&mut self, ty: &Type, context: &str) {
        let mut names = TypeIdentifierVisitor::default();
        names.visit_type(ty);
        for forbidden in names
            .identifiers
            .intersection(&FORBIDDEN_PUBLIC_TYPES.iter().copied().collect())
        {
            self.violations.push(format!(
                "{}: public {context} mentions {forbidden}",
                self.file.display()
            ));
        }
    }
}

impl<'ast> Visit<'ast> for PublicApiVisitor<'_> {
    fn visit_item_fn(&mut self, item: &'ast syn::ItemFn) {
        if is_public(&item.vis) {
            self.check_signature(&item.sig);
        }
        visit::visit_item_fn(self, item);
    }

    fn visit_item_struct(&mut self, item: &'ast syn::ItemStruct) {
        self.check_fields(&item.fields, &item.ident.to_string(), is_public(&item.vis));
        visit::visit_item_struct(self, item);
    }

    fn visit_item_enum(&mut self, item: &'ast syn::ItemEnum) {
        if is_public(&item.vis) {
            for variant in &item.variants {
                self.check_fields(&variant.fields, &item.ident.to_string(), true);
            }
        }
        visit::visit_item_enum(self, item);
    }

    fn visit_item_type(&mut self, item: &'ast syn::ItemType) {
        if is_public(&item.vis) || type_mentions_forbidden(&item.ty) {
            self.check_type(&item.ty, &item.ident.to_string());
        }
        visit::visit_item_type(self, item);
    }

    fn visit_item_use(&mut self, item: &'ast syn::ItemUse) {
        let mut identifiers = BTreeSet::new();
        let has_glob = use_identifiers(&item.tree, &mut identifiers);
        let from_publication = use_root(&item.tree) == Some("sysml_resolution".to_owned());
        for forbidden in FORBIDDEN_PUBLIC_TYPES {
            if from_publication && PUBLISHED_RESOLUTION_TYPES.contains(forbidden) {
                continue;
            }
            if identifiers.contains(*forbidden) {
                self.violations.push(format!(
                    "{}: use of {forbidden} can alias a forbidden implementation type",
                    self.file.display()
                ));
            }
        }
        if is_public(&item.vis) && has_glob {
            self.violations.push(format!(
                "{}: public glob use cannot prove the query facade is storage-free",
                self.file.display()
            ));
        }
        visit::visit_item_use(self, item);
    }

    fn visit_impl_item_fn(&mut self, item: &'ast syn::ImplItemFn) {
        if is_public(&item.vis) {
            self.check_signature(&item.sig);
        }
        visit::visit_impl_item_fn(self, item);
    }
}

#[derive(Default)]
struct TypeIdentifierVisitor {
    identifiers: BTreeSet<&'static str>,
}

impl<'ast> Visit<'ast> for TypeIdentifierVisitor {
    fn visit_ident(&mut self, ident: &'ast syn::Ident) {
        for forbidden in FORBIDDEN_PUBLIC_TYPES {
            if ident == forbidden {
                self.identifiers.insert(forbidden);
            }
        }
    }
}

fn use_identifiers(tree: &UseTree, output: &mut BTreeSet<String>) -> bool {
    match tree {
        UseTree::Path(path) => {
            output.insert(path.ident.to_string());
            use_identifiers(&path.tree, output)
        }
        UseTree::Name(name) => {
            output.insert(name.ident.to_string());
            false
        }
        UseTree::Rename(rename) => {
            output.insert(rename.ident.to_string());
            output.insert(rename.rename.to_string());
            false
        }
        UseTree::Group(group) => {
            let mut has_glob = false;
            for item in &group.items {
                // Do not short-circuit: every identifier contributes to the alias check.
                has_glob |= use_identifiers(item, output);
            }
            has_glob
        }
        UseTree::Glob(_) => true,
    }
}

/// The first segment of a use path, which names the crate the items come from.
fn use_root(tree: &UseTree) -> Option<String> {
    match tree {
        UseTree::Path(path) => Some(path.ident.to_string()),
        _ => None,
    }
}

fn type_mentions_forbidden(ty: &Type) -> bool {
    let mut names = TypeIdentifierVisitor::default();
    names.visit_type(ty);
    !names.identifiers.is_empty()
}

fn type_last_identifier(ty: &Type) -> Option<String> {
    let Type::Path(path) = ty else {
        return None;
    };
    path.path
        .segments
        .last()
        .map(|segment| segment.ident.to_string())
}

fn is_public(visibility: &Visibility) -> bool {
    matches!(visibility, Visibility::Public(_))
}

fn rust_sources(directory: &Path, output: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(directory).expect("read source directory") {
        let path = entry.expect("source entry").path();
        if path.is_dir() {
            rust_sources(&path, output);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            output.push(path);
        }
    }
    output.sort();
}

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crate is under repository/crates")
        .to_path_buf()
}
