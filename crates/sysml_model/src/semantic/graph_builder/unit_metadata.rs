//! Project unit-catalog metadata from attribute definitions onto graph node attributes.

use std::collections::HashMap;

use serde_json::{json, Value};
use sysml_v2_parser::ast::{
    AttributeBody, AttributeBodyElement, AttributeDef, AttributeUsage, BinaryOperator, Expression,
    Node, UnaryOperator,
};
use crate::semantic::ast_util::{subsetting_target, typing_target};

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
        let rendered = super::expressions::expression_to_debug_string(expr);
        attrs.insert(UNIT_VALUE_EXPR_KEY.to_string(), json!(rendered));
    }
    project_unit_body_metadata(attrs, typing_target(def.typing.as_deref()), &def.body);
}

pub fn project_attribute_usage_unit_metadata(
    attrs: &mut HashMap<String, Value>,
    usage: &AttributeUsage,
) {
    project_unit_body_metadata(attrs, typing_target(usage.typing.as_deref()), &usage.body);
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

/// Finds the direct-child `attribute` usage that redefines `key` (`:>> key = ...;`).
fn find_redefined_usage<'a>(
    elements: &'a [Node<AttributeBodyElement>],
    key: &str,
) -> Option<&'a AttributeUsage> {
    elements.iter().find_map(|element| match &element.value {
        AttributeBodyElement::AttributeUsage(usage)
            if subsetting_target(usage.value.redefines.as_deref()) == Some(key) =>
        {
            Some(&usage.value)
        }
        _ => None,
    })
}

/// Finds the direct-child `attribute` usage whose own name contains `needle`, e.g. the
/// `zeroDegree*InKelvin` member of an `IntervalScale` catalog definition (not a redefinition,
/// just a plain nested attribute usage with a conventionally-named identifier).
/// Finds the direct-child attribute member whose own name contains `needle` and returns its
/// value expression. Matches both `AttributeUsage` and `AttributeDef` shapes: a nested
/// `attribute name : Type = expr;` member can parse as either depending on its modifiers
/// (e.g. a leading `private` routes it through the definition grammar).
fn find_value_by_name_contains<'a>(
    elements: &'a [Node<AttributeBodyElement>],
    needle: &str,
) -> Option<&'a Node<Expression>> {
    elements.iter().find_map(|element| match &element.value {
        AttributeBodyElement::AttributeUsage(usage) if usage.value.name.contains(needle) => {
            usage.value.value.as_ref()
        }
        AttributeBodyElement::AttributeDef(def) if def.value.name.contains(needle) => {
            def.value.value.as_ref()
        }
        _ => None,
    })
}

fn usage_value_number(usage: &AttributeUsage) -> Option<f64> {
    usage.value.as_ref().and_then(|node| expr_as_number(&node.value))
}

fn usage_value_name(usage: &AttributeUsage) -> Option<String> {
    usage.value.as_ref().and_then(|node| expr_as_name(&node.value))
}

/// Evaluates a numeric literal expression (integers, reals, unit-suffixed quantities like
/// `229835/900 [K]`, and `+ - * /` arithmetic over literals) directly from the AST.
fn expr_as_number(expr: &Expression) -> Option<f64> {
    match expr {
        Expression::LiteralInteger(n) => Some(*n as f64),
        Expression::LiteralReal(raw) => raw.parse().ok(),
        Expression::LiteralWithUnit { value, .. } => expr_as_number(&value.value),
        Expression::UnaryOp { op, operand } => {
            let value = expr_as_number(&operand.value)?;
            match op {
                UnaryOperator::Plus => Some(value),
                UnaryOperator::Minus => Some(-value),
                _ => None,
            }
        }
        Expression::BinaryOp { op, left, right } => {
            let left = expr_as_number(&left.value)?;
            let right = expr_as_number(&right.value)?;
            match op {
                BinaryOperator::Add => Some(left + right),
                BinaryOperator::Sub => Some(left - right),
                BinaryOperator::Mul => Some(left * right),
                BinaryOperator::Div if right != 0.0 => Some(left / right),
                BinaryOperator::Pow | BinaryOperator::Exp => Some(left.powf(right)),
                _ => None,
            }
        }
        _ => None,
    }
}

/// Reads an identifier/string-valued expression, e.g. `prefix = kilo;` (a `FeatureRef` naming
/// another catalog entry) or `symbol = "k";` (a `LiteralString`).
fn expr_as_name(expr: &Expression) -> Option<String> {
    match expr {
        Expression::FeatureRef(name) => Some(name.clone()),
        Expression::LiteralString(s) => Some(s.clone()),
        _ => None,
    }
}

fn extract_unit_prefix_from_body(body: &AttributeBody) -> Option<UnitPrefixMeta> {
    let AttributeBody::Brace { elements } = body else {
        return None;
    };
    let conversion_factor = find_redefined_usage(elements, "conversionFactor")
        .and_then(usage_value_number)?;
    let symbol = find_redefined_usage(elements, "symbol").and_then(usage_value_name);
    Some(UnitPrefixMeta {
        symbol,
        conversion_factor,
    })
}

fn extract_interval_scale_from_body(body: &AttributeBody) -> Option<UnitConversionMeta> {
    let AttributeBody::Brace { elements } = body else {
        return None;
    };
    Some(interval_scale_meta(elements))
}

fn interval_scale_meta(elements: &[Node<AttributeBodyElement>]) -> UnitConversionMeta {
    let interval_unit = find_redefined_usage(elements, "unit").and_then(usage_value_name);
    let zero_offset_kelvin = find_value_by_name_contains(elements, "zeroDegree")
        .and_then(|node| expr_as_number(&node.value));
    UnitConversionMeta {
        kind: "IntervalScale".to_string(),
        reference_unit: Some("K".to_string()),
        conversion_factor: Some(1.0),
        prefix: None,
        interval_unit,
        zero_offset_kelvin,
    }
}

/// Reads the `:>> unitConversion: <Kind> { ... }` catalog shape directly from the AST: the
/// outer attribute's body has one redefined `unitConversion` usage whose `typing` names the
/// conversion kind and whose own nested body carries the kind-specific parameters.
fn extract_unit_conversion_from_body(body: &AttributeBody) -> Option<UnitConversionMeta> {
    let AttributeBody::Brace { elements } = body else {
        return None;
    };
    let conversion_usage = find_redefined_usage(elements, "unitConversion")?;
    let AttributeBody::Brace {
        elements: inner_elements,
    } = &conversion_usage.body
    else {
        return None;
    };
    let kind = typing_target(conversion_usage.typing.as_deref()).map(base_type_name);
    match kind {
        Some("IntervalScale") => Some(interval_scale_meta(inner_elements)),
        Some("ConversionByPrefix") => Some(UnitConversionMeta {
            kind: "ConversionByPrefix".to_string(),
            reference_unit: find_redefined_usage(inner_elements, "referenceUnit")
                .and_then(usage_value_name),
            conversion_factor: None,
            prefix: find_redefined_usage(inner_elements, "prefix").and_then(usage_value_name),
            interval_unit: None,
            zero_offset_kelvin: None,
        }),
        _ => {
            let reference_unit = find_redefined_usage(inner_elements, "referenceUnit")
                .and_then(usage_value_name);
            // `ConversionByConvention` is the catalog's default/fallback shape: accept it
            // whether or not the nested body is explicitly typed, as long as it carries a
            // `referenceUnit` redefinition.
            if kind == Some("ConversionByConvention") || reference_unit.is_some() {
                Some(UnitConversionMeta {
                    kind: "ConversionByConvention".to_string(),
                    reference_unit,
                    conversion_factor: find_redefined_usage(inner_elements, "conversionFactor")
                        .and_then(usage_value_number),
                    prefix: None,
                    interval_unit: None,
                    zero_offset_kelvin: None,
                })
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn attribute_def_body(content: &str) -> AttributeBody {
        use sysml_v2_parser::ast::{PackageBody, PackageBodyElement, RootElement};
        use sysml_v2_parser::parse;

        let parsed = parse(content).expect("parse");
        parsed
            .elements
            .iter()
            .find_map(|el| match &el.value {
                RootElement::Package(pkg) => match &pkg.value.body {
                    PackageBody::Brace { elements } => {
                        match elements.first().map(|node| &node.value) {
                            Some(PackageBodyElement::AttributeDef(def)) => {
                                Some(def.value.body.clone())
                            }
                            other => panic!("expected attribute def, got {other:?}"),
                        }
                    }
                    _ => None,
                },
                _ => None,
            })
            .expect("attribute def body")
    }

    #[test]
    fn extracts_conversion_by_convention_from_ast() {
        let body = attribute_def_body(
            "package P { attribute foo : LengthUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m; :>> conversionFactor = 3.048E-01; } } }",
        );
        let meta = extract_unit_conversion_from_body(&body).expect("meta");
        assert_eq!(meta.kind, "ConversionByConvention");
        assert_eq!(meta.reference_unit.as_deref(), Some("m"));
        assert!((meta.conversion_factor.unwrap() - 0.3048).abs() < 1e-6);
    }

    #[test]
    fn extracts_conversion_by_prefix_from_ast() {
        let body = attribute_def_body(
            "package P { attribute <km> kilometre : LengthUnit { :>> unitConversion: ConversionByPrefix { :>> prefix = kilo; :>> referenceUnit = m; } } }",
        );
        let meta = extract_unit_conversion_from_body(&body).expect("meta");
        assert_eq!(meta.kind, "ConversionByPrefix");
        assert_eq!(meta.reference_unit.as_deref(), Some("m"));
        assert_eq!(meta.prefix.as_deref(), Some("kilo"));
    }

    #[test]
    fn evaluates_division_addition_and_negation() {
        use sysml_v2_parser::Span;

        let lit = |n: i64| Box::new(Node::new(Span::dummy(), Expression::LiteralInteger(n)));
        assert_eq!(
            expr_as_number(&Expression::BinaryOp {
                op: BinaryOperator::Div,
                left: lit(229835),
                right: lit(900),
            }),
            Some(229835.0 / 900.0)
        );
        assert_eq!(
            expr_as_number(&Expression::UnaryOp {
                op: UnaryOperator::Minus,
                operand: lit(5),
            }),
            Some(-5.0)
        );
        assert_eq!(
            expr_as_number(&Expression::BinaryOp {
                op: BinaryOperator::Add,
                left: lit(2),
                right: lit(3),
            }),
            Some(5.0)
        );
    }

    #[test]
    fn fahrenheit_interval_body_zero_offset_from_ast() {
        let body = attribute_def_body(
            r#"package Units {
                attribute <'°F_abs'> 'degree fahrenheit absolute' : IntervalScale {
                    :>> unit = '°F';
                    private attribute zeroDegreeFahrenheitInKelvin: ThermodynamicTemperatureValue = 229835/900 [K];
                }
            }"#,
        );
        let conversion = extract_interval_scale_from_body(&body).expect("conversion");
        assert_eq!(conversion.interval_unit.as_deref(), Some("°F"));
        assert_eq!(conversion.zero_offset_kelvin, Some(229835.0 / 900.0));
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
