//! Project unit-catalog metadata from attribute definitions onto graph node attributes.

use std::collections::HashMap;

use serde_json::{json, Value};
use sysml_v2_parser::ast::{AttributeBody, AttributeBodyElement, AttributeDef, AttributeUsage};

use crate::semantic::graph_builder::expressions;

/// Keys stored on unit-related `attribute def` graph nodes.
pub const SHORT_NAME_KEY: &str = "shortName";
pub const UNIT_CONVERSION_KEY: &str = "unitConversion";
pub const UNIT_VALUE_EXPR_KEY: &str = "unitValueExpr";
pub const UNIT_PREFIX_KEY: &str = "unitPrefix";

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UnitConversionMeta {
    pub kind: String,
    pub reference_unit: Option<String>,
    pub conversion_factor: Option<f64>,
    pub prefix: Option<String>,
    pub interval_unit: Option<String>,
    pub zero_offset_kelvin: Option<f64>,
}

pub fn project_attribute_def_unit_metadata(attrs: &mut HashMap<String, Value>, def: &AttributeDef) {
    if let Some(short) = def.short_name.as_ref().filter(|s| !s.is_empty()) {
        attrs.insert(SHORT_NAME_KEY.to_string(), json!(short));
    }
    if let Some(expr) = &def.value {
        let rendered = expressions::expression_to_debug_string(expr);
        attrs.insert(UNIT_VALUE_EXPR_KEY.to_string(), json!(rendered));
    }
    project_unit_body_metadata(attrs, def.typing.as_deref(), &def.body);
}

pub fn project_attribute_usage_unit_metadata(
    attrs: &mut HashMap<String, Value>,
    usage: &AttributeUsage,
) {
    project_unit_body_metadata(attrs, usage.typing.as_deref(), &usage.body);
}

fn project_unit_body_metadata(
    attrs: &mut HashMap<String, Value>,
    typing: Option<&str>,
    body: &AttributeBody,
) {
    if typing.map(base_type_name) == Some("UnitPrefix") {
        if let Some(prefix) = extract_unit_prefix_from_body(body) {
            attrs.insert(
                UNIT_PREFIX_KEY.to_string(),
                json!({
                    "symbol": prefix.symbol,
                    "conversionFactor": prefix.conversion_factor,
                }),
            );
        }
    }
    let conversion = if typing.map(base_type_name) == Some("IntervalScale") {
        extract_interval_scale_from_body(body)
    } else {
        extract_unit_conversion_from_body(body)
    };
    if let Some(conversion) = conversion {
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

fn base_type_name(name: &str) -> &str {
    name.rsplit("::").next().unwrap_or(name).trim()
}

#[derive(Debug, Clone, PartialEq)]
struct UnitPrefixMeta {
    symbol: Option<String>,
    conversion_factor: f64,
}

fn extract_unit_prefix_from_body(body: &AttributeBody) -> Option<UnitPrefixMeta> {
    let block_text = attribute_body_as_text(body)?;
    let factor = extract_assignment(&block_text, "conversionFactor")
        .and_then(|raw| parse_factor_expression(&raw))?;
    Some(UnitPrefixMeta {
        symbol: extract_assignment(&block_text, "symbol"),
        conversion_factor: factor,
    })
}

fn attribute_body_as_text(body: &AttributeBody) -> Option<String> {
    let AttributeBody::Brace { elements } = body else {
        return None;
    };
    let mut block_text = String::new();
    for element in elements {
        append_body_element_text(&mut block_text, &element.value);
    }
    if block_text.is_empty() {
        None
    } else {
        Some(block_text)
    }
}

fn append_body_element_text(out: &mut String, element: &AttributeBodyElement) {
    match element {
        AttributeBodyElement::Other(text) => {
            if !out.is_empty() {
                out.push(' ');
            }
            out.push_str(text.trim());
        }
        AttributeBodyElement::AttributeDef(nested) => {
            if !out.is_empty() {
                out.push(' ');
            }
            out.push_str(&attribute_def_as_text(&nested.value));
        }
        AttributeBodyElement::AttributeUsage(nested) => {
            if !out.is_empty() {
                out.push(' ');
            }
            out.push_str(&attribute_usage_as_text(&nested.value));
        }
        AttributeBodyElement::Doc(_) | AttributeBodyElement::Error(_) => {
            if let AttributeBodyElement::Error(err) = element {
                if let Some(found) = &err.value.found {
                    if !out.is_empty() {
                        out.push(' ');
                    }
                    out.push_str(found);
                }
            }
        }
    }
}

fn attribute_usage_as_text(usage: &AttributeUsage) -> String {
    let mut out = String::new();
    let redefines = usage.redefines.as_deref();
    if let Some(r) = redefines {
        out.push_str(":>> ");
        out.push_str(r);
        out.push(' ');
    } else if let Some(s) = &usage.subsets {
        out.push_str(":> ");
        out.push_str(s);
        out.push(' ');
    }
    if redefines != Some(usage.name.as_str()) && !usage.name.is_empty() {
        out.push_str(&usage.name);
        out.push(' ');
    }
    if let Some(typing) = &usage.typing {
        out.push_str(": ");
        out.push_str(typing);
        out.push(' ');
    }
    if let AttributeBody::Brace { elements } = &usage.body {
        out.push_str("{ ");
        for element in elements {
            append_body_element_text(&mut out, &element.value);
            out.push(' ');
        }
        out.push_str("} ");
    }
    if let Some(expr) = &usage.value {
        out.push_str("= ");
        out.push_str(&expressions::expression_to_debug_string(expr));
        out.push_str("; ");
    }
    out
}

fn extract_unit_conversion_from_body(body: &AttributeBody) -> Option<UnitConversionMeta> {
    let block_text = attribute_body_as_text(body)?;
    extract_unit_conversion_from_text(&block_text)
}

fn extract_interval_scale_from_body(body: &AttributeBody) -> Option<UnitConversionMeta> {
    let block_text = attribute_body_as_text(body)?;
    let unit = extract_assignment(&block_text, "unit");
    let zero = block_text
        .find("zeroDegree")
        .and_then(|idx| parse_zero_point_kelvin(&block_text[idx..]));
    Some(UnitConversionMeta {
        kind: "IntervalScale".to_string(),
        reference_unit: Some("K".to_string()),
        conversion_factor: Some(1.0),
        prefix: None,
        interval_unit: unit,
        zero_offset_kelvin: zero,
    })
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
            append_body_element_text(&mut out, &element.value);
            out.push(' ');
        }
        out.push_str("} ");
    }
    if let Some(expr) = &def.value {
        out.push_str("= ");
        out.push_str(&expressions::expression_to_debug_string(expr));
        out.push_str("; ");
    }
    out
}

pub fn extract_unit_conversion_from_text(text: &str) -> Option<UnitConversionMeta> {
    if text.contains(": IntervalScale") || text.contains("IntervalScale {") {
        let unit = extract_assignment(text, "unit");
        let zero = text
            .find("zeroDegree")
            .and_then(|idx| parse_zero_point_kelvin(&text[idx..]));
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
    let tail = line.rsplit('=').next()?.trim().trim_end_matches(';').trim();
    let numeric = tail.split('[').next()?.trim();
    parse_factor_expression(numeric)
}

fn parse_factor_expression(raw: &str) -> Option<f64> {
    let mut trimmed = raw.trim();
    if trimmed.starts_with('(') && !trimmed.ends_with(')') {
        trimmed = trimmed[1..].trim();
    }
    while trimmed.starts_with('(') && trimmed.ends_with(')') {
        trimmed = trimmed[1..trimmed.len() - 1].trim();
    }
    let compact: String = trimmed.chars().filter(|c| !c.is_whitespace()).collect();
    if let Some(div) = compact.find('/') {
        let left = compact[..div].parse::<f64>().ok()?;
        let right = compact[div + 1..].parse::<f64>().ok()?;
        if right == 0.0 {
            return None;
        }
        return Some(left / right);
    }
    compact.parse::<f64>().ok()
}

fn strip_quotes(value: &str) -> String {
    let mut out = value.trim().to_string();
    if out.len() > 1
        && ((out.starts_with('\'') && out.ends_with('\''))
            || (out.starts_with('"') && out.ends_with('"')))
    {
        out = out[1..out.len() - 1].to_string();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn km_body_text_includes_reference_unit() {
        use sysml_v2_parser::ast::RootElement;
        use sysml_v2_parser::parse;

        let parsed = parse(
            "package P { attribute <km> kilometre : LengthUnit { :>> unitConversion: ConversionByPrefix { :>> prefix = kilo; :>> referenceUnit = m; } } }",
        )
        .expect("parse");
        let attr = parsed
            .elements
            .iter()
            .find_map(|el| match &el.value {
                RootElement::Package(pkg) => match &pkg.value.body {
                    sysml_v2_parser::ast::PackageBody::Brace { elements } => {
                        elements.first().map(|node| &node.value)
                    }
                    _ => None,
                },
                _ => None,
            })
            .expect("attr");
        let body = match attr {
            sysml_v2_parser::ast::PackageBodyElement::AttributeDef(def) => &def.body,
            _ => panic!("expected attribute def"),
        };
        if let sysml_v2_parser::ast::AttributeBody::Brace { elements } = body {
            let kinds: Vec<_> = elements
                .iter()
                .map(|el| match &el.value {
                    AttributeBodyElement::Error(err) => {
                        format!("Error({:?})", err.value.found)
                    }
                    other => format!("{other:?}"),
                })
                .collect();
            assert!(
                kinds.iter().any(|k| k.contains("referenceUnit")),
                "body elements: {kinds:?}"
            );
        }
        let text = attribute_body_as_text(body).expect("body text");
        assert!(
            text.contains("referenceUnit"),
            "body text missing referenceUnit: {text}"
        );
        let meta = extract_unit_conversion_from_text(&text).expect("meta");
        assert_eq!(meta.reference_unit.as_deref(), Some("m"));
    }

    #[test]
    fn extracts_conversion_by_convention() {
        let text = ":>> unitConversion: ConversionByConvention { :>> referenceUnit = m; :>> conversionFactor = 3.048E-01; }";
        let meta = extract_unit_conversion_from_text(text).expect("meta");
        assert_eq!(meta.kind, "ConversionByConvention");
        assert_eq!(meta.reference_unit.as_deref(), Some("m"));
        assert!((meta.conversion_factor.unwrap() - 0.3048).abs() < 1e-6);
    }

    #[test]
    fn projects_conversion_inside_wrapped_units_package() {
        use url::Url;

        use crate::semantic::graph_builder::build_graph_from_doc;
        use sysml_v2_parser::parse;

        let content = r#"
        package SIPrefixes {
            attribute kilo: UnitPrefix { :>> symbol = "k"; :>> conversionFactor = 1E3; }
        }
        package Units {
            attribute <m> metre : LengthUnit;
            attribute <km> kilometre : LengthUnit { :>> unitConversion: ConversionByPrefix { :>> prefix = kilo; :>> referenceUnit = m; } }
            attribute <ft> 'foot' : LengthUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m; :>> conversionFactor = 3.048E-01; } }
        }
        "#;
        let uri = Url::parse("file:///test/units.sysml").expect("uri");
        let parsed = parse(content).expect("parse");
        let graph = build_graph_from_doc(&parsed, &uri);
        let km = graph
            .nodes_for_uri(&uri)
            .into_iter()
            .find(|n| n.name == "kilometre")
            .expect("kilometre");
        assert!(
            km.attributes.contains_key(UNIT_CONVERSION_KEY),
            "km attrs: {:?}",
            km.attributes
        );
        let ft = graph
            .nodes_for_uri(&uri)
            .into_iter()
            .find(|n| n.name == "foot")
            .expect("foot");
        assert!(
            ft.attributes.contains_key(UNIT_CONVERSION_KEY),
            "ft attrs: {:?}",
            ft.attributes
        );
        let km_conv = km.attributes.get(UNIT_CONVERSION_KEY).expect("km conv");
        assert_eq!(
            km_conv.get("referenceUnit").and_then(|v| v.as_str()),
            Some("m"),
            "km conv json: {km_conv}"
        );
    }

    #[test]
    fn fahrenheit_interval_body_text_includes_zero_offset() {
        use sysml_v2_parser::ast::{PackageBody, PackageBodyElement, RootElement};
        use sysml_v2_parser::parse;

        let content = r#"package Units {
            attribute <'°F_abs'> 'degree fahrenheit absolute' : IntervalScale {
                :>> unit = '°F';
                private attribute zeroDegreeFahrenheitInKelvin: ThermodynamicTemperatureValue = 229835/900 [K];
            }
        }"#;
        let parsed = parse(content).expect("parse");
        let def = parsed
            .elements
            .iter()
            .find_map(|el| match &el.value {
                RootElement::Package(pkg) => match &pkg.value.body {
                    PackageBody::Brace { elements } => {
                        match elements.first().map(|node| &node.value) {
                            Some(PackageBodyElement::AttributeDef(def)) => Some(&def.value),
                            other => panic!("expected attribute def, got {other:?}"),
                        }
                    }
                    _ => None,
                },
                _ => None,
            })
            .expect("def");
        let block_text = attribute_body_as_text(&def.body).expect("body text");
        assert!(
            block_text.contains("zeroDegree"),
            "block_text={block_text:?}"
        );
        let conversion = extract_interval_scale_from_body(&def.body).expect("conversion");
        assert_eq!(
            conversion.zero_offset_kelvin,
            Some(229835.0 / 900.0),
            "block_text={block_text:?}"
        );
    }

    #[test]
    fn parses_fractional_zero_offset_kelvin() {
        let line = "zeroDegreeFahrenheitInKelvin: ThermodynamicTemperatureValue = 229835/900 [K]";
        assert_eq!(parse_zero_point_kelvin(line), Some(229835.0 / 900.0));
    }

    #[test]
    fn projects_fahrenheit_interval_scale_from_catalog_shape() {
        use url::Url;

        use crate::semantic::graph_builder::build_graph_from_doc;
        use sysml_v2_parser::parse;

        let content = r#"
        package Units {
            attribute <K> kelvin : ThermodynamicTemperatureUnit, TemperatureDifferenceUnit;
            attribute <'°F'> 'degree Fahrenheit' : TemperatureDifferenceUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = K; :>> conversionFactor = 5/9; } }
            attribute <'°F_abs'> 'degree fahrenheit absolute' : IntervalScale {
                :>> unit = '°F';
                private attribute zeroDegreeFahrenheitInKelvin: ThermodynamicTemperatureValue = 229835/900 [K];
            }
        }
        "#;
        let uri = Url::parse("file:///test/f-interval.sysml").expect("uri");
        let parsed = parse(content).expect("parse");
        let graph = build_graph_from_doc(&parsed, &uri);
        let f_abs = graph
            .nodes_for_uri(&uri)
            .into_iter()
            .find(|n| n.attributes.get(SHORT_NAME_KEY).and_then(|v| v.as_str()) == Some("°F_abs"))
            .expect("°F_abs");
        let conv = f_abs
            .attributes
            .get(UNIT_CONVERSION_KEY)
            .expect("interval conversion");
        assert_eq!(
            conv.get("zeroOffsetKelvin").and_then(|v| v.as_f64()),
            Some(229835.0 / 900.0),
            "conv={conv}"
        );
    }

    #[test]
    fn projects_interval_scale_from_catalog_shape() {
        use url::Url;

        use crate::semantic::graph_builder::build_graph_from_doc;
        use sysml_v2_parser::parse;

        let content = r#"
        package Units {
            attribute <K> kelvin : ThermodynamicTemperatureUnit, TemperatureDifferenceUnit;
            attribute <'°C'> 'degree celsius' : TemperatureDifferenceUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = K; :>> conversionFactor = 1; } }
            attribute <'°C_abs'> 'degree celsius absolute' : IntervalScale {
                attribute :>> unit = '°C';
                private attribute zeroDegreeCelsiusInKelvin: ThermodynamicTemperatureValue = 273.15 [K];
            }
        }
        "#;
        let uri = Url::parse("file:///test/interval.sysml").expect("uri");
        let parsed = parse(content).expect("parse");
        let graph = build_graph_from_doc(&parsed, &uri);
        let c_abs = graph
            .nodes_for_uri(&uri)
            .into_iter()
            .find(|n| n.attributes.get(SHORT_NAME_KEY).and_then(|v| v.as_str()) == Some("°C_abs"))
            .expect("°C_abs");
        let conv = c_abs
            .attributes
            .get(UNIT_CONVERSION_KEY)
            .expect("interval conversion");
        assert_eq!(
            conv.get("kind").and_then(|v| v.as_str()),
            Some("IntervalScale")
        );
        assert_eq!(
            conv.get("intervalUnit").and_then(|v| v.as_str()),
            Some("°C")
        );
        assert_eq!(
            conv.get("zeroOffsetKelvin").and_then(|v| v.as_f64()),
            Some(273.15)
        );
    }

    #[test]
    fn projects_prefix_and_conversion_from_catalog_shape() {
        use url::Url;

        use crate::semantic::graph_builder::build_graph_from_doc;
        use sysml_v2_parser::parse;

        let content = r#"
        package SIPrefixes {
            attribute kilo: UnitPrefix { :>> symbol = "k"; :>> conversionFactor = 1E3; }
        }
        package SI {
            attribute <m> metre : LengthUnit;
            attribute <km> kilometre : LengthUnit { :>> unitConversion: ConversionByPrefix { :>> prefix = kilo; :>> referenceUnit = m; } }
        }
        "#;
        let uri = Url::parse("file:///test/catalog.sysml").expect("uri");
        let parsed = parse(content).expect("parse");
        let graph = build_graph_from_doc(&parsed, &uri);
        let kilo = graph
            .nodes_for_uri(&uri)
            .into_iter()
            .find(|n| n.name == "kilo")
            .expect("kilo");
        assert!(
            kilo.attributes.contains_key(UNIT_PREFIX_KEY),
            "kilo attrs: {:?}",
            kilo.attributes
        );
        let km = graph
            .nodes_for_uri(&uri)
            .into_iter()
            .find(|n| n.name == "kilometre")
            .expect("kilometre");
        assert!(
            km.attributes.contains_key(UNIT_CONVERSION_KEY),
            "km attrs: {:?}",
            km.attributes
        );
    }
}
