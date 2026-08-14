//! Maps parser payload clauses onto semantic graph attributes.

use std::collections::HashMap;

use sysml_v2_parser::ast::{ActionUsage, PayloadClause, TransitionAccept, TriggerKind};
use url::Url;

use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::NodeId;

use super::expressions;

/// Authored payload/accept type-reference display text, kept off the legacy attribute map.
/// See `DeclaredSemanticFacts::payload_type_reference`/`accept_type_reference` for why these
/// stay separate from `relationships.typing`.
#[derive(Debug, Clone, Default)]
pub(super) struct PayloadTypeRefs {
    pub payload_type: Option<String>,
    pub accept_type: Option<String>,
}

/// Applies a [`PayloadTypeRefs`] onto the already-created node's declared facts. Must run after
/// `add_node_and_recurse` so the node exists.
pub(super) fn apply_payload_type_refs(
    g: &mut SemanticGraph,
    uri: &Url,
    qualified: &str,
    refs: &PayloadTypeRefs,
) {
    if refs.payload_type.is_none() && refs.accept_type.is_none() {
        return;
    }
    let node_id = NodeId::new(uri, qualified);
    if let Some(node) = g.get_node_mut(&node_id) {
        if let Some(ref payload_type) = refs.payload_type {
            node.declared_facts.payload_type_reference = Some(payload_type.clone());
        }
        if let Some(ref accept_type) = refs.accept_type {
            node.declared_facts.accept_type_reference = Some(accept_type.clone());
        }
    }
}

pub(super) fn insert_payload_clause_attrs(
    attrs: &mut HashMap<String, serde_json::Value>,
    kind: &str,
    clause: &PayloadClause,
) -> PayloadTypeRefs {
    attrs.insert("actionKind".to_string(), serde_json::json!(kind));
    attrs.insert("payloadName".to_string(), serde_json::json!(&clause.name));
    let mut refs = PayloadTypeRefs::default();
    if let Some(ref type_name) = clause.type_name {
        refs.payload_type = Some(type_name.clone());
        if kind == "accept" {
            attrs.insert("acceptName".to_string(), serde_json::json!(&clause.name));
            refs.accept_type = Some(type_name.clone());
        }
    }
    refs
}

pub(super) fn insert_action_payload_attrs(
    attrs: &mut HashMap<String, serde_json::Value>,
    action: &ActionUsage,
) -> PayloadTypeRefs {
    // Preserves original last-write-wins order: `send`'s attrs (if present) were inserted after
    // `accept`'s into the same map, so `send`'s payload_type takes precedence when both clauses
    // are present (an unusual but not-rejected combination).
    let mut refs = PayloadTypeRefs::default();
    if let Some(ref accept) = action.accept {
        refs = insert_payload_clause_attrs(attrs, "accept", accept);
    }
    if let Some(ref send) = action.send {
        let send_refs = insert_payload_clause_attrs(attrs, "send", send);
        if send_refs.payload_type.is_some() {
            refs.payload_type = send_refs.payload_type;
        }
        if send_refs.accept_type.is_some() {
            refs.accept_type = send_refs.accept_type;
        }
    }
    if action.accept.is_none() && action.send.is_none() {
        let name = action.name.to_ascii_lowercase();
        if name == "send" || name == "accept" {
            attrs.insert("actionKind".to_string(), serde_json::json!(name));
            if !action.type_name.trim().is_empty() {
                refs.payload_type = Some(action.type_name.clone());
            }
        }
    }
    refs
}

pub(super) fn insert_transition_accept_attrs(
    attrs: &mut HashMap<String, serde_json::Value>,
    accept: &TransitionAccept,
) -> PayloadTypeRefs {
    match accept {
        TransitionAccept::Payload(clause, _via) => {
            return insert_payload_clause_attrs(attrs, "accept", clause);
        }
        TransitionAccept::Shorthand(expr, _via) => {
            attrs.insert(
                "acceptExpression".to_string(),
                serde_json::json!(expressions::expression_to_debug_string(expr)),
            );
        }
        // `accept at/when/after <expr>` (§6 G8) -- a time/condition trigger rather than a
        // payload signal; OMG spec Annex `5-State-based Behavior-1.sysml`.
        TransitionAccept::TimeTrigger(trigger_kind, expr) => {
            attrs.insert(
                "triggerKind".to_string(),
                serde_json::json!(match trigger_kind {
                    TriggerKind::At => "at",
                    TriggerKind::When => "when",
                    TriggerKind::After => "after",
                }),
            );
            attrs.insert(
                "acceptExpression".to_string(),
                serde_json::json!(expressions::expression_to_debug_string(expr)),
            );
        }
    }
    PayloadTypeRefs::default()
}
