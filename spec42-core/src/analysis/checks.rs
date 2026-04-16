//! Semantic validation beyond syntax: port connectivity, type compatibility, etc.
//!
//! These checks use the semantic graph (parts, ports, connections) to report
//! diagnostics such as: unconnected ports, connection to non-port, port type mismatch.

use std::collections::HashSet;
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, Url};

use crate::analysis::helpers::*;
use crate::semantic_model::{resolve_expression_endpoint_strict, NodeId, ResolveResult, SemanticGraph};

fn has_import_in_scope(graph: &SemanticGraph, node: &crate::semantic_model::SemanticNode) -> bool {
    let mut current = Some(node.id.clone());
    while let Some(node_id) = current {
        let Some(scope_node) = graph.get_node(&node_id) else {
            break;
        };
        if graph
            .children_of(scope_node)
            .into_iter()
            .any(|child| child.element_kind == "import")
        {
            return true;
        }
        current = scope_node.parent_id.clone();
    }
    false
}

/// Returns LSP diagnostics for semantic rules in the given document.
/// Only runs when the document has been parsed and merged into the graph.
pub fn compute_semantic_diagnostics(graph: &SemanticGraph, uri: &Url) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    // 0) Explicit builder diagnostics (e.g. ambiguous endpoint resolution).
    for node in graph.nodes_for_uri(uri) {
        if node.element_kind != "diagnostic" {
            continue;
        }
        let code = node
            .attributes
            .get("code")
            .and_then(|v| v.as_str())
            .unwrap_or("semantic_diagnostic");
        let message = node
            .attributes
            .get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("semantic diagnostic")
            .to_string();
        if should_suppress_builder_diagnostic(graph, uri, node, code, &message) {
            continue;
        }
        diagnostics.push(diag(
            diagnostic_range(graph, node, None),
            DiagnosticSeverity::WARNING,
            "semantic",
            code,
            message,
        ));
    }

    // 1) Connection endpoints must be ports; port types must be compatible
    let connection_occurrences = graph.connection_edge_occurrences_for_uri(uri);
    for (src_id, tgt_id, connection_range) in connection_occurrences {
        if let (Some(src), Some(tgt)) = (graph.get_node(&src_id), graph.get_node(&tgt_id)) {
            if !is_port_like(&src.element_kind) {
                diagnostics.push(diag(
                    diagnostic_range(graph, src, Some(tgt)),
                    DiagnosticSeverity::WARNING,
                    "semantic",
                    "connection_endpoint_not_port",
                    format!(
                        "Connection source '{}' is not a port (element kind: {}).",
                        src.name, src.element_kind
                    ),
                ));
            }
            if !is_port_like(&tgt.element_kind) {
                diagnostics.push(diag(
                    diagnostic_range(graph, tgt, Some(src)),
                    DiagnosticSeverity::WARNING,
                    "semantic",
                    "connection_endpoint_not_port",
                    format!(
                        "Connection target '{}' is not a port (element kind: {}).",
                        tgt.name, tgt.element_kind
                    ),
                ));
            }
            if is_port_like(&src.element_kind) && is_port_like(&tgt.element_kind) {
                if let Some(msg) = port_type_mismatch(src, tgt) {
                    diagnostics.push(diag(
                        connection_range,
                        DiagnosticSeverity::WARNING,
                        "semantic",
                        "port_type_mismatch",
                        msg,
                    ));
                }
            }
        }
    }

    // 2) Unconnected ports (ports in this URI that are not an endpoint of any connection)
    let connected_port_keys: HashSet<String> = graph
        .connection_edge_node_pairs_for_uri(uri)
        .into_iter()
        .flat_map(|(a, b)| [a, b])
        .filter_map(|id| graph.get_node(&id))
        .filter_map(port_anchor_key)
        .collect();

    for node in graph.nodes_for_uri(uri) {
        if is_port_like(&node.element_kind)
            && node.element_kind == "port"
            && !is_synthetic(node)
            && is_declaration_port(graph, node)
            && port_anchor_key(node)
                .as_ref()
                .is_some_and(|key| !connected_port_keys.contains(key))
        {
            diagnostics.push(diag(
                diagnostic_range(graph, node, None),
                DiagnosticSeverity::INFORMATION,
                "semantic",
                "unconnected_port",
                format!("Port '{}' is not connected to any other port.", node.name),
            ));
        }
    }

    // 3) Duplicate connections (same pair of endpoints connected more than once)
    let mut seen_pairs: HashSet<(NodeId, NodeId)> = HashSet::new();
    for (src_id, tgt_id) in graph.connection_edge_node_pairs_for_uri(uri) {
        let pair = normalize_edge_pair(&src_id, &tgt_id);
        if !seen_pairs.insert(pair) {
            if let Some(tgt) = graph.get_node(&tgt_id) {
                diagnostics.push(diag(
                    diagnostic_range(graph, tgt, None),
                    DiagnosticSeverity::INFORMATION,
                    "semantic",
                    "duplicate_connection",
                    "Duplicate connection between the same two endpoints.".to_string(),
                ));
            }
        }
    }

    // 4) Multiplicity validation (syntax and interval sanity)
    for node in graph.nodes_for_uri(uri) {
        if let Some(multiplicity) = node.attributes.get("multiplicity").and_then(|v| v.as_str()) {
            if let Some(message) = multiplicity_issue_message(multiplicity) {
                diagnostics.push(diag(
                    diagnostic_range(graph, node, None),
                    DiagnosticSeverity::WARNING,
                    "semantic",
                    "invalid_multiplicity",
                    format!("Invalid multiplicity on '{}': {message}", node.name),
                ));
            }
        }
    }

    // 5) Stronger typing checks: declarations that name a type should resolve via typing/specializes.
    let mut unresolved_seen: HashSet<String> = HashSet::new();
    for node in graph.nodes_for_uri(uri) {
        if is_synthetic(node) {
            continue;
        }
        let Some(type_ref) = declared_type_ref(node) else {
            continue;
        };
        let normalized_type_ref = normalize_declared_type_ref(type_ref);
        if is_builtin_type_ref(&normalized_type_ref) {
            continue;
        }
        let has_resolved_type = !graph
            .outgoing_typing_or_specializes_targets(node)
            .is_empty();
        let resolved_via_import_scope = !crate::semantic_model::resolve_type_reference_targets(
            graph,
            node,
            type_ref,
            &[
                "part def",
                "port def",
                "interface",
                "item def",
                "attribute def",
                "action def",
                "actor def",
                "occurrence def",
                "flow def",
                "allocation def",
                "state def",
                "requirement def",
                "use case def",
                "concern def",
                "enum def",
                "alias",
                "kermlDecl",
            ],
        )
        .is_empty();
        let resolved_via_graph_name_fallback = graph.nodes_named(&normalized_type_ref).iter().any(
            |candidate| {
                matches!(
                    candidate.element_kind.as_str(),
                    "part def"
                        | "port def"
                        | "interface"
                        | "item def"
                        | "attribute def"
                        | "action def"
                        | "actor def"
                        | "occurrence def"
                        | "flow def"
                        | "allocation def"
                        | "state def"
                        | "requirement def"
                        | "use case def"
                        | "concern def"
                        | "enum def"
                        | "alias"
                        | "kermlDecl"
                )
            },
        );
        let allow_graph_name_fallback = !has_import_in_scope(graph, node);
        if has_resolved_type
            || resolved_via_import_scope
            || (allow_graph_name_fallback && resolved_via_graph_name_fallback)
        {
            continue;
        }
        let Some(range) = unresolved_type_diagnostic_range(node) else {
            continue;
        };
        let key = format!(
            "{}|{}|{}:{}:{}:{}:{}",
            node.id.qualified_name,
            normalized_type_ref,
            range.start.line,
            range.start.character,
            range.end.line,
            range.end.character,
            node.name
        );
        if !unresolved_seen.insert(key) {
            continue;
        }
        diagnostics.push(diag(
            range,
            DiagnosticSeverity::WARNING,
            "semantic",
            "unresolved_type_reference",
            format!(
                "Type reference '{}' for '{}' could not be resolved in the semantic graph.",
                type_ref, node.name
            ),
        ));

        // Debug observability: print normalized type ref and node id when enabled.
        if crate::common::util::env_flag_enabled("SPEC42_TRACE_UNRESOLVED_TYPES", false) {
            tracing::info!(
                uri = %uri,
                node_id = %node.id.qualified_name,
                node_kind = %node.element_kind,
                node_name = %node.name,
                type_ref_raw = %type_ref,
                type_ref_normalized = %normalized_type_ref,
                "unresolved type reference"
            );
        }
    }

    // 6) Redefines consistency, when the parser/graph captures a `redefines` attribute.
    for node in graph.nodes_for_uri(uri) {
        let Some(redefines_raw) = node.attributes.get("redefines").and_then(|v| v.as_str()) else {
            continue;
        };
        if redefines_raw.trim().is_empty() {
            diagnostics.push(diag(
                diagnostic_range(graph, node, None),
                DiagnosticSeverity::WARNING,
                "semantic",
                "invalid_redefines_reference",
                format!("Element '{}' has an empty redefines target.", node.name),
            ));
            continue;
        }
        if redefines_raw.trim() == node.id.qualified_name {
            diagnostics.push(diag(
                diagnostic_range(graph, node, None),
                DiagnosticSeverity::WARNING,
                "semantic",
                "invalid_redefines_reference",
                format!("Element '{}' cannot redefine itself.", node.name),
            ));
        }
    }

    diagnostics
}

fn should_suppress_builder_diagnostic(
    graph: &SemanticGraph,
    uri: &Url,
    node: &crate::semantic_model::SemanticNode,
    code: &str,
    message: &str,
) -> bool {
    if !matches!(code, "unresolved_satisfy_source" | "unresolved_satisfy_target") {
        return false;
    }
    let Some(reference_name) = extract_single_quoted_value(message) else {
        return false;
    };
    if matches!(
        resolve_expression_endpoint_strict(graph, uri, Some(diagnostic_container_prefix(node)), &reference_name),
        ResolveResult::Resolved(_)
    ) {
        return true;
    }
    matches!(
        resolve_expression_endpoint_strict(graph, uri, None, &reference_name),
        ResolveResult::Resolved(_)
    )
}

fn extract_single_quoted_value(message: &str) -> Option<String> {
    let start = message.find('\'')?;
    let rest = &message[start + 1..];
    let end = rest.find('\'')?;
    Some(rest[..end].to_string())
}

fn diagnostic_container_prefix(node: &crate::semantic_model::SemanticNode) -> &str {
    node.id
        .qualified_name
        .rsplit_once("::")
        .map(|(prefix, _)| prefix)
        .unwrap_or("")
}

/// Default semantic checks (port connectivity, type compatibility, unconnected ports, duplicate connections).
/// Implements [crate::host::config::SemanticCheckProvider] for use in [crate::host::config::Spec42Config].
#[derive(Debug, Default)]
pub struct DefaultSemanticChecks;

impl crate::host::config::SemanticCheckProvider for DefaultSemanticChecks {
    fn compute_diagnostics(&self, graph: &SemanticGraph, uri: &Url) -> Vec<Diagnostic> {
        compute_semantic_diagnostics(graph, uri)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::semantic_model::{build_graph_from_doc, SemanticNode};
    use tower_lsp::lsp_types::{Position, Range};

    #[test]
    fn parse_port_type_conjugated() {
        let (base, conj) = parse_port_type("~PowerPort");
        assert_eq!(base, "PowerPort");
        assert!(conj);
    }

    #[test]
    fn parse_port_type_plain() {
        let (base, conj) = parse_port_type("PowerPort");
        assert_eq!(base, "PowerPort");
        assert!(!conj);
    }

    #[test]
    fn port_type_mismatch_different_base() {
        let src = node_with_port_type("~PowerPort");
        let tgt = node_with_port_type("~TelemetryPort");
        let msg = port_type_mismatch(&src, &tgt);
        assert!(msg.is_some());
        assert!(msg.unwrap().contains("do not match"));
    }

    #[test]
    fn port_type_compatible() {
        let src = node_with_port_type("~PowerPort");
        let tgt = node_with_port_type("PowerPort");
        assert!(port_type_mismatch(&src, &tgt).is_none());
    }

    #[test]
    fn port_type_both_conjugated() {
        let src = node_with_port_type("~PowerPort");
        let tgt = node_with_port_type("~PowerPort");
        let msg = port_type_mismatch(&src, &tgt);
        assert!(msg.is_some());
        assert!(msg.unwrap().contains("same conjugation"));
    }

    fn node_with_port_type(port_type: &str) -> SemanticNode {
        use std::collections::HashMap;
        let mut attrs = HashMap::new();
        attrs.insert("portType".to_string(), serde_json::json!(port_type));
        SemanticNode {
            id: NodeId {
                uri: Url::parse("file:///test.sysml").unwrap(),
                qualified_name: "Test::p".to_string(),
            },
            element_kind: "port".to_string(),
            name: "p".to_string(),
            range: Range::new(Position::new(0, 0), Position::new(0, 0)),
            attributes: attrs,
            parent_id: None,
        }
    }

    #[test]
    fn declared_type_ref_ignores_empty_values() {
        use std::collections::HashMap;
        let mut attrs = HashMap::new();
        attrs.insert("partType".to_string(), serde_json::json!(""));
        let node = SemanticNode {
            id: NodeId {
                uri: Url::parse("file:///test.sysml").unwrap(),
                qualified_name: "Test::display".to_string(),
            },
            element_kind: "part".to_string(),
            name: "display".to_string(),
            range: Range::new(Position::new(0, 0), Position::new(0, 0)),
            attributes: attrs,
            parent_id: None,
        };
        assert!(declared_type_ref(&node).is_none());
    }

    #[test]
    fn port_type_mismatch_is_anchored_to_connection_statement() {
        let input = r#"
            package P {
                part def Left {
                    port p : ~PowerPort;
                }
                part def Right {
                    port p : ~PowerPort;
                }
                part def Top {
                    part l : Left;
                    part r : Right;
                    connect l.p to r.p;
                }
            }
        "#;
        let root = sysml_v2_parser::parse(input).expect("parse");
        let uri = Url::parse("file:///test.sysml").expect("uri");
        let graph = build_graph_from_doc(&root, &uri);
        let diags = compute_semantic_diagnostics(&graph, &uri);
        let mismatch = diags
            .iter()
            .find(|d| {
                d.code.as_ref()
                    == Some(&tower_lsp::lsp_types::NumberOrString::String(
                        "port_type_mismatch".to_string(),
                    ))
            })
            .expect("expected mismatch diagnostic");

        // "connect l.p to r.p;" line (0-based, accounting for leading newline/indentation in input string)
        assert_eq!(mismatch.range.start.line, 11);
        assert_eq!(mismatch.range.end.line, 11);
    }

    #[test]
    fn multiplicity_validator_rejects_negative_and_reversed_ranges() {
        let negative = multiplicity_issue_message("[-1]");
        assert!(negative.is_some());
        assert!(negative.expect("negative issue").contains("non-negative"));

        let reversed = multiplicity_issue_message("[5..2]");
        assert!(reversed.is_some());
        assert!(reversed.expect("reversed issue").contains("greater than"));
    }

    #[test]
    fn multiplicity_validator_accepts_common_valid_forms() {
        assert!(multiplicity_issue_message("[0..1]").is_none());
        assert!(multiplicity_issue_message("[1..*]").is_none());
        assert!(multiplicity_issue_message("[3]").is_none());
    }

    #[test]
    fn same_document_part_type_references_resolve_to_later_part_defs() {
        let input = r#"
            package ContextPackage {
                part def LaunchSite {
                    part vehicleAssemblyBuilding : VehicleAssemblyBuilding;
                }

                part def VehicleAssemblyBuilding;

                part def Apollo11MissionContext {
                    part vehicleAssemblyBuilding : VehicleAssemblyBuilding;
                }
            }
        "#;
        let root = sysml_v2_parser::parse(input).expect("parse");
        let uri = Url::parse("file:///context.sysml").expect("uri");
        let graph = build_graph_from_doc(&root, &uri);
        let diags = compute_semantic_diagnostics(&graph, &uri);
        let unresolved: Vec<_> = diags
            .iter()
            .filter(|d| {
                d.code.as_ref()
                    == Some(&tower_lsp::lsp_types::NumberOrString::String(
                        "unresolved_type_reference".to_string(),
                    ))
            })
            .collect();
        assert!(
            unresolved.is_empty(),
            "expected later same-document part defs to resolve: {unresolved:#?}"
        );
    }

    #[test]
    fn imported_part_defs_resolve_for_part_usages() {
        use crate::workspace::{
            ingest_parsed_scan_entries, parse_scanned_entries, rebuild_all_document_links,
            store_document_text, ServerState,
        };

        let temp = tempfile::tempdir().expect("temp dir");
        let root = temp.path().canonicalize().expect("canonical root");
        let root_uri = Url::from_file_path(&root).expect("root uri");
        let components_uri =
            Url::from_file_path(root.join("TechnicalComponentsPackage.sysml")).expect("components");
        let system_uri =
            Url::from_file_path(root.join("SystemPackage.sysml")).expect("system");

        let mut state = ServerState::default();
        state.workspace_roots = vec![root_uri.clone()];

        store_document_text(
            &mut state,
            &components_uri,
            "package TechnicalComponentsPackage { part def ApolloSpacecraft; part def ExtravehicularMobilityUnit; }".to_string(),
        );
        store_document_text(
            &mut state,
            &system_uri,
            "package SystemPackage { private import TechnicalComponentsPackage::*; part def Apollo11MissionSystem { part spacecraft : ApolloSpacecraft; part spaceSuits[2] : ExtravehicularMobilityUnit; } }".to_string(),
        );

        let entries = parse_scanned_entries(
            vec![
                (
                    components_uri.clone(),
                    "package TechnicalComponentsPackage { part def ApolloSpacecraft; part def ExtravehicularMobilityUnit; }"
                        .to_string(),
                ),
                (
                    system_uri.clone(),
                    "package SystemPackage { private import TechnicalComponentsPackage::*; part def Apollo11MissionSystem { part spacecraft : ApolloSpacecraft; part spaceSuits[2] : ExtravehicularMobilityUnit; } }"
                        .to_string(),
                ),
            ],
            false,
        );
        ingest_parsed_scan_entries(&mut state, entries);
        rebuild_all_document_links(&mut state);

        let diags = compute_semantic_diagnostics(&state.semantic_graph, &system_uri);
        let unresolved: Vec<_> = diags
            .iter()
            .filter(|d| {
                d.code.as_ref()
                    == Some(&tower_lsp::lsp_types::NumberOrString::String(
                        "unresolved_type_reference".to_string(),
                    ))
            })
            .collect();
        assert!(
            unresolved.is_empty(),
            "expected imported part defs to resolve: {unresolved:#?}"
        );
    }

    #[test]
    fn same_name_redefines_usage_does_not_report_self_reference() {
        let input = r#"
            package Office {
                part def Laptop {
                    attribute name : String;
                }
                part office: Office {
                    part laptop1: Laptop{
                        attribute :>> name = "My Laptop";
                    }
                }
            }
        "#;
        let root = sysml_v2_parser::parse(input).expect("parse");
        let uri = Url::parse("file:///demo.sysml").expect("uri");
        let graph = build_graph_from_doc(&root, &uri);
        let diags = compute_semantic_diagnostics(&graph, &uri);
        let invalid_redefines: Vec<_> = diags
            .iter()
            .filter(|d| {
                d.code.as_ref()
                    == Some(&tower_lsp::lsp_types::NumberOrString::String(
                        "invalid_redefines_reference".to_string(),
                    ))
            })
            .collect();
        assert!(
            invalid_redefines.is_empty(),
            "same-name redefines usage should be allowed: {invalid_redefines:#?}"
        );
    }

    #[test]
    fn forward_declared_satisfy_reference_does_not_emit_unresolved_diagnostic() {
        let input = r#"
            package P {
                part def Drone;
                part droneInstance : Drone;
                satisfy EnduranceReq by droneInstance;

                requirement def EnduranceReq {
                    subject drone : Drone;
                    require constraint { doc /* placeholder */ }
                }
            }
        "#;
        let root = sysml_v2_parser::parse(input).expect("parse");
        let uri = Url::parse("file:///forward_satisfy.sysml").expect("uri");
        let graph = build_graph_from_doc(&root, &uri);
        let diags = compute_semantic_diagnostics(&graph, &uri);
        let unresolved_satisfy: Vec<_> = diags
            .iter()
            .filter(|d| {
                d.code.as_ref()
                    == Some(&tower_lsp::lsp_types::NumberOrString::String(
                        "unresolved_satisfy_source".to_string(),
                    ))
                    || d.code.as_ref()
                        == Some(&tower_lsp::lsp_types::NumberOrString::String(
                            "unresolved_satisfy_target".to_string(),
                        ))
            })
            .collect();
        assert!(
            unresolved_satisfy.is_empty(),
            "forward-declared satisfy references should be resolved after full graph build: {unresolved_satisfy:#?}"
        );
    }

    #[test]
    fn redefined_port_usage_keeps_connection_endpoints_port_typed() {
        let input = r#"
            package House {
                port def PowerOutletPort {}

                part def ElectricGrid {
                    port outlets[1..*] : PowerOutletPort;
                }

                part def Room {
                    port outlet : PowerOutletPort;
                }

                part def Home {
                    part electricGrid : ElectricGrid;
                    part livingRoom : Room {
                        attribute :>> outlet :> electricGrid.outlets;
                        connect outlet to outlet;
                    }
                }
            }
        "#;
        let root = sysml_v2_parser::parse(input).expect("parse");
        let uri = Url::parse("file:///house_like.sysml").expect("uri");
        let graph = build_graph_from_doc(&root, &uri);
        let diags = compute_semantic_diagnostics(&graph, &uri);
        let endpoint_not_port: Vec<_> = diags
            .iter()
            .filter(|d| {
                d.code.as_ref()
                    == Some(&tower_lsp::lsp_types::NumberOrString::String(
                        "connection_endpoint_not_port".to_string(),
                    ))
            })
            .collect();
        assert!(
            endpoint_not_port.is_empty(),
            "redefined outlet should stay port-like for connection analysis: {endpoint_not_port:#?}"
        );
    }
}
