//! Ingest unit definitions from semantic graph nodes.

use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::{DeclaredUnitConversion, SemanticNode};
use crate::semantic::units::registry::{UnitDef, UnitRegistry};
use crate::semantic::units::type_resolver::{base_type_name, is_unit_type_name_in_graph};

pub fn ingest_units_from_graph(graph: &SemanticGraph, registry: &mut UnitRegistry) {
    let node_ids: Vec<_> = graph.node_index_by_id.keys().cloned().collect();

    for node_id in &node_ids {
        let Some(node) = graph.get_node(node_id) else {
            continue;
        };
        if !is_unit_catalog_element_kind(&node.element_kind) {
            continue;
        }
        if let Some((name, symbol, factor)) = unit_prefix_from_node(node) {
            registry.ingest_unit_prefix(&name, symbol.as_deref(), factor);
        }
    }

    ingest_non_interval_unit_defs(graph, registry, &node_ids);
    ingest_interval_scale_unit_defs(graph, registry, &node_ids);
}

fn ingest_non_interval_unit_defs(
    graph: &SemanticGraph,
    registry: &mut UnitRegistry,
    node_ids: &[crate::semantic::model::NodeId],
) {
    for node_id in node_ids {
        let Some(node) = graph.get_node(node_id) else {
            continue;
        };
        if !is_unit_catalog_element_kind(&node.element_kind) {
            continue;
        }
        if unit_prefix_from_node(node).is_some() || is_interval_scale_node(node) {
            continue;
        }
        if let Some(def) = unit_def_from_graph_node(graph, node, registry) {
            registry.ingest_unit_def(def);
        }
    }
}

fn ingest_interval_scale_unit_defs(
    graph: &SemanticGraph,
    registry: &mut UnitRegistry,
    node_ids: &[crate::semantic::model::NodeId],
) {
    for node_id in node_ids {
        let Some(node) = graph.get_node(node_id) else {
            continue;
        };
        if !is_unit_catalog_element_kind(&node.element_kind) {
            continue;
        }
        if unit_prefix_from_node(node).is_some() || !is_interval_scale_node(node) {
            continue;
        }
        if let Some(def) = unit_def_from_graph_node(graph, node, registry) {
            registry.ingest_unit_def(def);
        }
    }
}

fn is_interval_scale_node(node: &SemanticNode) -> bool {
    node.declared_facts
        .relationships
        .typing
        .first()
        .map(|target| target.reference.as_str())
        .map(base_type_name)
        == Some("IntervalScale")
}

fn is_unit_catalog_element_kind(kind: &crate::ElementKind) -> bool {
    *kind == crate::ElementKind::AttributeDef || *kind == crate::ElementKind::Attribute
}

fn unit_prefix_from_node(node: &SemanticNode) -> Option<(String, Option<String>, f64)> {
    if node
        .declared_facts
        .relationships
        .typing
        .first()
        .map(|target| target.reference.as_str())
        .map(base_type_name)
        != Some("UnitPrefix")
    {
        return None;
    }
    let prefix = node.declared_facts.unit.as_ref()?.prefix.as_ref()?;
    Some((
        node.name.clone(),
        prefix.symbol.clone(),
        prefix.conversion_factor,
    ))
}

fn unit_def_from_graph_node(
    graph: &SemanticGraph,
    node: &SemanticNode,
    registry: &UnitRegistry,
) -> Option<UnitDef> {
    let attribute_type = node
        .declared_facts
        .relationships
        .typing
        .first()
        .map(|target| target.reference.as_str())?;
    let attribute_type_base = base_type_name(attribute_type);
    if attribute_type_base == "UnitPrefix" {
        return None;
    }

    let unit_facts = node.declared_facts.unit.as_ref();
    let short_name = node
        .declared_facts
        .short_name
        .clone()
        .filter(|s| !s.is_empty());
    let unit_value_expr = unit_facts
        .and_then(|u| u.value_expr.clone())
        .filter(|s| !s.is_empty());
    let has_conversion = unit_facts.is_some_and(|u| u.conversion.is_some());

    let symbol = short_name.or_else(|| {
        if unit_value_expr.is_some() || has_conversion {
            Some(node.name.clone())
        } else {
            None
        }
    })?;

    let dimension = if attribute_type_base == "IntervalScale" {
        "ThermodynamicTemperatureUnit".to_string()
    } else if is_unit_type_name_in_graph(graph, attribute_type) {
        attribute_type_base.to_string()
    } else {
        return None;
    };

    let mut reference_unit = None;
    let mut conversion_factor = 1.0_f64;
    let mut conversion_offset = 0.0_f64;
    if let Some(meta) = unit_facts.and_then(|u| u.conversion.as_ref()) {
        apply_conversion_meta(
            meta,
            registry,
            &mut reference_unit,
            &mut conversion_factor,
            &mut conversion_offset,
        );
    }
    let algebraic_expr = unit_value_expr;
    Some(UnitDef {
        symbol,
        dimension,
        reference_unit,
        conversion_factor,
        conversion_offset,
        algebraic_expr,
    })
}

fn apply_conversion_meta(
    meta: &DeclaredUnitConversion,
    registry: &UnitRegistry,
    reference_unit: &mut Option<String>,
    conversion_factor: &mut f64,
    conversion_offset: &mut f64,
) {
    match meta.kind.as_str() {
        "ConversionByConvention" => {
            reference_unit.clone_from(&meta.reference_unit);
            if let Some(factor) = meta.conversion_factor {
                *conversion_factor = factor;
            }
        }
        "ConversionByPrefix" => {
            reference_unit.clone_from(&meta.reference_unit);
            if let Some(prefix) = meta.prefix.as_deref() {
                if let Some(factor) = registry.prefix_factor_by_name(prefix) {
                    *conversion_factor = factor;
                }
            }
        }
        "IntervalScale" => {
            *reference_unit = Some("K".to_string());
            if let Some(interval_unit) = meta.interval_unit.as_deref() {
                if let Some(base) = registry.get(interval_unit) {
                    *conversion_factor = base.conversion_factor;
                }
            }
            if let Some(zero) = meta.zero_offset_kelvin {
                *conversion_offset = zero;
            }
        }
        _ => {}
    }
}
