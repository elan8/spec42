
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use super::*;
use crate::semantic::text_span::{TextPosition, TextRange};
use url::Url;

use crate::semantic::model::{ElementKind, SemanticNode};

fn range() -> TextRange {
    TextRange::new(TextPosition::new(0, 0), TextPosition::new(0, 1))
}

fn add_node(
    graph: &mut SemanticGraph,
    uri: &Url,
    qualified_name: &str,
    element_kind: &str,
    name: &str,
    parent_id: Option<&NodeId>,
    attributes: HashMap<String, Value>,
) -> NodeId {
    let id = NodeId::new(uri, qualified_name);
    let node = SemanticNode {
        id: id.clone(),
        element_kind: ElementKind::parse(element_kind),
        name: name.to_string(),
        range: range(),
        attributes,
        declared_facts: Default::default(),
        parent_id: parent_id.cloned(),
    };
    let idx = graph.graph.add_node(node);
    graph.node_index_by_id.insert(id.clone(), idx);
    graph
        .nodes_by_uri
        .entry(uri.clone())
        .or_default()
        .push(id.clone());
    graph
        .node_ids_by_qualified_name
        .entry(qualified_name.to_string())
        .or_default()
        .push(id.clone());
    if let Some(pid) = parent_id {
        graph
            .children_by_parent_id
            .entry(pid.clone())
            .or_default()
            .push(id.clone());
    }
    id
}

fn node_attr<'a>(graph: &'a SemanticGraph, id: &NodeId, key: &str) -> Option<&'a Value> {
    graph.get_node(id).and_then(|node| node.attributes.get(key))
}

fn register_units_fixture(graph: &mut SemanticGraph) {
    use crate::semantic::graph_builder::build_graph_from_doc;

    const UNITS_FIXTURE_SYSML: &str = r#"
            package Units {
                attribute <m> 'metre' : LengthUnit;
                attribute <s> second : TimeUnit;
                attribute <cm> 'centimetre' : LengthUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m; :>> conversionFactor = 1E-02; } }
                attribute <ft> 'foot' : LengthUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m; :>> conversionFactor = 3.048E-01; } }
                attribute <kg> 'kilogram' : MassUnit;
                attribute <K> kelvin : ThermodynamicTemperatureUnit, TemperatureDifferenceUnit;
                attribute <'°C'> 'degree celsius (temperature difference)' : TemperatureDifferenceUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = K; :>> conversionFactor = 1; } }
                attribute <'°F'> 'degree Fahrenheit (temperature difference)' : TemperatureDifferenceUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = K; :>> conversionFactor = 5/9; } }
                attribute <'°C_abs'> 'degree celsius (absolute temperature scale)' : IntervalScale {
                    attribute :>> unit = '°C';
                    private attribute zeroDegreeCelsiusInKelvin: ThermodynamicTemperatureValue = 273.15 [K];
                }
                attribute <'°F_abs'> 'degree fahrenheit (absolute temperature scale)' : IntervalScale {
                    :>> unit = '°F';
                    private attribute zeroDegreeFahrenheitInKelvin: ThermodynamicTemperatureValue = 229835/900 [K];
                }
            }
        "#;

    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let root: PathBuf = std::env::temp_dir().join(format!("spec42-units-{unique}"));
    let path = root
        .join("sysml.library")
        .join("Domain Libraries")
        .join("Quantities and Units");
    fs::create_dir_all(&path).expect("create fixture path");
    let file = path.join("FixtureUnits.sysml");
    fs::write(&file, UNITS_FIXTURE_SYSML).expect("write fixture");
    let uri = Url::from_file_path(&file).expect("fixture uri");
    let parsed = sysml_v2_parser::parse(UNITS_FIXTURE_SYSML).expect("parse units fixture");
    let doc_graph = build_graph_from_doc(&parsed, &uri);
    graph.merge(doc_graph);
}

#[test]
fn evaluates_reference_chain() {
    let mut graph = SemanticGraph::new();
    let uri = Url::parse("file:///C:/workspace/ref.sysml").expect("uri");
    let owner = add_node(
        &mut graph,
        &uri,
        "Demo::Rocket",
        "part def",
        "Rocket",
        None,
        HashMap::new(),
    );
    let _a = add_node(
        &mut graph,
        &uri,
        "Demo::Rocket::a",
        "attribute",
        "a",
        Some(&owner),
        HashMap::from([("value".to_string(), Value::String("2".to_string()))]),
    );
    let b = add_node(
        &mut graph,
        &uri,
        "Demo::Rocket::b",
        "attribute",
        "b",
        Some(&owner),
        HashMap::from([("value".to_string(), Value::String("a + 3".to_string()))]),
    );
    evaluate_expressions(&mut graph);
    assert_eq!(
        node_attr(&graph, &b, EVALUATED_VALUE_KEY),
        Some(&Value::Number(serde_json::Number::from(5)))
    );
}

#[test]
fn evaluates_unit_conversion_addition() {
    let mut graph = SemanticGraph::new();
    register_units_fixture(&mut graph);
    let uri = Url::parse("file:///C:/workspace/unit-add.sysml").expect("uri");
    let node = add_node(
        &mut graph,
        &uri,
        "Demo::value",
        "attribute",
        "value",
        None,
        HashMap::from([(
            "value".to_string(),
            Value::String("1 [m] + 50 [cm]".to_string()),
        )]),
    );
    evaluate_expressions(&mut graph);
    assert_eq!(
        node_attr(&graph, &node, EVALUATION_STATUS_KEY),
        Some(&Value::String(STATUS_OK.to_string()))
    );
    assert_eq!(
        node_attr(&graph, &node, EVALUATED_UNIT_KEY),
        Some(&Value::String("m".to_string()))
    );
    assert_eq!(
        node_attr(&graph, &node, EVALUATED_VALUE_KEY),
        Some(&Value::Number(
            serde_json::Number::from_f64(1.5).expect("num")
        ))
    );
}

#[test]
fn evaluates_double_bracket_unit_syntax() {
    let mut graph = SemanticGraph::new();
    register_units_fixture(&mut graph);
    let uri = Url::parse("file:///C:/workspace/unit-double-bracket.sysml").expect("uri");
    let node = add_node(
        &mut graph,
        &uri,
        "Demo::value",
        "attribute",
        "value",
        None,
        HashMap::from([(
            "value".to_string(),
            Value::String("1 [[m]] + 50 [[cm]]".to_string()),
        )]),
    );
    evaluate_expressions(&mut graph);
    assert_eq!(
        node_attr(&graph, &node, EVALUATION_STATUS_KEY),
        Some(&Value::String(STATUS_OK.to_string()))
    );
    assert_eq!(
        node_attr(&graph, &node, EVALUATED_UNIT_KEY),
        Some(&Value::String("m".to_string()))
    );
}

#[test]
fn supports_si_to_imperial_when_registry_has_pair() {
    let mut graph = SemanticGraph::new();
    register_units_fixture(&mut graph);
    let uri = Url::parse("file:///C:/workspace/unit-imperial.sysml").expect("uri");
    let node = add_node(
        &mut graph,
        &uri,
        "Demo::value",
        "attribute",
        "value",
        None,
        HashMap::from([(
            "value".to_string(),
            Value::String("1 [m] + 1 [ft]".to_string()),
        )]),
    );
    evaluate_expressions(&mut graph);
    assert_eq!(
        node_attr(&graph, &node, EVALUATION_STATUS_KEY),
        Some(&Value::String(STATUS_OK.to_string()))
    );
}

#[test]
fn rejects_incompatible_unit_addition() {
    let mut graph = SemanticGraph::new();
    register_units_fixture(&mut graph);
    let uri = Url::parse("file:///C:/workspace/unit-bad.sysml").expect("uri");
    let node = add_node(
        &mut graph,
        &uri,
        "Demo::value",
        "attribute",
        "value",
        None,
        HashMap::from([(
            "value".to_string(),
            Value::String("1 [m] + 2 [kg]".to_string()),
        )]),
    );
    evaluate_expressions(&mut graph);
    assert_eq!(
        node_attr(&graph, &node, EVALUATION_STATUS_KEY),
        Some(&Value::String(STATUS_TYPE_ERROR.to_string()))
    );
}

#[test]
fn evaluates_affine_absolute_temperature_addition() {
    let mut graph = SemanticGraph::new();
    register_units_fixture(&mut graph);
    let uri = Url::parse("file:///C:/workspace/unit-affine.sysml").expect("uri");
    let node = add_node(
        &mut graph,
        &uri,
        "Demo::value",
        "attribute",
        "value",
        None,
        HashMap::from([(
            "value".to_string(),
            Value::String("0 [°C_abs] + 32 [°F_abs]".to_string()),
        )]),
    );
    evaluate_expressions(&mut graph);
    assert_eq!(
        node_attr(&graph, &node, EVALUATION_STATUS_KEY),
        Some(&Value::String(STATUS_OK.to_string()))
    );
    assert_eq!(
        node_attr(&graph, &node, EVALUATED_UNIT_KEY),
        Some(&Value::String("°C_abs".to_string()))
    );
    let value = node_attr(&graph, &node, EVALUATED_VALUE_KEY)
        .and_then(Value::as_f64)
        .expect("numeric");
    assert!((value - 0.0).abs() < 1e-9);
}

#[test]
fn canonicalizes_multiply_divide_units_and_values() {
    let mut graph = SemanticGraph::new();
    register_units_fixture(&mut graph);
    let uri = Url::parse("file:///C:/workspace/unit-canonical.sysml").expect("uri");
    let area = add_node(
        &mut graph,
        &uri,
        "Demo::area",
        "attribute",
        "area",
        None,
        HashMap::from([(
            "value".to_string(),
            Value::String("2 [cm] * 3 [m]".to_string()),
        )]),
    );
    let speed = add_node(
        &mut graph,
        &uri,
        "Demo::speed",
        "attribute",
        "speed",
        None,
        HashMap::from([(
            "value".to_string(),
            Value::String("10 [m] / 2 [s]".to_string()),
        )]),
    );
    evaluate_expressions(&mut graph);

    assert_eq!(
        node_attr(&graph, &area, EVALUATED_UNIT_KEY),
        Some(&Value::String("m^2".to_string()))
    );
    let area_value = node_attr(&graph, &area, EVALUATED_VALUE_KEY)
        .and_then(Value::as_f64)
        .expect("area value");
    assert!((area_value - 0.06).abs() < 1e-9);

    assert_eq!(
        node_attr(&graph, &speed, EVALUATED_UNIT_KEY),
        Some(&Value::String("m/s".to_string()))
    );
    let speed_value = node_attr(&graph, &speed, EVALUATED_VALUE_KEY)
        .and_then(Value::as_f64)
        .expect("speed value");
    assert!((speed_value - 5.0).abs() < 1e-9);
}

#[test]
fn rejects_affine_units_in_multiplication() {
    let mut graph = SemanticGraph::new();
    register_units_fixture(&mut graph);
    let uri = Url::parse("file:///C:/workspace/unit-affine-mul.sysml").expect("uri");
    let node = add_node(
        &mut graph,
        &uri,
        "Demo::value",
        "attribute",
        "value",
        None,
        HashMap::from([(
            "value".to_string(),
            Value::String("1 [°C_abs] * 2 [m]".to_string()),
        )]),
    );
    evaluate_expressions(&mut graph);
    assert_eq!(
        node_attr(&graph, &node, EVALUATION_STATUS_KEY),
        Some(&Value::String(STATUS_UNSUPPORTED.to_string()))
    );
}

#[test]
fn evaluates_inline_analysis_constraint_from_owner_attributes() {
    let mut graph = SemanticGraph::new();
    let uri = Url::parse("file:///C:/workspace/analysis-inline.sysml").expect("uri");
    let requirement = add_node(
        &mut graph,
        &uri,
        "Demo::Req",
        "requirement def",
        "Req",
        None,
        HashMap::from([(
            ANALYSIS_CONSTRAINTS_KEY.to_string(),
            serde_json::json!([{
                "kind": "require_constraint",
                "expression": "measured <= limit",
                "params": [],
            }]),
        )]),
    );
    let _measured = add_node(
        &mut graph,
        &uri,
        "Demo::Req::measured",
        "attribute",
        "measured",
        Some(&requirement),
        HashMap::from([("value".to_string(), Value::String("4".to_string()))]),
    );
    let _limit = add_node(
        &mut graph,
        &uri,
        "Demo::Req::limit",
        "attribute",
        "limit",
        Some(&requirement),
        HashMap::from([("value".to_string(), Value::String("5".to_string()))]),
    );
    evaluate_expressions(&mut graph);
    assert_eq!(
        node_attr(&graph, &requirement, ANALYSIS_EVAL_STATUS_KEY),
        Some(&Value::String(STATUS_OK.to_string()))
    );
    assert_eq!(
        node_attr(&graph, &requirement, ANALYSIS_CONSTRAINT_PASSED_KEY),
        Some(&Value::Bool(true))
    );
}

#[test]
fn marks_analysis_constraint_as_failed_when_comparison_fails() {
    let mut graph = SemanticGraph::new();
    let uri = Url::parse("file:///C:/workspace/analysis-inline-fail.sysml").expect("uri");
    let requirement = add_node(
        &mut graph,
        &uri,
        "Demo::Req",
        "requirement def",
        "Req",
        None,
        HashMap::from([(
            ANALYSIS_CONSTRAINTS_KEY.to_string(),
            serde_json::json!([{
                "kind": "require_constraint",
                "expression": "measured <= limit",
                "params": [],
            }]),
        )]),
    );
    let _measured = add_node(
        &mut graph,
        &uri,
        "Demo::Req::measured",
        "attribute",
        "measured",
        Some(&requirement),
        HashMap::from([("value".to_string(), Value::String("8".to_string()))]),
    );
    let _limit = add_node(
        &mut graph,
        &uri,
        "Demo::Req::limit",
        "attribute",
        "limit",
        Some(&requirement),
        HashMap::from([("value".to_string(), Value::String("5".to_string()))]),
    );
    evaluate_expressions(&mut graph);
    assert_eq!(
        node_attr(&graph, &requirement, ANALYSIS_EVAL_STATUS_KEY),
        Some(&Value::String("failed_constraint".to_string()))
    );
    assert_eq!(
        node_attr(&graph, &requirement, ANALYSIS_CONSTRAINT_PASSED_KEY),
        Some(&Value::Bool(false))
    );
}

#[test]
fn evaluates_boolean_precedence_and_parentheses_in_analysis_expression() {
    let mut graph = SemanticGraph::new();
    let uri = Url::parse("file:///C:/workspace/analysis-precedence.sysml").expect("uri");
    let requirement = add_node(
        &mut graph,
        &uri,
        "Demo::Req",
        "requirement def",
        "Req",
        None,
        HashMap::from([(
            ANALYSIS_EXPRESSION_KEY.to_string(),
            Value::String("not measured > limit and limit == 5 or false".to_string()),
        )]),
    );
    let _measured = add_node(
        &mut graph,
        &uri,
        "Demo::Req::measured",
        "attribute",
        "measured",
        Some(&requirement),
        HashMap::from([("value".to_string(), Value::String("4".to_string()))]),
    );
    let _limit = add_node(
        &mut graph,
        &uri,
        "Demo::Req::limit",
        "attribute",
        "limit",
        Some(&requirement),
        HashMap::from([("value".to_string(), Value::String("5".to_string()))]),
    );
    evaluate_expressions(&mut graph);
    assert_eq!(
        node_attr(&graph, &requirement, ANALYSIS_EVAL_STATUS_KEY),
        Some(&Value::String(STATUS_OK.to_string()))
    );
    assert_eq!(
        node_attr(&graph, &requirement, ANALYSIS_EVAL_VALUE_KEY),
        Some(&Value::Bool(true))
    );
}

#[test]
fn supports_arithmetic_and_unit_conversion_in_analysis_comparison() {
    let mut graph = SemanticGraph::new();
    register_units_fixture(&mut graph);
    let uri = Url::parse("file:///C:/workspace/analysis-units.sysml").expect("uri");
    let requirement = add_node(
        &mut graph,
        &uri,
        "Demo::Req",
        "requirement def",
        "Req",
        None,
        HashMap::from([(
            ANALYSIS_EXPRESSION_KEY.to_string(),
            Value::String("measured + margin <= limit".to_string()),
        )]),
    );
    let _measured = add_node(
        &mut graph,
        &uri,
        "Demo::Req::measured",
        "attribute",
        "measured",
        Some(&requirement),
        HashMap::from([("value".to_string(), Value::String("90 [cm]".to_string()))]),
    );
    let _margin = add_node(
        &mut graph,
        &uri,
        "Demo::Req::margin",
        "attribute",
        "margin",
        Some(&requirement),
        HashMap::from([("value".to_string(), Value::String("0.2 [m]".to_string()))]),
    );
    let _limit = add_node(
        &mut graph,
        &uri,
        "Demo::Req::limit",
        "attribute",
        "limit",
        Some(&requirement),
        HashMap::from([("value".to_string(), Value::String("1.2 [m]".to_string()))]),
    );
    evaluate_expressions(&mut graph);
    assert_eq!(
        node_attr(&graph, &requirement, ANALYSIS_EVAL_STATUS_KEY),
        Some(&Value::String(STATUS_OK.to_string()))
    );
    assert_eq!(
        node_attr(&graph, &requirement, ANALYSIS_EVAL_VALUE_KEY),
        Some(&Value::Bool(true))
    );
}

#[test]
fn supports_multi_term_arithmetic_operands_in_analysis_comparison() {
    let mut graph = SemanticGraph::new();
    let uri = Url::parse("file:///C:/workspace/analysis-parenthesized.sysml").expect("uri");
    let requirement = add_node(
        &mut graph,
        &uri,
        "Demo::Req",
        "requirement def",
        "Req",
        None,
        HashMap::from([(
            ANALYSIS_EXPRESSION_KEY.to_string(),
            Value::String("allowed - estimated - uncertainty >= 0".to_string()),
        )]),
    );
    let _allowed = add_node(
        &mut graph,
        &uri,
        "Demo::Req::allowed",
        "attribute",
        "allowed",
        Some(&requirement),
        HashMap::from([("value".to_string(), Value::String("2.0".to_string()))]),
    );
    let _estimated = add_node(
        &mut graph,
        &uri,
        "Demo::Req::estimated",
        "attribute",
        "estimated",
        Some(&requirement),
        HashMap::from([("value".to_string(), Value::String("1.7".to_string()))]),
    );
    let _uncertainty = add_node(
        &mut graph,
        &uri,
        "Demo::Req::uncertainty",
        "attribute",
        "uncertainty",
        Some(&requirement),
        HashMap::from([("value".to_string(), Value::String("0.1".to_string()))]),
    );
    evaluate_expressions(&mut graph);
    assert_eq!(
        node_attr(&graph, &requirement, ANALYSIS_EVAL_STATUS_KEY),
        Some(&Value::String(STATUS_OK.to_string()))
    );
    assert_eq!(
        node_attr(&graph, &requirement, ANALYSIS_EVAL_VALUE_KEY),
        Some(&Value::Bool(true))
    );
}

#[test]
fn supports_truncated_comparison_rhs_by_assuming_zero() {
    let mut graph = SemanticGraph::new();
    let uri = Url::parse("file:///C:/workspace/analysis-truncated-rhs.sysml").expect("uri");
    let requirement = add_node(
        &mut graph,
        &uri,
        "Demo::Req",
        "requirement def",
        "Req",
        None,
        HashMap::from([(
            ANALYSIS_EXPRESSION_KEY.to_string(),
            Value::String("allowed - estimated - uncertainty >=".to_string()),
        )]),
    );
    let _allowed = add_node(
        &mut graph,
        &uri,
        "Demo::Req::allowed",
        "attribute",
        "allowed",
        Some(&requirement),
        HashMap::from([("value".to_string(), Value::String("2.0".to_string()))]),
    );
    let _estimated = add_node(
        &mut graph,
        &uri,
        "Demo::Req::estimated",
        "attribute",
        "estimated",
        Some(&requirement),
        HashMap::from([("value".to_string(), Value::String("1.7".to_string()))]),
    );
    let _uncertainty = add_node(
        &mut graph,
        &uri,
        "Demo::Req::uncertainty",
        "attribute",
        "uncertainty",
        Some(&requirement),
        HashMap::from([("value".to_string(), Value::String("0.1".to_string()))]),
    );
    evaluate_expressions(&mut graph);
    assert_eq!(
        node_attr(&graph, &requirement, ANALYSIS_EVAL_STATUS_KEY),
        Some(&Value::String(STATUS_OK.to_string()))
    );
    assert_eq!(
        node_attr(&graph, &requirement, ANALYSIS_EVAL_VALUE_KEY),
        Some(&Value::Bool(true))
    );
}

#[test]
fn reports_type_mismatch_for_incompatible_analysis_units() {
    let mut graph = SemanticGraph::new();
    register_units_fixture(&mut graph);
    let uri = Url::parse("file:///C:/workspace/analysis-unit-type-error.sysml").expect("uri");
    let requirement = add_node(
        &mut graph,
        &uri,
        "Demo::Req",
        "requirement def",
        "Req",
        None,
        HashMap::from([(
            ANALYSIS_EXPRESSION_KEY.to_string(),
            Value::String("measured < limit".to_string()),
        )]),
    );
    let _measured = add_node(
        &mut graph,
        &uri,
        "Demo::Req::measured",
        "attribute",
        "measured",
        Some(&requirement),
        HashMap::from([("value".to_string(), Value::String("1 [m]".to_string()))]),
    );
    let _limit = add_node(
        &mut graph,
        &uri,
        "Demo::Req::limit",
        "attribute",
        "limit",
        Some(&requirement),
        HashMap::from([("value".to_string(), Value::String("2 [kg]".to_string()))]),
    );
    evaluate_expressions(&mut graph);
    assert_eq!(
        node_attr(&graph, &requirement, ANALYSIS_EVAL_STATUS_KEY),
        Some(&Value::String(STATUS_TYPE_ERROR.to_string()))
    );
    let message = node_attr(&graph, &requirement, ANALYSIS_EVAL_ERROR_KEY)
        .and_then(Value::as_str)
        .unwrap_or_default();
    assert!(
        message.contains("type or unit mismatch")
            || message.contains("incompatible units")
            || message.contains("dimensioned and unitless")
    );
}

#[test]
fn reports_unresolved_reference_for_analysis_expression() {
    let mut graph = SemanticGraph::new();
    let uri = Url::parse("file:///C:/workspace/analysis-unresolved.sysml").expect("uri");
    let requirement = add_node(
        &mut graph,
        &uri,
        "Demo::Req",
        "requirement def",
        "Req",
        None,
        HashMap::from([(
            ANALYSIS_EXPRESSION_KEY.to_string(),
            Value::String("measured <= missingLimit".to_string()),
        )]),
    );
    let _measured = add_node(
        &mut graph,
        &uri,
        "Demo::Req::measured",
        "attribute",
        "measured",
        Some(&requirement),
        HashMap::from([("value".to_string(), Value::String("4".to_string()))]),
    );
    evaluate_expressions(&mut graph);
    assert_eq!(
        node_attr(&graph, &requirement, ANALYSIS_EVAL_STATUS_KEY),
        Some(&Value::String(STATUS_UNKNOWN.to_string()))
    );
    let message = node_attr(&graph, &requirement, ANALYSIS_EVAL_ERROR_KEY)
        .and_then(Value::as_str)
        .unwrap_or_default();
    assert!(message.contains("could not be resolved"));
}

#[test]
fn evaluates_calc_invocation_in_analysis_comparison() {
    let mut graph = SemanticGraph::new();
    let uri = Url::parse("file:///C:/workspace/analysis-calc-call.sysml").expect("uri");
    let _calc = add_node(
        &mut graph,
        &uri,
        "Demo::Margin",
        "calc def",
        "Margin",
        None,
        HashMap::from([
            (
                "parameters".to_string(),
                serde_json::json!([
                    {"direction":"in","name":"limit","type":"Real"},
                    {"direction":"in","name":"measured","type":"Real"},
                    {"direction":"in","name":"allowance","type":"Real"}
                ]),
            ),
            (
                ANALYSIS_EXPRESSION_KEY.to_string(),
                Value::String("limit - measured - allowance".to_string()),
            ),
        ]),
    );
    let requirement = add_node(
        &mut graph,
        &uri,
        "Demo::Req",
        "requirement def",
        "Req",
        None,
        HashMap::from([(
            ANALYSIS_EXPRESSION_KEY.to_string(),
            Value::String("Margin(limit, measured, allowance) >= 0".to_string()),
        )]),
    );
    let _limit = add_node(
        &mut graph,
        &uri,
        "Demo::Req::limit",
        "attribute",
        "limit",
        Some(&requirement),
        HashMap::from([("value".to_string(), Value::String("2.0".to_string()))]),
    );
    let _measured = add_node(
        &mut graph,
        &uri,
        "Demo::Req::measured",
        "attribute",
        "measured",
        Some(&requirement),
        HashMap::from([("value".to_string(), Value::String("1.7".to_string()))]),
    );
    let _allowance = add_node(
        &mut graph,
        &uri,
        "Demo::Req::allowance",
        "attribute",
        "allowance",
        Some(&requirement),
        HashMap::from([("value".to_string(), Value::String("0.1".to_string()))]),
    );
    evaluate_expressions(&mut graph);
    assert_eq!(
        node_attr(&graph, &requirement, ANALYSIS_EVAL_VALUE_KEY),
        Some(&Value::Bool(true))
    );
}

#[test]
fn evaluates_calc_invocation_with_named_arguments() {
    let mut graph = SemanticGraph::new();
    let uri = Url::parse("file:///C:/workspace/analysis-calc-call-named.sysml").expect("uri");
    let _calc = add_node(
        &mut graph,
        &uri,
        "Demo::Margin",
        "calc def",
        "Margin",
        None,
        HashMap::from([
            (
                "parameters".to_string(),
                serde_json::json!([
                    {"direction":"in","name":"limit","type":"Real"},
                    {"direction":"in","name":"measured","type":"Real"},
                    {"direction":"in","name":"allowance","type":"Real"}
                ]),
            ),
            (
                ANALYSIS_EXPRESSION_KEY.to_string(),
                Value::String("limit - measured - allowance".to_string()),
            ),
        ]),
    );
    let requirement = add_node(
        &mut graph,
        &uri,
        "Demo::Req",
        "requirement def",
        "Req",
        None,
        HashMap::from([(
            ANALYSIS_EXPRESSION_KEY.to_string(),
            Value::String(
                "Margin(measured=measured, allowance=allowance, limit=limit) >= 0".to_string(),
            ),
        )]),
    );
    let _limit = add_node(
        &mut graph,
        &uri,
        "Demo::Req::limit",
        "attribute",
        "limit",
        Some(&requirement),
        HashMap::from([("value".to_string(), Value::String("2.0".to_string()))]),
    );
    let _measured = add_node(
        &mut graph,
        &uri,
        "Demo::Req::measured",
        "attribute",
        "measured",
        Some(&requirement),
        HashMap::from([("value".to_string(), Value::String("1.7".to_string()))]),
    );
    let _allowance = add_node(
        &mut graph,
        &uri,
        "Demo::Req::allowance",
        "attribute",
        "allowance",
        Some(&requirement),
        HashMap::from([("value".to_string(), Value::String("0.1".to_string()))]),
    );
    evaluate_expressions(&mut graph);
    assert_eq!(
        node_attr(&graph, &requirement, ANALYSIS_EVAL_VALUE_KEY),
        Some(&Value::Bool(true))
    );
}

#[test]
fn rejects_mixed_positional_and_named_invocation_arguments() {
    let mut graph = SemanticGraph::new();
    let uri = Url::parse("file:///C:/workspace/analysis-calc-call-mixed.sysml").expect("uri");
    let _calc = add_node(
        &mut graph,
        &uri,
        "Demo::Margin",
        "calc def",
        "Margin",
        None,
        HashMap::from([
            (
                "parameters".to_string(),
                serde_json::json!([
                    {"direction":"in","name":"limit","type":"Real"},
                    {"direction":"in","name":"measured","type":"Real"},
                    {"direction":"in","name":"allowance","type":"Real"}
                ]),
            ),
            (
                ANALYSIS_EXPRESSION_KEY.to_string(),
                Value::String("limit - measured - allowance".to_string()),
            ),
        ]),
    );
    let requirement = add_node(
        &mut graph,
        &uri,
        "Demo::Req",
        "requirement def",
        "Req",
        None,
        HashMap::from([(
            ANALYSIS_EXPRESSION_KEY.to_string(),
            Value::String("Margin(limit, measured=measured, allowance=allowance) >= 0".to_string()),
        )]),
    );
    let _limit = add_node(
        &mut graph,
        &uri,
        "Demo::Req::limit",
        "attribute",
        "limit",
        Some(&requirement),
        HashMap::from([("value".to_string(), Value::String("2.0".to_string()))]),
    );
    let _measured = add_node(
        &mut graph,
        &uri,
        "Demo::Req::measured",
        "attribute",
        "measured",
        Some(&requirement),
        HashMap::from([("value".to_string(), Value::String("1.7".to_string()))]),
    );
    let _allowance = add_node(
        &mut graph,
        &uri,
        "Demo::Req::allowance",
        "attribute",
        "allowance",
        Some(&requirement),
        HashMap::from([("value".to_string(), Value::String("0.1".to_string()))]),
    );
    evaluate_expressions(&mut graph);
    assert_eq!(
        node_attr(&graph, &requirement, ANALYSIS_EVAL_STATUS_KEY),
        Some(&Value::String(STATUS_UNSUPPORTED.to_string()))
    );
}

#[test]
fn evaluates_constraint_invocation_as_boolean_predicate() {
    let mut graph = SemanticGraph::new();
    let uri = Url::parse("file:///C:/workspace/analysis-constraint-call.sysml").expect("uri");
    let _constraint = add_node(
        &mut graph,
        &uri,
        "Demo::WithinLimit",
        "constraint def",
        "WithinLimit",
        None,
        HashMap::from([
            (
                "parameters".to_string(),
                serde_json::json!([
                    {"direction":"in","name":"measured","type":"Real"},
                    {"direction":"in","name":"limit","type":"Real"}
                ]),
            ),
            (
                ANALYSIS_EXPRESSION_KEY.to_string(),
                Value::String("measured <= limit".to_string()),
            ),
        ]),
    );
    let requirement = add_node(
        &mut graph,
        &uri,
        "Demo::Req",
        "requirement def",
        "Req",
        None,
        HashMap::from([(
            ANALYSIS_EXPRESSION_KEY.to_string(),
            Value::String("WithinLimit(measured, limit)".to_string()),
        )]),
    );
    let _measured = add_node(
        &mut graph,
        &uri,
        "Demo::Req::measured",
        "attribute",
        "measured",
        Some(&requirement),
        HashMap::from([("value".to_string(), Value::String("4".to_string()))]),
    );
    let _limit = add_node(
        &mut graph,
        &uri,
        "Demo::Req::limit",
        "attribute",
        "limit",
        Some(&requirement),
        HashMap::from([("value".to_string(), Value::String("5".to_string()))]),
    );
    evaluate_expressions(&mut graph);
    assert_eq!(
        node_attr(&graph, &requirement, ANALYSIS_EVAL_VALUE_KEY),
        Some(&Value::Bool(true))
    );
}

#[test]
fn evaluates_analysis_subject_member_path_with_local_budget() {
    let mut graph = SemanticGraph::new();
    let uri = Url::parse("file:///C:/workspace/analysis-subject-rollup.sysml").expect("uri");
    let analysis = add_node(
        &mut graph,
        &uri,
        "Demo::PowerAnalysis",
        "analysis def",
        "PowerAnalysis",
        None,
        HashMap::from([(
            ANALYSIS_EXPRESSION_KEY.to_string(),
            Value::String("sum(robot.mobility.drivePowerW) <= powerBudgetW".to_string()),
        )]),
    );
    let robot = add_node(
        &mut graph,
        &uri,
        "Demo::PowerAnalysis::robot",
        "subject",
        "robot",
        Some(&analysis),
        HashMap::new(),
    );
    let mobility = add_node(
        &mut graph,
        &uri,
        "Demo::PowerAnalysis::robot::mobility",
        "part",
        "mobility",
        Some(&robot),
        HashMap::new(),
    );
    let _drive = add_node(
        &mut graph,
        &uri,
        "Demo::PowerAnalysis::robot::mobility::drivePowerW",
        "attribute",
        "drivePowerW",
        Some(&mobility),
        HashMap::from([("value".to_string(), Value::String("28".to_string()))]),
    );
    let _budget = add_node(
        &mut graph,
        &uri,
        "Demo::PowerAnalysis::powerBudgetW",
        "attribute",
        "powerBudgetW",
        Some(&analysis),
        HashMap::from([("value".to_string(), Value::String("55".to_string()))]),
    );

    evaluate_expressions(&mut graph);

    assert_eq!(
        node_attr(&graph, &analysis, ANALYSIS_EVAL_STATUS_KEY),
        Some(&Value::String(STATUS_OK.to_string()))
    );
    assert_eq!(
        node_attr(&graph, &analysis, ANALYSIS_EVAL_VALUE_KEY),
        Some(&Value::Bool(true))
    );
    assert_eq!(
        node_attr(&graph, &analysis, ANALYSIS_COMPUTED_VALUE_KEY),
        Some(&serde_json::json!(28))
    );
}

#[test]
fn resolves_analysis_subject_member_path_via_typing_without_expansion() {
    use crate::semantic::relationships::add_typing_edge_if_exists;

    let mut graph = SemanticGraph::new();
    let uri = Url::parse("file:///C:/workspace/analysis-subject-typing.sysml").expect("uri");
    let robot_def = add_node(
        &mut graph,
        &uri,
        "Demo::Robot",
        "part def",
        "Robot",
        None,
        HashMap::new(),
    );
    let mobility_part = add_node(
        &mut graph,
        &uri,
        "Demo::Robot::mobility",
        "part",
        "mobility",
        Some(&robot_def),
        HashMap::new(),
    );
    let mobility_def = add_node(
        &mut graph,
        &uri,
        "Demo::MobilitySubsystem",
        "part def",
        "MobilitySubsystem",
        None,
        HashMap::new(),
    );
    let _drive = add_node(
        &mut graph,
        &uri,
        "Demo::MobilitySubsystem::drivePowerW",
        "attribute",
        "drivePowerW",
        Some(&mobility_def),
        HashMap::from([("value".to_string(), Value::String("28".to_string()))]),
    );
    let analysis = add_node(
        &mut graph,
        &uri,
        "Demo::PowerAnalysis",
        "analysis def",
        "PowerAnalysis",
        None,
        HashMap::from([(
            ANALYSIS_EXPRESSION_KEY.to_string(),
            Value::String("sum(robot.mobility.drivePowerW) <= powerBudgetW".to_string()),
        )]),
    );
    let _budget = add_node(
        &mut graph,
        &uri,
        "Demo::PowerAnalysis::powerBudgetW",
        "attribute",
        "powerBudgetW",
        Some(&analysis),
        HashMap::from([("value".to_string(), Value::String("55".to_string()))]),
    );
    let _robot = add_node(
        &mut graph,
        &uri,
        "Demo::PowerAnalysis::robot",
        "subject",
        "robot",
        Some(&analysis),
        HashMap::new(),
    );
    add_typing_edge_if_exists(
        &mut graph,
        &uri,
        "Demo::PowerAnalysis::robot",
        "Demo::Robot",
        None,
    );
    add_typing_edge_if_exists(
        &mut graph,
        &uri,
        &mobility_part.qualified_name,
        "Demo::MobilitySubsystem",
        None,
    );

    evaluate_expressions(&mut graph);
    assert_eq!(
        node_attr(&graph, &analysis, ANALYSIS_EVAL_STATUS_KEY),
        Some(&Value::String(STATUS_OK.to_string()))
    );
    assert_eq!(
        node_attr(&graph, &analysis, ANALYSIS_EVAL_VALUE_KEY),
        Some(&Value::Bool(true))
    );
}

#[test]
fn skips_unbound_constraint_def_analysis_evaluation() {
    let mut graph = SemanticGraph::new();
    let uri = Url::parse("file:///C:/workspace/analysis-def-skip.sysml").expect("uri");
    let constraint_def = add_node(
        &mut graph,
        &uri,
        "Demo::EnduranceMargin",
        "constraint def",
        "EnduranceMargin",
        None,
        HashMap::from([(
            ANALYSIS_EXPRESSION_KEY.to_string(),
            Value::String("measured <= limit".to_string()),
        )]),
    );
    evaluate_expressions(&mut graph);
    assert_eq!(
        node_attr(&graph, &constraint_def, ANALYSIS_EVAL_STATUS_KEY),
        None
    );
    assert_eq!(
        node_attr(&graph, &constraint_def, ANALYSIS_EVAL_ERROR_KEY),
        None
    );
}

#[test]
fn skips_calc_def_analysis_evaluation() {
    let mut graph = SemanticGraph::new();
    let uri = Url::parse("file:///C:/workspace/calc-def-skip.sysml").expect("uri");
    let calc_def = add_node(
        &mut graph,
        &uri,
        "Demo::MarginEstimate",
        "calc def",
        "MarginEstimate",
        None,
        HashMap::from([(
            ANALYSIS_EXPRESSION_KEY.to_string(),
            Value::String("limit - measured".to_string()),
        )]),
    );
    evaluate_expressions(&mut graph);
    assert_eq!(node_attr(&graph, &calc_def, ANALYSIS_EVAL_STATUS_KEY), None);
    assert_eq!(node_attr(&graph, &calc_def, ANALYSIS_EVAL_ERROR_KEY), None);
}

#[test]
fn evaluates_builtin_sum_over_quantities() {
    let mut graph = SemanticGraph::new();
    register_units_fixture(&mut graph);
    let uri = Url::parse("file:///C:/workspace/sum.sysml").expect("uri");

    // Workspace values.
    let owner = add_node(&mut graph, &uri, "P", "package", "P", None, HashMap::new());
    let _a = add_node(
        &mut graph,
        &uri,
        "P::a",
        "attribute",
        "a",
        Some(&owner),
        HashMap::from([("value".to_string(), Value::String("2 [kg]".to_string()))]),
    );
    let _b = add_node(
        &mut graph,
        &uri,
        "P::b",
        "attribute",
        "b",
        Some(&owner),
        HashMap::from([("value".to_string(), Value::String("3 [kg]".to_string()))]),
    );

    let expr_id = add_node(
        &mut graph,
        &uri,
        "P::total",
        "attribute",
        "total",
        Some(&owner),
        HashMap::from([("value".to_string(), Value::String("sum(a, b)".to_string()))]),
    );

    evaluate_expressions(&mut graph);
    let evaluated = node_attr(&graph, &expr_id, EVALUATED_VALUE_KEY)
        .cloned()
        .expect("evaluated value");
    assert_eq!(evaluated, Value::Number(serde_json::Number::from(5)));
    let unit = node_attr(&graph, &expr_id, EVALUATED_UNIT_KEY)
        .and_then(Value::as_str)
        .expect("evaluated unit");
    assert_eq!(unit, "kg");
}

#[test]
fn evaluates_builtin_count_min_max_avg() {
    let mut graph = SemanticGraph::new();
    register_units_fixture(&mut graph);
    let uri = Url::parse("file:///C:/workspace/aggs.sysml").expect("uri");
    let owner = add_node(&mut graph, &uri, "P", "package", "P", None, HashMap::new());

    let _a = add_node(
        &mut graph,
        &uri,
        "P::a",
        "attribute",
        "a",
        Some(&owner),
        HashMap::from([("value".to_string(), Value::String("2 [cm]".to_string()))]),
    );
    let _b = add_node(
        &mut graph,
        &uri,
        "P::b",
        "attribute",
        "b",
        Some(&owner),
        HashMap::from([("value".to_string(), Value::String("1 [m]".to_string()))]),
    );
    let min_id = add_node(
        &mut graph,
        &uri,
        "P::minV",
        "attribute",
        "minV",
        Some(&owner),
        HashMap::from([("value".to_string(), Value::String("min(a, b)".to_string()))]),
    );
    let max_id = add_node(
        &mut graph,
        &uri,
        "P::maxV",
        "attribute",
        "maxV",
        Some(&owner),
        HashMap::from([("value".to_string(), Value::String("max(a, b)".to_string()))]),
    );
    let avg_id = add_node(
        &mut graph,
        &uri,
        "P::avgV",
        "attribute",
        "avgV",
        Some(&owner),
        HashMap::from([("value".to_string(), Value::String("avg(a, b)".to_string()))]),
    );
    let count_id = add_node(
        &mut graph,
        &uri,
        "P::countV",
        "attribute",
        "countV",
        Some(&owner),
        HashMap::from([(
            "value".to_string(),
            Value::String("count(a, b)".to_string()),
        )]),
    );

    evaluate_expressions(&mut graph);

    assert_eq!(
        node_attr(&graph, &count_id, EVALUATED_VALUE_KEY),
        Some(&Value::Number(serde_json::Number::from(2)))
    );

    // min(a=2cm, b=1m=100cm) => 2cm (keeps first-arg unit)
    assert_eq!(
        node_attr(&graph, &min_id, EVALUATED_VALUE_KEY),
        Some(&Value::Number(serde_json::Number::from(2)))
    );
    assert_eq!(
        node_attr(&graph, &min_id, EVALUATED_UNIT_KEY).and_then(Value::as_str),
        Some("cm")
    );

    // max(a=2cm, b=100cm) => 100cm
    assert_eq!(
        node_attr(&graph, &max_id, EVALUATED_VALUE_KEY),
        Some(&Value::Number(serde_json::Number::from(100)))
    );
    assert_eq!(
        node_attr(&graph, &max_id, EVALUATED_UNIT_KEY).and_then(Value::as_str),
        Some("cm")
    );

    // avg(2cm, 100cm) => 51cm
    assert_eq!(
        node_attr(&graph, &avg_id, EVALUATED_VALUE_KEY),
        Some(&Value::Number(serde_json::Number::from(51)))
    );
    assert_eq!(
        node_attr(&graph, &avg_id, EVALUATED_UNIT_KEY).and_then(Value::as_str),
        Some("cm")
    );
}

#[test]
fn evaluates_analysis_case_with_calc_collection_rollup() {
    let mut graph = SemanticGraph::new();
    register_units_fixture(&mut graph);
    let uri = Url::parse("file:///C:/workspace/calc-rollup-analysis.sysml").expect("uri");
    let _calc = add_node(
        &mut graph,
        &uri,
        "Demo::SubsystemMassSum",
        "calc def",
        "SubsystemMassSum",
        None,
        HashMap::from([
            (
                "parameters".to_string(),
                serde_json::json!([{"direction":"in","name":"parts","type":""}]),
            ),
            (
                ANALYSIS_EXPRESSION_KEY.to_string(),
                Value::String("sum(parts.massKg)".to_string()),
            ),
        ]),
    );
    let analysis = add_node(
        &mut graph,
        &uri,
        "Demo::MassAnalysis",
        "analysis def",
        "MassAnalysis",
        None,
        HashMap::from([(
            ANALYSIS_EXPRESSION_KEY.to_string(),
            Value::String(
                "SubsystemMassSum(parts=(robot.engine, robot.chassis)) <= massLimitKg".to_string(),
            ),
        )]),
    );
    let robot = add_node(
        &mut graph,
        &uri,
        "Demo::MassAnalysis::robot",
        "subject",
        "robot",
        Some(&analysis),
        HashMap::new(),
    );
    let engine = add_node(
        &mut graph,
        &uri,
        "Demo::MassAnalysis::robot::engine",
        "part",
        "engine",
        Some(&robot),
        HashMap::new(),
    );
    let _engine_mass = add_node(
        &mut graph,
        &uri,
        "Demo::MassAnalysis::robot::engine::massKg",
        "attribute",
        "massKg",
        Some(&engine),
        HashMap::from([("value".to_string(), Value::String("2 [kg]".to_string()))]),
    );
    let chassis = add_node(
        &mut graph,
        &uri,
        "Demo::MassAnalysis::robot::chassis",
        "part",
        "chassis",
        Some(&robot),
        HashMap::new(),
    );
    let _chassis_mass = add_node(
        &mut graph,
        &uri,
        "Demo::MassAnalysis::robot::chassis::massKg",
        "attribute",
        "massKg",
        Some(&chassis),
        HashMap::from([("value".to_string(), Value::String("3 [kg]".to_string()))]),
    );
    let _limit = add_node(
        &mut graph,
        &uri,
        "Demo::MassAnalysis::massLimitKg",
        "attribute",
        "massLimitKg",
        Some(&analysis),
        HashMap::from([("value".to_string(), Value::String("10 [kg]".to_string()))]),
    );

    evaluate_expressions(&mut graph);

    assert_eq!(
        node_attr(&graph, &analysis, ANALYSIS_EVAL_STATUS_KEY),
        Some(&Value::String(STATUS_OK.to_string()))
    );
    assert_eq!(
        node_attr(&graph, &analysis, ANALYSIS_COMPUTED_VALUE_KEY),
        Some(&serde_json::json!(5))
    );
}

#[test]
fn evaluates_sum_over_bound_part_collection_projection() {
    let mut graph = SemanticGraph::new();
    register_units_fixture(&mut graph);
    let uri = Url::parse("file:///C:/workspace/agg-collection.sysml").expect("uri");
    let owner = add_node(&mut graph, &uri, "P", "package", "P", None, HashMap::new());

    // Two parts with mass attributes.
    let engine = add_node(
        &mut graph,
        &uri,
        "P::engine",
        "part",
        "engine",
        Some(&owner),
        HashMap::new(),
    );
    let _engine_mass = add_node(
        &mut graph,
        &uri,
        "P::engine::massKg",
        "attribute",
        "massKg",
        Some(&engine),
        HashMap::from([("value".to_string(), Value::String("2 [kg]".to_string()))]),
    );
    let chassis = add_node(
        &mut graph,
        &uri,
        "P::chassis",
        "part",
        "chassis",
        Some(&owner),
        HashMap::new(),
    );
    let _chassis_mass = add_node(
        &mut graph,
        &uri,
        "P::chassis::massKg",
        "attribute",
        "massKg",
        Some(&chassis),
        HashMap::from([("value".to_string(), Value::String("3 [kg]".to_string()))]),
    );

    // Calc def that expects a collection and rolls it up.
    let _calc = add_node(
        &mut graph,
        &uri,
        "P::TotalMass",
        "calc def",
        "TotalMass",
        Some(&owner),
        HashMap::from([
            (
                "parameters".to_string(),
                serde_json::json!([
                    {"direction":"in","name":"parts","type":"Part[*]"}
                ]),
            ),
            (
                ANALYSIS_EXPRESSION_KEY.to_string(),
                Value::String("sum(parts.massKg)".to_string()),
            ),
        ]),
    );

    let total = add_node(
        &mut graph,
        &uri,
        "P::total",
        "attribute",
        "total",
        Some(&owner),
        HashMap::from([(
            "value".to_string(),
            Value::String("TotalMass(parts=(engine, chassis))".to_string()),
        )]),
    );

    evaluate_expressions(&mut graph);

    assert_eq!(
        node_attr(&graph, &total, EVALUATED_VALUE_KEY),
        Some(&Value::Number(serde_json::Number::from(5)))
    );
    assert_eq!(
        node_attr(&graph, &total, EVALUATED_UNIT_KEY).and_then(Value::as_str),
        Some("kg")
    );
}
