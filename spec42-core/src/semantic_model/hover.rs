use super::{SemanticGraph, SemanticNode};

/// Builds a signature string from node attributes (partType, specializes, etc.).
pub(crate) fn signature_from_node(node: &SemanticNode) -> Option<String> {
    let kind = node.element_kind.as_str();
    let mult = node
        .attributes
        .get("multiplicity")
        .and_then(|v| v.as_str())
        .map(|m| format!(" {}", m))
        .unwrap_or_default();
    let (type_attr, type_suffix) = match kind {
        "part def" | "part" => (
            node.attributes
                .get("partType")
                .or_else(|| node.attributes.get("specializes")),
            " : ",
        ),
        "attribute def" | "attribute" => (node.attributes.get("attributeType"), " : "),
        "port def" | "port" => (node.attributes.get("portType"), " : "),
        "actor def" => (node.attributes.get("actorType"), " : "),
        "item def" => (node.attributes.get("specializes"), " :> "),
        "item" => (node.attributes.get("itemType"), " : "),
        _ => (None, ""),
    };
    let type_part = type_attr
        .and_then(|v| v.as_str())
        .map(|t| format!("{}{}", type_suffix, t))
        .unwrap_or_default();
    Some(format!("{} {}{}{};", kind, node.name, type_part, mult))
}

pub fn hover_markdown_for_node(graph: &SemanticGraph, node: &SemanticNode, show_location: bool) -> String {
    let mut md = format!("**{}** `{}`\n\n", node.element_kind, node.name);
    let code_block =
        signature_from_node(node).unwrap_or_else(|| format!("{} {};", node.element_kind, node.name));
    md.push_str("```sysml\n");
    md.push_str(&code_block);
    md.push_str("\n```\n\n");

    if let Some(parent_id) = &node.parent_id {
        if let Some(parent) = graph.get_node(parent_id) {
            md.push_str(&format!("*Container:* `{}`\n\n", parent.name));
        }
    }

    if let Some(type_name) = node
        .attributes
        .get("partType")
        .or_else(|| node.attributes.get("attributeType"))
        .or_else(|| node.attributes.get("portType"))
        .or_else(|| node.attributes.get("actorType"))
        .or_else(|| node.attributes.get("itemType"))
        .and_then(|value| value.as_str())
    {
        md.push_str(&format!("*Type:* `{}`\n\n", type_name));
    }

    if let Some(multiplicity) = node
        .attributes
        .get("multiplicity")
        .and_then(|value| value.as_str())
    {
        md.push_str(&format!("*Multiplicity:* `{}`\n\n", multiplicity));
    }

    let typed_targets = graph.outgoing_typing_or_specializes_targets(node);
    if let Some(target) = typed_targets.first() {
        md.push_str(&format!(
            "*Resolves to:* `{}` ({})\n\n",
            target.name, target.element_kind
        ));
    }

    if show_location {
        md.push_str(&format!("*Defined in:* {}", node.id.uri.path()));
    }

    md
}
