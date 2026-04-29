//! Semantic validation beyond syntax: port connectivity, type compatibility, etc.
//!
//! These checks use the semantic graph (parts, ports, connections) to report
//! diagnostics such as: unconnected ports, connection to non-port, port type mismatch.

mod builder_diagnostics;
mod import_resolution;

use std::collections::HashSet;
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, Url};

use crate::analysis::helpers::*;
use crate::semantic_model::{resolve_member_via_type, NodeId, ResolveResult, SemanticGraph};
use builder_diagnostics::should_suppress_builder_diagnostic;
use import_resolution::{has_import_in_scope, import_target, import_target_resolves};

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
                if let Some(msg) = port_compatibility_mismatch(graph, src, tgt) {
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
            && !node.attributes.contains_key("redefines")
            && !node.attributes.contains_key("subsetsFeature")
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

    // 5) Import targets should resolve to known namespace/member declarations.
    for node in graph.nodes_for_uri(uri) {
        if node.element_kind != "import" || import_target_resolves(graph, node) {
            continue;
        }
        let Some(target) = import_target(node) else {
            continue;
        };
        diagnostics.push(diag(
            diagnostic_range(graph, node, None),
            DiagnosticSeverity::WARNING,
            "semantic",
            "unresolved_import_target",
            format!(
                "Imported package/member '{}' could not be resolved in the semantic graph.",
                target
            ),
        ));
    }

    // 6) Stronger typing checks: declarations that name a type should resolve via typing/specializes.
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
        let resolved_via_graph_name_fallback =
            graph
                .nodes_named(&normalized_type_ref)
                .iter()
                .any(|candidate| {
                    candidate.id.uri == *uri
                        && matches!(
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
                });
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

    // 7) Specialization references should resolve to known definitions.
    let mut unresolved_specializes_seen: HashSet<String> = HashSet::new();
    for node in graph.nodes_for_uri(uri) {
        if is_synthetic(node) {
            continue;
        }
        for specializes_ref in declared_specializes_refs(node) {
            let normalized = normalize_declared_type_ref(&specializes_ref);
            if normalized.is_empty() || is_builtin_type_ref(&normalized) {
                continue;
            }
            let resolved_via_import_scope = !crate::semantic_model::resolve_type_reference_targets(
                graph,
                node,
                &specializes_ref,
                &[
                    "part def",
                    "port def",
                    "action def",
                    "state def",
                    "flow def",
                    "allocation def",
                    "requirement def",
                    "use case def",
                    "attribute def",
                    "enum def",
                    "item def",
                    "actor def",
                    "occurrence def",
                    "interface",
                    "concern def",
                    "alias",
                    "kermlDecl",
                ],
            )
            .is_empty();
            let resolved_via_graph_name_fallback =
                graph.nodes_named(&normalized).iter().any(|candidate| {
                    candidate.id.uri == *uri
                        && matches!(
                            candidate.element_kind.as_str(),
                            "part def"
                                | "port def"
                                | "action def"
                                | "state def"
                                | "flow def"
                                | "allocation def"
                                | "requirement def"
                                | "use case def"
                                | "attribute def"
                                | "enum def"
                                | "item def"
                                | "actor def"
                                | "occurrence def"
                                | "interface"
                                | "concern def"
                                | "alias"
                                | "kermlDecl"
                        )
                });
            let allow_graph_name_fallback = !has_import_in_scope(graph, node);
            if resolved_via_import_scope
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
                normalized,
                range.start.line,
                range.start.character,
                range.end.line,
                range.end.character,
                node.name
            );
            if !unresolved_specializes_seen.insert(key) {
                continue;
            }
            diagnostics.push(diag(
                range,
                DiagnosticSeverity::WARNING,
                "semantic",
                "unresolved_specializes_reference",
                format!(
                    "Specializes reference '{}' for '{}' could not be resolved in the semantic graph.",
                    specializes_ref, node.name
                ),
            ));
        }
    }

    // 8) Redefines consistency, when the parser/graph captures a `redefines` attribute.
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

    // 9) Inherited feature value assignment must use explicit redefinition (`:>>`).
    for node in graph.nodes_for_uri(uri) {
        if !node.attributes.contains_key("value") || node.attributes.contains_key("redefines") {
            continue;
        }
        let Some(owner_id) = node.parent_id.as_ref() else {
            continue;
        };
        let Some(owner) = graph.get_node(owner_id) else {
            continue;
        };
        let feature_name = node.name.trim();
        if feature_name.is_empty() {
            continue;
        }
        let ResolveResult::Resolved(target_id) =
            resolve_member_via_type(graph, owner, feature_name)
        else {
            continue;
        };
        let Some(target) = graph.get_node(&target_id) else {
            continue;
        };
        if target.id == node.id {
            continue;
        }
        if target.name.trim() != feature_name {
            continue;
        }
        diagnostics.push(diag(
            diagnostic_range(graph, node, Some(target)),
            DiagnosticSeverity::ERROR,
            "semantic",
            "implicit_redefinition_without_operator",
            format!(
                "Feature '{}' overrides inherited {} '{}' but is missing explicit redefinition ':>>'.",
                feature_name, target.element_kind, target.name
            ),
        ));
    }

    diagnostics
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
    use crate::semantic_model::{
        add_cross_document_edges_for_uri, build_graph_from_doc, SemanticNode,
    };
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
        let graph = SemanticGraph::new();
        let src = node_with_port_type("~PowerPort");
        let tgt = node_with_port_type("~TelemetryPort");
        let msg = port_compatibility_mismatch(&graph, &src, &tgt);
        assert!(msg.is_some());
        assert!(msg.unwrap().contains("do not match"));
    }

    #[test]
    fn port_type_compatible() {
        let graph = SemanticGraph::new();
        let src = node_with_port_type("~PowerPort");
        let tgt = node_with_port_type("PowerPort");
        assert!(port_compatibility_mismatch(&graph, &src, &tgt).is_none());
    }

    #[test]
    fn port_type_both_conjugated() {
        let graph = SemanticGraph::new();
        let src = node_with_port_type("~PowerPort");
        let tgt = node_with_port_type("~PowerPort");
        let msg = port_compatibility_mismatch(&graph, &src, &tgt);
        assert!(msg.is_some());
        assert!(msg.unwrap().contains("same conjugation"));
    }

    #[test]
    fn port_type_plain_to_plain_is_compatible() {
        let graph = SemanticGraph::new();
        let src = node_with_port_type("PowerPort");
        let tgt = node_with_port_type("PowerPort");
        assert!(port_compatibility_mismatch(&graph, &src, &tgt).is_none());
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
        let system_uri = Url::from_file_path(root.join("SystemPackage.sysml")).expect("system");

        let mut state = ServerState {
            workspace_roots: vec![root_uri.clone()],
            ..ServerState::default()
        };

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
    fn unresolved_import_target_emits_diagnostic() {
        let input = r#"
            package Demo {
                import MissingLibrary::*;
            }
        "#;
        let root = sysml_v2_parser::parse(input).expect("parse");
        let uri = Url::parse("file:///unresolved_import.sysml").expect("uri");
        let graph = build_graph_from_doc(&root, &uri);
        let diags = compute_semantic_diagnostics(&graph, &uri);
        let unresolved_imports: Vec<_> = diags
            .iter()
            .filter(|d| {
                d.code.as_ref()
                    == Some(&tower_lsp::lsp_types::NumberOrString::String(
                        "unresolved_import_target".to_string(),
                    ))
            })
            .collect();
        assert!(
            !unresolved_imports.is_empty(),
            "expected unresolved import diagnostic for MissingLibrary::*"
        );
    }

    #[test]
    fn import_target_resolves_without_unresolved_diagnostic_when_package_exists() {
        let input = r#"
            package Shared {}
            package Demo {
                import Shared::*;
            }
        "#;
        let root = sysml_v2_parser::parse(input).expect("parse");
        let uri = Url::parse("file:///resolved_import.sysml").expect("uri");
        let graph = build_graph_from_doc(&root, &uri);
        let diags = compute_semantic_diagnostics(&graph, &uri);
        let unresolved_imports: Vec<_> = diags
            .iter()
            .filter(|d| {
                d.code.as_ref()
                    == Some(&tower_lsp::lsp_types::NumberOrString::String(
                        "unresolved_import_target".to_string(),
                    ))
            })
            .collect();
        assert!(
            unresolved_imports.is_empty(),
            "existing import target should not emit unresolved import diagnostic: {unresolved_imports:#?}"
        );
    }

    #[test]
    fn unresolved_specializes_reference_emits_diagnostic_for_symbol_form() {
        let input = r#"
            package P {
                import MissingLibrary::*;
                part def Child :> MissingBase {}
            }
        "#;
        let root = sysml_v2_parser::parse(input).expect("parse");
        let uri = Url::parse("file:///unresolved_specializes.sysml").expect("uri");
        let graph = build_graph_from_doc(&root, &uri);
        let diags = compute_semantic_diagnostics(&graph, &uri);
        let unresolved_specializes: Vec<_> = diags
            .iter()
            .filter(|d| {
                d.code.as_ref()
                    == Some(&tower_lsp::lsp_types::NumberOrString::String(
                        "unresolved_specializes_reference".to_string(),
                    ))
            })
            .collect();
        assert!(
            !unresolved_specializes.is_empty(),
            "expected unresolved_specializes_reference diagnostic for missing base"
        );
    }

    #[test]
    fn resolved_specializes_reference_does_not_emit_diagnostic() {
        let input = r#"
            package P {
                part def Base {}
                part def Child :> Base {}
            }
        "#;
        let root = sysml_v2_parser::parse(input).expect("parse");
        let uri = Url::parse("file:///resolved_specializes.sysml").expect("uri");
        let graph = build_graph_from_doc(&root, &uri);
        let diags = compute_semantic_diagnostics(&graph, &uri);
        let unresolved_specializes: Vec<_> = diags
            .iter()
            .filter(|d| {
                d.code.as_ref()
                    == Some(&tower_lsp::lsp_types::NumberOrString::String(
                        "unresolved_specializes_reference".to_string(),
                    ))
            })
            .collect();
        assert!(
            unresolved_specializes.is_empty(),
            "resolved base should not emit unresolved_specializes_reference: {unresolved_specializes:#?}"
        );
    }

    #[test]
    fn multi_base_specializes_emits_when_one_base_is_missing() {
        let input = r#"
            package P {
                part def BaseA {}
                part def Child :> BaseA {}
            }
        "#;
        let root = sysml_v2_parser::parse(input).expect("parse");
        let uri = Url::parse("file:///multi_base_specializes.sysml").expect("uri");
        let mut graph = build_graph_from_doc(&root, &uri);
        let child_id = graph
            .nodes_for_uri(&uri)
            .into_iter()
            .find(|node| node.element_kind == "part def" && node.name == "Child")
            .map(|node| node.id.clone())
            .expect("child node");
        graph
            .get_node_mut(&child_id)
            .expect("child node mut")
            .attributes
            .insert(
                "specializes".to_string(),
                serde_json::json!(["BaseA", "MissingBase"]),
            );
        crate::semantic_model::add_cross_document_edges_for_uri(&mut graph, &uri);

        let diags = compute_semantic_diagnostics(&graph, &uri);
        let unresolved_specializes: Vec<_> = diags
            .iter()
            .filter(|d| {
                d.code.as_ref()
                    == Some(&tower_lsp::lsp_types::NumberOrString::String(
                        "unresolved_specializes_reference".to_string(),
                    ))
            })
            .collect();
        assert!(
            unresolved_specializes.iter().any(|diag| diag
                .message
                .contains("MissingBase")),
            "expected unresolved specializes diagnostic to mention missing base: {unresolved_specializes:#?}"
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
    fn inherited_attribute_value_assignment_without_explicit_redefines_emits_error() {
        let input = r#"
            package P {
                part def Base {
                    attribute mass : Real;
                }
                part def Child :> Base {
                    attribute mass = 1200;
                }
            }
        "#;
        let root = sysml_v2_parser::parse(input).expect("parse");
        let uri = Url::parse("file:///implicit_redefine_attribute.sysml").expect("uri");
        let graph = build_graph_from_doc(&root, &uri);
        let implicit_redefine: Vec<_> = compute_semantic_diagnostics(&graph, &uri)
            .into_iter()
            .filter(|d| {
                d.code.as_ref()
                    == Some(&tower_lsp::lsp_types::NumberOrString::String(
                        "implicit_redefinition_without_operator".to_string(),
                    ))
            })
            .collect();
        assert!(
            !implicit_redefine.is_empty(),
            "expected implicit redefinition diagnostic for inherited attribute assignment"
        );
        assert!(
            implicit_redefine
                .iter()
                .all(|d| d.severity == Some(DiagnosticSeverity::ERROR)),
            "expected implicit redefinition diagnostics to be errors: {implicit_redefine:#?}"
        );
    }

    #[test]
    fn explicit_redefines_operator_suppresses_implicit_redefinition_diagnostic() {
        let input = r#"
            package P {
                part def Base {
                    attribute mass : Real;
                }
                part def Child :> Base {
                    attribute :>> mass = 1200;
                }
            }
        "#;
        let root = sysml_v2_parser::parse(input).expect("parse");
        let uri = Url::parse("file:///explicit_redefine_attribute.sysml").expect("uri");
        let graph = build_graph_from_doc(&root, &uri);
        let implicit_redefine: Vec<_> = compute_semantic_diagnostics(&graph, &uri)
            .into_iter()
            .filter(|d| {
                d.code.as_ref()
                    == Some(&tower_lsp::lsp_types::NumberOrString::String(
                        "implicit_redefinition_without_operator".to_string(),
                    ))
            })
            .collect();
        assert!(
            implicit_redefine.is_empty(),
            "did not expect implicit redefinition diagnostic with explicit :>>: {implicit_redefine:#?}"
        );
    }

    #[test]
    fn local_value_assignment_without_inheritance_does_not_emit_implicit_redefinition() {
        let input = r#"
            package P {
                part def Child {
                    attribute mass = 1200;
                }
            }
        "#;
        let root = sysml_v2_parser::parse(input).expect("parse");
        let uri = Url::parse("file:///local_assignment.sysml").expect("uri");
        let graph = build_graph_from_doc(&root, &uri);
        let implicit_redefine: Vec<_> = compute_semantic_diagnostics(&graph, &uri)
            .into_iter()
            .filter(|d| {
                d.code.as_ref()
                    == Some(&tower_lsp::lsp_types::NumberOrString::String(
                        "implicit_redefinition_without_operator".to_string(),
                    ))
            })
            .collect();
        assert!(
            implicit_redefine.is_empty(),
            "local value assignment should not be treated as implicit redefinition: {implicit_redefine:#?}"
        );
    }

    #[test]
    fn inherited_part_and_port_value_assignments_emit_implicit_redefinition_error() {
        let input = r#"
            package P {
                part def Engine {}
                port def PowerPort {}
                part def Base {
                    part engine : Engine;
                    port outlet : PowerPort;
                }
                part def Child :> Base {
                    attribute engine = replacementEngine;
                    attribute outlet = replacementOutlet;
                }
            }
        "#;
        let root = sysml_v2_parser::parse(input).expect("parse");
        let uri = Url::parse("file:///implicit_redefine_part_port.sysml").expect("uri");
        let graph = build_graph_from_doc(&root, &uri);
        let implicit_redefine: Vec<_> = compute_semantic_diagnostics(&graph, &uri)
            .into_iter()
            .filter(|d| {
                d.code.as_ref()
                    == Some(&tower_lsp::lsp_types::NumberOrString::String(
                        "implicit_redefinition_without_operator".to_string(),
                    ))
            })
            .collect();
        assert!(
            implicit_redefine
                .iter()
                .any(|d| d.message.contains("inherited part")),
            "expected inherited part implicit redefinition diagnostic: {implicit_redefine:#?}"
        );
        assert!(
            implicit_redefine
                .iter()
                .any(|d| d.message.contains("inherited port")),
            "expected inherited port implicit redefinition diagnostic: {implicit_redefine:#?}"
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
    fn typed_member_chain_satisfy_reference_does_not_emit_unresolved_diagnostic() {
        let architecture = r#"
            package WebShopArchitecture {
                part def CheckoutService {}
                part def WebShopSystem {
                    part checkoutService : CheckoutService;
                }
            }
        "#;
        let instance = r#"
            package WebShopExample {
                import WebShopArchitecture::*;
                part webshopSystem : WebShopSystem;
                requirement checkoutLatency;
                satisfy checkoutLatency by webshopSystem.checkoutService;
            }
        "#;

        let architecture_uri =
            Url::parse("file:///typed_member_satisfy_architecture.sysml").expect("arch uri");
        let instance_uri = Url::parse("file:///typed_member_satisfy_instance.sysml").expect("instance uri");
        let architecture_root = sysml_v2_parser::parse(architecture).expect("parse architecture");
        let instance_root = sysml_v2_parser::parse(instance).expect("parse instance");

        let mut graph = build_graph_from_doc(&architecture_root, &architecture_uri);
        graph.merge(build_graph_from_doc(&instance_root, &instance_uri));
        add_cross_document_edges_for_uri(&mut graph, &instance_uri);

        let diags = compute_semantic_diagnostics(&graph, &instance_uri);
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
            "typed-member satisfy references should be resolved/suppressed after full graph build: {unresolved_satisfy:#?}"
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

    #[test]
    fn delegated_port_usage_does_not_emit_unconnected_port() {
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
                    }
                }
            }
        "#;
        let root = sysml_v2_parser::parse(input).expect("parse");
        let uri = Url::parse("file:///delegated_unconnected.sysml").expect("uri");
        let graph = build_graph_from_doc(&root, &uri);
        let diags = compute_semantic_diagnostics(&graph, &uri);
        let unconnected_outlet: Vec<_> = diags
            .iter()
            .filter(|d| {
                d.code.as_ref()
                    == Some(&tower_lsp::lsp_types::NumberOrString::String(
                        "unconnected_port".to_string(),
                    ))
                    && d.message.contains("outlet")
            })
            .collect();
        assert!(
            unconnected_outlet.is_empty(),
            "delegated/redefined outlet should not be treated as dangling: {unconnected_outlet:#?}"
        );
    }

    #[test]
    fn concrete_unconnected_port_still_emits_diagnostic() {
        let input = r#"
            package P {
                port def PowerPort {}
                part def Device {
                    port outlet : PowerPort;
                }
                part device1 : Device {
                    port localOutlet : PowerPort;
                }
            }
        "#;
        let root = sysml_v2_parser::parse(input).expect("parse");
        let uri = Url::parse("file:///concrete_unconnected.sysml").expect("uri");
        let graph = build_graph_from_doc(&root, &uri);
        let diags = compute_semantic_diagnostics(&graph, &uri);
        let unconnected_local_outlet: Vec<_> = diags
            .iter()
            .filter(|d| {
                d.code.as_ref()
                    == Some(&tower_lsp::lsp_types::NumberOrString::String(
                        "unconnected_port".to_string(),
                    ))
                    && d.message.contains("localOutlet")
            })
            .collect();
        assert!(
            !unconnected_local_outlet.is_empty(),
            "concrete instance ports with no wiring should still report unconnected_port"
        );
    }

    #[test]
    fn compatible_different_port_defs_do_not_emit_port_type_mismatch() {
        let input = r#"
            package P {
                item def Water;

                port def DeviceWaterInletPort {
                    in item water : Water;
                }

                port def WaterSpigotPort {
                    out item water : Water;
                }

                part def Dishwasher {
                    port waterInlet : DeviceWaterInletPort;
                }

                part def Kitchen {
                    port waterSpigot : WaterSpigotPort;
                }

                part def Home {
                    part dishwasher : Dishwasher;
                    part kitchen : Kitchen;
                    connect dishwasher.waterInlet to kitchen.waterSpigot;
                }
            }
        "#;
        let root = sysml_v2_parser::parse(input).expect("parse");
        let uri = Url::parse("file:///compatible_ports.sysml").expect("uri");
        let graph = build_graph_from_doc(&root, &uri);
        let diags = compute_semantic_diagnostics(&graph, &uri);
        let mismatches: Vec<_> = diags
            .iter()
            .filter(|d| {
                d.code.as_ref()
                    == Some(&tower_lsp::lsp_types::NumberOrString::String(
                        "port_type_mismatch".to_string(),
                    ))
            })
            .collect();
        assert!(
            mismatches.is_empty(),
            "feature-compatible different port definitions should not mismatch: {mismatches:#?}"
        );
    }

    #[test]
    fn incompatible_port_features_emit_port_type_mismatch() {
        let input = r#"
            package P {
                item def Water;
                item def Air;

                port def DeviceWaterInletPort {
                    in item water : Water;
                }

                port def AirSpigotPort {
                    out item air : Air;
                }

                part def Dishwasher {
                    port waterInlet : DeviceWaterInletPort;
                }

                part def Kitchen {
                    port airSpigot : AirSpigotPort;
                }

                part def Home {
                    part dishwasher : Dishwasher;
                    part kitchen : Kitchen;
                    connect dishwasher.waterInlet to kitchen.airSpigot;
                }
            }
        "#;
        let root = sysml_v2_parser::parse(input).expect("parse");
        let uri = Url::parse("file:///incompatible_ports.sysml").expect("uri");
        let graph = build_graph_from_doc(&root, &uri);
        let diags = compute_semantic_diagnostics(&graph, &uri);
        let mismatches: Vec<_> = diags
            .iter()
            .filter(|d| {
                d.code.as_ref()
                    == Some(&tower_lsp::lsp_types::NumberOrString::String(
                        "port_type_mismatch".to_string(),
                    ))
            })
            .collect();
        assert!(
            !mismatches.is_empty(),
            "incompatible port features should emit mismatch diagnostics"
        );
    }

    #[test]
    fn private_import_chain_emits_unresolved_type_reference() {
        let core = sysml_v2_parser::parse("package Core { attribute def Name; }").expect("core");
        let domain =
            sysml_v2_parser::parse("package Domain { private import Core::*; }").expect("domain");
        let usage = sysml_v2_parser::parse(
            "package Demo { import Domain::*; part def Consumer { attribute groupName : Name; } }",
        )
        .expect("usage");

        let core_uri = Url::parse("file:///workspace/core.sysml").expect("core uri");
        let domain_uri = Url::parse("file:///workspace/domain.sysml").expect("domain uri");
        let usage_uri = Url::parse("file:///workspace/use.sysml").expect("usage uri");

        let mut graph = SemanticGraph::new();
        graph.merge(build_graph_from_doc(&core, &core_uri));
        graph.merge(build_graph_from_doc(&domain, &domain_uri));
        graph.merge(build_graph_from_doc(&usage, &usage_uri));
        add_cross_document_edges_for_uri(&mut graph, &usage_uri);

        let diags = compute_semantic_diagnostics(&graph, &usage_uri);
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
            !unresolved.is_empty(),
            "private import chain should keep unresolved_type_reference: {diags:#?}"
        );
    }

    #[test]
    fn private_import_chain_emits_unresolved_type_reference_after_incremental_relink() {
        let core = sysml_v2_parser::parse("package Core { attribute def Name; }").expect("core");
        let domain =
            sysml_v2_parser::parse("package Domain { private import Core::*; }").expect("domain");
        let usage = sysml_v2_parser::parse(
            "package Demo { import Domain::*; part def Consumer { attribute groupName : Name; } }",
        )
        .expect("usage");

        let core_uri = Url::parse("file:///workspace/core.sysml").expect("core uri");
        let domain_uri = Url::parse("file:///workspace/domain.sysml").expect("domain uri");
        let usage_uri = Url::parse("file:///workspace/use.sysml").expect("usage uri");

        let mut graph = SemanticGraph::new();
        graph.merge(build_graph_from_doc(&core, &core_uri));
        add_cross_document_edges_for_uri(&mut graph, &core_uri);
        graph.merge(build_graph_from_doc(&domain, &domain_uri));
        add_cross_document_edges_for_uri(&mut graph, &domain_uri);
        graph.merge(build_graph_from_doc(&usage, &usage_uri));
        add_cross_document_edges_for_uri(&mut graph, &usage_uri);

        let diags = compute_semantic_diagnostics(&graph, &usage_uri);
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
            !unresolved.is_empty(),
            "incremental relink should keep unresolved_type_reference: {diags:#?}"
        );
    }
}
