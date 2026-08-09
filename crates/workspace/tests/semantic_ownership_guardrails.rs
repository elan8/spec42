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
];

#[test]
fn post_construction_semantic_and_diagnostic_consumers_do_not_read_relationship_projections() {
    let root = repository_root();
    let roots = [
        root.join("crates/sysml_model/src/semantic"),
        root.join("crates/sysml_diagnostics/src"),
    ];
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

#[derive(Default)]
struct RelationshipProjectionAttributeVisitor {
    keys: Vec<String>,
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
