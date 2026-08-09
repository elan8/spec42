//! Architectural ownership contracts for post-construction semantic consumers.
//!
//! Graph builders are intentionally excluded because they project parser facts into the
//! semantic graph. Projection and rendering modules are also excluded: they may format an
//! explicit projection, but must not be used as semantic or diagnostic decision makers.

use std::fs;
use std::path::{Path, PathBuf};

use syn::visit::Visit;
use syn::{Expr, ExprLit, ExprMethodCall, Lit, Member};

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
fn visitor_catches_receiver_and_whitespace_variations_without_reading_comments_or_strings() {
    let source = r#"
        fn example(node: Node) {
            let _ = node.attributes . get ( "partType" );
            let _ = (&node.attributes).contains_key("redefines");
            let _ = node.attributes.get("typeRef");
            // node.attributes.get("portType");
            let text = "node.attributes.get(\\\"actionType\\\")";
            let _ = text;
        }
    "#;
    let parsed = syn::parse_file(source).expect("parse guardrail fixture");
    let mut visitor = RelationshipProjectionAttributeVisitor::default();
    visitor.visit_file(&parsed);
    assert_eq!(visitor.keys, vec!["partType", "redefines", "typeRef"]);
}

#[derive(Default)]
struct RelationshipProjectionAttributeVisitor {
    keys: Vec<String>,
}

impl<'ast> Visit<'ast> for RelationshipProjectionAttributeVisitor {
    fn visit_expr_method_call(&mut self, call: &'ast ExprMethodCall) {
        if !matches!(call.method.to_string().as_str(), "get" | "contains_key")
            || !receiver_is_attributes(&call.receiver)
        {
            syn::visit::visit_expr_method_call(self, call);
            return;
        }
        if let Some(key) = call.args.first().and_then(string_literal) {
            if RELATIONSHIP_PROJECTION_KEYS
                .iter()
                .any(|candidate| *candidate == key)
            {
                self.keys.push(key);
            }
        }
        syn::visit::visit_expr_method_call(self, call);
    }
}

fn receiver_is_attributes(expr: &Expr) -> bool {
    match expr {
        Expr::Field(field) => matches!(&field.member, Member::Named(name) if name == "attributes"),
        Expr::Paren(paren) => receiver_is_attributes(&paren.expr),
        Expr::Reference(reference) => receiver_is_attributes(&reference.expr),
        _ => false,
    }
}

fn string_literal(expr: &Expr) -> Option<String> {
    let Expr::Lit(ExprLit {
        lit: Lit::Str(value),
        ..
    }) = expr
    else {
        return None;
    };
    Some(value.value())
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
        "sequence_views",
        "explicit_views",
    ];
    const EXCLUDED_MODULES: &[&str] = &[
        "dto.rs",
        "view_projection.rs",
        "interconnection_projection.rs",
        "component_view.rs",
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
