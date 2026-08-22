//! Architectural ownership contracts for post-construction semantic consumers.
//!
//! Graph builders are intentionally excluded because they project parser facts into the
//! semantic graph. Only DTO/projection serialization modules are excluded, except the
//! legacy sequence extractor: it is isolated until an explicit canonical sequence-role/profile
//! identity provider replaces its existing name-based role classification.

use std::fs;
use std::path::{Path, PathBuf};

use syn::visit::Visit;
use syn::LitStr;

const RELATIONSHIP_PROJECTION_KEYS: &[&str] = &[
    "partType",
    "refType",
    "attributeType",
    "portType",
    "actionType",
    "actorType",
    "itemType",
    "occurrenceType",
    "flowType",
    "allocationType",
    "stateType",
    "requirementType",
    "useCaseType",
    "concernType",
    "viewType",
    "viewpointType",
    "renderingType",
    "subjectType",
    "analysisType",
    "verificationType",
    "connectionType",
    "metadataType",
    "keywordType",
    "specializes",
    "subsetsFeature",
    "redefines",
    "referencesFeature",
    "crossesFeature",
    "typeRef",
    "valueType",
    "endType",
    "metaclassRole",
    "refTarget",
    "keyword",
    // `parameterType` is a pure duplicate of the
    // first `DeclaredRelationshipFacts::typing` target (same family as `partType`/`portType`/
    // `refType`/`attributeType` above). `payloadType`/`acceptType` are not relationship-typing
    // duplicates -- they back the genuinely separate `DeclaredSemanticFacts::
    // payload_type_reference`/`accept_type_reference` facts -- but share the same enforcement:
    // post-construction consumers must read the typed fact, not the legacy projection key.
    "parameterType",
    "payloadType",
    "acceptType",
];

/// `*Type` attribute projections that have been retired outright: their producers were removed
/// because `DeclaredRelationshipFacts::typing` already carries the same authored typing, and their
/// consumers now read that typed fact. Unlike `RELATIONSHIP_PROJECTION_KEYS`, which only bars
/// post-construction *consumers*, these keys must not reappear anywhere in production source —
/// including the graph builders that used to write them.
const RETIRED_TYPING_PROJECTION_KEYS: &[&str] = &[
    // Redundant with `attach_declared_subsetting_family`'s
    // `DeclaredRelationshipFacts::reference_subsetting`/`cross_subsetting` typed facts: an
    // exhaustive sweep found no reader anywhere.
    "referencesFeature",
    "crossesFeature",
    "actionType",
    "actorType",
    "allocationType",
    "analysisType",
    "calcType",
    "caseType",
    "concernType",
    "connectionType",
    "constraintType",
    "enumerationType",
    "flowType",
    "itemType",
    "keywordType",
    "metadataType",
    "objectiveType",
    "occurrenceType",
    "requirementType",
    "stakeholderType",
    "stateType",
    "subjectType",
    "useCaseType",
    "verificationType",
    "viewType",
    "viewpointType",
];

const MEMBERSHIP_IMPORT_PROJECTION_KEYS: &[&str] = &[
    "visibility",
    "importTarget",
    "importAll",
    "recursive",
    "isExpose",
];

#[test]
fn membership_and_import_attribute_keys_are_confined_to_presentation() {
    let root = repository_root();
    let mut violations = Vec::new();
    visit_repository_rust_files(&root.join("crates"), &mut |path| {
        if is_presentation_only_membership_key_owner(path) {
            return;
        }
        let source = fs::read_to_string(path).expect("read production Rust module");
        let parsed = syn::parse_file(&source)
            .unwrap_or_else(|error| panic!("{}: Rust parse error: {error}", path.display()));
        let mut visitor = MembershipImportProjectionKeyVisitor::default();
        visitor.visit_file(&parsed);
        for key in visitor.keys {
            violations.push(format!("{}: forbidden attribute key {key}", path.display()));
        }
    });
    assert!(
        violations.is_empty(),
        "membership/import semantics must use typed graph facts, not projection keys:\n{}",
        violations.join("\n")
    );
}

#[test]
fn membership_import_key_guard_rejects_code_and_ignores_comments() {
    let cases = [
        (
            r#"fn f(node: Node) { let _ = node.attributes.get("importTarget"); }"#,
            vec!["importTarget"],
        ),
        (
            r#"// attributes.get("visibility"); const EXAMPLE: &str = "safe prose";"#,
            vec![],
        ),
    ];
    for (source, expected) in cases {
        let parsed = syn::parse_file(source).expect("guard fixture parses");
        let mut visitor = MembershipImportProjectionKeyVisitor::default();
        visitor.visit_file(&parsed);
        assert_eq!(visitor.keys, expected);
    }
}

#[test]
fn post_construction_semantic_and_diagnostic_consumers_do_not_read_relationship_projections() {
    let root = repository_root();
    let roots = [root.join("crates/sysml_diagnostics/src")];
    let mut violations = Vec::new();
    for scan_root in roots {
        visit_production_modules(&scan_root, &mut |path| {
            let source = fs::read_to_string(path).expect("read production Rust module");
            let parsed = syn::parse_file(&source)
                .unwrap_or_else(|error| panic!("{}: Rust parse error: {error}", path.display()));
            let mut visitor = RelationshipProjectionAttributeVisitor::default();
            visitor.visit_file(&parsed);
            for key in visitor.keys {
                violations.push(format!("{}: attributes access for {key}", path.display()));
            }
        });
    }
    assert!(
        violations.is_empty(),
        "post-construction semantic and diagnostic decisions must consume declared facts or resolved edges, not relationship projection attributes:\n{}",
        violations.join("\n")
    );
}

#[test]
fn visitor_catches_direct_helper_const_and_loop_indirection_without_reading_comments_or_strings() {
    let fixtures = [
        (
            "direct",
            r#"fn example(node: Node) { let _ = node.attributes . get ( "partType" ); }"#,
            vec!["partType"],
        ),
        (
            "helper and const",
            r#"
                const TYPE_KEY: &str = "refType";
                fn read_attribute(node: Node, key: &str) { let _ = node.attributes.get(key); }
                fn example(node: Node) { read_attribute(node, TYPE_KEY); }
            "#,
            vec!["refType"],
        ),
        (
            "loop",
            r#"fn example(node: Node) { for key in ["stateType"] { let _ = node.attributes.get(key); } }"#,
            vec!["stateType"],
        ),
        (
            "comments and encoded code",
            r#"
                // node.attributes.get("portType");
                const TEXT: &str = "node.attributes.get(\\\"actionType\\\")";
            "#,
            vec![],
        ),
    ];
    for (name, source, expected) in fixtures {
        let parsed = syn::parse_file(source)
            .unwrap_or_else(|error| panic!("parse {name} guardrail fixture: {error}"));
        let mut visitor = RelationshipProjectionAttributeVisitor::default();
        visitor.visit_file(&parsed);
        assert_eq!(visitor.keys, expected, "{name} fixture");
    }
}

#[test]
fn retired_typing_projection_keys_are_absent_from_production_source() {
    let root = repository_root();
    let mut violations = Vec::new();
    visit_repository_rust_files(&root.join("crates"), &mut |path| {
        let source = fs::read_to_string(path).expect("read production Rust module");
        let parsed = syn::parse_file(&source)
            .unwrap_or_else(|error| panic!("{}: Rust parse error: {error}", path.display()));
        let mut visitor = RetiredTypingKeyVisitor::default();
        visitor.visit_file(&parsed);
        for key in visitor.keys {
            violations.push(format!("{}: retired attribute key {key}", path.display()));
        }
    });
    assert!(
        violations.is_empty(),
        "retired `*Type` projections must not be reintroduced; the authored typing lives in \
         DeclaredRelationshipFacts::typing:\n{}",
        violations.join("\n")
    );
}

#[test]
fn retired_typing_key_guard_rejects_producers_and_consumers_but_not_comments() {
    let cases = [
        (
            r#"fn f(a: &mut Map) { a.insert("stateType".to_string(), json!(t)); }"#,
            vec!["stateType"],
        ),
        (
            r#"fn f(node: Node) { let _ = node.attributes.get("itemType"); }"#,
            vec!["itemType"],
        ),
        (
            r#"// attributes.get("viewType"); const SAFE: &str = "prose about typing";"#,
            vec![],
        ),
    ];
    for (source, expected) in cases {
        let parsed = syn::parse_file(source).expect("retired-key guard fixture parses");
        let mut visitor = RetiredTypingKeyVisitor::default();
        visitor.visit_file(&parsed);
        assert_eq!(visitor.keys, expected);
    }
}

#[derive(Default)]
struct RetiredTypingKeyVisitor {
    keys: Vec<String>,
}

impl<'ast> Visit<'ast> for RetiredTypingKeyVisitor {
    fn visit_lit_str(&mut self, literal: &'ast LitStr) {
        let key = literal.value();
        if RETIRED_TYPING_PROJECTION_KEYS.contains(&key.as_str()) {
            self.keys.push(key);
        }
        syn::visit::visit_lit_str(self, literal);
    }
}

#[derive(Default)]
struct RelationshipProjectionAttributeVisitor {
    keys: Vec<String>,
}

#[derive(Default)]
struct MembershipImportProjectionKeyVisitor {
    keys: Vec<String>,
}

impl<'ast> Visit<'ast> for MembershipImportProjectionKeyVisitor {
    fn visit_lit_str(&mut self, literal: &'ast LitStr) {
        let key = literal.value();
        if MEMBERSHIP_IMPORT_PROJECTION_KEYS.contains(&key.as_str()) {
            self.keys.push(key);
        }
        syn::visit::visit_lit_str(self, literal);
    }
}

impl<'ast> Visit<'ast> for RelationshipProjectionAttributeVisitor {
    fn visit_lit_str(&mut self, literal: &'ast LitStr) {
        let key = literal.value();
        if RELATIONSHIP_PROJECTION_KEYS
            .iter()
            .any(|candidate| *candidate == key)
        {
            self.keys.push(key);
        }
        syn::visit::visit_lit_str(self, literal);
    }
}

fn visit_production_modules(root: &Path, visit: &mut impl FnMut(&Path)) {
    for entry in fs::read_dir(root).expect("read production module directory") {
        let path = entry.expect("read directory entry").path();
        if is_excluded(&path) {
            continue;
        }
        if path.is_dir() {
            visit_production_modules(&path, visit);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            visit(&path);
        }
    }
}

fn is_excluded(path: &Path) -> bool {
    const EXCLUDED_DIRECTORIES: &[&str] = &[
        "graph_builder",
        "model_projection",
        "visualization",
        "ibd",
        "prepared_view",
        // Isolated legacy debt: requires a canonical sequence-role/profile identity provider.
        "sequence_views",
    ];
    const EXCLUDED_MODULES: &[&str] = &[
        "dto.rs",
        "view_projection.rs",
        "interconnection_projection.rs",
        "component_view.rs",
        // Owns relationship spelling serialization, not a consumer decision.
        "model.rs",
        // Canonical debug/snapshot serialization of already-typed declared relationship facts.
        // This renderer never reads the legacy `attributes` projection map.
        "graph_sexpr.rs",
        // Boundary DTO projection (`project_source_text_attributes`,
        // `project_relationship_target_attributes`, `project_expression_text_attributes`):
        // projects typed facts onto a transport DTO's `attributes` map at construction sites
        // only, never reading the map back into a semantic decision.
        "model_projection.rs",
    ];
    path.components().any(|component| {
        EXCLUDED_DIRECTORIES
            .iter()
            .any(|excluded| component.as_os_str() == *excluded)
    }) || path
        .file_name()
        .is_some_and(|name| EXCLUDED_MODULES.contains(&name.to_string_lossy().as_ref()))
}

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("workspace repository root")
        .to_path_buf()
}

fn visit_repository_rust_files(root: &Path, visit: &mut impl FnMut(&Path)) {
    for entry in fs::read_dir(root).expect("read repository crate directory") {
        let path = entry.expect("read repository crate entry").path();
        if path.file_name().is_some_and(|name| name == "target") {
            continue;
        }
        if path.is_dir() {
            visit_repository_rust_files(&path, visit);
        } else if path.extension().is_some_and(|extension| extension == "rs")
            && path
                .components()
                .any(|component| component.as_os_str() == "src")
        {
            visit(&path);
        }
    }
}

fn is_presentation_only_membership_key_owner(path: &Path) -> bool {
    // Compare against a `/`-normalized path so this matches on Windows too,
    // where `path` carries `\` separators but the allowlist below is written
    // with `/` (matching the crate-relative form used everywhere else in this file).
    let normalized = path.to_string_lossy().replace('\\', "/");
    normalized.ends_with("crates/language_service/src/presentation_hover.rs")
        || normalized.ends_with("crates/workspace/src/comparison/relationships.rs")
}
