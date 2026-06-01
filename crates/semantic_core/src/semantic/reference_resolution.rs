use std::collections::HashSet;

use url::Url;

use crate::semantic::graph::SemanticGraph;
use crate::semantic::import_resolution::resolve_imported_node_ids_for_simple_name;
use crate::semantic::model::{NodeId, SemanticNode};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolveResult<T> {
    Resolved(T),
    Ambiguous,
    Unresolved,
}

fn is_namespace_kind(kind: &str) -> bool {
    matches!(
        kind,
        "package"
            | "requirement def"
            | "requirement"
            | "use case def"
            | "use case"
            | "analysis def"
            | "analysis"
            | "verification def"
            | "verification"
            | "concern def"
            | "concern"
    )
}

fn resolve_context_node_for_prefix<'a>(
    g: &'a SemanticGraph,
    uri: &Url,
    prefix: &str,
) -> Option<&'a SemanticNode> {
    let owner_id = NodeId::new(uri, prefix);
    if let Some(owner) = g.get_node(&owner_id) {
        return Some(owner);
    }

    let suffix = format!("::{}", prefix);
    g.nodes_by_uri
        .get(uri)
        .into_iter()
        .flatten()
        .filter_map(|node_id| g.get_node(node_id))
        .filter(|node| {
            is_namespace_kind(&node.element_kind)
                && (node.id.qualified_name == prefix || node.id.qualified_name.ends_with(&suffix))
        })
        .min_by_key(|node| node.id.qualified_name.len())
}

/// Resolve an endpoint expression (e.g. `a.b`, `A::B`) to a node id.
pub fn resolve_expression_endpoint_strict(
    g: &SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    expression: &str,
) -> ResolveResult<NodeId> {
    let expr_normalized = expression.replace('.', "::");
    let mut expression_forms = Vec::new();
    expression_forms.push(expression.to_string());
    if expr_normalized != expression {
        expression_forms.push(expr_normalized.clone());
    }

    let mut candidates = Vec::new();
    if let Some(prefix) = container_prefix {
        for form in &expression_forms {
            candidates.push(format!("{}::{}", prefix, form));
        }
    }
    candidates.extend(expression_forms.clone());

    for candidate in &candidates {
        let node_id = NodeId::new(uri, candidate);
        if let Some(node) = g.get_node(&node_id) {
            if node.element_kind != "import" {
                return ResolveResult::Resolved(node_id);
            }
        }
    }
    if let Some(prefix) = container_prefix {
        if !expression.contains("::") && !expression.contains('.') {
            if let Some(owner) = resolve_context_node_for_prefix(g, uri, prefix) {
                if let ResolveResult::Resolved(member_id) =
                    resolve_member_via_type(g, owner, expression)
                {
                    return ResolveResult::Resolved(member_id);
                }
                let imported_matches =
                    resolve_imported_node_ids_for_simple_name(g, owner, expression);
                if imported_matches.len() == 1 {
                    return ResolveResult::Resolved(imported_matches[0].clone());
                }
                if imported_matches.len() > 1 {
                    return ResolveResult::Ambiguous;
                }
            }
        }

        // Member chains under a container (e.g. `battery.powerOut` in a part def body) resolve
        // through typed part usages and features on their definitions.
        let segments: Vec<&str> = expr_normalized
            .split("::")
            .filter(|segment| !segment.is_empty())
            .collect();
        if segments.len() > 1 {
            if let ResolveResult::Resolved(mut current_id) =
                resolve_expression_endpoint_strict(g, uri, container_prefix, segments[0])
            {
                let mut resolved_all = true;
                for member in segments.iter().skip(1) {
                    let Some(owner) = g.get_node(&current_id) else {
                        resolved_all = false;
                        break;
                    };
                    match resolve_member_via_type(g, owner, member) {
                        ResolveResult::Resolved(next_id) => current_id = next_id,
                        ResolveResult::Ambiguous => return ResolveResult::Ambiguous,
                        ResolveResult::Unresolved => {
                            resolved_all = false;
                            break;
                        }
                    }
                }
                if resolved_all {
                    return ResolveResult::Resolved(current_id);
                }
            }
        }
    }

    let suffixes: Vec<String> = expression_forms
        .iter()
        .map(|form| format!("::{}", form))
        .collect();
    let mut matches: Vec<&NodeId> = g
        .nodes_by_uri
        .get(uri)
        .into_iter()
        .flatten()
        .filter(|node_id| {
            (expression_forms.contains(&node_id.qualified_name)
                || suffixes
                    .iter()
                    .any(|suffix| node_id.qualified_name.ends_with(suffix)))
                && g.get_node(node_id)
                    .is_some_and(|node| node.element_kind != "import")
        })
        .collect();
    matches.sort_by_key(|node_id| node_id.qualified_name.len());
    matches.dedup_by(|a, b| a.qualified_name == b.qualified_name);
    if matches.len() == 1 {
        ResolveResult::Resolved(matches[0].clone())
    } else if matches.len() > 1 {
        ResolveResult::Ambiguous
    } else {
        ResolveResult::Unresolved
    }
}

/// Resolve `member` declared on a supertype of `owner` (does not match direct children of `owner`).
///
/// Walks the typing/specialization chain from the nearest type outward and returns the first
/// matching member so redefinitions on a specialized `part def` win over inherited declarations.
pub fn resolve_inherited_member_via_type(
    g: &SemanticGraph,
    owner: &SemanticNode,
    member: &str,
) -> ResolveResult<NodeId> {
    use std::collections::VecDeque;

    let mut visited: HashSet<NodeId> = HashSet::new();
    let mut queue: VecDeque<NodeId> = g
        .outgoing_typing_or_specializes_targets(owner)
        .into_iter()
        .map(|n| n.id.clone())
        .collect();

    while let Some(type_id) = queue.pop_front() {
        if !visited.insert(type_id.clone()) {
            continue;
        }
        let children: Vec<_> = g.child_named(&type_id, member);
        match children.len() {
            0 => {}
            1 => return ResolveResult::Resolved(children[0].id.clone()),
            _ => return ResolveResult::Ambiguous,
        }
        if let Some(type_node) = g.get_node(&type_id) {
            for base in g.outgoing_typing_or_specializes_targets(type_node) {
                queue.push_back(base.id.clone());
            }
        }
    }
    ResolveResult::Unresolved
}

/// Resolve `member` through typing/specialization starting from `owner`.
pub fn resolve_member_via_type(
    g: &SemanticGraph,
    owner: &SemanticNode,
    member: &str,
) -> ResolveResult<NodeId> {
    let direct_children: Vec<NodeId> = g
        .child_named(&owner.id, member)
        .into_iter()
        .filter(|child| child.element_kind != "import")
        .map(|child| child.id.clone())
        .collect();
    match direct_children.len() {
        1 => return ResolveResult::Resolved(direct_children.into_iter().next().expect("one child")),
        n if n > 1 => return ResolveResult::Ambiguous,
        _ => {}
    }

    resolve_inherited_member_via_type(g, owner, member)
}

#[cfg(test)]
mod tests {
    use url::Url;

    use crate::semantic::model::RelationshipKind;
    use crate::semantic::source::{SysmlDocument, SysmlDocumentSourceKind};
    use crate::semantic::workspace_graph::build_semantic_graph_from_documents;

    use super::{
        resolve_expression_endpoint_strict, resolve_inherited_member_via_type,
        resolve_member_via_type, ResolveResult,
    };

    #[test]
    fn member_chain_resolves_through_typed_part_usage_after_workspace_link() {
        let doc = SysmlDocument::from_memory_path(
            "workspace",
            "model.sysml",
            r#"package Demo {
  part def Battery { port powerOut; }
  part def Controller { port powerIn; }
  part def System {
    part battery : Battery;
    part controller : Controller;
    connect battery.powerOut to controller.powerIn;
  }
}"#
            .to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("workspace doc");
        let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
        let uri = Url::parse("memory://workspace/model.sysml").expect("uri");
        let result = resolve_expression_endpoint_strict(
            &graph,
            &uri,
            Some("Demo::System"),
            "battery.powerOut",
        );
        assert!(
            matches!(result, ResolveResult::Resolved(_)),
            "expected member chain to resolve after workspace link, got {result:?}"
        );
    }

    #[test]
    fn three_segment_member_chain_resolves_nested_typed_parts() {
        let doc = SysmlDocument::from_memory_path(
            "workspace",
            "model.sysml",
            r#"package Demo {
  part def Motor { port cmd; }
  part def Propulsion { part unit1 : Motor; }
  part def FlightController { port motorCmd; }
  part def FlightStack {
    part flightController : FlightController;
  }
  part def Drone {
    part flightControl : FlightStack;
    part propulsion : Propulsion;
    connect flightControl.flightController.motorCmd to propulsion.unit1.cmd;
  }
}"#
            .to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("workspace doc");
        let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
        let uri = Url::parse("memory://workspace/model.sysml").expect("uri");
        let source = resolve_expression_endpoint_strict(
            &graph,
            &uri,
            Some("Demo::Drone"),
            "flightControl.flightController.motorCmd",
        );
        let target = resolve_expression_endpoint_strict(
            &graph,
            &uri,
            Some("Demo::Drone"),
            "propulsion.unit1.cmd",
        );
        assert!(
            matches!(source, ResolveResult::Resolved(_)),
            "expected nested source endpoint, got {source:?}"
        );
        assert!(
            matches!(target, ResolveResult::Resolved(_)),
            "expected nested target endpoint, got {target:?}"
        );
    }

    #[test]
    fn inherited_member_resolution_skips_shadowing_child_on_owner() {
        let doc = SysmlDocument::from_memory_path(
            "workspace",
            "implicit_redefine.sysml",
            r#"package P {
  part def Base {
    attribute mass : Real;
  }
  part def Child :> Base {
    attribute mass = 1200;
  }
}"#
            .to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("workspace doc");
        let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
        let uri = Url::parse("memory://workspace/implicit_redefine.sysml").expect("uri");
        let child = graph
            .nodes_for_uri(&uri)
            .into_iter()
            .find(|node| node.element_kind == "part def" && node.name == "Child")
            .expect("child part def");
        let child_mass = graph
            .child_named(&child.id, "mass")
            .into_iter()
            .next()
            .expect("child mass");
        let base_mass = graph
            .nodes_for_uri(&uri)
            .into_iter()
            .find(|node| {
                node.name == "mass"
                    && matches!(node.element_kind.as_str(), "attribute" | "attribute def")
                    && node.id != child_mass.id
            })
            .expect("base mass");

        assert_eq!(
            resolve_member_via_type(&graph, child, "mass"),
            ResolveResult::Resolved(child_mass.id.clone())
        );
        assert_eq!(
            resolve_inherited_member_via_type(&graph, child, "mass"),
            ResolveResult::Resolved(base_mass.id.clone())
        );
    }

    #[test]
    fn requirement_usage_inherited_status_resolves() {
        let doc = SysmlDocument::from_memory_path(
            "workspace",
            "req_status.sysml",
            r#"package Demo {
  enum def RequirementStatusKind {
    enum approved;
  }
  requirement def ManagedRequirement {
    attribute status : RequirementStatusKind;
  }
  requirement def UserRequirement :> ManagedRequirement;
  requirement def Need :> UserRequirement;
  requirement need : Need {
    attribute status = "approved";
  }
}"#
            .to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("workspace doc");
        let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
        let uri = Url::parse("memory://workspace/req_status.sysml").expect("uri");
        let need = graph
            .nodes_for_uri(&uri)
            .into_iter()
            .find(|node| node.element_kind == "requirement" && node.name == "need")
            .expect("need usage");
        let need_def = graph
            .outgoing_typing_or_specializes_targets(need)
            .into_iter()
            .find(|node| node.name == "Need")
            .expect("Need def typing");
        let need_def_bases: Vec<_> = graph
            .outgoing_typing_or_specializes_targets(need_def)
            .iter()
            .map(|node| node.name.clone())
            .collect();
        assert!(
            need_def_bases.iter().any(|name| name == "UserRequirement"),
            "Need def should specialize UserRequirement, got {need_def_bases:?}"
        );
        let managed = graph
            .nodes_for_uri(&uri)
            .into_iter()
            .find(|node| node.name == "ManagedRequirement")
            .expect("ManagedRequirement def");
        let managed_status = graph
            .child_named(&managed.id, "status")
            .into_iter()
            .next()
            .expect("ManagedRequirement status");
        assert_eq!(managed_status.element_kind, "attribute def");
        let status_attr = graph
            .child_named(&need.id, "status")
            .into_iter()
            .next()
            .expect("status attribute");
        assert!(
            status_attr.attributes.contains_key("value"),
            "status attribute should carry value, attrs={:?}",
            status_attr.attributes
        );
        let inherited = resolve_inherited_member_via_type(&graph, need, "status");
        assert!(
            matches!(inherited, ResolveResult::Resolved(_)),
            "expected inherited status, got {inherited:?}"
        );
    }

    #[test]
    fn port_def_specialization_inherits_attribute() {
        let doc = SysmlDocument::from_memory_path(
            "workspace",
            "port_inherit.sysml",
            r#"package P {
  port def BasePort {
    attribute width : Real;
  }
  port def WidePort :> BasePort;
  part def Host {
    port p : WidePort {
      attribute width = 42;
    }
  }
}"#
            .to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("workspace doc");
        let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
        let uri = Url::parse("memory://workspace/port_inherit.sysml").expect("uri");
        let wide_port = graph
            .nodes_for_uri(&uri)
            .into_iter()
            .find(|node| node.element_kind == "port def" && node.name == "WidePort")
            .expect("WidePort def");
        let base_port = graph
            .nodes_for_uri(&uri)
            .into_iter()
            .find(|node| node.element_kind == "port def" && node.name == "BasePort")
            .expect("BasePort def");
        let base_width = graph
            .child_named(&base_port.id, "width")
            .into_iter()
            .next()
            .expect("BasePort width");
        let host = graph
            .nodes_for_uri(&uri)
            .into_iter()
            .find(|node| node.element_kind == "part def" && node.name == "Host")
            .expect("Host def");
        let port_usage = graph
            .child_named(&host.id, "p")
            .into_iter()
            .next()
            .expect("port usage p");
        assert_eq!(
            resolve_inherited_member_via_type(&graph, port_usage, "width"),
            ResolveResult::Resolved(base_width.id.clone())
        );
        assert!(
            graph
                .outgoing_targets_by_kind(wide_port, RelationshipKind::Specializes)
                .iter()
                .any(|node| node.id == base_port.id),
            "WidePort should specialize BasePort"
        );
    }

    #[test]
    fn enum_def_specialization_preserves_inheritance_chain() {
        let doc = SysmlDocument::from_memory_path(
            "workspace",
            "enum_inherit.sysml",
            r#"package P {
  enum def BaseEnum {
    enum a;
  }
  enum def ChildEnum :> BaseEnum;
}"#
            .to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("workspace doc");
        let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
        let uri = Url::parse("memory://workspace/enum_inherit.sysml").expect("uri");
        let child = graph
            .nodes_for_uri(&uri)
            .into_iter()
            .find(|node| node.element_kind == "enum def" && node.name == "ChildEnum")
            .expect("ChildEnum def");
        let base = graph
            .nodes_for_uri(&uri)
            .into_iter()
            .find(|node| node.element_kind == "enum def" && node.name == "BaseEnum")
            .expect("BaseEnum def");
        assert_eq!(
            child.attributes.get("specializes").and_then(|v| v.as_str()),
            Some("BaseEnum")
        );
        assert!(
            graph
                .outgoing_targets_by_kind(child, RelationshipKind::Specializes)
                .iter()
                .any(|node| node.id == base.id),
            "ChildEnum should specialize BaseEnum"
        );
    }

    #[test]
    fn use_case_def_specialization_resolves() {
        let doc = SysmlDocument::from_memory_path(
            "workspace",
            "usecase_inherit.sysml",
            r#"package P {
  case def Case;
  use case def MyUseCase :> Case;
}"#
            .to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("workspace doc");
        let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
        let uri = Url::parse("memory://workspace/usecase_inherit.sysml").expect("uri");
        let use_case = graph
            .nodes_for_uri(&uri)
            .into_iter()
            .find(|node| node.element_kind == "use case def" && node.name == "MyUseCase")
            .expect("MyUseCase def");
        let case_def = graph
            .nodes_for_uri(&uri)
            .into_iter()
            .find(|node| node.element_kind == "case def" && node.name == "Case")
            .expect("Case def");
        assert!(
            graph
                .outgoing_targets_by_kind(use_case, RelationshipKind::Specializes)
                .iter()
                .any(|node| node.id == case_def.id),
            "MyUseCase should specialize Case"
        );
    }

    #[test]
    fn inherited_member_resolution_prefers_specialized_redefinition() {
        use crate::semantic::evaluation::evaluate_expressions;

        let doc = SysmlDocument::from_memory_path(
            "workspace",
            "subsystem-specialize.sysml",
            r#"package Demo {
  part def RobotSubsystem {
    attribute powerDrawW : Real;
  }
  part def MobilitySubsystem :> RobotSubsystem {
    attribute :>> powerDrawW = 28;
  }
  part def Robot {
    part mobility : MobilitySubsystem;
  }
  analysis def PowerAnalysis {
    attribute powerBudgetW : Real = 55;
    subject robot : Robot;
    return ref withinBudget {
      return robot.mobility.powerDrawW <= powerBudgetW;
    }
  }
}"#
            .to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("workspace doc");
        let (mut graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
        evaluate_expressions(&mut graph);
        let uri = Url::parse("memory://workspace/subsystem-specialize.sysml").expect("uri");
        let analysis = graph
            .nodes_for_uri(&uri)
            .into_iter()
            .find(|node| node.element_kind == "analysis def" && node.name == "PowerAnalysis")
            .expect("analysis");
        assert_eq!(
            analysis
                .attributes
                .get("analysisEvaluationStatus")
                .and_then(|value| value.as_str()),
            Some("ok"),
            "specialized :>> attribute values should resolve for analysis roll-up"
        );
    }
}
