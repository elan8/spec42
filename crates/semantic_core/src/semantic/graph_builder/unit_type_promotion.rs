//! Promote KerML library declaration text for unit/quantity types into `attribute def` graph nodes.

use std::collections::HashMap;

use url::Url;

use super::{add_node_and_recurse, qualified_name_for_node};
use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::NodeId;
use crate::semantic::relationships::{add_specializes_edge_if_exists, add_typing_edge_if_exists};
use crate::semantic::text_span::TextRange;
use crate::semantic::units::type_resolver::is_unit_type_name;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedUnitAttributeDef {
    pub name: String,
    pub specializes: Option<String>,
    pub m_ref_unit: Option<String>,
}

/// Returns true when KerML modeled declaration text declares a quantity/unit type def.
pub fn try_parse_unit_attribute_def(text: &str) -> Option<ParsedUnitAttributeDef> {
    let normalized = text.trim().trim_end_matches(';');
    let lower = normalized.to_ascii_lowercase();
    if !lower.contains("attribute") || !lower.contains("def") {
        return None;
    }
    let name = extract_declared_name_after_def(normalized)?;
    if !is_unit_type_name(&name) && !name.ends_with("Value") {
        return None;
    }
    let specializes = extract_specializes_target(normalized);
    let m_ref_unit = extract_mref_unit_type(normalized);
    Some(ParsedUnitAttributeDef {
        name,
        specializes,
        m_ref_unit,
    })
}

pub fn materialize_unit_attribute_def_from_kerml(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    parsed: &ParsedUnitAttributeDef,
    span: &TextRange,
) {
    let qualified = qualified_name_for_node(
        g,
        uri,
        container_prefix,
        &parsed.name,
        "attribute def",
    );
    let mut attrs = HashMap::new();
    if let Some(ref base) = parsed.specializes {
        attrs.insert("attributeType".to_string(), serde_json::json!(base));
        if parsed.name.ends_with("Value") {
            attrs.insert("specializes".to_string(), serde_json::json!(base));
        }
    }
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "attribute def",
        parsed.name.clone(),
        *span,
        attrs,
        Some(parent_id),
    );
    if let Some(ref base) = parsed.specializes {
        if parsed.name.ends_with("Unit") {
            add_typing_edge_if_exists(g, uri, &qualified, base, container_prefix);
        } else {
            add_specializes_edge_if_exists(g, uri, &qualified, base, container_prefix);
        }
    }
    if let Some(ref unit_type) = parsed.m_ref_unit {
        let mref_qualified = qualified_name_for_node(
            g,
            uri,
            Some(qualified.as_str()),
            "mRef",
            "attribute def",
        );
        let mut mref_attrs = HashMap::new();
        mref_attrs.insert("attributeType".to_string(), serde_json::json!(unit_type));
        add_node_and_recurse(
            g,
            uri,
            &mref_qualified,
            "attribute def",
            "mRef".to_string(),
            *span,
            mref_attrs,
            Some(&NodeId::new(uri, &qualified)),
        );
        add_typing_edge_if_exists(g, uri, &mref_qualified, unit_type, Some(qualified.as_str()));
    }
}

fn extract_declared_name_after_def(text: &str) -> Option<String> {
    let tokens: Vec<&str> = text.split_whitespace().collect();
    let def_pos = tokens
        .iter()
        .position(|t| t.eq_ignore_ascii_case("def"))?;
    let mut i = def_pos + 1;
    while i < tokens.len() {
        let tok = tokens[i].trim_end_matches(|c: char| !c.is_alphanumeric() && c != '_');
        if tok.eq_ignore_ascii_case("id") || tok.starts_with('\'') {
            i += 1;
            continue;
        }
        if tok.eq_ignore_ascii_case(":>")
            || tok == ":>"
            || tok.eq_ignore_ascii_case("specializes")
            || tok == ":"
            || tok.starts_with('{')
        {
            break;
        }
        let name = sanitize_identifier(tok);
        if !name.is_empty() {
            return Some(name);
        }
        break;
    }
    None
}

fn extract_specializes_target(text: &str) -> Option<String> {
    let normalized = text.replace(":>", " :> ");
    let tokens: Vec<&str> = normalized.split_whitespace().collect();
    for (idx, tok) in tokens.iter().enumerate() {
        if *tok == ":>" || tok.eq_ignore_ascii_case("specializes") {
            if let Some(next) = tokens.get(idx + 1) {
                let name = sanitize_identifier(next.trim_end_matches(|c: char| !c.is_alphanumeric() && c != '_'));
                if !name.is_empty() {
                    return Some(name);
                }
            }
        }
    }
    None
}

fn extract_mref_unit_type(text: &str) -> Option<String> {
    let lower = text.to_ascii_lowercase();
    let idx = lower.find("mref")?;
    let after = &text[idx..];
    let colon = after.find(':')?;
    let rest = after[colon + 1..].trim_start();
    let end = rest
        .find([';', '{', '}'])
        .unwrap_or(rest.len());
    let type_name = sanitize_identifier(rest[..end].trim());
    if is_unit_type_name(&type_name) {
        Some(type_name)
    } else {
        None
    }
}

fn sanitize_identifier(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_unit_type_specialization() {
        let parsed = try_parse_unit_attribute_def(
            "abstract attribute def SimpleUnit :> MeasurementUnit;",
        )
        .expect("SimpleUnit");
        assert_eq!(parsed.name, "SimpleUnit");
        assert_eq!(parsed.specializes.as_deref(), Some("MeasurementUnit"));
    }

    #[test]
    fn parses_length_unit_and_value_mref() {
        let parsed = try_parse_unit_attribute_def(
            "attribute def LengthValue :> ScalarQuantityValue { attribute :>> mRef : LengthUnit; }",
        )
        .expect("LengthValue");
        assert_eq!(parsed.name, "LengthValue");
        assert_eq!(parsed.m_ref_unit.as_deref(), Some("LengthUnit"));
    }
}
