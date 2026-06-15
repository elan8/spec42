//! Ingest unit definitions from semantic graph nodes.

use serde_json::Value;

use crate::semantic::graph::SemanticGraph;
use crate::semantic::graph_builder::unit_metadata::{
    SHORT_NAME_KEY, UNIT_CONVERSION_KEY, UNIT_PREFIX_KEY, UNIT_VALUE_EXPR_KEY,
};
use crate::semantic::model::SemanticNode;
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
    node.attributes
        .get("attributeType")
        .and_then(|v| v.as_str())
        .map(base_type_name)
        == Some("IntervalScale")
}

fn is_unit_catalog_element_kind(kind: &str) -> bool {
    kind == "attribute def" || kind == "attribute"
}

fn unit_prefix_from_node(node: &SemanticNode) -> Option<(String, Option<String>, f64)> {
    if node
        .attributes
        .get("attributeType")
        .and_then(|v| v.as_str())
        .map(base_type_name)
        != Some("UnitPrefix")
    {
        return None;
    }
    let obj = node.attributes.get(UNIT_PREFIX_KEY)?.as_object()?;
    let factor = obj.get("conversionFactor").and_then(|v| v.as_f64())?;
    let symbol = obj
        .get("symbol")
        .and_then(|v| v.as_str())
        .map(str::to_string);
    Some((node.name.clone(), symbol, factor))
}

fn unit_def_from_graph_node(
    graph: &SemanticGraph,
    node: &SemanticNode,
    registry: &UnitRegistry,
) -> Option<UnitDef> {
    let attribute_type = node
        .attributes
        .get("attributeType")
        .and_then(|v| v.as_str())?;
    let attribute_type_base = base_type_name(attribute_type);
    if attribute_type_base == "UnitPrefix" {
        return None;
    }

    let short_name = node
        .attributes
        .get(SHORT_NAME_KEY)
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .filter(|s| !s.is_empty());
    let unit_value_expr = node
        .attributes
        .get(UNIT_VALUE_EXPR_KEY)
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .filter(|s| !s.is_empty());
    let has_conversion = node.attributes.contains_key(UNIT_CONVERSION_KEY);

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
    if let Some(meta) = node.attributes.get(UNIT_CONVERSION_KEY) {
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
    meta: &Value,
    registry: &UnitRegistry,
    reference_unit: &mut Option<String>,
    conversion_factor: &mut f64,
    conversion_offset: &mut f64,
) {
    let Some(obj) = meta.as_object() else {
        return;
    };
    let kind = obj.get("kind").and_then(|v| v.as_str()).unwrap_or("");
    match kind {
        "ConversionByConvention" => {
            *reference_unit = obj
                .get("referenceUnit")
                .and_then(|v| v.as_str())
                .map(str::to_string);
            if let Some(factor) = obj.get("conversionFactor").and_then(|v| v.as_f64()) {
                *conversion_factor = factor;
            }
        }
        "ConversionByPrefix" => {
            *reference_unit = obj
                .get("referenceUnit")
                .and_then(|v| v.as_str())
                .map(str::to_string);
            if let Some(prefix) = obj.get("prefix").and_then(|v| v.as_str()) {
                if let Some(factor) = registry.prefix_factor_by_name(prefix) {
                    *conversion_factor = factor;
                }
            }
        }
        "IntervalScale" => {
            *reference_unit = Some("K".to_string());
            if let Some(interval_unit) = obj.get("intervalUnit").and_then(|v| v.as_str()) {
                if let Some(base) = registry.get(interval_unit) {
                    *conversion_factor = base.conversion_factor;
                }
            }
            if let Some(zero) = obj.get("zeroOffsetKelvin").and_then(|v| v.as_f64()) {
                *conversion_offset = zero;
            }
        }
        _ => {}
    }
}
