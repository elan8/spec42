//! Graph-first unit type resolution using MeasurementUnit specialization chains.

use std::collections::{HashSet, VecDeque};

use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::{ElementKind, SemanticNode};

const MEASUREMENT_UNIT_ROOTS: &[&str] = &[
    "MeasurementUnit",
    "SimpleUnit",
    "DerivedUnit",
    "ScalarMeasurementReference",
];

pub fn base_type_name(name: &str) -> &str {
    name.rsplit("::").next().unwrap_or(name).trim()
}

pub fn is_unit_type_name(name: &str) -> bool {
    let base = base_type_name(name);
    base.ends_with("Unit") || MEASUREMENT_UNIT_ROOTS.contains(&base)
}

pub fn is_unit_type_name_in_graph(graph: &SemanticGraph, name: &str) -> bool {
    let base = base_type_name(name);
    if is_unit_type_name(base) {
        return true;
    }
    unit_type_ancestors(graph, base)
        .iter()
        .map(|ancestor| base_type_name(ancestor))
        .any(|ancestor| MEASUREMENT_UNIT_ROOTS.contains(&ancestor))
}

pub fn quantity_value_to_unit_type_name(type_name: &str) -> Option<String> {
    let base = base_type_name(type_name);
    base.strip_suffix("Value")
        .map(|stripped| format!("{stripped}Unit"))
}

pub fn unit_type_for_quantity_value<'a>(
    graph: &'a SemanticGraph,
    quantity_node: &SemanticNode,
) -> Option<&'a SemanticNode> {
    for target in graph.outgoing_typing_or_specializes_targets(quantity_node) {
        if is_unit_type_name(&target.name) {
            return Some(target);
        }
    }
    for child in graph.children_of(quantity_node) {
        if child.name == "mRef" || child.name.ends_with("::mRef") {
            for target in graph.outgoing_typing_or_specializes_targets(child) {
                if is_unit_type_name(&target.name) {
                    return Some(target);
                }
            }
            if let Some(type_ref) = child
                .attributes
                .get("attributeType")
                .and_then(|v| v.as_str())
            {
                if is_unit_type_name(type_ref) {
                    return graph.nodes_named(type_ref).into_iter().next();
                }
            }
        }
    }
    quantity_node
        .attributes
        .get("attributeType")
        .and_then(|v| v.as_str())
        .and_then(|type_ref| {
            quantity_value_to_unit_type_name(type_ref)
                .and_then(|unit_name| graph.nodes_named(&unit_name).into_iter().next())
        })
}

pub fn unit_type_for_quantity_type_name(graph: &SemanticGraph, type_name: &str) -> Option<String> {
    let normalized = type_name.rsplit("::").next().unwrap_or(type_name).trim();
    if is_unit_type_name(normalized) {
        return Some(normalized.to_string());
    }
    for node in graph.nodes_named(type_name) {
        if node.element_kind != ElementKind::AttributeDef {
            continue;
        }
        if let Some(unit_node) = unit_type_for_quantity_value(graph, node) {
            return Some(
                unit_node
                    .name
                    .rsplit("::")
                    .next()
                    .unwrap_or(&unit_node.name)
                    .to_string(),
            );
        }
        for target in graph.outgoing_typing_or_specializes_targets(node) {
            let base = target.name.rsplit("::").next().unwrap_or(&target.name);
            if let Some(unit) = quantity_value_to_unit_type_name(base) {
                return Some(unit);
            }
            if is_unit_type_name(base) {
                return Some(base.to_string());
            }
        }
    }
    quantity_value_to_unit_type_name(normalized)
}

pub fn is_measurement_unit_compatible(graph: &SemanticGraph, expected: &str, actual: &str) -> bool {
    let expected_base = expected.rsplit("::").next().unwrap_or(expected);
    let actual_base = actual.rsplit("::").next().unwrap_or(actual);
    if expected_base == actual_base {
        return true;
    }
    if quantity_value_to_unit_type_name(expected_base).as_deref() == Some(actual_base)
        || quantity_value_to_unit_type_name(actual_base).as_deref() == Some(expected_base)
    {
        return true;
    }
    const ALIASES: &[(&str, &str)] = &[
        ("ElectricPotentialDifferenceUnit", "ElectricPotentialUnit"),
        ("ElectricPotentialUnit", "ElectricPotentialDifferenceUnit"),
    ];
    if ALIASES
        .iter()
        .any(|(left, right)| expected_base == *left && actual_base == *right)
    {
        return true;
    }
    let expected_ancestors = unit_type_ancestors(graph, expected_base);
    let actual_ancestors = unit_type_ancestors(graph, actual_base);
    if expected_ancestors.contains(actual_base) || actual_ancestors.contains(expected_base) {
        return true;
    }
    false
}

fn unit_type_ancestors(graph: &SemanticGraph, type_name: &str) -> HashSet<String> {
    let mut ancestors = HashSet::new();
    let mut queue: VecDeque<String> = graph
        .nodes_named(type_name)
        .into_iter()
        .flat_map(|node| {
            graph
                .outgoing_typing_or_specializes_targets(node)
                .into_iter()
                .map(|t| t.name.clone())
                .collect::<Vec<_>>()
        })
        .collect();
    ancestors.insert(type_name.to_string());
    while let Some(current) = queue.pop_front() {
        if !ancestors.insert(current.clone()) {
            continue;
        }
        for node in graph.nodes_named(&current) {
            for target in graph.outgoing_typing_or_specializes_targets(node) {
                queue.push_back(target.name.clone());
            }
        }
    }
    ancestors
}

#[cfg(test)]
mod tests {
    use url::Url;

    use crate::semantic::graph_builder::build_graph_from_doc;
    use crate::semantic::relationships::link_workspace_relationships;
    use sysml_v2_parser::parse;

    use super::*;

    const ISQ_QUANTITIES: &str = r#"
package ISQElectromagnetism {
    attribute def ElectricPotentialDifferenceValue;
    attribute def ElectricPotentialDifferenceUnit;
}
package ElectricalQuantities {
    private import ISQElectromagnetism::*;
    attribute def Voltage :> ElectricPotentialDifferenceValue;
}
"#;

    #[test]
    fn resolves_unit_type_from_quantity_specialization() {
        let uri = Url::parse("file:///test/quantities.sysml").expect("uri");
        let parsed = parse(ISQ_QUANTITIES).expect("parse");
        let mut graph = build_graph_from_doc(&parsed, &uri);
        link_workspace_relationships(&mut graph);
        let _voltage = graph
            .nodes_named("Voltage")
            .into_iter()
            .find(|n| n.element_kind == ElementKind::AttributeDef)
            .expect("Voltage def");
        let unit_type = unit_type_for_quantity_type_name(&graph, "Voltage").expect("unit type");
        assert_eq!(unit_type, "ElectricPotentialDifferenceUnit");
    }

    #[test]
    fn recognizes_custom_unit_type_by_measurement_ancestor() {
        let uri = Url::parse("file:///test/custom-units.sysml").expect("uri");
        let parsed = parse(
            r#"
            package Measurement {
                attribute def MeasurementUnit;
                attribute def CustomMeasure :> MeasurementUnit;
            }
            "#,
        )
        .expect("parse");
        let mut graph = build_graph_from_doc(&parsed, &uri);
        link_workspace_relationships(&mut graph);

        assert!(is_unit_type_name_in_graph(&graph, "CustomMeasure"));
        assert!(is_unit_type_name_in_graph(
            &graph,
            "Measurement::CustomMeasure"
        ));
    }
}
