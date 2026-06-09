use std::collections::HashMap;

use sysml_v2_parser::ast::{
    ActorRedefinitionAssignment, ActorUsage, FirstSuccession, IncludeUseCase, RefRedefinition,
    ThenAction, ThenDone, ThenUseCaseUsage, UseCaseDefBody,
};
use url::Url;

use super::{add_node_and_recurse, qualified_name_for_node};
use crate::semantic::ast_util::span_to_range;
use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::{NodeId, RelationshipKind};
use crate::semantic::relationships::{add_edge_if_both_exist, add_typing_edge_if_exists};

/// Tracks `first` / `then` succession steps for use-case and verification bodies.
pub(super) struct CaseSuccessionChain {
    previous: Option<String>,
}

impl CaseSuccessionChain {
    pub fn new() -> Self {
        Self { previous: None }
    }

    pub fn materialize_first_succession(
        &mut self,
        g: &mut SemanticGraph,
        uri: &Url,
        parent_id: &NodeId,
        succession: &FirstSuccession,
        span: &sysml_v2_parser::Span,
    ) {
        let target_name = &succession.target;
        let qualified = qualified_name_for_node(
            g,
            uri,
            Some(parent_id.qualified_name.as_str()),
            target_name,
            "succession",
        );
        let mut attrs = HashMap::new();
        attrs.insert("successionKind".to_string(), serde_json::json!("first"));
        add_node_and_recurse(
            g,
            uri,
            &qualified,
            "succession",
            target_name.clone(),
            span_to_range(span),
            attrs,
            Some(parent_id),
        );
        add_edge_if_both_exist(
            g,
            uri,
            &parent_id.qualified_name,
            &qualified,
            RelationshipKind::Flow,
        );
        if let Some(parent_node) = g.get_node_mut(parent_id) {
            parent_node.attributes.insert(
                "firstSuccessionTarget".to_string(),
                serde_json::json!(target_name.as_str()),
            );
        }
        self.previous = Some(qualified);
    }

    pub fn chain_then_action(
        &mut self,
        g: &mut SemanticGraph,
        uri: &Url,
        container_prefix: Option<&str>,
        parent_id: &NodeId,
        then_action: &sysml_v2_parser::Node<ThenAction>,
    ) {
        let action = &then_action.value.action.value;
        let action_qualified = qualified_name_for_node(
            g,
            uri,
            Some(parent_id.qualified_name.as_str()),
            &action.name,
            "action",
        );
        let mut attrs = HashMap::new();
        attrs.insert(
            "actionType".to_string(),
            serde_json::json!(action.type_name.as_str()),
        );
        add_node_and_recurse(
            g,
            uri,
            &action_qualified,
            "action",
            action.name.clone(),
            span_to_range(&then_action.span),
            attrs,
            Some(parent_id),
        );
        if !action.type_name.is_empty() {
            add_typing_edge_if_exists(
                g,
                uri,
                &action_qualified,
                action.type_name.as_str(),
                container_prefix,
            );
        }
        add_edge_if_both_exist(
            g,
            uri,
            &parent_id.qualified_name,
            &action_qualified,
            RelationshipKind::Perform,
        );
        if let Some(previous_step) = self.previous.as_ref() {
            add_edge_if_both_exist(
                g,
                uri,
                previous_step,
                &action_qualified,
                RelationshipKind::Flow,
            );
        }
        self.previous = Some(action_qualified);
    }

    pub fn chain_then_done(
        &mut self,
        g: &mut SemanticGraph,
        uri: &Url,
        parent_id: &NodeId,
        done: &sysml_v2_parser::Node<ThenDone>,
    ) {
        let qualified = qualified_name_for_node(
            g,
            uri,
            Some(parent_id.qualified_name.as_str()),
            "_verdict",
            "verdict",
        );
        add_node_and_recurse(
            g,
            uri,
            &qualified,
            "verdict",
            "done".to_string(),
            span_to_range(&done.span),
            HashMap::new(),
            Some(parent_id),
        );
        if let Some(previous_step) = self.previous.as_ref() {
            add_edge_if_both_exist(
                g,
                uri,
                previous_step,
                &qualified,
                RelationshipKind::Flow,
            );
        }
        self.previous = Some(qualified);
    }

    pub fn chain_then_use_case(
        &mut self,
        g: &mut SemanticGraph,
        uri: &Url,
        parent_id: &NodeId,
        then_use_case: &ThenUseCaseUsage,
        span: &sysml_v2_parser::Span,
        container_prefix: Option<&str>,
    ) {
        let use_case = &then_use_case.use_case;
        let name = &use_case.value.name;
        let qualified = qualified_name_for_node(
            g,
            uri,
            Some(parent_id.qualified_name.as_str()),
            name,
            "use case",
        );
        let mut attrs = HashMap::new();
        if let Some(ref typing) = use_case.value.type_name {
            attrs.insert("useCaseType".to_string(), serde_json::json!(typing));
        }
        attrs.insert("isThen".to_string(), serde_json::json!(true));
        add_node_and_recurse(
            g,
            uri,
            &qualified,
            "use case",
            name.clone(),
            span_to_range(span),
            attrs,
            Some(parent_id),
        );
        let node_id = NodeId::new(uri, &qualified);
        if let Some(ref typing) = use_case.value.type_name {
            add_typing_edge_if_exists(g, uri, &qualified, typing, container_prefix);
        }
        if let UseCaseDefBody::Brace { elements } = &use_case.value.body {
            build_from_use_case_body(elements, uri, Some(&qualified), &node_id, g);
        }
        let flow_from = self
            .previous
            .as_deref()
            .unwrap_or(parent_id.qualified_name.as_str());
        add_edge_if_both_exist(g, uri, flow_from, &qualified, RelationshipKind::Flow);
        self.previous = Some(qualified);
    }
}

pub(super) fn add_include_use_case_node(
    g: &mut SemanticGraph,
    uri: &Url,
    parent_id: &NodeId,
    include: &IncludeUseCase,
    span: crate::semantic::text_span::TextRange,
    container_prefix: Option<&str>,
) {
    let qualified = qualified_name_for_node(
        g,
        uri,
        Some(parent_id.qualified_name.as_str()),
        &include.name,
        "include use case",
    );
    let mut attrs = HashMap::new();
    attrs.insert(
        "includeTarget".to_string(),
        serde_json::json!(include.name.as_str()),
    );
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "include use case",
        include.name.clone(),
        span,
        attrs,
        Some(parent_id),
    );
    add_typing_edge_if_exists(g, uri, &qualified, &include.name, container_prefix);
    let include_id = NodeId::new(uri, &qualified);
    if let UseCaseDefBody::Brace { elements } = &include.body {
        build_from_use_case_body(elements, uri, Some(&qualified), &include_id, g);
    }
}

pub(super) fn mark_subject_ref(g: &mut SemanticGraph, parent_id: &NodeId) {
    if let Some(parent_node) = g.get_node_mut(parent_id) {
        parent_node
            .attributes
            .insert("hasSubject".to_string(), serde_json::json!(true));
        parent_node
            .attributes
            .insert("subjectRef".to_string(), serde_json::json!(true));
    }
}

pub(super) fn add_actor_usage_node(
    g: &mut SemanticGraph,
    uri: &Url,
    parent_id: &NodeId,
    actor: &ActorUsage,
    span: &sysml_v2_parser::Span,
    container_prefix: Option<&str>,
) {
    let name = &actor.name;
    let qualified = qualified_name_for_node(
        g,
        uri,
        Some(parent_id.qualified_name.as_str()),
        name,
        "actor",
    );
    let range = span_to_range(span);
    let mut attrs = HashMap::new();
    attrs.insert(
        "actorType".to_string(),
        serde_json::json!(&actor.type_name),
    );
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "actor",
        name.clone(),
        range,
        attrs,
        Some(parent_id),
    );
    add_typing_edge_if_exists(g, uri, &qualified, &actor.type_name, container_prefix);
}

pub(super) fn add_actor_redefinition_assignment_node(
    g: &mut SemanticGraph,
    uri: &Url,
    parent_id: &NodeId,
    assignment: &ActorRedefinitionAssignment,
    span: &sysml_v2_parser::Span,
) {
    let qualified = qualified_name_for_node(
        g,
        uri,
        Some(parent_id.qualified_name.as_str()),
        &assignment.name,
        "actor redefinition",
    );
    let mut attrs = HashMap::new();
    attrs.insert("rhs".to_string(), serde_json::json!(assignment.rhs.as_str()));
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "actor redefinition",
        assignment.name.clone(),
        span_to_range(span),
        attrs,
        Some(parent_id),
    );
}

pub(super) fn add_ref_redefinition_node(
    g: &mut SemanticGraph,
    uri: &Url,
    parent_id: &NodeId,
    redef: &RefRedefinition,
    span: &sysml_v2_parser::Span,
) {
    let qualified = qualified_name_for_node(
        g,
        uri,
        Some(parent_id.qualified_name.as_str()),
        &redef.name,
        "ref redefinition",
    );
    let mut attrs = HashMap::new();
    attrs.insert("body".to_string(), serde_json::json!(redef.body.as_str()));
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "ref redefinition",
        redef.name.clone(),
        span_to_range(span),
        attrs,
        Some(parent_id),
    );
}

/// Wire case-body elements shared across use-case, analysis, and verification walkers.
pub(super) fn wire_extended_case_body_element(
    g: &mut SemanticGraph,
    uri: &Url,
    parent_id: &NodeId,
    node: &sysml_v2_parser::Node<sysml_v2_parser::ast::UseCaseDefBodyElement>,
    container_prefix: Option<&str>,
    chain: Option<&mut CaseSuccessionChain>,
) -> bool {
    use sysml_v2_parser::ast::UseCaseDefBodyElement as UCBE;
    match &node.value {
        UCBE::SubjectRef(_) => {
            mark_subject_ref(g, parent_id);
            true
        }
        UCBE::ActorUsage(actor_node) => {
            add_actor_usage_node(
                g,
                uri,
                parent_id,
                &actor_node.value,
                &actor_node.span,
                container_prefix,
            );
            false
        }
        UCBE::ActorRedefinitionAssignment(assignment) => {
            add_actor_redefinition_assignment_node(
                g,
                uri,
                parent_id,
                &assignment.value,
                &assignment.span,
            );
            false
        }
        UCBE::RefRedefinition(redef) => {
            add_ref_redefinition_node(g, uri, parent_id, &redef.value, &redef.span);
            false
        }
        UCBE::FirstSuccession(succession) => {
            if let Some(chain) = chain {
                chain.materialize_first_succession(
                    g,
                    uri,
                    parent_id,
                    &succession.value,
                    &succession.span,
                );
            }
            false
        }
        UCBE::ThenUseCaseUsage(then_use_case) => {
            if let Some(chain) = chain {
                chain.chain_then_use_case(
                    g,
                    uri,
                    parent_id,
                    &then_use_case.value,
                    &then_use_case.span,
                    container_prefix,
                );
            }
            false
        }
        _ => false,
    }
}

pub(super) fn build_from_use_case_body(
    elements: &[sysml_v2_parser::Node<sysml_v2_parser::ast::UseCaseDefBodyElement>],
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    g: &mut SemanticGraph,
) {
    use sysml_v2_parser::ast::UseCaseDefBodyElement as UCBE;
    let mut chain = CaseSuccessionChain::new();
    let mut then_action_count = 0usize;
    for node in elements {
        if wire_extended_case_body_element(
            g,
            uri,
            parent_id,
            node,
            container_prefix,
            Some(&mut chain),
        ) {
            continue;
        }
        match &node.value {
            UCBE::ActorUsage(_)
            | UCBE::ActorRedefinitionAssignment(_)
            | UCBE::RefRedefinition(_)
            | UCBE::FirstSuccession(_)
            | UCBE::ThenUseCaseUsage(_)
            | UCBE::SubjectRef(_) => continue,
            UCBE::ThenAction(then_action) => {
                then_action_count += 1;
                chain.chain_then_action(g, uri, container_prefix, parent_id, then_action);
            }
            UCBE::ThenDone(done) => {
                chain.chain_then_done(g, uri, parent_id, done);
            }
            UCBE::SubjectDecl(sd) => {
                let name = sd.value.name.clone();
                let qualified = qualified_name_for_node(
                    g,
                    uri,
                    Some(parent_id.qualified_name.as_str()),
                    &name,
                    "subject",
                );
                let mut attrs = HashMap::new();
                attrs.insert(
                    "subjectType".to_string(),
                    serde_json::json!(sd.value.type_name.as_str()),
                );
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "subject",
                    name,
                    span_to_range(&sd.span),
                    attrs,
                    Some(parent_id),
                );
                add_typing_edge_if_exists(
                    g,
                    uri,
                    &qualified,
                    sd.value.type_name.as_str(),
                    container_prefix,
                );
            }
            UCBE::Objective(obj) => {
                let objective_name = &obj.value.requirement.value.name;
                let qualified = qualified_name_for_node(
                    g,
                    uri,
                    Some(parent_id.qualified_name.as_str()),
                    objective_name,
                    "objective",
                );
                let mut attrs = HashMap::new();
                attrs.insert(
                    "objectiveBindingKind".to_string(),
                    serde_json::json!("case_result_default"),
                );
                if let Some(type_name) = obj.value.requirement.value.type_name.as_ref() {
                    attrs.insert("objectiveType".to_string(), serde_json::json!(type_name));
                }
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "objective",
                    objective_name.clone(),
                    span_to_range(&obj.span),
                    attrs,
                    Some(parent_id),
                );
                if let Some(type_name) = obj.value.requirement.value.type_name.as_ref() {
                    add_typing_edge_if_exists(g, uri, &qualified, type_name, container_prefix);
                }
            }
            UCBE::IncludeUseCase(include_node) => {
                add_include_use_case_node(
                    g,
                    uri,
                    parent_id,
                    &include_node.value,
                    span_to_range(&include_node.span),
                    container_prefix,
                );
            }
            UCBE::ThenIncludeUseCase(then_include) => {
                add_include_use_case_node(
                    g,
                    uri,
                    parent_id,
                    &then_include.value.include.value,
                    span_to_range(&then_include.span),
                    container_prefix,
                );
            }
            UCBE::Error(_) | UCBE::Doc(_) | UCBE::Other(_) | UCBE::Annotation(_) => {}
            _ => {}
        }
    }
    if then_action_count > 0 {
        if let Some(parent_node) = g.get_node_mut(parent_id) {
            parent_node.attributes.insert(
                "thenActionCount".to_string(),
                serde_json::json!(then_action_count),
            );
        }
    }
}
