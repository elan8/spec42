//! Ingest unit definitions from semantic graph nodes.

use serde_json::Value;

use crate::semantic::units::registry::{UnitDef, UnitRegistry};
use crate::semantic::graph::SemanticGraph;
use crate::semantic::graph_builder::unit_metadata::{SHORT_NAME_KEY, UNIT_CONVERSION_KEY};
use crate::semantic::model::SemanticNode;
use crate::semantic::units::type_resolver::is_unit_type_name;

pub fn ingest_units_from_graph(graph: &SemanticGraph, registry: &mut UnitRegistry) {
    let node_ids: Vec<_> = graph.node_index_by_id.keys().cloned().collect();
    for node_id in node_ids {
        let Some(node) = graph.get_node(&node_id) else {
            continue;
        };
        if node.element_kind != "attribute def" {
            continue;
        }
        if let Some(def) = unit_def_from_graph_node(node) {
            if !registry.has_symbol(&def.symbol) {
                registry.ingest_unit_def(def);
            }
        }
    }
}

fn unit_def_from_graph_node(node: &SemanticNode) -> Option<UnitDef> {
    let symbol = node
        .attributes
        .get(SHORT_NAME_KEY)
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .filter(|s| !s.is_empty())
        .or_else(|| {
            let dimension = node.attributes.get("attributeType").and_then(|v| v.as_str())?;
            is_unit_type_name(dimension).then(|| node.name.clone())
        })?;
    let dimension = node
        .attributes
        .get("attributeType")
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .filter(|d| is_unit_type_name(d))?;
    let mut reference_unit = None;
    let mut conversion_factor = 1.0_f64;
    let mut conversion_offset = 0.0_f64;
    if let Some(meta) = node.attributes.get(UNIT_CONVERSION_KEY) {
        apply_conversion_meta(meta, &mut reference_unit, &mut conversion_factor, &mut conversion_offset);
    }
    let algebraic_expr = node
        .attributes
        .get("unitValueExpr")
        .and_then(|v| v.as_str())
        .map(str::to_string);
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
                if prefix == "kilo" {
                    *conversion_factor = 1E3;
                } else if prefix == "mega" {
                    *conversion_factor = 1E6;
                } else if prefix == "milli" {
                    *conversion_factor = 1E-3;
                }
            }
        }
        "IntervalScale" => {
            *reference_unit = Some("K".to_string());
            if let Some(interval_unit) = obj.get("intervalUnit").and_then(|v| v.as_str()) {
                // absolute scale references difference unit for linear factor
                let _ = interval_unit;
            }
            if let Some(zero) = obj.get("zeroOffsetKelvin").and_then(|v| v.as_f64()) {
                *conversion_offset = zero;
            }
        }
        _ => {}
    }
}
