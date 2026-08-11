use sysml_model::{EvaluatedValue, SemanticGraph, SemanticNode};

fn evaluated_value_to_inline_text(value: &EvaluatedValue) -> String {
    match value {
        EvaluatedValue::Integer(value) => value.to_string(),
        EvaluatedValue::Real(value) => value.to_string(),
        EvaluatedValue::Boolean(value) => value.to_string(),
        EvaluatedValue::String(value) => value.clone(),
    }
}

fn attr_str<'a>(node: &'a SemanticNode, key: &str) -> Option<&'a str> {
    node.attributes.get(key).and_then(|value| value.as_str())
}

fn first_attr_str<'a>(node: &'a SemanticNode, keys: &[&str]) -> Option<&'a str> {
    keys.iter().find_map(|key| attr_str(node, key))
}

fn json_value_to_inline_text(value: &serde_json::Value) -> Option<String> {
    match value {
        serde_json::Value::String(text) => Some(text.clone()),
        serde_json::Value::Number(number) => Some(number.to_string()),
        serde_json::Value::Bool(boolean) => Some(boolean.to_string()),
        serde_json::Value::Null => None,
        serde_json::Value::Array(values) => {
            let lines: Vec<String> = values
                .iter()
                .filter_map(json_value_to_inline_text)
                .filter(|line| !line.trim().is_empty())
                .collect();
            if lines.is_empty() {
                None
            } else {
                Some(lines.join(", "))
            }
        }
        _ => serde_json::to_string(value).ok(),
    }
}

fn array_attr_lines(node: &SemanticNode, key: &str) -> Vec<String> {
    node.attributes
        .get(key)
        .and_then(|value| value.as_array())
        .map(|values| {
            values
                .iter()
                .filter_map(|value| value.as_str())
                .map(str::trim)
                .filter(|line| !line.is_empty())
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

/// Appends one field as a tightly-stacked line (hard line break via trailing double-space, not a
/// full paragraph break) so a run of short facts reads as one compact block rather than a column
/// of separately-spaced paragraphs.
fn append_field(md: &mut String, label: &str, value: &str) {
    if value.trim().is_empty() {
        return;
    }
    md.push_str(&format!("**{}:** `{}`  \n", label, value));
}

fn append_plain_field(md: &mut String, label: &str, value: &str) {
    if value.trim().is_empty() {
        return;
    }
    md.push_str(&format!("**{}:** {}  \n", label, value));
}

fn declared_type(node: &SemanticNode) -> Option<&str> {
    first_attr_str(
        node,
        &[
            "partType",
            "subjectType",
            "attributeType",
            "portType",
            "actorType",
            "itemType",
            "parameterType",
            "stateType",
            "requirementType",
            "objectiveType",
            "refType",
            "type",
        ],
    )
}

fn append_attribute_value(md: &mut String, node: &SemanticNode, label: &str, keys: &[&str]) {
    if let Some(value) = keys.iter().find_map(|key| {
        node.attributes
            .get(*key)
            .and_then(json_value_to_inline_text)
            .filter(|value| !value.trim().is_empty())
    }) {
        append_field(md, label, &value);
    }
}

fn append_multiline_section(md: &mut String, title: &str, lines: &[String]) {
    if lines.is_empty() {
        return;
    }
    md.push_str(&format!("\n**{}:**\n\n", title));
    md.push_str("```text\n");
    for line in lines {
        md.push_str(line);
        md.push('\n');
    }
    md.push_str("```\n");
}

/// Builds a signature string from node attributes (partType, specializes, etc.).
pub fn signature_from_node(node: &SemanticNode) -> Option<String> {
    let kind = node.element_kind.as_str();
    let multiplicity = attr_str(node, "multiplicity")
        .map(|m| format!(" {}", m))
        .unwrap_or_default();
    let value_suffix = node
        .attributes
        .get("value")
        .and_then(json_value_to_inline_text)
        .map(|value| format!(" = {}", value))
        .unwrap_or_default();

    let signature = match kind {
        "part def" => {
            let prefix = attr_str(node, "definitionPrefix")
                .map(|p| format!("{p} "))
                .unwrap_or_default();
            let specializes = attr_str(node, "specializes")
                .map(|base| format!(" :> {}", base))
                .unwrap_or_default();
            format!("{prefix}part def {}{specializes};", node.name)
        }
        "part" => {
            let type_part = attr_str(node, "partType")
                .map(|t| format!(" : {}", t))
                .unwrap_or_default();
            format!(
                "part {}{}{}{};",
                node.name, type_part, multiplicity, value_suffix
            )
        }
        "subject" => {
            let type_part = attr_str(node, "subjectType")
                .map(|t| format!(" : {}", t))
                .unwrap_or_default();
            format!("subject {}{};", node.name, type_part)
        }
        "attribute def" => {
            let type_part = attr_str(node, "attributeType")
                .map(|t| format!(" : {}", t))
                .unwrap_or_default();
            format!("attribute def {}{};", node.name, type_part)
        }
        "attribute" => {
            let type_part = attr_str(node, "attributeType")
                .map(|t| format!(" : {}", t))
                .unwrap_or_default();
            format!(
                "attribute {}{}{}{};",
                node.name, type_part, multiplicity, value_suffix
            )
        }
        "port def" => {
            let specializes = attr_str(node, "specializes")
                .map(|base| format!(" :> {}", base))
                .unwrap_or_default();
            format!("port def {}{specializes};", node.name)
        }
        "port" => {
            let type_part = attr_str(node, "portType")
                .map(|t| format!(" : {}", t))
                .unwrap_or_default();
            format!("port {}{}{};", node.name, type_part, multiplicity)
        }
        "item def" => {
            let specializes = attr_str(node, "specializes")
                .map(|base| format!(" :> {}", base))
                .unwrap_or_default();
            format!("item def {}{specializes};", node.name)
        }
        "individual def" => {
            let specializes = attr_str(node, "specializes")
                .map(|base| format!(" :> {}", base))
                .unwrap_or_default();
            format!("individual def {}{specializes};", node.name)
        }
        "item" => {
            let type_part = attr_str(node, "itemType")
                .map(|t| format!(" : {}", t))
                .unwrap_or_default();
            format!("item {}{}{};", node.name, type_part, multiplicity)
        }
        "enumeration" => {
            let type_part = first_attr_str(node, &["enumerationType", "type"])
                .map(|t| format!(" : {}", t))
                .unwrap_or_default();
            format!("enum {}{}{};", node.name, type_part, multiplicity)
        }
        "opaque member" => {
            let keyword = node.source_text.keyword.as_deref().unwrap_or("opaque");
            format!("{} {};", keyword, node.name)
        }
        "require constraint" => {
            let expression = attr_str(node, "expression").unwrap_or("");
            if expression.trim().is_empty() {
                "require constraint {};".to_string()
            } else {
                format!("require constraint {{ {} }};", expression.trim())
            }
        }
        "stakeholder" => format!("stakeholder {};", node.name),
        "purpose" => format!("purpose {};", node.name),
        "verified requirement" => format!("verify requirement {};", node.name),
        "view rendering" => {
            let type_part = attr_str(node, "renderingType")
                .map(|t| format!(" : {}", t))
                .unwrap_or_default();
            format!("render {}{};", node.name, type_part)
        }
        "ref redefinition" => {
            let body = node.source_text.body.as_deref().unwrap_or("{}");
            format!("ref :>> {} {{ {} }};", node.name, body.trim())
        }
        "actor redefinition" => {
            let rhs = attr_str(node, "rhs").unwrap_or("");
            format!("actor :>> {} = {};", node.name, rhs.trim())
        }
        "include use case" => {
            let target = attr_str(node, "includeTarget").unwrap_or(node.name.as_str());
            format!("include {};", target)
        }
        "filter" => {
            let condition = attr_str(node, "condition").unwrap_or("");
            if condition.trim().is_empty() {
                "filter {};".to_string()
            } else {
                format!("filter {};", condition.trim())
            }
        }
        "verdict" => {
            let token = attr_str(node, "rawVerdictToken").unwrap_or("done");
            format!("return {} {};", node.name, token)
        }
        "occurrence" => {
            let type_part = attr_str(node, "occurrenceType")
                .map(|t| format!(" : {}", t))
                .unwrap_or_default();
            format!("occurrence {}{};", node.name, type_part)
        }
        "flow" => {
            let type_part = attr_str(node, "flowType")
                .map(|t| format!(" : {}", t))
                .unwrap_or_default();
            format!("flow {}{};", node.name, type_part)
        }
        "action def" | "requirement def" | "requirement" | "concern" | "use case def"
        | "use case" | "interface" | "interface def" | "frame" | "state" | "actor" => {
            format!("{} {};", kind, node.name)
        }
        "in out parameter" => {
            let direction = attr_str(node, "direction").unwrap_or("in");
            let type_part = first_attr_str(node, &["parameterType", "type"])
                .map(|t| format!(" : {}", t))
                .unwrap_or_default();
            format!("{direction} {}{type_part};", node.name)
        }
        "import" => {
            let visibility = attr_str(node, "visibility")
                .map(|v| {
                    let normalized = v.trim();
                    if normalized.starts_with("Public") {
                        "public ".to_string()
                    } else if normalized.starts_with("Private") {
                        "private ".to_string()
                    } else {
                        String::new()
                    }
                })
                .unwrap_or_default();
            let recursive = node
                .attributes
                .get("recursive")
                .and_then(|value| value.as_bool())
                .filter(|enabled| *enabled)
                .map(|_| "recursive ")
                .unwrap_or_default();
            let target = attr_str(node, "importTarget").unwrap_or(node.name.as_str());
            format!("{visibility}import {recursive}{target};")
        }
        "feature decl" | "classifier decl" => node
            .source_text
            .text
            .clone()
            .unwrap_or_else(|| format!("{} {};", kind, node.name)),
        _ => format!("{} {};", kind, node.name),
    };

    Some(signature)
}

pub fn hover_markdown_for_node(
    graph: &SemanticGraph,
    node: &SemanticNode,
    show_location: bool,
) -> String {
    let mut md = String::new();
    let code_block = signature_from_node(node)
        .unwrap_or_else(|| format!("{} {};", node.element_kind, node.name));
    md.push_str("```sysml\n");
    md.push_str(&code_block);
    md.push_str("\n```\n");

    // Everything below the signature -- doc prose, then the compact fact block -- lives behind a
    // single divider, matching rust-analyzer's code/rule/details layout instead of interleaving
    // rule-separated bands per field.
    let mut body = String::new();

    if let Some(doc) = node.source_text.doc.as_deref() {
        let doc = doc.trim();
        if !doc.is_empty() {
            body.push_str(doc);
            body.push_str("\n\n");
        }
    }

    append_field(&mut body, "Kind", node.element_kind.as_str());
    append_field(&mut body, "Qualified name", &node.id.qualified_name);

    if let Some(parent_id) = &node.parent_id {
        if let Some(parent) = graph.get_node(parent_id) {
            if !parent.id.qualified_name.trim().is_empty() {
                append_field(&mut body, "Container", &parent.id.qualified_name);
            }
        }
    }

    if let Some(type_name) = declared_type(node) {
        append_field(&mut body, "Declared type", type_name);
    }

    let typed_targets = graph.outgoing_typing_or_specializes_targets(node);
    if let Some(first_target) = typed_targets.first() {
        // Suppress only when every target is already spelled out verbatim in the declared-type
        // text (the common single-target case, e.g. `: Foo` resolving to `Foo`) -- a multi-target
        // `:> A, B` clause should still surface targets the declared-type text doesn't already
        // name, instead of silently dropping everything past the first.
        let should_show_targets = match declared_type(node) {
            Some(type_name) => typed_targets
                .iter()
                .any(|target| type_name.trim() != target.name.trim()),
            None => true,
        };
        if should_show_targets {
            let label = if first_target.element_kind.is_definition() {
                "Resolved type"
            } else {
                "Resolves to"
            };
            let qualified_names: Vec<&str> = typed_targets
                .iter()
                .map(|target| target.id.qualified_name.as_str())
                .collect();
            append_field(&mut body, label, &qualified_names.join(", "));
        }
    }

    append_attribute_value(&mut body, node, "Multiplicity", &["multiplicity"]);
    append_attribute_value(&mut body, node, "Value", &["value", "defaultValue"]);
    if let Some(evaluation) = graph
        .evaluation_facts_for(node)
        .and_then(|facts| facts.expression.as_ref())
    {
        if let Some(value) = evaluation
            .value
            .as_ref()
            .map(evaluated_value_to_inline_text)
        {
            append_field(&mut body, "Evaluated value", &value);
        }
        if let Some(unit) = evaluation.unit.as_deref().filter(|unit| !unit.is_empty()) {
            append_field(&mut body, "Unit", unit);
        }
    }

    if show_location {
        append_plain_field(&mut body, "Defined in", node.id.uri.path());
    }

    let constraint_lines = array_attr_lines(node, "requirementConstraints");
    append_multiline_section(&mut body, "Constraint body", &constraint_lines);

    if !body.trim().is_empty() {
        md.push_str("\n---\n\n");
        md.push_str(body.trim_end());
        md.push('\n');
    }

    md
}

#[cfg(test)]
mod tests {
    use sysml_model::{build_graph_from_doc, SemanticGraph, SemanticNode};
    use sysml_v2_parser::parse;
    use url::Url;

    use super::{hover_markdown_for_node, signature_from_node};

    fn graph_node<'a>(
        graph: &'a SemanticGraph,
        uri: &Url,
        kind: &str,
        name: &str,
    ) -> &'a SemanticNode {
        graph
            .nodes_for_uri(uri)
            .into_iter()
            .find(|node| node.element_kind == kind && node.name == name)
            .unwrap_or_else(|| panic!("expected {kind} node named {name}"))
    }

    #[test]
    fn hover_includes_doc_comment_text() {
        let input = r#"package P {
  part def Widget {
    doc /* A widget that does widget things. */
  }
}"#;
        let root = parse(input).expect("parse");
        let uri = Url::parse("file:///w.sysml").expect("uri");
        let graph = build_graph_from_doc(&root, &uri);
        let widget = graph_node(&graph, &uri, "part def", "Widget");
        let hover = hover_markdown_for_node(&graph, widget, false);
        assert!(
            hover.contains("A widget that does widget things."),
            "hover should include the doc comment text: {hover}"
        );
    }

    #[test]
    fn hover_omits_doc_section_when_no_doc_comment_present() {
        let input = r#"package P {
  part def Widget;
}"#;
        let root = parse(input).expect("parse");
        let uri = Url::parse("file:///w.sysml").expect("uri");
        let graph = build_graph_from_doc(&root, &uri);
        let widget = graph_node(&graph, &uri, "part def", "Widget");
        let hover = hover_markdown_for_node(&graph, widget, false);
        assert!(
            hover.starts_with("```sysml"),
            "hover with no doc comment should go straight from the signature block to fields: {hover}"
        );
    }

    #[test]
    fn hover_shows_all_specialization_targets_not_just_first() {
        let input = r#"package P {
  part def A;
  part def B;
  part def C :> A, B;
}"#;
        let root = parse(input).expect("parse");
        let uri = Url::parse("file:///c.sysml").expect("uri");
        let graph = build_graph_from_doc(&root, &uri);
        let c = graph_node(&graph, &uri, "part def", "C");
        let hover = hover_markdown_for_node(&graph, c, false);
        assert!(
            hover.contains("P::A") && hover.contains("P::B"),
            "hover should list every specialization target, not just the first: {hover}"
        );
    }

    #[test]
    fn require_constraint_signature_contains_keyword() {
        let input = r#"package P {
  requirement def Safety {
    require constraint { speed <= 120 }
  }
}"#;
        let root = parse(input).expect("parse");
        let uri = Url::parse("file:///req.sysml").expect("uri");
        let graph = build_graph_from_doc(&root, &uri);
        let constraint = graph_node(&graph, &uri, "require constraint", "_requireConstraint_0");
        let signature = signature_from_node(constraint).expect("signature");
        assert!(
            signature.contains("require constraint"),
            "signature should mention require constraint: {signature}"
        );
    }

    #[test]
    fn stakeholder_hover_includes_kind_line() {
        let input = r#"package P {
  requirement def Safety {
    stakeholder auditor;
  }
}"#;
        let root = parse(input).expect("parse");
        let uri = Url::parse("file:///req.sysml").expect("uri");
        let graph = build_graph_from_doc(&root, &uri);
        let stakeholder = graph
            .nodes_for_uri(&uri)
            .into_iter()
            .find(|node| node.element_kind == "stakeholder")
            .expect("stakeholder node");
        let hover = hover_markdown_for_node(&graph, stakeholder, false);
        assert!(
            hover.contains("stakeholder"),
            "hover should include stakeholder kind: {hover}"
        );
    }

    #[test]
    fn enumeration_signature_uses_enum_keyword() {
        let input = r#"package P {
  enum def Status;
  part def Vehicle {
    enum status : Status;
  }
}"#;
        let root = parse(input).expect("parse");
        let uri = Url::parse("file:///enum.sysml").expect("uri");
        let graph = build_graph_from_doc(&root, &uri);
        let enumeration = graph_node(&graph, &uri, "enumeration", "status");
        let signature = signature_from_node(enumeration).expect("signature");
        assert!(
            signature.starts_with("enum status"),
            "enumeration signature should use enum keyword: {signature}"
        );
    }

    #[test]
    fn hover_includes_multiline_doc_comment_text_via_typed_source_text_fact() {
        let input = r#"package P {
  part def Widget {
    doc /* First line of documentation.
    Second line of documentation. */
  }
}"#;
        let root = parse(input).expect("parse");
        let uri = Url::parse("file:///w-multiline.sysml").expect("uri");
        let graph = build_graph_from_doc(&root, &uri);
        let widget = graph_node(&graph, &uri, "part def", "Widget");
        assert!(
            widget.source_text.doc.is_some(),
            "doc text should be captured on the typed source_text fact"
        );
        let hover = hover_markdown_for_node(&graph, widget, false);
        assert!(
            hover.contains("First line of documentation.")
                && hover.contains("Second line of documentation."),
            "hover should surface the full multi-line doc comment unchanged: {hover}"
        );
    }
}
