//! Canonical S-expression projection of the typed semantic graph.
//!
//! This is a diagnostic projection, not a serialization format. It reads only
//! semantic graph state: parser trees, source ranges, caches, and display-oriented
//! attributes are excluded. Ordering is canonical, so construction order does not
//! affect the rendering.

use std::collections::HashMap;
use std::fmt::Write;

use petgraph::visit::{EdgeRef, IntoEdgeReferences};
use url::Url;

use crate::semantic::graph::{PendingExpressionRelationship, PendingRelationship, SemanticGraph};
use crate::semantic::model::{
    DeclaredExpression, DeclaredFeatureProperties, DeclaredFeatureValueKind,
    DeclaredMultiplicityBound, ExpressionResultRole, NodeId, SemanticEdge, SemanticNode,
};

const FORMAT_ROOT: &str = "semantic-graph";

struct CanonicalIdentities {
    document_labels: HashMap<Url, String>,
}

impl CanonicalIdentities {
    fn from_graph(graph: &SemanticGraph) -> Self {
        let mut documents = graph
            .nodes_by_uri
            .iter()
            .map(|(uri, ids)| {
                let mut nodes = ids
                    .iter()
                    .filter_map(|id| graph.get_node(id))
                    .map(node_sort_key)
                    .collect::<Vec<_>>();
                nodes.sort();
                // The URI breaks ties only to assign an otherwise invisible ordinal.
                // It is never emitted; a single document therefore remains stable when
                // moved, while duplicate qualified names remain unambiguous.
                (nodes, uri.clone())
            })
            .collect::<Vec<_>>();
        documents.sort_by(|(left_key, left_uri), (right_key, right_uri)| {
            left_key
                .cmp(right_key)
                .then_with(|| left_uri.as_str().cmp(right_uri.as_str()))
        });
        let document_labels = documents
            .into_iter()
            .enumerate()
            .map(|(index, (_, uri))| (uri, format!("d{index}")))
            .collect();
        Self { document_labels }
    }

    fn document(&self, uri: &Url) -> &str {
        self.document_labels
            .get(uri)
            .map(String::as_str)
            .expect("every semantic node and pending relationship has a document label")
    }

    fn node(&self, node: &SemanticNode) -> String {
        format!(
            "(node (document {}) (qualified-name {}))",
            atom(self.document(&node.id.uri)),
            atom(&node.id.qualified_name)
        )
    }

    fn node_id(&self, id: &NodeId) -> String {
        format!(
            "(node (document {}) (qualified-name {}))",
            atom(self.document(&id.uri)),
            atom(&id.qualified_name)
        )
    }
}

impl SemanticGraph {
    /// Renders this graph using the canonical diagnostic S-expression.
    ///
    /// The projection includes containment, resolved and pending typed relationships,
    /// and selected declared/effective facts. It intentionally excludes source ranges,
    /// document paths, caches, and legacy display attributes.
    pub fn to_semantic_sexpr(&self) -> String {
        let identities = CanonicalIdentities::from_graph(self);
        let mut output = format!("({FORMAT_ROOT}\n");
        render_containment(self, &identities, &mut output);
        render_resolved_relationships(self, &identities, &mut output);
        render_pending_relationships(&identities, &self.pending_relationships, &mut output);
        render_pending_expression_relationships(
            &identities,
            &self.pending_expression_relationships,
            &mut output,
        );
        output.push(')');
        output
    }
}

fn render_containment(
    graph: &SemanticGraph,
    identities: &CanonicalIdentities,
    output: &mut String,
) {
    write_indent(output, 1);
    output.push_str("(containment\n");
    let mut roots = graph
        .graph
        .node_weights()
        .filter(|node| {
            node.parent_id
                .as_ref()
                .is_none_or(|parent| graph.get_node(parent).is_none())
        })
        .collect::<Vec<_>>();
    roots.sort_by_key(|node| (node_sort_key(node), identities.node(node)));
    for root in roots {
        render_node(graph, identities, root, 2, output);
    }
    write_indent(output, 1);
    output.push_str(")\n");
}

fn render_node(
    graph: &SemanticGraph,
    identities: &CanonicalIdentities,
    node: &SemanticNode,
    depth: usize,
    output: &mut String,
) {
    write_indent(output, depth);
    let _ = write!(
        output,
        "(element (kind {}) (id {}) (name {})",
        atom(node.element_kind.as_str()),
        identities.node(node),
        atom(&node.name),
    );
    if let Some(declared_name) = &node.declared_name {
        let _ = write!(output, " (declared-name {})", atom(declared_name));
    }
    render_declared_facts(node, output);
    render_effective_facts(graph, identities, node, output);

    let mut children = graph.children_of(node);
    children.sort_by_key(|child| (node_sort_key(child), identities.node(child)));
    if children.is_empty() {
        output.push_str(")\n");
        return;
    }
    output.push('\n');
    write_indent(output, depth + 1);
    output.push_str("(contains\n");
    for child in children {
        render_node(graph, identities, child, depth + 2, output);
    }
    write_indent(output, depth + 1);
    output.push_str(")\n");
    write_indent(output, depth);
    output.push_str(")\n");
}

fn render_declared_facts(node: &SemanticNode, output: &mut String) {
    let facts = &node.declared_facts;
    if facts.multiplicity.is_none()
        && facts.feature_value.is_none()
        && facts.feature_properties.is_none()
        && facts.own_expression.is_none()
    {
        return;
    }
    output.push_str(" (declared");
    if let Some(properties) = &facts.feature_properties {
        render_feature_properties(properties, output);
    }
    if let Some(multiplicity) = &facts.multiplicity {
        let bounds = multiplicity.direct_bounds();
        let _ = write!(
            output,
            " (multiplicity (lower {}) (upper {}) (ordered {})",
            render_bound(bounds.lower),
            render_bound(bounds.upper),
            multiplicity.is_ordered
        );
        if let Some(unique) = multiplicity.is_unique {
            let _ = write!(output, " (unique {unique})");
        }
        output.push_str(if multiplicity.is_implied {
            " (provenance implied))"
        } else {
            " (provenance authored))"
        });
    }
    if let Some(feature_value) = &facts.feature_value {
        let _ = write!(
            output,
            " (feature-value (kind {}) ",
            feature_value_kind(feature_value.kind)
        );
        render_expression(&feature_value.expression, output);
        output.push(')');
    }
    if let Some(expression) = &facts.own_expression {
        output.push_str(" (own-expression ");
        render_expression(expression, output);
        output.push(')');
    }
    output.push(')');
}

fn render_feature_properties(properties: &DeclaredFeatureProperties, output: &mut String) {
    let has_property = properties.direction.is_some()
        || properties.is_abstract
        || properties.is_variation
        || properties.is_individual
        || properties.is_derived
        || properties.is_constant
        || properties.is_end
        || properties.is_composite.is_some()
        || properties.is_reference.is_some()
        || properties.is_conjugated
        || properties.is_ordered.is_some()
        || properties.is_unique.is_some()
        || properties.is_portion
        || properties.portion_kind.is_some();
    if !has_property {
        return;
    }
    output.push_str(" (properties");
    if let Some(direction) = &properties.direction {
        let _ = write!(output, " (direction {})", atom(direction));
    }
    for (name, value) in [
        ("abstract", properties.is_abstract),
        ("variation", properties.is_variation),
        ("individual", properties.is_individual),
        ("derived", properties.is_derived),
        ("constant", properties.is_constant),
        ("end", properties.is_end),
        ("conjugated", properties.is_conjugated),
        ("portion", properties.is_portion),
    ] {
        if value {
            let _ = write!(output, " ({name} true)");
        }
    }
    for (name, value) in [
        ("composite", properties.is_composite),
        ("reference", properties.is_reference),
        ("ordered", properties.is_ordered),
        ("unique", properties.is_unique),
    ] {
        if let Some(value) = value {
            let _ = write!(output, " ({name} {value})");
        }
    }
    if let Some(portion_kind) = &properties.portion_kind {
        let _ = write!(output, " (portion-kind {})", atom(portion_kind));
    }
    output.push(')');
}

fn render_effective_facts(
    graph: &SemanticGraph,
    identities: &CanonicalIdentities,
    node: &SemanticNode,
    output: &mut String,
) {
    let Some(facts) = graph.effective_facts_for(node) else {
        return;
    };
    output.push_str(" (effective");
    if let Some(multiplicity) = facts.implied_multiplicity {
        let _ = write!(
            output,
            " (implied-multiplicity (lower {}) (upper {}) (ordered {})",
            multiplicity.lower,
            multiplicity
                .upper
                .map_or_else(|| "unbounded".to_string(), |upper| upper.to_string()),
            multiplicity.is_ordered
        );
        if let Some(unique) = multiplicity.is_unique {
            let _ = write!(output, " (unique {unique})");
        }
        output.push(')');
    }
    if let Some(featuring_type) = &facts.featuring_type {
        let _ = write!(
            output,
            " (featuring-type {})",
            identities.node_id(featuring_type)
        );
    }
    if let Some(binding) = &facts.implied_feature_value_binding {
        let _ = write!(
            output,
            " (implied-feature-value-binding (owner {}) (role {}))",
            identities.node_id(&binding.expression_result.owner_id),
            expression_result_role(binding.expression_result.role)
        );
    }
    output.push(')');
}

fn render_resolved_relationships(
    graph: &SemanticGraph,
    identities: &CanonicalIdentities,
    output: &mut String,
) {
    let mut relationships = graph
        .graph
        .edge_references()
        .filter_map(|edge| {
            let source = graph.graph.node_weight(edge.source())?;
            let target = graph.graph.node_weight(edge.target())?;
            Some((source, target, edge.weight()))
        })
        .map(|(source, target, edge)| {
            render_resolved_relationship(identities, source, target, edge)
        })
        .collect::<Vec<_>>();
    relationships.sort();
    write_indent(output, 1);
    output.push_str("(relationships\n");
    for relationship in relationships {
        write_indent(output, 2);
        output.push_str(&relationship);
        output.push('\n');
    }
    write_indent(output, 1);
    output.push_str(")\n");
}

fn render_resolved_relationship(
    identities: &CanonicalIdentities,
    source: &SemanticNode,
    target: &SemanticNode,
    edge: &SemanticEdge,
) -> String {
    let mut output = format!(
        "({} (status resolved) (from {}) (to {})",
        edge.kind.as_str(),
        identities.node(source),
        identities.node(target)
    );
    if let Some(connect) = &edge.connect {
        output.push_str(" (connect");
        let _ = write!(
            output,
            " (source-expression {}) (target-expression {})",
            atom(&connect.source_expression),
            atom(&connect.target_expression)
        );
        if let Some(prefix) = &connect.container_prefix {
            let _ = write!(output, " (container-prefix {})", atom(prefix));
        }
        if connect.is_interface_usage {
            output.push_str(" (interface-usage true)");
        }
        if let Some(interface_type) = &connect.interface_type {
            let _ = write!(output, " (interface-type {})", atom(interface_type));
        }
        output.push(')');
    }
    if let Some(flow) = &edge.flow {
        output.push_str(" (flow");
        for (name, value) in [
            ("payload-expression", flow.payload_expression.as_ref()),
            ("source-expression", flow.source_expression.as_ref()),
            ("target-expression", flow.target_expression.as_ref()),
            ("payload-type-id", flow.payload_type_id.as_ref()),
        ] {
            if let Some(value) = value {
                let _ = write!(output, " ({name} {})", atom(value));
            }
        }
        output.push(')');
    }
    output.push(')');
    output
}

fn render_pending_relationships(
    identities: &CanonicalIdentities,
    pending: &[PendingRelationship],
    output: &mut String,
) {
    let mut pending = pending
        .iter()
        .map(|relationship| {
            let target_kinds = relationship
                .target_kinds
                .as_ref()
                .map(|kinds| {
                    let mut kinds = kinds
                        .iter()
                        .map(|kind| atom(kind.as_str()))
                        .collect::<Vec<_>>();
                    kinds.sort();
                    format!(" (target-kinds {})", kinds.join(" "))
                })
                .unwrap_or_default();
            format!(
                "({} (status pending) (document {}) (source-qualified {}) (target-qualified {}){})",
                relationship.kind.as_str(),
                atom(identities.document(&relationship.uri)),
                atom(&relationship.source_qualified),
                atom(&relationship.target_qualified),
                target_kinds
            )
        })
        .collect::<Vec<_>>();
    pending.sort();
    render_block("pending-relationships", &pending, output);
}

fn render_pending_expression_relationships(
    identities: &CanonicalIdentities,
    pending: &[PendingExpressionRelationship],
    output: &mut String,
) {
    let mut pending = pending
        .iter()
        .map(|relationship| {
            let mut output = format!(
                "({} (status pending-expression) (document {}) (source-expression {}) (target-expression {})",
                relationship.kind.as_str(),
                atom(identities.document(&relationship.uri)),
                atom(&relationship.source_expression),
                atom(&relationship.target_expression),
            );
            if let Some(prefix) = &relationship.container_prefix {
                let _ = write!(output, " (container-prefix {})", atom(prefix));
            }
            if relationship.is_interface_usage {
                output.push_str(" (interface-usage true)");
            }
            if let Some(interface_type) = &relationship.interface_type {
                let _ = write!(output, " (interface-type {})", atom(interface_type));
            }
            output.push(')');
            output
        })
        .collect::<Vec<_>>();
    pending.sort();
    render_block("pending-expression-relationships", &pending, output);
}

fn render_block(name: &str, entries: &[String], output: &mut String) {
    write_indent(output, 1);
    let _ = writeln!(output, "({name}");
    for entry in entries {
        write_indent(output, 2);
        output.push_str(entry);
        output.push('\n');
    }
    write_indent(output, 1);
    output.push_str(")\n");
}

fn render_expression(expression: &DeclaredExpression, output: &mut String) {
    let _ = write!(output, "(expression (kind {})", atom(&expression.kind));
    if let Some(literal) = &expression.literal {
        let _ = write!(output, " (literal {})", canonical_json(literal));
    }
    if let Some(reference) = &expression.reference {
        let _ = write!(output, " (reference {})", atom(reference));
    }
    if let Some(operator) = &expression.operator {
        let _ = write!(output, " (operator {})", atom(operator));
    }
    if !expression.children.is_empty() {
        output.push_str(" (children");
        for child in &expression.children {
            output.push(' ');
            render_expression(child, output);
        }
        output.push(')');
    }
    if !expression.arguments.is_empty() {
        output.push_str(" (arguments");
        for argument in &expression.arguments {
            output.push_str(" (argument");
            if let Some(name) = &argument.name {
                let _ = write!(output, " (name {})", atom(name));
            }
            output.push(' ');
            render_expression(&argument.value, output);
            output.push(')');
        }
        output.push(')');
    }
    output.push(')');
}

fn render_bound(bound: DeclaredMultiplicityBound) -> String {
    match bound {
        DeclaredMultiplicityBound::Unbounded => "unbounded".to_string(),
        DeclaredMultiplicityBound::Integer(value) => value.to_string(),
        DeclaredMultiplicityBound::NonIntegerLiteral => "non-integer-literal".to_string(),
        DeclaredMultiplicityBound::Unevaluated => "unevaluated".to_string(),
    }
}

fn feature_value_kind(kind: DeclaredFeatureValueKind) -> &'static str {
    match kind {
        DeclaredFeatureValueKind::Default => "default",
        DeclaredFeatureValueKind::Initial => "initial",
        DeclaredFeatureValueKind::Bound => "bound",
        DeclaredFeatureValueKind::Override => "override",
    }
}

fn expression_result_role(role: ExpressionResultRole) -> &'static str {
    match role {
        ExpressionResultRole::FeatureValue => "feature-value",
    }
}

fn canonical_json(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Null => "null".to_string(),
        serde_json::Value::Bool(value) => value.to_string(),
        serde_json::Value::Number(value) => value.to_string(),
        serde_json::Value::String(value) => atom(value),
        serde_json::Value::Array(values) => {
            let values = values
                .iter()
                .map(canonical_json)
                .collect::<Vec<_>>()
                .join(" ");
            if values.is_empty() {
                "(array)".to_string()
            } else {
                format!("(array {values})")
            }
        }
        serde_json::Value::Object(values) => {
            let mut entries = values.iter().collect::<Vec<_>>();
            entries.sort_by_key(|(key, _)| *key);
            let values = entries
                .into_iter()
                .map(|(key, value)| format!(" ({} {})", atom(key), canonical_json(value)))
                .collect::<String>();
            format!("(object{values})")
        }
    }
}

fn node_sort_key(node: &SemanticNode) -> (String, String, String, String) {
    let mut declared = String::new();
    render_declared_facts(node, &mut declared);
    (
        node.id.qualified_name.clone(),
        node.element_kind.as_str().to_string(),
        node.name.clone(),
        declared,
    )
}

fn atom(value: &str) -> String {
    serde_json::to_string(value).expect("strings serialize")
}

fn write_indent(output: &mut String, depth: usize) {
    output.push_str(&"  ".repeat(depth));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::semantic::model::{DeclaredSemanticFacts, ElementKind, NodeId};
    use crate::semantic::pipeline::{build_and_link_graph, patch_graph_for_document};
    use crate::semantic::source::{SysmlDocument, SysmlDocumentSourceKind};
    use crate::semantic::text_span::{TextPosition, TextRange};
    use serde_json::json;

    fn document(path: &str, content: &str) -> SysmlDocument {
        SysmlDocument::from_memory_path(
            "semantic-sexpr-test",
            path,
            content.to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("memory document")
    }

    #[test]
    fn renders_containment_resolved_edges_and_provenance() {
        let document = document(
            "model.sysml",
            "package P { part def Engine; part engine : Engine; }",
        );
        let (graph, _) = build_and_link_graph(&[document]).expect("semantic graph");

        assert_eq!(
            graph.to_semantic_sexpr(),
            r#"(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "P"))) (name "P") (declared-name "P")
      (contains
        (element (kind "part def") (id (node (document "d0") (qualified-name "P::Engine"))) (name "Engine") (declared-name "Engine") (declared))
        (element (kind "part") (id (node (document "d0") (qualified-name "P::engine"))) (name "engine") (declared-name "engine") (declared (properties (composite true) (reference false) (ordered false))))
      )
    )
  )
  (relationships
    (typing (status resolved) (from (node (document "d0") (qualified-name "P::engine"))) (to (node (document "d0") (qualified-name "P::Engine"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)"#
        );
    }

    #[test]
    fn source_uris_and_legacy_attributes_do_not_change_projection() {
        let content = "package P { part def Engine; part engine : Engine; }";
        let (left, _) = build_and_link_graph(&[document("one.sysml", content)]).expect("left");
        let (right, _) =
            build_and_link_graph(&[document("another/path.sysml", content)]).expect("right");
        assert_eq!(left.to_semantic_sexpr(), right.to_semantic_sexpr());

        let uri = Url::parse("memory://semantic-sexpr-test/display.sysml").expect("URI");
        let node = SemanticNode {
            id: NodeId::new(&uri, "DisplayOnly"),
            element_kind: ElementKind::Part,
            declared_name: Some("DisplayOnly".to_string()),
            name: "DisplayOnly".to_string(),
            range: TextRange::new(TextPosition::new(0, 0), TextPosition::new(0, 0)),
            attributes: [(
                "pretendSemanticFact".to_string(),
                json!("not-authoritative"),
            )]
            .into_iter()
            .collect(),
            declared_facts: DeclaredSemanticFacts::default(),
            parent_id: None,
        };
        let mut graph = SemanticGraph::new();
        graph.insert_workspace_node(node);
        assert!(!graph.to_semantic_sexpr().contains("pretendSemanticFact"));
    }

    #[test]
    fn duplicate_qualified_names_remain_unambiguous_across_documents() {
        let documents = [
            document("alpha.sysml", "package P { part def Engine; }"),
            document("beta.sysml", "package P { part def Engine; }"),
        ];
        let (graph, _) = build_and_link_graph(&documents).expect("semantic graph");
        let output = graph.to_semantic_sexpr();
        assert!(output.contains("(document \"d0\") (qualified-name \"P\")"));
        assert!(output.contains("(document \"d1\") (qualified-name \"P\")"));
    }

    #[test]
    fn full_and_incremental_builds_have_the_same_projection() {
        let documents = vec![
            document("types.sysml", "package Types { part def Engine; }"),
            document(
                "uses.sysml",
                "package Uses { private import Types::*; part engine : Engine; }",
            ),
        ];
        let (full, _) = build_and_link_graph(&documents).expect("full graph");

        let mut incremental = SemanticGraph::new();
        for document in &documents {
            let parsed = sysml_v2_parser::parse(&document.content).expect("parse");
            patch_graph_for_document(&mut incremental, &document.uri, Some(&parsed), true);
        }
        assert_eq!(full.to_semantic_sexpr(), incremental.to_semantic_sexpr());
    }

    #[test]
    fn document_and_parallel_order_do_not_change_projection() {
        let first = document("a.sysml", "package A { part def Engine; }");
        let second = document("b.sysml", "package B { part engine : A::Engine; }");
        let (forward, _) = build_and_link_graph(&[first.clone(), second.clone()]).expect("forward");
        let (reverse, _) = build_and_link_graph(&[second, first]).expect("reverse");
        assert_eq!(forward.to_semantic_sexpr(), reverse.to_semantic_sexpr());
    }
}
