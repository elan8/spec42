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
const MODEL_IMPLEMENTATION_OWNERS: &[&str] = &["sysml_query"];
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

    let known = MODEL_IMPLEMENTATION_OWNERS
        .iter()
        .chain(TRANSITIONAL_DIRECT_CONSUMERS)
        .copied()
        .collect::<BTreeSet<_>>();
    assert_eq!(
        direct_model_dependencies, known,
        "direct sysml_model dependency inventory changed; additions bypass the query boundary, \
         while removals must shrink TRANSITIONAL_DIRECT_CONSUMERS in the same change"
    );
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
        "SemanticCompleteness",
        "SemanticConfiguration",
        "SemanticModel",
        "SemanticModelIdentity",
        "SemanticPhase",
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
        for forbidden in FORBIDDEN_PUBLIC_TYPES {
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
