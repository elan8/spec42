//! Project unit-catalog metadata from attribute definitions onto graph node attributes.

use std::collections::HashMap;

use serde_json::{json, Value};
use sysml_v2_parser::ast::{AttributeBody, AttributeBodyElement, AttributeDef};

use crate::semantic::graph_builder::expressions;

/// Keys stored on unit-related `attribute def` graph nodes.
pub const SHORT_NAME_KEY: &str = "shortName";
pub const UNIT_CONVERSION_KEY: &str = "unitConversion";
pub const UNIT_VALUE_EXPR_KEY: &str = "unitValueExpr";

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UnitConversionMeta {
    pub kind: String,
    pub reference_unit: Option<String>,
    pub conversion_factor: Option<f64>,
    pub prefix: Option<String>,
    pub interval_unit: Option<String>,
    pub zero_offset_kelvin: Option<f64>,
}

pub fn project_attribute_def_unit_metadata(
    attrs: &mut HashMap<String, Value>,
    def: &AttributeDef,
) {
    if let Some(short) = def.short_name.as_ref().filter(|s| !s.is_empty()) {
        attrs.insert(SHORT_NAME_KEY.to_string(), json!(short));
    }
    if let Some(expr) = &def.value {
        let rendered = expressions::expression_to_debug_string(expr);
        attrs.insert(UNIT_VALUE_EXPR_KEY.to_string(), json!(rendered));
    }
    if let Some(conversion) = extract_unit_conversion_from_body(&def.body) {
        attrs.insert(
            UNIT_CONVERSION_KEY.to_string(),
            json!({
                "kind": conversion.kind,
                "referenceUnit": conversion.reference_unit,
                "conversionFactor": conversion.conversion_factor,
                "prefix": conversion.prefix,
                "intervalUnit": conversion.interval_unit,
                "zeroOffsetKelvin": conversion.zero_offset_kelvin,
            }),
        );
    }
}

fn extract_unit_conversion_from_body(body: &AttributeBody) -> Option<UnitConversionMeta> {
    let AttributeBody::Brace { elements } = body else {
        return None;
    };
    let mut block_text = String::new();
    for element in elements {
        match &element.value {
            AttributeBodyElement::Other(text) => {
                if !block_text.is_empty() {
                    block_text.push(' ');
                }
                block_text.push_str(text.trim());
            }
            AttributeBodyElement::AttributeDef(nested) => {
                let nested_text = attribute_def_as_text(&nested.value);
                if !block_text.is_empty() {
                    block_text.push(' ');
                }
                block_text.push_str(&nested_text);
            }
            _ => {}
        }
    }
    if block_text.is_empty() {
        return None;
    }
    extract_unit_conversion_from_text(&block_text)
}

fn attribute_def_as_text(def: &AttributeDef) -> String {
    let mut out = String::from("attribute ");
    if let Some(short) = &def.short_name {
        out.push('<');
        out.push_str(short);
        out.push_str("> ");
    }
    out.push_str(&def.name);
    if let Some(typing) = &def.typing {
        out.push_str(" : ");
        out.push_str(typing);
    }
    if let AttributeBody::Brace { elements } = &def.body {
        out.push_str(" { ");
        for element in elements {
            if let AttributeBodyElement::Other(text) = &element.value {
                out.push_str(text.trim());
                out.push(' ');
            }
        }
        out.push_str("} ");
    }
    out
}

pub fn extract_unit_conversion_from_text(text: &str) -> Option<UnitConversionMeta> {
    if text.contains(": IntervalScale") || text.contains("IntervalScale {") {
        let unit = extract_assignment(text, "unit");
        let zero = text
            .lines()
            .find(|line| line.contains("zeroDegree") && line.contains('='))
            .and_then(|line| parse_zero_point_kelvin(line));
        return Some(UnitConversionMeta {
            kind: "IntervalScale".to_string(),
            reference_unit: Some("K".to_string()),
            conversion_factor: Some(1.0),
            prefix: None,
            interval_unit: unit,
            zero_offset_kelvin: zero,
        });
    }
    if text.contains("ConversionByPrefix") {
        return Some(UnitConversionMeta {
            kind: "ConversionByPrefix".to_string(),
            reference_unit: extract_assignment(text, "referenceUnit"),
            conversion_factor: None,
            prefix: extract_assignment(text, "prefix"),
            interval_unit: None,
            zero_offset_kelvin: None,
        });
    }
    if text.contains("ConversionByConvention") || text.contains("referenceUnit") {
        return Some(UnitConversionMeta {
            kind: "ConversionByConvention".to_string(),
            reference_unit: extract_assignment(text, "referenceUnit"),
            conversion_factor: extract_assignment(text, "conversionFactor")
                .and_then(|raw| parse_factor_expression(&raw)),
            prefix: None,
            interval_unit: None,
            zero_offset_kelvin: None,
        });
    }
    None
}

fn extract_assignment(line: &str, key: &str) -> Option<String> {
    let needle = format!("{key} =");
    let idx = line.find(&needle)?;
    let value_start = idx + needle.len();
    let rest = line[value_start..].trim_start();
    let end = rest.find(';').unwrap_or(rest.len());
    let value = rest[..end].trim();
    if value.is_empty() {
        None
    } else {
        Some(strip_quotes(value))
    }
}

fn parse_zero_point_kelvin(line: &str) -> Option<f64> {
    let equals = line.find('=')?;
    let bracket = line[equals + 1..].find('[')?;
    let raw = line[equals + 1..equals + 1 + bracket].trim();
    parse_factor_expression(raw)
}

fn parse_factor_expression(raw: &str) -> Option<f64> {
    let trimmed = raw.trim();
    if let Some(div) = trimmed.find('/') {
        let left = trimmed[..div].trim().parse::<f64>().ok()?;
        let right = trimmed[div + 1..].trim().parse::<f64>().ok()?;
        if right == 0.0 {
            return None;
        }
        return Some(left / right);
    }
    trimmed.parse::<f64>().ok()
}

fn strip_quotes(value: &str) -> String {
    let mut out = value.trim().to_string();
    if out.starts_with('\'') && out.ends_with('\'') && out.len() > 1 {
        out = out[1..out.len() - 1].to_string();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_conversion_by_convention() {
        let text = ":>> unitConversion: ConversionByConvention { :>> referenceUnit = m; :>> conversionFactor = 3.048E-01; }";
        let meta = extract_unit_conversion_from_text(text).expect("meta");
        assert_eq!(meta.kind, "ConversionByConvention");
        assert_eq!(meta.reference_unit.as_deref(), Some("m"));
        assert!((meta.conversion_factor.unwrap() - 0.3048).abs() < 1e-6);
    }

    #[test]
    fn extracts_conversion_by_prefix() {
        let text = ":>> unitConversion: ConversionByPrefix { :>> prefix = kilo; :>> referenceUnit = m; }";
        let meta = extract_unit_conversion_from_text(text).expect("meta");
        assert_eq!(meta.kind, "ConversionByPrefix");
        assert_eq!(meta.prefix.as_deref(), Some("kilo"));
        assert_eq!(meta.reference_unit.as_deref(), Some("m"));
    }
}
