use crate::semantic::{SemanticGraph, SemanticNode};

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

fn append_field(md: &mut String, label: &str, value: &str) {
    if value.trim().is_empty() {
        return;
    }
    md.push_str(&format!("*{}:* `{}`\n\n", label, value));
}

fn append_multiline_section(md: &mut String, title: &str, lines: &[String]) {
    if lines.is_empty() {
        return;
    }
    md.push_str(&format!("*{}:*\n\n", title));
    md.push_str("```text\n");
    for line in lines {
        md.push_str(line);
        md.push('\n');
    }
    md.push_str("```\n\n");
}

/// Builds a signature string from node attributes (partType, specializes, etc.).
pub(crate) fn signature_from_node(node: &SemanticNode) -> Option<String> {
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
        "item" => {
            let type_part = attr_str(node, "itemType")
                .map(|t| format!(" : {}", t))
                .unwrap_or_default();
            format!("item {}{}{};", node.name, type_part, multiplicity)
        }
        "actor def" => {
            let type_part = attr_str(node, "actorType")
                .map(|t| format!(" : {}", t))
                .unwrap_or_default();
            format!("actor def {}{};", node.name, type_part)
        }
        "action def" | "requirement def" | "requirement" | "concern" | "use case def"
        | "use case" | "interface" | "frame" | "state" => {
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
        "feature decl" | "classifier decl" => attr_str(node, "text")
            .map(str::to_string)
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
    md.push_str("\n```\n\n");

    if let Some(parent_id) = &node.parent_id {
        if let Some(parent) = graph.get_node(parent_id) {
            if !parent.name.trim().is_empty() {
                append_field(&mut md, "In", &parent.name);
            }
        }
    }

    let typed_targets = graph.outgoing_typing_or_specializes_targets(node);
    if let Some(target) = typed_targets.first() {
        let declared_type = first_attr_str(
            node,
            &[
                "partType",
                "subjectType",
                "attributeType",
                "portType",
                "actorType",
                "itemType",
            ],
        );
        let should_show_target = match declared_type {
            Some(type_name) => type_name.trim() != target.name.trim(),
            None => true,
        };
        if should_show_target {
            md.push_str(&format!("*Resolves to:* `{}`\n\n", target.name));
        }
    }

    let constraint_lines = array_attr_lines(node, "requirementConstraints");
    append_multiline_section(&mut md, "Constraint body", &constraint_lines);

    if show_location {
        md.push_str(&format!("*Defined in:* {}", node.id.uri.path()));
    }

    md
}
