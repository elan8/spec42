use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::Value;
use syn::visit::{self, Visit};
use syn::{Fields, Item, ReturnType, Signature, Type, UseTree, Visibility};

/// Every crate that consumes SysML through the facade. None may name an authority crate.
const DESIGNATED_CONSUMERS: &[&str] = &[
    "generator_api",
    "generator_conformance",
    "kpar",
    "language_service",
    "lsp_server",
    "server",
    "spec42-resolution-benchmark",
    "spec42-semantic-benchmark",
    "spec42-snapshot",
    "sysml_diagnostics",
    "sysml_tokens",
    "workspace",
];

/// Consumers that read diagnostics only as published facts and must not carry the reporting
/// policy crate; hosts that render diagnostics legitimately depend on it.
const FACADE_ONLY_DIAGNOSTIC_CONSUMERS: &[&str] =
    &["spec42-resolution-benchmark", "spec42-snapshot"];

/// The authority chain: a crate that no consumer may depend on.
/// `sysml_contract` is not an authority, but it is on this list for the same reason: the facade
/// re-exports the vocabulary verbatim, so a consumer that named the contract crate directly would
/// depend on where a type currently lives rather than on the facade that publishes it.
const AUTHORITY_CRATES: &[&str] = &[
    "sysml-v2-parser",
    "sysml_contract",
    "sysml_resolution",
    "sysml_source",
];

const FORBIDDEN_PUBLIC_TYPES: &[&str] = &[
    "ParsedDocument",
    "ParseResult",
    "ParseError",
    "RootElement",
    "sysml_v2_parser",
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

/// Published immutable contracts whose names intentionally overlap the generic raw-storage ban.
const PUBLISHED_RESOLUTION_TYPES: &[&str] = &["EvaluationState"];

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
            for authority in AUTHORITY_CRATES {
                assert!(
                    !dependency_names.contains(authority),
                    "designated consumer {name} must reach {authority} through sysml_query"
                );
            }
            if FACADE_ONLY_DIAGNOSTIC_CONSUMERS.contains(&name) {
                assert!(
                    !dependency_names.contains("sysml_diagnostics"),
                    "designated semantic consumer {name} must use the facade diagnostic service"
                );
            }
        }
    }

    assert!(
        direct_model_dependencies.is_empty(),
        "the deleted sysml_model crate must not return as a dependency: {direct_model_dependencies:?}"
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
            || source.contains("AdmittedSource")
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
    // Normal dependencies only: a dev-dependency never reaches the graph a consumer resolves.
    let actual_dependencies = resolution["dependencies"]
        .as_array()
        .expect("resolution dependencies")
        .iter()
        .filter(|dependency| dependency["kind"].is_null())
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
            "blake3".to_owned(),
            "hashbrown".to_owned(),
            "rayon".to_owned(),
            "serde".to_owned(),
            "source_identity".to_owned(),
            "spec42_constraint_manifest".to_owned(),
            "sysml-v2-parser".to_owned(),
            "sysml_contract".to_owned(),
            "sysml_source".to_owned(),
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
        BTreeSet::from(["sysml_contract".to_owned(), "sysml_resolution".to_owned(),]),
        "the query facade depends on the semantic authority and the vocabulary it re-exports, and \
         on nothing else: a third dependency here is a consumer-visible surface the facade would \
         be publishing without owning"
    );
    assert!(
        query["features"]
            .as_object()
            .expect("query features")
            .is_empty(),
        "the facade must not offer a feature that can admit another dependency"
    );
}

/// The modules that read element details from the publication must not be able to reach the
/// mutable graph again.
///
/// Deleting the reconstruction helpers is not self-enforcing. A `&SemanticGraph` parameter added
/// back to one of these functions, or a `sysml_model` import for "just one fact", restores exactly
/// the second source of truth this migration removed -- and nothing else in the tree would fail.
/// The guard is therefore on the module surface rather than on any one helper.
#[test]
fn the_migrated_inspector_and_symbol_modules_cannot_return_to_the_graph() {
    let root = repository_root();
    let migrated = [
        "crates/lsp_server/src/views/feature_inspector.rs",
        "crates/lsp_server/src/lsp_runtime/symbols.rs",
    ];
    // `SemanticGraph`/`SemanticNode` are the handles; `sysml_model` is the crate that owns them
    // and every legacy evaluation and relationship helper besides. `evaluation_facts_for` and
    // `expression_evaluation_for` are named individually because they are the node-keyed
    // evaluation reads this migration replaced, and they would be reachable through a re-export.
    let forbidden = [
        "SemanticGraph",
        "SemanticNode",
        "sysml_model",
        "evaluation_facts_for",
        "expression_evaluation_for",
        "ExpressionEvaluationQuery",
        "outgoing_targets_by_kind",
        "incoming_relationships(",
        "nodes_named",
        "node_ids_by_qualified_name",
        "resolve_inherited_member_via_type",
    ];
    let mut violations = Vec::new();
    for module in migrated {
        let path = root.join(module);
        let source = fs::read_to_string(&path).unwrap_or_else(|error| {
            panic!("read migrated module {module}: {error}");
        });
        for name in forbidden {
            if source.contains(name) {
                violations.push(format!("{module}: reaches {name}"));
            }
        }
    }
    assert!(
        violations.is_empty(),
        "these modules read the immutable publication and must not reach the graph:\n{}",
        violations.join("\n")
    );
}

/// Host validation reads the immutable publication, and nothing on that path may reach the graph.
///
/// The three surfaces that report diagnostics -- workspace validation assembly, LSP computation
/// and publication, and the server's batch validation -- now consume one published result. Nothing
/// makes that self-enforcing: a helper taking `&SemanticGraph` added to any of them would quietly
/// reintroduce a second engine deciding the same codes, and every test would still pass.
///
/// `sysml_diagnostics` is on the list because it is the reporting layer: it may filter and render
/// published values, never decide one.
#[test]
fn migrated_validation_paths_cannot_return_to_the_graph() {
    let root = repository_root();
    let migrated = [
        "crates/workspace/src/snapshot/validation.rs",
        "crates/workspace/src/validation/mod.rs",
        "crates/workspace/src/validation/built_workspace.rs",
        "crates/workspace/src/validation/report.rs",
        "crates/lsp_server/src/analysis/diagnostics_core.rs",
        "crates/lsp_server/src/analysis/diagnostics_adapter.rs",
        "crates/lsp_server/src/lsp_runtime/diagnostics.rs",
        "crates/sysml_diagnostics/src/reporting.rs",
        "crates/sysml_diagnostics/src/types.rs",
        "crates/sysml_diagnostics/src/lib.rs",
    ];
    // The graph handles, the deleted entry points, and the node-keyed helpers a rule would need to
    // decide anything for itself.
    //
    // `sysml_model` itself is not banned here: `publication.rs` admits the host's own
    // `SysmlDocument` values, reading their URI, content and source kind and nothing else. That is
    // the source-admission boundary, and banning the crate would ban carrying a document across
    // it. Everything the crate owns that *decides* meaning is banned by name below.
    let forbidden = [
        "SemanticGraph",
        "SemanticNode",
        "collect_diagnostics_from_graph",
        "compute_semantic_diagnostics",
        "collect_document_diagnostics_from_model",
        "evaluation_facts_for",
        "expression_evaluation_for",
        "resolve_import_target",
        "resolve_type_reference_targets",
        "resolve_inherited_member_via_type",
        "resolve_expression_endpoint_strict",
        "outgoing_targets_by_kind",
        "nodes_for_uri",
        "node_ids_by_qualified_name",
    ];
    let mut violations = Vec::new();
    for module in migrated {
        let path = root.join(module);
        let source = fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("read migrated module {module}: {error}"));
        for name in forbidden {
            if source.contains(name) {
                violations.push(format!("{module}: reaches {name}"));
            }
        }
    }
    assert!(
        violations.is_empty(),
        "these modules read the immutable publication and must not reach the graph:\n{}",
        violations.join("\n")
    );
}

/// The graph-backed diagnostic engine stays deleted.
///
/// Deleting the modules is not self-enforcing: a new check module, or a few functions added to the
/// surviving reporting crate, would report the same codes over `&SemanticGraph` again and nothing
/// would fail. The engine's own files are named so restoring one is a test failure rather than a
/// review question.
#[test]
fn the_graph_backed_diagnostic_engine_stays_deleted() {
    let root = repository_root();
    let deleted = [
        "crates/sysml_diagnostics/src/checks",
        "crates/sysml_diagnostics/src/engine.rs",
        "crates/sysml_diagnostics/src/engine_impl.rs",
        "crates/sysml_diagnostics/src/helpers.rs",
        "crates/sysml_diagnostics/src/model.rs",
        "crates/sysml_diagnostics/src/document.rs",
        "crates/sysml_diagnostics/src/ordering.rs",
        "crates/sysml_diagnostics/src/shared_rules.rs",
        "crates/sysml_diagnostics/src/kind_rules.rs",
        "crates/sysml_diagnostics/src/relationship_endpoint_messages.rs",
        "crates/sysml_diagnostics/src/pending_relationship_diagnostics.rs",
        "crates/lsp_server/src/analysis/checks.rs",
        "crates/lsp_server/src/analysis/checks",
    ];
    let restored = deleted
        .into_iter()
        .filter(|path| root.join(path).exists())
        .collect::<Vec<_>>();
    assert!(
        restored.is_empty(),
        "sysml_resolution owns these rules; they must not return: {restored:?}"
    );
}

/// The consumer-side semantic reconstruction the inspector used to carry must stay deleted.
///
/// Each of these recovered a relationship target by normalizing authored text and searching for a
/// name, or walked a hierarchy the type index now answers. They are named rather than described
/// because a returning implementation would most likely return under its old name.
#[test]
fn the_inspector_reconstruction_helpers_stay_deleted() {
    let root = repository_root();
    let mut sources = Vec::new();
    rust_sources(&root.join("crates/lsp_server/src"), &mut sources);
    let deleted = [
        "declared_target_candidates",
        "relationship_targets_with_fallback",
        "relationship_targets_with_typed_fallback",
        "typing_targets_from_typed_facts",
        "typed_subsetting_family_targets",
        "effective_typing_targets",
        "inherited_attributes_for_part_def",
        "inherited_attribute_hint_lines",
    ];
    let mut violations = Vec::new();
    for file in sources {
        let source = fs::read_to_string(&file).expect("read lsp_server source");
        for name in deleted {
            if source.contains(name) {
                violations.push(format!("{}: reintroduces {name}", file.display()));
            }
        }
    }
    assert!(
        violations.is_empty(),
        "the publication owns these answers now:\n{}",
        violations.join("\n")
    );
}

/// The inspector's wire contract must not grow a generic attribute map again.
///
/// The map was how presentation became a second truth store: a consumer read `evaluatedValue` and
/// `attributeType` out of it and never learned that the publication had settled either. Every key
/// it carried has a typed field now, and the two evaluation channels are their own types.
#[test]
fn the_inspector_dto_has_no_generic_attribute_map() {
    let source = fs::read_to_string(repository_root().join("crates/lsp_server/src/views/dto.rs"))
        .expect("read view DTOs");
    let syntax = syn::parse_file(&source).expect("parse view DTOs");
    let mut violations = Vec::new();
    for item in syntax.items {
        let Item::Struct(item) = item else {
            continue;
        };
        if !item.ident.to_string().starts_with("SysmlFeatureInspector") {
            continue;
        }
        let Fields::Named(fields) = item.fields else {
            continue;
        };
        for field in fields.named {
            let name = field
                .ident
                .as_ref()
                .map(ToString::to_string)
                .unwrap_or_default();
            let ty = type_last_identifier(&field.ty).unwrap_or_default();
            if name == "attributes" || ty == "HashMap" || ty == "BTreeMap" {
                violations.push(format!("{}::{name} is a generic map", item.ident));
            }
        }
    }
    assert!(
        violations.is_empty(),
        "the feature inspector projects typed facts, not an attribute map:\n{}",
        violations.join("\n")
    );
}

#[test]
fn query_facade_public_api_contains_no_raw_semantic_storage() {
    assert_source_tree_has_no_raw_semantic_storage(
        &repository_root().join("crates/sysml_query/src"),
    );
}

/// The vocabulary crate is where the storage-free rule has teeth.
///
/// The facade scan can only catch a raw type that reaches the facade's own signatures. Every name
/// the facade re-exports is defined in `sysml_contract`, so as types move there this is the tree
/// that decides whether the published vocabulary names an implementation type. A contract crate
/// that mentioned a parse tree or a resolution graph would have stopped being a contract.
#[test]
fn the_contract_crate_public_api_contains_no_raw_semantic_storage() {
    assert_source_tree_has_no_raw_semantic_storage(
        &repository_root().join("crates/sysml_contract/src"),
    );
}

#[test]
fn the_session_actor_contains_no_raw_semantic_storage() {
    assert_source_tree_has_no_raw_semantic_storage(
        &repository_root().join("crates/session_actor/src"),
    );
}

#[test]
fn workspace_cannot_restore_the_retired_semantic_publication_wrapper() {
    let root = repository_root();
    let files = [root.join("crates/workspace/src/lib.rs")];
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

/// The published vocabulary's remaining owned strings, named one by one.
///
/// The facade rule is identities, enums and borrowed views -- never owned storage -- and an owned
/// `String` or `Box<str>` field is owned storage. Every one of these is allocated per element of
/// every result that carries the struct, and every one duplicates bytes the publication already
/// holds; a consumer that only wanted to compare, sort or index two elements pays for a copy it
/// never reads. Element identities have already left the list: a handle is a `SymbolId` and the
/// string encoding materialises through `symbol_token` only where one crosses a boundary.
///
/// The list is an inventory of what is left to do, not a set of allowances. It may only shrink:
/// adding a name requires deleting this comment's claim, and the borrowed-views work drains it.
/// Owned strings a *consumer supplies* when it asks a question. A query input is constructed by
/// the caller, from a name it already holds as text, and is dropped when the answer comes back;
/// there is no published storage for it to borrow from. Owning is the correct shape here, so this
/// set is expected to stay -- but it is asserted exactly, so a *product* cannot be smuggled in by
/// calling itself an input.
const FACADE_OWNED_STRING_INPUT_FIELDS: &[&str] = &[
    "AffectedDocument::identity",
    "EditorProbe::document",
    "QualifiedElementReference::qualified_name",
    "QualifiedReferenceProbe::qualified_name",
];

/// Owned strings a *query product* still carries. Every entry here is a copy of text the
/// publication already stores, handed to the consumer as a fresh allocation: the facade rule says
/// a product carries a handle or a borrowed view instead. This list is the remaining debt and is
/// only ever meant to shrink.
const FACADE_OWNED_STRING_PRODUCT_FIELDS: &[&str] = &[
    "AuthoredUnit::authored",
    "DiagramEdge::semantic_id",
    "DiagramRelationship::kind",
    "DiagramRelationship::semantic_id",
    "DiagramStateTransition::semantic_id",
    "DiagramStateVertex::label",
    "DiagramViewCatalogEntry::name",
    "Documentation::text",
    "ElementInspection::qualified_name",
    "PackageTargets::qualified_name",
    "QualifiedReferenceTarget::qualified_name",
    "SymbolEntry::qualified_name",
    "SyntaxDiagnostic::message",
    "SyntaxOutlineNode::name",
];
/// The count is asserted separately from the membership so a swap -- one field drained and another
/// added in the same change -- cannot pass as a no-op.
#[test]
fn the_published_vocabulary_grows_no_new_owned_string_fields() {
    let root = repository_root();
    let mut violations = Vec::new();
    let mut owned = BTreeSet::new();
    visit_public_api(
        &root.join("crates/sysml_query/src"),
        &mut violations,
        &mut owned,
    );
    visit_public_api(
        &root.join("crates/sysml_contract/src"),
        &mut violations,
        &mut owned,
    );
    // The facade re-exports the publication's result types verbatim, so a field of one of those
    // is a facade field wherever the struct happens to be declared. Scanning only the two facade
    // trees would let the inventory hide behind a `pub use`.
    let published = facade_reexported_names(&root.join("crates/sysml_query/src"));
    let mut republished = BTreeSet::new();
    visit_public_api(
        &root.join("crates/sysml_resolution/src"),
        &mut Vec::new(),
        &mut republished,
    );
    owned.extend(republished.into_iter().filter(|entry| {
        entry
            .split_once("::")
            .is_some_and(|(container, _)| published.contains(container))
    }));
    let inputs: BTreeSet<String> = FACADE_OWNED_STRING_INPUT_FIELDS
        .iter()
        .map(|name| (*name).to_owned())
        .collect();
    let products: BTreeSet<String> = FACADE_OWNED_STRING_PRODUCT_FIELDS
        .iter()
        .map(|name| (*name).to_owned())
        .collect();
    let overlap: Vec<&String> = inputs.intersection(&products).collect();
    assert!(
        overlap.is_empty(),
        "a field is listed as both a query input and a query product; it is one or the \
         other:\n{overlap:#?}"
    );
    let allowed: BTreeSet<String> = inputs.union(&products).cloned().collect();
    let added: Vec<&String> = owned.difference(&allowed).collect();
    assert!(
        added.is_empty(),
        "a published struct grew an owned string field; carry a handle or a borrowed view \
         instead, or -- if a consumer supplies it as a query input -- add it to \
         FACADE_OWNED_STRING_INPUT_FIELDS:\n{added:#?}"
    );
    let drained: Vec<&String> = allowed.difference(&owned).collect();
    assert!(
        drained.is_empty(),
        "these owned string fields are gone; delete them from the input or product list so the \
         lists keep counting what is actually left:\n{drained:#?}"
    );
    assert_eq!(
        owned.len(),
        FACADE_OWNED_STRING_INPUT_FIELDS.len() + FACADE_OWNED_STRING_PRODUCT_FIELDS.len(),
        "the owned-string inventory changed size"
    );
}

/// Every type name the facade re-exports from the authority or the contract crate.
fn facade_reexported_names(facade_root: &Path) -> BTreeSet<String> {
    let mut files = Vec::new();
    rust_sources(facade_root, &mut files);
    let mut names = BTreeSet::new();
    for file in files {
        let source = fs::read_to_string(&file).expect("read Rust source");
        let syntax = syn::parse_file(&source).expect("parse Rust source");
        for item in syntax.items {
            let Item::Use(item_use) = item else {
                continue;
            };
            if !is_public(&item_use.vis) {
                continue;
            }
            let root = use_root(&item_use.tree);
            if root.as_deref() != Some("sysml_resolution")
                && root.as_deref() != Some("sysml_contract")
            {
                continue;
            }
            use_identifiers(&item_use.tree, &mut names);
        }
    }
    names
}

/// `String` and `Box<str>` exactly -- not a `String` inside an `Option`, a `Vec` or a map, which
/// are their own storage questions, and not a borrowed `&str`, which is what the rule asks for.
fn is_owned_string(ty: &Type) -> bool {
    let Type::Path(path) = ty else {
        return false;
    };
    if path.qself.is_some() {
        return false;
    }
    let Some(segment) = path.path.segments.last() else {
        return false;
    };
    match segment.ident.to_string().as_str() {
        "String" => matches!(segment.arguments, syn::PathArguments::None),
        "Box" => {
            let syn::PathArguments::AngleBracketed(arguments) = &segment.arguments else {
                return false;
            };
            matches!(
                arguments.args.first(),
                Some(syn::GenericArgument::Type(Type::Path(inner)))
                    if inner.path.is_ident("str")
            )
        }
        _ => false,
    }
}

fn assert_source_tree_has_no_raw_semantic_storage(source_root: &Path) {
    let mut violations = Vec::new();
    let mut owned_strings = BTreeSet::new();
    visit_public_api(source_root, &mut violations, &mut owned_strings);
    assert!(
        violations.is_empty(),
        "{} exposes forbidden implementation types:\n{}",
        source_root.display(),
        violations.join("\n")
    );
}

/// Walks one crate's sources with the public-API visitor, collecting both the forbidden-type
/// violations and the inventory of owned-string fields the published structs still carry.
fn visit_public_api(
    source_root: &Path,
    violations: &mut Vec<String>,
    owned_strings: &mut BTreeSet<String>,
) {
    let mut files = Vec::new();
    rust_sources(source_root, &mut files);
    for file in files {
        let source = fs::read_to_string(&file).expect("read Rust source");
        let syntax = syn::parse_file(&source).expect("parse Rust source");
        PublicApiVisitor {
            file: &file,
            violations,
            owned_strings,
        }
        .visit_file(&syntax);
    }
}

struct PublicApiVisitor<'a> {
    file: &'a Path,
    violations: &'a mut Vec<String>,
    /// `Struct::field` for every public struct field typed exactly `String` or `Box<str>`.
    owned_strings: &'a mut BTreeSet<String>,
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

    /// Records every public field of a published struct whose type is an owned string.
    ///
    /// The facade rule is identities, enums and borrowed views -- never owned storage. An owned
    /// `String` in a published struct is storage: it is allocated per element of every result and
    /// duplicates a byte sequence the publication already holds, so a consumer that only wanted to
    /// compare or index pays for a copy it never reads. Each one left is an inventory item, not an
    /// allowance.
    fn record_owned_string_fields(&mut self, fields: &Fields, container: &str) {
        for field in fields {
            let Some(ident) = field.ident.as_ref() else {
                continue;
            };
            if is_public(&field.vis) && is_owned_string(&field.ty) {
                self.owned_strings.insert(format!("{container}::{ident}"));
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
        if is_public(&item.vis) {
            self.record_owned_string_fields(&item.fields, &item.ident.to_string());
        }
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

/// The host crates stay split by responsibility: the generic actor knows no SysML crate, the
/// provisioning crate reads no SysML through anything but the facade (via kpar), and the batch
/// host carries neither storage nor async runtime nor protocol.
#[test]
fn host_crates_keep_their_declared_dependency_sets() {
    let root = repository_root();
    let output = Command::new(env!("CARGO"))
        .args(["metadata", "--format-version", "1", "--no-deps"])
        .current_dir(&root)
        .output()
        .expect("run cargo metadata");
    assert!(output.status.success());
    let metadata: Value = serde_json::from_slice(&output.stdout).expect("parse cargo metadata");
    let packages = metadata["packages"].as_array().expect("packages array");
    let normal_dependencies = |name: &str| -> BTreeSet<String> {
        packages
            .iter()
            .find(|package| package["name"] == name)
            .unwrap_or_else(|| panic!("{name} package"))["dependencies"]
            .as_array()
            .expect("dependencies")
            .iter()
            .filter(|dependency| dependency["kind"].is_null())
            .map(|dependency| {
                dependency["rename"]
                    .as_str()
                    .or_else(|| dependency["name"].as_str())
                    .expect("dependency name")
                    .to_owned()
            })
            .collect()
    };
    let set = |names: &[&str]| {
        names
            .iter()
            .map(|name| (*name).to_owned())
            .collect::<BTreeSet<_>>()
    };

    assert_eq!(
        normal_dependencies("session_actor"),
        set(&["thiserror", "tokio", "tracing"]),
        "session_actor is a generic actor and names no SysML crate"
    );
    assert_eq!(
        normal_dependencies("library_catalog"),
        set(&[
            "directories",
            "kpar",
            "serde",
            "sysml_query",
            "tempfile",
            "toml",
            "walkdir",
            "zip"
        ]),
        "library_catalog provisions library roots and nothing else"
    );
    assert_eq!(
        normal_dependencies("workspace"),
        set(&[
            "language_service",
            "library_catalog",
            "serde",
            "serde_json",
            "sysml_diagnostics",
            "sysml_query",
            "tempfile",
            "thiserror",
            "url",
        ]),
        "workspace is a batch host over the facade"
    );
    assert_eq!(
        normal_dependencies("lsp_server"),
        set(&[
            "base64",
            "clap",
            "generator_api",
            "generator_host",
            "glob",
            "language_service",
            "petgraph",
            "rayon",
            "serde",
            "serde_json",
            "session_actor",
            "sha2",
            "spec42-generator-protocol",
            "sysml_diagnostics",
            "sysml_query",
            "sysml_tokens",
            "tempfile",
            "thiserror",
            "tokio",
            "toml",
            "tower-lsp",
            "tracing",
            "tracing-subscriber",
        ]),
        "lsp_server is the editor host and owns no batch path: it must not depend on `workspace`"
    );
    assert!(
        !normal_dependencies("lsp_server").contains("workspace"),
        "the editor host and the batch host are siblings; validation lives in `workspace`"
    );
    assert_eq!(
        normal_dependencies("server"),
        set(&[
            "clap",
            "directories",
            "generator_api",
            "generator_host",
            "kpar",
            "library_catalog",
            "lsp_server",
            "rquickjs",
            "serde",
            "serde_json",
            "sha2",
            "sysml_diagnostics",
            "sysml_query",
            "tempfile",
            "tokio",
            "toml",
            "tower-lsp",
            "workspace",
            "zip",
        ]),
        "server reaches validation through `workspace`; `lsp_server` is the launch-only edge"
    );
}

/// `lsp_server` owns no validation pipeline and no batch entry point.
///
/// The dependency-set assertion above proves the crate cannot call into `workspace`; this proves
/// the pipeline did not simply get rewritten in place under the old module name.
#[test]
fn the_editor_host_declares_no_validation_module() {
    let root = repository_root();
    for path in [
        "crates/lsp_server/src/validation",
        "crates/lsp_server/src/validation.rs",
    ] {
        assert!(
            !root.join(path).exists(),
            "{path} came back; batch validation belongs to `workspace::validation`"
        );
    }
    assert!(
        root.join("crates/workspace/src/validation/mod.rs").exists(),
        "the batch validation path lives in `workspace::validation`"
    );
}

// -- Host boundary guards ------------------------------------------------------------------

/// The host crates. Everything they know about a model comes from the facade's typed answers.
const HOST_CRATES: &[&str] = &["lsp_server", "server"];

/// Every host function that still takes SysML text as a parameter.
///
/// A host may carry a document's text — that is what an editor session does — but deriving a
/// fact from it is the syntax authority's job. Each entry is a `(path, fn name)` pair that
/// exists today; the list only ever shrinks. Adding a new text-taking function to a host fails
/// this test, which is the point: the next `parse this line` helper has to justify itself.
const HOST_TEXT_ENTRY_POINT_ALLOWLIST: &[(&str, &str)] = &[
    // Document lifecycle: text arriving from the client on its way to `SourceService::admit`.
    (
        "crates/lsp_server/src/common/util.rs",
        "apply_incremental_change",
    ),
    ("crates/lsp_server/src/common/util.rs", "parse_for_editor"),
    ("crates/lsp_server/src/session/handle.rs", "admit_text"),
    (
        "crates/lsp_server/src/session/handle.rs",
        "store_document_text_fast",
    ),
    (
        "crates/lsp_server/src/session/handle.rs",
        "refresh_document",
    ),
    (
        "crates/lsp_server/src/session/services.rs",
        "parse_scanned_entry",
    ),
    (
        "crates/lsp_server/src/session/services.rs",
        "store_document_text",
    ),
    (
        "crates/lsp_server/src/session/services.rs",
        "store_document_text_fast",
    ),
    (
        "crates/lsp_server/src/session/services.rs",
        "refresh_document",
    ),
    (
        "crates/lsp_server/src/lsp_runtime/documents/sync.rs",
        "watched_file_content_already_current",
    ),
    // Text projection: slicing a range the authority settled out of the text it settled it over.
    (
        "crates/lsp_server/src/views/feature_inspector.rs",
        "slice_range",
    ),
    ("crates/lsp_server/src/language/mod.rs", "format_document"),
    // Known debt: code actions and probes that read SysML text rather than asking the syntax
    // service. These are the D10 entries Proposal C names; the list only ever shrinks.
    (
        "crates/lsp_server/src/common/util.rs",
        "untyped_part_usage_diagnostics",
    ),
    (
        "crates/lsp_server/src/common/util.rs",
        "import_statement_ranges",
    ),
    (
        "crates/lsp_server/src/language/mod.rs",
        "suggest_wrap_in_package",
    ),
    (
        "crates/lsp_server/src/language/mod.rs",
        "suggest_create_definition_for_unresolved_type_quick_fix",
    ),
    (
        "crates/lsp_server/src/language/mod.rs",
        "suggest_create_matching_part_def_quick_fix",
    ),
    (
        "crates/lsp_server/src/language/mod.rs",
        "suggest_explicit_redefinition_quick_fix",
    ),
    (
        "crates/lsp_server/src/language/mod.rs",
        "suggest_create_verification_case",
    ),
    (
        "crates/lsp_server/src/language/mod.rs",
        "suggest_create_usage_from_definition",
    ),
    (
        "crates/lsp_server/src/language/mod.rs",
        "suggest_qualify_ambiguous_name_quick_fixes",
    ),
    (
        "crates/lsp_server/src/language/mod.rs",
        "suggest_add_import_quick_fixes",
    ),
    (
        "crates/lsp_server/src/language/symbols.rs",
        "find_reference_ranges",
    ),
    (
        "crates/lsp_server/src/lsp_runtime/navigation.rs",
        "collect_document_links",
    ),
    (
        "crates/lsp_server/src/lsp_runtime/navigation.rs",
        "selection_ranges_for_positions",
    ),
    (
        "crates/lsp_server/src/semantic_tokens/mod.rs",
        "semantic_tokens_full",
    ),
    (
        "crates/lsp_server/src/semantic_tokens/mod.rs",
        "semantic_tokens_range",
    ),
];

/// Document-keyed maps a host declares as a field.
///
/// A map from `Url`/`PathBuf` to derived data is a cache of something the authority already
/// owns, and a second place for it to go stale. The session's index is the one legitimate
/// document-keyed map in a host; everything else reads the publication.
const HOST_DOCUMENT_KEYED_FIELD_ALLOWLIST: &[(&str, &str)] = &[
    // The editor session's document index: the one document-keyed map a host may own, because a
    // session's open documents are its own state and not a derivation of the publication.
    ("crates/lsp_server/src/session/state.rs", "ServerState"),
];

#[test]
fn no_sysml_text_entry_points_in_hosts() {
    let root = repository_root();
    let mut violations = Vec::new();
    for crate_name in HOST_CRATES {
        for file in host_sources(&root, crate_name) {
            let rel = relative(&root, &file);
            let allowed = HOST_TEXT_ENTRY_POINT_ALLOWLIST
                .iter()
                .filter(|(path, _)| *path == rel)
                .map(|(_, name)| *name)
                .collect::<Vec<_>>();
            let mut visitor = TextEntryPointVisitor {
                file: rel,
                allowed,
                violations: &mut violations,
            };
            visitor.visit_file(&parse_host_file(&file));
        }
    }
    assert!(
        violations.is_empty(),
        "these host functions take SysML text as a parameter; ask the syntax service instead:\n{}",
        violations.join("\n")
    );
}

#[test]
fn hosts_declare_no_document_keyed_maps_outside_the_session_allow_list() {
    let root = repository_root();
    let mut violations = Vec::new();
    for crate_name in HOST_CRATES {
        for file in host_sources(&root, crate_name) {
            let rel = relative(&root, &file);
            let allowed = HOST_DOCUMENT_KEYED_FIELD_ALLOWLIST
                .iter()
                .filter(|(path, _)| *path == rel)
                .map(|(_, name)| *name)
                .collect::<Vec<_>>();
            let mut visitor = DocumentKeyedFieldVisitor {
                file: rel,
                allowed,
                violations: &mut violations,
            };
            visitor.visit_file(&parse_host_file(&file));
        }
    }
    assert!(
        violations.is_empty(),
        "these host fields key derived data by document; the publication is the owner:\n{}",
        violations.join("\n")
    );
}

/// Production sources of one host crate: `src/**`, with `#[cfg(test)]` modules excluded by the
/// visitors below rather than by text slicing, so a test fixture never trips a guard.
fn host_sources(root: &Path, crate_name: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();
    rust_sources(
        &root.join("crates").join(crate_name).join("src"),
        &mut files,
    );
    files
}

fn relative(root: &Path, file: &Path) -> String {
    file.strip_prefix(root)
        .unwrap_or(file)
        .to_string_lossy()
        .replace('\\', "/")
}

fn parse_host_file(file: &Path) -> syn::File {
    let source = fs::read_to_string(file).expect("read host source");
    syn::parse_file(&source).unwrap_or_else(|error| panic!("parse {}: {error}", file.display()))
}

fn is_test_gated(attributes: &[syn::Attribute]) -> bool {
    attributes.iter().any(|attribute| {
        attribute.path().is_ident("cfg")
            && attribute
                .parse_args::<syn::Meta>()
                .is_ok_and(|meta| meta.path().is_ident("test"))
    })
}

/// A parameter named for SysML source text, typed as text.
fn is_text_parameter(argument: &syn::FnArg) -> Option<String> {
    let syn::FnArg::Typed(typed) = argument else {
        return None;
    };
    let syn::Pat::Ident(ident) = typed.pat.as_ref() else {
        return None;
    };
    let name = ident.ident.to_string();
    if !matches!(name.as_str(), "source" | "text" | "content" | "line") {
        return None;
    }
    is_text_type(&typed.ty).then_some(name)
}

fn is_text_type(ty: &Type) -> bool {
    match ty {
        Type::Reference(reference) => is_text_type(&reference.elem),
        Type::Path(path) => path
            .path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == "str" || segment.ident == "String"),
        _ => false,
    }
}

struct TextEntryPointVisitor<'a> {
    file: String,
    allowed: Vec<&'a str>,
    violations: &'a mut Vec<String>,
}

impl TextEntryPointVisitor<'_> {
    fn check(&mut self, signature: &Signature) {
        let name = signature.ident.to_string();
        if self.allowed.contains(&name.as_str()) {
            return;
        }
        for argument in &signature.inputs {
            if let Some(parameter) = is_text_parameter(argument) {
                self.violations
                    .push(format!("{}: fn {name}({parameter}: text)", self.file));
                return;
            }
        }
    }
}

impl<'ast> Visit<'ast> for TextEntryPointVisitor<'_> {
    fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
        if is_test_gated(&item.attrs) {
            return;
        }
        visit::visit_item_mod(self, item);
    }

    fn visit_item_fn(&mut self, item: &'ast syn::ItemFn) {
        if is_test_gated(&item.attrs) {
            return;
        }
        self.check(&item.sig);
        visit::visit_item_fn(self, item);
    }

    fn visit_impl_item_fn(&mut self, item: &'ast syn::ImplItemFn) {
        if is_test_gated(&item.attrs) {
            return;
        }
        self.check(&item.sig);
        visit::visit_impl_item_fn(self, item);
    }

    fn visit_trait_item_fn(&mut self, item: &'ast syn::TraitItemFn) {
        self.check(&item.sig);
        visit::visit_trait_item_fn(self, item);
    }
}

struct DocumentKeyedFieldVisitor<'a> {
    file: String,
    allowed: Vec<&'a str>,
    violations: &'a mut Vec<String>,
}

/// `HashMap<Url, _>`, `BTreeMap<Url, _>`, or the same keyed by `PathBuf`.
fn document_keyed_map(ty: &Type) -> bool {
    let Type::Path(path) = ty else {
        return false;
    };
    let Some(segment) = path.path.segments.last() else {
        return false;
    };
    if segment.ident != "HashMap" && segment.ident != "BTreeMap" {
        return false;
    }
    let syn::PathArguments::AngleBracketed(arguments) = &segment.arguments else {
        return false;
    };
    let Some(syn::GenericArgument::Type(Type::Path(key))) = arguments.args.first() else {
        return false;
    };
    key.path
        .segments
        .last()
        .is_some_and(|key| key.ident == "Url" || key.ident == "PathBuf")
}

impl<'ast> Visit<'ast> for DocumentKeyedFieldVisitor<'_> {
    fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
        if is_test_gated(&item.attrs) {
            return;
        }
        visit::visit_item_mod(self, item);
    }

    fn visit_item_struct(&mut self, item: &'ast syn::ItemStruct) {
        if is_test_gated(&item.attrs) {
            return;
        }
        let name = item.ident.to_string();
        if self.allowed.contains(&name.as_str()) {
            return;
        }
        if let Fields::Named(fields) = &item.fields {
            for field in &fields.named {
                if document_keyed_map(&field.ty) {
                    let field_name = field
                        .ident
                        .as_ref()
                        .map(ToString::to_string)
                        .unwrap_or_default();
                    self.violations
                        .push(format!("{}: {name}.{field_name}", self.file));
                }
            }
        }
    }
}
