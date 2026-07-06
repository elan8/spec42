//! Maps parser payload clauses onto semantic graph attributes.

use std::collections::HashMap;

use sysml_v2_parser::ast::{ActionUsage, PayloadClause, TransitionAccept};

use super::expressions;

pub(super) fn insert_payload_clause_attrs(
    attrs: &mut HashMap<String, serde_json::Value>,
    kind: &str,
    clause: &PayloadClause,
) {
    attrs.insert("actionKind".to_string(), serde_json::json!(kind));
    attrs.insert("payloadName".to_string(), serde_json::json!(&clause.name));
    if let Some(ref type_name) = clause.type_name {
        attrs.insert("payloadType".to_string(), serde_json::json!(type_name));
        if kind == "accept" {
            attrs.insert("acceptName".to_string(), serde_json::json!(&clause.name));
            attrs.insert("acceptType".to_string(), serde_json::json!(type_name));
        }
    }
}

pub(super) fn insert_action_payload_attrs(
    attrs: &mut HashMap<String, serde_json::Value>,
    action: &ActionUsage,
) {
    if let Some(ref accept) = action.accept {
        insert_payload_clause_attrs(attrs, "accept", accept);
    }
    if let Some(ref send) = action.send {
        insert_payload_clause_attrs(attrs, "send", send);
    }
    if action.accept.is_none() && action.send.is_none() {
        let name = action.name.to_ascii_lowercase();
        if name == "send" || name == "accept" {
            attrs.insert("actionKind".to_string(), serde_json::json!(name));
            if !action.type_name.trim().is_empty() {
                attrs.insert(
                    "payloadType".to_string(),
                    serde_json::json!(action.type_name.as_str()),
                );
            }
        }
    }
}

pub(super) fn insert_transition_accept_attrs(
    attrs: &mut HashMap<String, serde_json::Value>,
    accept: &TransitionAccept,
) {
    match accept {
        TransitionAccept::Payload(clause, _via) => {
            insert_payload_clause_attrs(attrs, "accept", clause);
        }
        TransitionAccept::Shorthand(expr, _via) => {
            attrs.insert(
                "acceptExpression".to_string(),
                serde_json::json!(expressions::expression_to_debug_string(expr)),
            );
        }
    }
}
