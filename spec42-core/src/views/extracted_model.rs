//! Activity and sequence diagram extraction for sysml/model response.

use crate::syntax::ast_util::identification_name;
use serde::Serialize;
use sysml_v2_parser::ast::{
    ActionDefBody, ActionDefBodyElement, PackageBody, PackageBodyElement, PartDefBody,
    PartDefBodyElement, PartUsageBody, PartUsageBodyElement, RootElement,
};
use sysml_v2_parser::{RootNamespace, Span};

fn expr_to_string(n: &sysml_v2_parser::Node<sysml_v2_parser::Expression>) -> String {
    use sysml_v2_parser::Expression;
    match &n.value {
        Expression::FeatureRef(s) => s.clone(),
        Expression::MemberAccess(base, member) => {
            let b = expr_to_string(base);
            if b.is_empty() {
                member.clone()
            } else {
                format!("{b}.{member}")
            }
        }
        Expression::Index { base, index } => {
            let b = expr_to_string(base);
            let i = expr_to_string(index);
            if b.is_empty() {
                String::new()
            } else if i.is_empty() {
                format!("{b}#()")
            } else {
                format!("{b}#({i})")
            }
        }
        Expression::Bracket(inner) => expr_to_string(inner),
        Expression::LiteralString(s) => s.clone(),
        Expression::LiteralInteger(i) => i.to_string(),
        Expression::LiteralReal(s) => s.clone(),
        Expression::LiteralBoolean(b) => b.to_string(),
        Expression::LiteralWithUnit { value, unit } => {
            let v = expr_to_string(value);
            let u = expr_to_string(unit);
            if u.is_empty() {
                v
            } else {
                format!("{v} [{u}]")
            }
        }
        Expression::BinaryOp { op, left, right } => {
            format!(
                "({} {} {})",
                expr_to_string(left),
                op,
                expr_to_string(right)
            )
        }
        Expression::UnaryOp { op, operand } => format!("({}{})", op, expr_to_string(operand)),
        Expression::Tuple(items) => items
            .iter()
            .map(expr_to_string)
            .collect::<Vec<_>>()
            .join(", "),
        Expression::Null => String::new(),
    }
}

/// Position DTO for JSON (matches vscode sysmlModelTypes)
#[derive(Debug, Clone, Serialize)]
pub struct PositionDto {
    pub line: u32,
    pub character: u32,
}

/// Range DTO for JSON
#[derive(Debug, Clone, Serialize)]
pub struct RangeDto {
    pub start: PositionDto,
    pub end: PositionDto,
}

fn span_to_range_dto(span: &Span) -> RangeDto {
    let (start_line, start_char, end_line, end_char) = span.to_lsp_range();
    RangeDto {
        start: PositionDto {
            line: start_line,
            character: start_char,
        },
        end: PositionDto {
            line: end_line,
            character: end_char,
        },
    }
}

fn normalized_last_segment(value: &str) -> String {
    let normalized = value.replace('.', "::");
    normalized
        .rsplit("::")
        .find(|segment| !segment.trim().is_empty())
        .unwrap_or("")
        .trim()
        .to_string()
}

fn is_sequence_type_name(value: &str, expected: &str) -> bool {
    normalized_last_segment(value).eq_ignore_ascii_case(expected)
}

// ---------------------------------------------------------------------------
// Activity diagrams
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityDiagramDto {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub package_path: String,
    pub source_kind: String,
    pub actions: Vec<ActivityActionDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interface: Option<ActivityInterfaceDto>,
    pub decisions: Vec<DecisionNodeDto>,
    pub flows: Vec<ControlFlowDto>,
    pub states: Vec<ActivityStateDto>,
    pub range: RangeDto,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityActionDto {
    pub name: String,
    #[serde(rename = "type")]
    pub action_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<RangeDto>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityInterfaceDto {
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DecisionNodeDto {
    pub name: String,
    pub condition: String,
    pub branches: Vec<BranchDto>,
    pub range: RangeDto,
}

#[derive(Debug, Clone, Serialize)]
pub struct BranchDto {
    pub condition: String,
    pub target: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControlFlowDto {
    pub from: String,
    pub to: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guard: Option<String>,
    pub range: RangeDto,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityStateDto {
    pub name: String,
    #[serde(rename = "type")]
    pub state_type: String,
    pub range: RangeDto,
}

// ---------------------------------------------------------------------------
// Sequence diagrams
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SequenceDiagramDto {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub package_path: String,
    pub source_kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    pub lifelines: Vec<SequenceLifelineDto>,
    pub messages: Vec<SequenceMessageDto>,
    pub activations: Vec<SequenceActivationDto>,
    pub fragments: Vec<SequenceFragmentDto>,
    pub range: RangeDto,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SequenceLifelineDto {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    pub range: RangeDto,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SequenceMessageDto {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub from: String,
    pub to: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub order: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    pub range: RangeDto,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SequenceActivationDto {
    pub id: String,
    pub name: String,
    pub on_lifeline: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    pub range: RangeDto,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SequenceOperandDto {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guard: Option<String>,
    pub message_ids: Vec<String>,
    pub fragments: Vec<SequenceFragmentDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    pub range: RangeDto,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SequenceFragmentDto {
    pub id: String,
    pub name: String,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guard: Option<String>,
    pub message_ids: Vec<String>,
    pub operands: Vec<SequenceOperandDto>,
    pub fragments: Vec<SequenceFragmentDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    pub range: RangeDto,
}

#[derive(Debug, Default)]
struct SequenceBuildState {
    messages: Vec<SequenceMessageDto>,
    activations: Vec<SequenceActivationDto>,
    next_order: usize,
}

fn normalize_sequence_reference(value: &str, local_ids: &std::collections::HashMap<String, String>) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    if let Some(resolved) = local_ids.get(trimmed) {
        return resolved.clone();
    }
    normalized_last_segment(trimmed)
}

fn normalize_sequence_fragment_references(
    fragment: &mut SequenceFragmentDto,
    local_ids: &std::collections::HashMap<String, String>,
) {
    if let Some(target_ref) = fragment.target_ref.as_mut() {
        *target_ref = normalize_sequence_reference(target_ref, local_ids);
    }
    for operand in &mut fragment.operands {
        for nested in &mut operand.fragments {
            normalize_sequence_fragment_references(nested, local_ids);
        }
    }
    for nested in &mut fragment.fragments {
        normalize_sequence_fragment_references(nested, local_ids);
    }
}

fn normalize_sequence_diagram_references(diagram: &mut SequenceDiagramDto) {
    let mut local_ids = std::collections::HashMap::new();

    for lifeline in &diagram.lifelines {
        local_ids.insert(lifeline.name.clone(), lifeline.id.clone());
        local_ids.insert(normalized_last_segment(&lifeline.id), lifeline.id.clone());
    }
    for message in &diagram.messages {
        local_ids.insert(message.name.clone(), message.id.clone());
        local_ids.insert(normalized_last_segment(&message.id), message.id.clone());
    }

    for message in &mut diagram.messages {
        message.from = normalize_sequence_reference(&message.from, &local_ids);
        message.to = normalize_sequence_reference(&message.to, &local_ids);
    }
    for activation in &mut diagram.activations {
        activation.on_lifeline = normalize_sequence_reference(&activation.on_lifeline, &local_ids);
        if let Some(start_message) = activation.start_message.as_mut() {
            *start_message = normalize_sequence_reference(start_message, &local_ids);
        }
        if let Some(finish_message) = activation.finish_message.as_mut() {
            *finish_message = normalize_sequence_reference(finish_message, &local_ids);
        }
    }
    for fragment in &mut diagram.fragments {
        normalize_sequence_fragment_references(fragment, &local_ids);
    }
}

// ---------------------------------------------------------------------------
// Extraction
// ---------------------------------------------------------------------------

fn join_segments(segments: &[String]) -> String {
    segments
        .iter()
        .filter(|segment| !segment.trim().is_empty())
        .cloned()
        .collect::<Vec<_>>()
        .join("::")
}

fn with_segment(base: &[String], segment: String) -> Vec<String> {
    if segment.trim().is_empty() {
        base.to_vec()
    } else {
        let mut next = base.to_vec();
        next.push(segment);
        next
    }
}

fn activity_diagram_id(qualified_segments: &[String], source_kind: &str) -> String {
    let qualified = join_segments(qualified_segments);
    if qualified.is_empty() {
        source_kind.to_string()
    } else {
        format!("{qualified}::{source_kind}")
    }
}

fn package_path_from_segments(package_segments: &[String]) -> String {
    join_segments(package_segments)
}

fn extract_performer_diagram_from_performs(
    name: &str,
    qualified_segments: &[String],
    package_segments: &[String],
    performs: &[sysml_v2_parser::Node<sysml_v2_parser::ast::Perform>],
    range: RangeDto,
) -> Option<ActivityDiagramDto> {
    if performs.is_empty() {
        return None;
    }

    let actions = performs
        .iter()
        .enumerate()
        .map(|(index, perform)| {
            let perform_name = if perform.value.action_name.trim().is_empty() {
                perform
                    .value
                    .type_name
                    .clone()
                    .unwrap_or_else(|| format!("perform_{}", index + 1))
            } else {
                perform.value.action_name.clone()
            };
            ActivityActionDto {
                name: perform_name,
                action_type: "action".to_string(),
                kind: Some("perform".to_string()),
                inputs: None,
                outputs: None,
                range: Some(span_to_range_dto(&perform.span)),
            }
        })
        .collect::<Vec<_>>();

    let flows = actions
        .windows(2)
        .enumerate()
        .map(|(index, window)| ControlFlowDto {
            from: window[0].name.clone(),
            to: window[1].name.clone(),
            condition: None,
            guard: Some("perform-sequence".to_string()),
            range: performs
                .get(index + 1)
                .map(|perform| span_to_range_dto(&perform.span))
                .unwrap_or_else(|| range.clone()),
        })
        .collect::<Vec<_>>();

    Some(ActivityDiagramDto {
        id: activity_diagram_id(qualified_segments, "performer"),
        name: if name.trim().is_empty() {
            "performer".to_string()
        } else {
            name.to_string()
        },
        package_path: package_path_from_segments(package_segments),
        source_kind: "performer".to_string(),
        actions,
        interface: None,
        decisions: vec![],
        flows,
        states: vec![],
        range,
    })
}

fn extract_performer_diagram_from_part_def(
    node: &sysml_v2_parser::Node<sysml_v2_parser::ast::PartDef>,
    package_segments: &[String],
    parent_segments: &[String],
) -> Option<ActivityDiagramDto> {
    let name = identification_name(&node.identification);
    let qualified_segments = with_segment(parent_segments, name.clone());
    let elements = match &node.body {
        PartDefBody::Brace { elements } => elements,
        PartDefBody::Semicolon => return None,
    };
    let performs = elements
        .iter()
        .filter_map(|element| match &element.value {
            PartDefBodyElement::Perform(perform) => Some(perform.clone()),
            _ => None,
        })
        .collect::<Vec<_>>();
    extract_performer_diagram_from_performs(
        &name,
        &qualified_segments,
        package_segments,
        &performs,
        span_to_range_dto(&node.span),
    )
}

fn extract_performer_diagram_from_part_usage(
    node: &sysml_v2_parser::Node<sysml_v2_parser::ast::PartUsage>,
    package_segments: &[String],
    parent_segments: &[String],
) -> Option<ActivityDiagramDto> {
    let name = node.value.name.clone();
    let qualified_segments = with_segment(parent_segments, name.clone());
    let elements = match &node.body {
        PartUsageBody::Brace { elements } => elements,
        PartUsageBody::Semicolon => return None,
    };
    let performs = elements
        .iter()
        .filter_map(|element| match &element.value {
            PartUsageBodyElement::Perform(perform) => Some(perform.clone()),
            _ => None,
        })
        .collect::<Vec<_>>();
    extract_performer_diagram_from_performs(
        &name,
        &qualified_segments,
        package_segments,
        &performs,
        span_to_range_dto(&node.span),
    )
}

fn collect_activity_diagrams_from_part_def_body(
    elements: &[sysml_v2_parser::Node<PartDefBodyElement>],
    package_segments: &[String],
    parent_segments: &[String],
    out: &mut Vec<ActivityDiagramDto>,
) {
    for element in elements {
        if let PartDefBodyElement::PartUsage(part_usage) = &element.value {
            collect_activity_diagrams_from_part_usage(
                part_usage,
                package_segments,
                parent_segments,
                out,
            );
        }
    }
}

fn collect_activity_diagrams_from_part_usage_body(
    elements: &[sysml_v2_parser::Node<PartUsageBodyElement>],
    package_segments: &[String],
    parent_segments: &[String],
    out: &mut Vec<ActivityDiagramDto>,
) {
    for element in elements {
        if let PartUsageBodyElement::PartUsage(part_usage) = &element.value {
            collect_activity_diagrams_from_part_usage(
                part_usage,
                package_segments,
                parent_segments,
                out,
            );
        }
    }
}

fn collect_activity_diagrams_from_part_def(
    node: &sysml_v2_parser::Node<sysml_v2_parser::ast::PartDef>,
    package_segments: &[String],
    parent_segments: &[String],
    out: &mut Vec<ActivityDiagramDto>,
) {
    let name = identification_name(&node.identification);
    let qualified_segments = with_segment(parent_segments, name.clone());

    if let Some(diagram) =
        extract_performer_diagram_from_part_def(node, package_segments, parent_segments)
    {
        out.push(diagram);
    }

    if let PartDefBody::Brace { elements } = &node.body {
        collect_activity_diagrams_from_part_def_body(
            elements,
            package_segments,
            &qualified_segments,
            out,
        );
    }
}

fn collect_activity_diagrams_from_part_usage(
    node: &sysml_v2_parser::Node<sysml_v2_parser::ast::PartUsage>,
    package_segments: &[String],
    parent_segments: &[String],
    out: &mut Vec<ActivityDiagramDto>,
) {
    let name = node.value.name.clone();
    let qualified_segments = with_segment(parent_segments, name.clone());

    if let Some(diagram) =
        extract_performer_diagram_from_part_usage(node, package_segments, parent_segments)
    {
        out.push(diagram);
    }

    if let PartUsageBody::Brace { elements } = &node.body {
        collect_activity_diagrams_from_part_usage_body(
            elements,
            package_segments,
            &qualified_segments,
            out,
        );
    }
}

fn collect_activity_diagrams_from_package_elements(
    elements: &[sysml_v2_parser::Node<PackageBodyElement>],
    package_segments: &[String],
    parent_segments: &[String],
    out: &mut Vec<ActivityDiagramDto>,
) {
    use sysml_v2_parser::ast::PackageBodyElement as PBE;
    for node in elements {
        match &node.value {
            PBE::ActionDef(action) => out.push(extract_activity_from_action(
                action,
                package_segments,
                parent_segments,
            )),
            PBE::PartDef(part_def) => {
                collect_activity_diagrams_from_part_def(
                    part_def,
                    package_segments,
                    parent_segments,
                    out,
                );
            }
            PBE::PartUsage(part_usage) => {
                collect_activity_diagrams_from_part_usage(
                    part_usage,
                    package_segments,
                    parent_segments,
                    out,
                );
            }
            PBE::Package(package) => {
                if let PackageBody::Brace { elements: inner } = &package.body {
                    let package_name = identification_name(&package.identification);
                    let next_package_segments =
                        with_segment(package_segments, package_name.clone());
                    let next_parent_segments = with_segment(parent_segments, package_name);
                    collect_activity_diagrams_from_package_elements(
                        inner,
                        &next_package_segments,
                        &next_parent_segments,
                        out,
                    );
                }
            }
            PBE::LibraryPackage(package) => {
                if let PackageBody::Brace { elements: inner } = &package.body {
                    let package_name = identification_name(&package.identification);
                    let next_package_segments =
                        with_segment(package_segments, package_name.clone());
                    let next_parent_segments = with_segment(parent_segments, package_name);
                    collect_activity_diagrams_from_package_elements(
                        inner,
                        &next_package_segments,
                        &next_parent_segments,
                        out,
                    );
                }
            }
            _ => {}
        }
    }
}

/// Extracts activity diagrams from ActionDef nodes.
/// Each ActionDef or performer context becomes one ActivityDiagramDto.
pub fn extract_activity_diagrams(root: &RootNamespace) -> Vec<ActivityDiagramDto> {
    let mut out = Vec::new();
    for node in &root.elements {
        match &node.value {
            RootElement::Package(package) => {
                if let PackageBody::Brace { elements } = &package.body {
                    let package_name = identification_name(&package.identification);
                    let package_segments = if package_name.is_empty() {
                        vec![]
                    } else {
                        vec![package_name]
                    };
                    collect_activity_diagrams_from_package_elements(
                        elements,
                        &package_segments,
                        &package_segments,
                        &mut out,
                    );
                }
            }
            RootElement::Namespace(namespace) => {
                if let PackageBody::Brace { elements } = &namespace.body {
                    let namespace_name = identification_name(&namespace.identification);
                    let package_segments = if namespace_name.is_empty() {
                        vec![]
                    } else {
                        vec![namespace_name]
                    };
                    collect_activity_diagrams_from_package_elements(
                        elements,
                        &package_segments,
                        &package_segments,
                        &mut out,
                    );
                }
            }
            RootElement::LibraryPackage(package) => {
                if let PackageBody::Brace { elements } = &package.body {
                    let package_name = identification_name(&package.identification);
                    let package_segments = if package_name.is_empty() {
                        vec![]
                    } else {
                        vec![package_name]
                    };
                    collect_activity_diagrams_from_package_elements(
                        elements,
                        &package_segments,
                        &package_segments,
                        &mut out,
                    );
                }
            }
            RootElement::Import(_) => {}
        }
    }
    out
}

fn sequence_diagram_id(qualified_segments: &[String], source_kind: &str) -> String {
    let qualified = join_segments(qualified_segments);
    if qualified.is_empty() {
        source_kind.to_string()
    } else {
        format!("{qualified}::{source_kind}")
    }
}

fn extract_attribute_value_from_part_usage(
    node: &sysml_v2_parser::Node<sysml_v2_parser::ast::PartUsage>,
    wanted_name: &str,
) -> Option<String> {
    let PartUsageBody::Brace { elements } = &node.body else {
        return None;
    };
    elements.iter().find_map(|element| match &element.value {
        PartUsageBodyElement::AttributeUsage(attribute)
            if attribute.value.name.eq_ignore_ascii_case(wanted_name) =>
        {
            attribute
                .value
                .value
                .as_ref()
                .map(expr_to_string)
                .map(|value| value.trim_matches('"').to_string())
        }
        _ => None,
    })
}

fn extract_ref_value_from_part_usage(
    node: &sysml_v2_parser::Node<sysml_v2_parser::ast::PartUsage>,
    wanted_name: &str,
) -> Option<String> {
    let PartUsageBody::Brace { elements } = &node.body else {
        return None;
    };
    elements.iter().find_map(|element| match &element.value {
        PartUsageBodyElement::Ref(reference) if reference.value.name.eq_ignore_ascii_case(wanted_name) => {
            reference
                .value
                .value
                .as_ref()
                .map(expr_to_string)
                .map(|value| value.trim_matches('"').to_string())
        }
        _ => None,
    })
}

fn part_usage_message_kind(node: &sysml_v2_parser::Node<sysml_v2_parser::ast::PartUsage>) -> Option<&'static str> {
    match normalized_last_segment(&node.value.type_name).to_ascii_lowercase().as_str() {
        "synchronouscall" => Some("sync"),
        "asynchronousmessage" => Some("async"),
        "returnmessage" => Some("return"),
        "creationmessage" => Some("create"),
        _ => None,
    }
}

fn part_usage_fragment_kind(
    node: &sysml_v2_parser::Node<sysml_v2_parser::ast::PartUsage>,
) -> Option<&'static str> {
    match normalized_last_segment(&node.value.type_name).to_ascii_lowercase().as_str() {
        "altfragment" => Some("alt"),
        "optfragment" => Some("opt"),
        "loopfragment" => Some("loop"),
        "interactionref" => Some("ref"),
        "combinedfragment" => Some("group"),
        _ => None,
    }
}

fn part_usage_is_lifeline(node: &sysml_v2_parser::Node<sysml_v2_parser::ast::PartUsage>) -> bool {
    is_sequence_type_name(&node.value.type_name, "Lifeline")
}

fn part_usage_is_activation(node: &sysml_v2_parser::Node<sysml_v2_parser::ast::PartUsage>) -> bool {
    is_sequence_type_name(&node.value.type_name, "Activation")
}

fn part_usage_is_operand(node: &sysml_v2_parser::Node<sysml_v2_parser::ast::PartUsage>) -> bool {
    is_sequence_type_name(&node.value.type_name, "InteractionOperand")
}

fn part_def_is_sequence_scenario(
    node: &sysml_v2_parser::Node<sysml_v2_parser::ast::PartDef>,
) -> bool {
    node.value
        .specializes
        .as_deref()
        .is_some_and(|value| is_sequence_type_name(value, "InteractionScenario"))
}

fn part_usage_is_sequence_scenario(
    node: &sysml_v2_parser::Node<sysml_v2_parser::ast::PartUsage>,
) -> bool {
    is_sequence_type_name(&node.value.type_name, "InteractionScenario")
}

fn collect_sequence_items_from_part_usage_body(
    elements: &[sysml_v2_parser::Node<PartUsageBodyElement>],
    qualified_segments: &[String],
    state: &mut SequenceBuildState,
) -> (
    Vec<String>,
    Vec<SequenceFragmentDto>,
    Vec<SequenceActivationDto>,
) {
    let mut message_ids = Vec::new();
    let mut fragments = Vec::new();
    let mut activations = Vec::new();

    for element in elements {
        let PartUsageBodyElement::PartUsage(part_usage) = &element.value else {
            continue;
        };
        if let Some(message) = extract_sequence_message(part_usage, qualified_segments, state) {
            message_ids.push(message.id.clone());
            state.messages.push(message);
            continue;
        }
        if let Some(activation) = extract_sequence_activation(part_usage, qualified_segments) {
            activations.push(activation.clone());
            state.activations.push(activation);
            continue;
        }
        if let Some(fragment) = extract_sequence_fragment(part_usage, qualified_segments, state) {
            fragments.push(fragment);
        }
    }

    (message_ids, fragments, activations)
}

fn extract_sequence_message(
    node: &sysml_v2_parser::Node<sysml_v2_parser::ast::PartUsage>,
    parent_segments: &[String],
    state: &mut SequenceBuildState,
) -> Option<SequenceMessageDto> {
    let kind = part_usage_message_kind(node)?;
    let name = node.value.name.clone();
    let qualified_segments = with_segment(parent_segments, name.clone());
    let from = extract_ref_value_from_part_usage(node, "from")
        .or_else(|| extract_ref_value_from_part_usage(node, "source"))?;
    let to = extract_ref_value_from_part_usage(node, "to")
        .or_else(|| extract_ref_value_from_part_usage(node, "target"))?;
    let label = extract_attribute_value_from_part_usage(node, "label")
        .or_else(|| extract_attribute_value_from_part_usage(node, "message"))
        .or_else(|| (!name.trim().is_empty()).then_some(name.clone()));
    state.next_order += 1;
    Some(SequenceMessageDto {
        id: join_segments(&qualified_segments),
        name,
        kind: kind.to_string(),
        from,
        to,
        label,
        order: state.next_order,
        uri: None,
        range: span_to_range_dto(&node.span),
    })
}

fn extract_sequence_activation(
    node: &sysml_v2_parser::Node<sysml_v2_parser::ast::PartUsage>,
    parent_segments: &[String],
) -> Option<SequenceActivationDto> {
    if !part_usage_is_activation(node) {
        return None;
    }
    let name = node.value.name.clone();
    let qualified_segments = with_segment(parent_segments, name.clone());
    let on_lifeline = extract_ref_value_from_part_usage(node, "on")
        .or_else(|| extract_ref_value_from_part_usage(node, "lifeline"))?;
    Some(SequenceActivationDto {
        id: join_segments(&qualified_segments),
        name,
        on_lifeline,
        start_message: extract_ref_value_from_part_usage(node, "startMessage")
            .or_else(|| extract_ref_value_from_part_usage(node, "start")),
        finish_message: extract_ref_value_from_part_usage(node, "finishMessage")
            .or_else(|| extract_ref_value_from_part_usage(node, "finish")),
        uri: None,
        range: span_to_range_dto(&node.span),
    })
}

fn extract_sequence_operand(
    node: &sysml_v2_parser::Node<sysml_v2_parser::ast::PartUsage>,
    parent_segments: &[String],
    state: &mut SequenceBuildState,
) -> Option<SequenceOperandDto> {
    if !part_usage_is_operand(node) {
        return None;
    }
    let name = node.value.name.clone();
    let qualified_segments = with_segment(parent_segments, name.clone());
    let PartUsageBody::Brace { elements } = &node.body else {
        return None;
    };
    let (message_ids, fragments, _activations) =
        collect_sequence_items_from_part_usage_body(elements, &qualified_segments, state);
    Some(SequenceOperandDto {
        id: join_segments(&qualified_segments),
        name,
        guard: extract_attribute_value_from_part_usage(node, "guard")
            .or_else(|| extract_attribute_value_from_part_usage(node, "condition")),
        message_ids,
        fragments,
        uri: None,
        range: span_to_range_dto(&node.span),
    })
}

fn extract_sequence_fragment(
    node: &sysml_v2_parser::Node<sysml_v2_parser::ast::PartUsage>,
    parent_segments: &[String],
    state: &mut SequenceBuildState,
) -> Option<SequenceFragmentDto> {
    let kind = part_usage_fragment_kind(node)?;
    let name = node.value.name.clone();
    let qualified_segments = with_segment(parent_segments, name.clone());
    let guard = extract_attribute_value_from_part_usage(node, "guard")
        .or_else(|| extract_attribute_value_from_part_usage(node, "condition"));
    let target_ref = if kind == "ref" {
        extract_ref_value_from_part_usage(node, "target")
            .or_else(|| extract_ref_value_from_part_usage(node, "interaction"))
    } else {
        None
    };

    let mut message_ids = Vec::new();
    let mut fragments = Vec::new();
    let mut operands = Vec::new();

    if let PartUsageBody::Brace { elements } = &node.body {
        for element in elements {
            let PartUsageBodyElement::PartUsage(part_usage) = &element.value else {
                continue;
            };
            if let Some(operand) = extract_sequence_operand(part_usage, &qualified_segments, state) {
                operands.push(operand);
                continue;
            }
            if let Some(message) = extract_sequence_message(part_usage, &qualified_segments, state) {
                message_ids.push(message.id.clone());
                state.messages.push(message);
                continue;
            }
            if let Some(fragment) = extract_sequence_fragment(part_usage, &qualified_segments, state)
            {
                fragments.push(fragment);
                continue;
            }
            if let Some(activation) = extract_sequence_activation(part_usage, &qualified_segments) {
                state.activations.push(activation);
            }
        }
    }

    Some(SequenceFragmentDto {
        id: join_segments(&qualified_segments),
        name,
        kind: kind.to_string(),
        guard,
        message_ids,
        operands,
        fragments,
        target_ref,
        uri: None,
        range: span_to_range_dto(&node.span),
    })
}

fn extract_sequence_diagram_from_part_def(
    node: &sysml_v2_parser::Node<sysml_v2_parser::ast::PartDef>,
    package_segments: &[String],
    parent_segments: &[String],
) -> Option<SequenceDiagramDto> {
    if !part_def_is_sequence_scenario(node) {
        return None;
    }
    let name = identification_name(&node.identification);
    let qualified_segments = with_segment(parent_segments, name.clone());
    let PartDefBody::Brace { elements } = &node.body else {
        return None;
    };
    let mut lifelines = Vec::new();
    let mut fragments = Vec::new();
    let mut state = SequenceBuildState::default();

    for element in elements {
        let PartDefBodyElement::PartUsage(part_usage) = &element.value else {
            continue;
        };
        if part_usage_is_lifeline(part_usage) {
            lifelines.push(SequenceLifelineDto {
                id: join_segments(&with_segment(&qualified_segments, part_usage.value.name.clone())),
                name: part_usage.value.name.clone(),
                uri: None,
                range: span_to_range_dto(&part_usage.span),
            });
            continue;
        }
        if let Some(message) = extract_sequence_message(part_usage, &qualified_segments, &mut state)
        {
            state.messages.push(message);
            continue;
        }
        if let Some(activation) = extract_sequence_activation(part_usage, &qualified_segments) {
            state.activations.push(activation);
            continue;
        }
        if let Some(fragment) = extract_sequence_fragment(part_usage, &qualified_segments, &mut state)
        {
            fragments.push(fragment);
        }
    }

    let mut diagram = SequenceDiagramDto {
        id: sequence_diagram_id(&qualified_segments, "sequence"),
        name,
        package_path: package_path_from_segments(package_segments),
        source_kind: "partDef".to_string(),
        uri: None,
        lifelines,
        messages: state.messages,
        activations: state.activations,
        fragments,
        range: span_to_range_dto(&node.span),
    };
    normalize_sequence_diagram_references(&mut diagram);
    Some(diagram)
}

fn extract_sequence_diagram_from_part_usage(
    node: &sysml_v2_parser::Node<sysml_v2_parser::ast::PartUsage>,
    package_segments: &[String],
    parent_segments: &[String],
) -> Option<SequenceDiagramDto> {
    if !part_usage_is_sequence_scenario(node) {
        return None;
    }
    let name = node.value.name.clone();
    let qualified_segments = with_segment(parent_segments, name.clone());
    let PartUsageBody::Brace { elements } = &node.body else {
        return None;
    };
    let mut lifelines = Vec::new();
    let mut fragments = Vec::new();
    let mut state = SequenceBuildState::default();

    for element in elements {
        let PartUsageBodyElement::PartUsage(part_usage) = &element.value else {
            continue;
        };
        if part_usage_is_lifeline(part_usage) {
            lifelines.push(SequenceLifelineDto {
                id: join_segments(&with_segment(&qualified_segments, part_usage.value.name.clone())),
                name: part_usage.value.name.clone(),
                uri: None,
                range: span_to_range_dto(&part_usage.span),
            });
            continue;
        }
        if let Some(message) = extract_sequence_message(part_usage, &qualified_segments, &mut state)
        {
            state.messages.push(message);
            continue;
        }
        if let Some(activation) = extract_sequence_activation(part_usage, &qualified_segments) {
            state.activations.push(activation);
            continue;
        }
        if let Some(fragment) = extract_sequence_fragment(part_usage, &qualified_segments, &mut state)
        {
            fragments.push(fragment);
        }
    }

    let mut diagram = SequenceDiagramDto {
        id: sequence_diagram_id(&qualified_segments, "sequence"),
        name,
        package_path: package_path_from_segments(package_segments),
        source_kind: "partUsage".to_string(),
        uri: None,
        lifelines,
        messages: state.messages,
        activations: state.activations,
        fragments,
        range: span_to_range_dto(&node.span),
    };
    normalize_sequence_diagram_references(&mut diagram);
    Some(diagram)
}

fn collect_sequence_diagrams_from_package_elements(
    elements: &[sysml_v2_parser::Node<PackageBodyElement>],
    package_segments: &[String],
    parent_segments: &[String],
    out: &mut Vec<SequenceDiagramDto>,
) {
    use sysml_v2_parser::ast::PackageBodyElement as PBE;
    for node in elements {
        match &node.value {
            PBE::PartDef(part_def) => {
                if let Some(diagram) =
                    extract_sequence_diagram_from_part_def(part_def, package_segments, parent_segments)
                {
                    out.push(diagram);
                }
            }
            PBE::PartUsage(part_usage) => {
                if let Some(diagram) = extract_sequence_diagram_from_part_usage(
                    part_usage,
                    package_segments,
                    parent_segments,
                ) {
                    out.push(diagram);
                }
            }
            PBE::Package(package) => {
                if let PackageBody::Brace { elements: inner } = &package.body {
                    let package_name = identification_name(&package.identification);
                    let next_package_segments =
                        with_segment(package_segments, package_name.clone());
                    let next_parent_segments = with_segment(parent_segments, package_name);
                    collect_sequence_diagrams_from_package_elements(
                        inner,
                        &next_package_segments,
                        &next_parent_segments,
                        out,
                    );
                }
            }
            PBE::LibraryPackage(package) => {
                if let PackageBody::Brace { elements: inner } = &package.body {
                    let package_name = identification_name(&package.identification);
                    let next_package_segments =
                        with_segment(package_segments, package_name.clone());
                    let next_parent_segments = with_segment(parent_segments, package_name);
                    collect_sequence_diagrams_from_package_elements(
                        inner,
                        &next_package_segments,
                        &next_parent_segments,
                        out,
                    );
                }
            }
            _ => {}
        }
    }
}

pub fn extract_sequence_diagrams(root: &RootNamespace) -> Vec<SequenceDiagramDto> {
    let mut out = Vec::new();
    for node in &root.elements {
        match &node.value {
            RootElement::Package(package) => {
                if let PackageBody::Brace { elements } = &package.body {
                    let package_name = identification_name(&package.identification);
                    let package_segments = if package_name.is_empty() {
                        vec![]
                    } else {
                        vec![package_name]
                    };
                    collect_sequence_diagrams_from_package_elements(
                        elements,
                        &package_segments,
                        &package_segments,
                        &mut out,
                    );
                }
            }
            RootElement::Namespace(namespace) => {
                if let PackageBody::Brace { elements } = &namespace.body {
                    let namespace_name = identification_name(&namespace.identification);
                    let package_segments = if namespace_name.is_empty() {
                        vec![]
                    } else {
                        vec![namespace_name]
                    };
                    collect_sequence_diagrams_from_package_elements(
                        elements,
                        &package_segments,
                        &package_segments,
                        &mut out,
                    );
                }
            }
            RootElement::LibraryPackage(package) => {
                if let PackageBody::Brace { elements } = &package.body {
                    let package_name = identification_name(&package.identification);
                    let package_segments = if package_name.is_empty() {
                        vec![]
                    } else {
                        vec![package_name]
                    };
                    collect_sequence_diagrams_from_package_elements(
                        elements,
                        &package_segments,
                        &package_segments,
                        &mut out,
                    );
                }
            }
            RootElement::Import(_) => {}
        }
    }
    out
}

fn extract_activity_from_action(
    node: &sysml_v2_parser::Node<sysml_v2_parser::ast::ActionDef>,
    package_segments: &[String],
    parent_segments: &[String],
) -> ActivityDiagramDto {
    let name = identification_name(&node.identification);
    let qualified_segments = with_segment(parent_segments, name.clone());
    let range = span_to_range_dto(&node.span);
    let mut actions = Vec::new();
    let mut flows = Vec::new();
    let mut states = Vec::new();
    let mut interface_inputs = Vec::new();
    let mut interface_outputs = Vec::new();
    if let ActionDefBody::Brace { elements } = &node.body {
        for (i, element) in elements.iter().enumerate() {
            match &element.value {
                ActionDefBodyElement::InOutDecl(in_out) => {
                    let param_name = if in_out.value.name.trim().is_empty() {
                        format!("param_{}", i)
                    } else {
                        in_out.value.name.clone()
                    };
                    match in_out.value.direction {
                        sysml_v2_parser::ast::InOut::In => interface_inputs.push(param_name),
                        sysml_v2_parser::ast::InOut::Out => interface_outputs.push(param_name),
                        sysml_v2_parser::ast::InOut::InOut => {
                            interface_inputs.push(param_name.clone());
                            interface_outputs.push(param_name);
                        }
                    }
                }
                ActionDefBodyElement::Perform(perform) => {
                    let perform_name = if perform.value.action_name.trim().is_empty() {
                        perform
                            .value
                            .type_name
                            .clone()
                            .unwrap_or_else(|| format!("perform_{}", i))
                    } else {
                        perform.value.action_name.clone()
                    };
                    actions.push(ActivityActionDto {
                        name: perform_name,
                        action_type: "action".to_string(),
                        kind: Some("perform".to_string()),
                        inputs: None,
                        outputs: None,
                        range: Some(span_to_range_dto(&perform.span)),
                    });
                }
                ActionDefBodyElement::ActionUsage(usage) => {
                    let u = usage.as_ref();
                    let mut inputs = Vec::new();
                    if let Some((ref accept_name, _accept_type)) = &u.accept {
                        inputs.push(accept_name.clone());
                    }
                    actions.push(ActivityActionDto {
                        name: u.name.clone(),
                        action_type: "action".to_string(),
                        // VS Code Action Flow view filters allowed node kinds. Use a compatible kind
                        // for action usages so they appear as regular action nodes.
                        kind: Some("action".to_string()),
                        inputs: if inputs.is_empty() {
                            None
                        } else {
                            Some(inputs)
                        },
                        outputs: None,
                        range: Some(span_to_range_dto(&u.span)),
                    });
                }
                ActionDefBodyElement::Bind(bind) => {
                    let left = expr_to_string(&bind.value.left);
                    let right = expr_to_string(&bind.value.right);
                    if !left.is_empty() && !right.is_empty() {
                        flows.push(ControlFlowDto {
                            from: left,
                            to: right,
                            condition: None,
                            guard: Some("bind".to_string()),
                            range: span_to_range_dto(&bind.span),
                        });
                    }
                }
                ActionDefBodyElement::Flow(flow) => {
                    let from = expr_to_string(&flow.value.from);
                    let to = expr_to_string(&flow.value.to);
                    if !from.is_empty() && !to.is_empty() {
                        flows.push(ControlFlowDto {
                            from,
                            to,
                            condition: None,
                            guard: Some("flow".to_string()),
                            range: span_to_range_dto(&flow.span),
                        });
                    }
                }
                ActionDefBodyElement::FirstStmt(first) => {
                    let from = expr_to_string(&first.value.first);
                    let to = expr_to_string(&first.value.then);
                    if !from.is_empty() && !to.is_empty() {
                        flows.push(ControlFlowDto {
                            from,
                            to,
                            condition: None,
                            guard: Some("first".to_string()),
                            range: span_to_range_dto(&first.span),
                        });
                    }
                }
                ActionDefBodyElement::MergeStmt(merge) => {
                    let m = expr_to_string(&merge.value.merge);
                    states.push(ActivityStateDto {
                        name: if m.is_empty() {
                            format!("merge_{}", i)
                        } else {
                            m
                        },
                        state_type: "merge".to_string(),
                        range: span_to_range_dto(&merge.span),
                    });
                }
                ActionDefBodyElement::Error(_) | ActionDefBodyElement::Doc(_) => {}
                _ => {}
            }
        }
    }

    // Synthesize action nodes referenced by flow endpoints so the UI can render sequencing like
    // `first validateRoute then startMission;` even when the parser doesn't surface those steps
    // as ActionUsage/Perform nodes.
    fn endpoint_to_step_name(endpoint: &str) -> Option<String> {
        let s = endpoint.trim();
        if s.is_empty() {
            return None;
        }
        // `foo::bar` -> `foo`, `foo.bar` -> `foo`
        let step = s
            .split_once("::")
            .map(|(head, _)| head)
            .or_else(|| s.split_once('.').map(|(head, _)| head))
            .unwrap_or(s)
            .trim();
        if step.is_empty() {
            None
        } else {
            Some(step.to_string())
        }
    }

    let existing_action_names: std::collections::HashSet<String> =
        actions.iter().map(|a| a.name.clone()).collect();
    let mut referenced_step_names: std::collections::HashSet<String> =
        std::collections::HashSet::new();

    let interface_param_names: std::collections::HashSet<String> = interface_inputs
        .iter()
        .chain(interface_outputs.iter())
        .cloned()
        .collect();

    for f in &flows {
        if let Some(step) = endpoint_to_step_name(&f.from) {
            if !interface_param_names.contains(&step) {
                referenced_step_names.insert(step);
            }
        }
        if let Some(step) = endpoint_to_step_name(&f.to) {
            if !interface_param_names.contains(&step) {
                referenced_step_names.insert(step);
            }
        }
    }

    // Avoid turning the activity itself into a node (e.g., `ExecutePatrol::route`).
    referenced_step_names.remove(&name);

    for step in referenced_step_names {
        if existing_action_names.contains(&step) {
            continue;
        }
        actions.push(ActivityActionDto {
            name: step,
            action_type: "action".to_string(),
            kind: Some("action".to_string()),
            inputs: None,
            outputs: None,
            range: None,
        });
    }

    let interface = if interface_inputs.is_empty() && interface_outputs.is_empty() {
        None
    } else {
        Some(ActivityInterfaceDto {
            inputs: interface_inputs,
            outputs: interface_outputs,
        })
    };
    ActivityDiagramDto {
        id: activity_diagram_id(&qualified_segments, "actionDef"),
        name: if name.is_empty() {
            "action".to_string()
        } else {
            name
        },
        package_path: package_path_from_segments(package_segments),
        source_kind: "actionDef".to_string(),
        actions,
        interface,
        decisions: vec![],
        flows,
        states,
        range,
    }
}

#[cfg(test)]
mod tests {
    use super::{extract_activity_diagrams, extract_sequence_diagrams};
    use sysml_v2_parser::parse;

    #[test]
    fn extract_activity_diagrams_exposes_in_out_as_interface_metadata() {
        let input = r#"
            package P {
                action def UpdateDisplay {
                    in currentTime : TimeValue;
                    out displayText : String;
                }
            }
        "#;

        let root = parse(input).expect("parse");
        let diagrams = extract_activity_diagrams(&root);
        let diagram = diagrams
            .iter()
            .find(|d| d.name == "UpdateDisplay")
            .expect("diagram");

        assert!(
            diagram.actions.is_empty(),
            "interface declarations should not become flow steps"
        );
        assert_eq!(
            diagram.interface.as_ref().map(|itf| itf.inputs.clone()),
            Some(vec!["currentTime".to_string()])
        );
        assert_eq!(
            diagram.interface.as_ref().map(|itf| itf.outputs.clone()),
            Some(vec!["displayText".to_string()])
        );
        assert!(
            diagram.flows.is_empty(),
            "should not synthesize pseudo-flows"
        );
    }

    #[test]
    fn extract_activity_diagrams_includes_perform_steps() {
        let input = r#"
            package P {
                action def ExecuteMission {
                    in route : Route;
                    perform action captureVideo : CaptureVideo;
                    out report : MissionReport;
                }
            }
        "#;

        let root = parse(input).expect("parse");
        let diagrams = extract_activity_diagrams(&root);
        let diagram = diagrams
            .iter()
            .find(|d| d.name == "ExecuteMission")
            .expect("diagram");
        let action_names: Vec<_> = diagram.actions.iter().map(|a| a.name.as_str()).collect();

        assert_eq!(action_names, vec!["captureVideo"]);
        assert_eq!(diagram.actions[0].kind.as_deref(), Some("perform"));
        assert_eq!(
            diagram.interface.as_ref().map(|itf| itf.inputs.clone()),
            Some(vec!["route".to_string()])
        );
        assert_eq!(
            diagram.interface.as_ref().map(|itf| itf.outputs.clone()),
            Some(vec!["report".to_string()])
        );
        assert!(
            diagram.flows.is_empty(),
            "perform-only diagrams should not invent ordering edges"
        );
    }

    #[test]
    fn extract_activity_diagrams_includes_usage_bind_and_flows() {
        let input = r#"
            package P {
                action def ExecuteMission {
                    in route : Route;
                    action captureVideo : CaptureVideo;
                    bind route = captureVideo;
                    flow captureVideo to route;
                    first captureVideo then route;
                    merge route;
                    out report : MissionReport;
                }
            }
        "#;

        let root = parse(input).expect("parse");
        let diagrams = extract_activity_diagrams(&root);
        let diagram = diagrams
            .iter()
            .find(|d| d.name == "ExecuteMission")
            .expect("diagram");

        assert!(
            diagram
                .actions
                .iter()
                .any(|a| a.name == "captureVideo" && a.kind.as_deref() == Some("action")),
            "expected action usage step to be emitted as a regular action node kind"
        );
        assert!(
            diagram
                .flows
                .iter()
                .any(|f| f.guard.as_deref() == Some("bind")),
            "expected bind to be represented as a guarded flow edge"
        );
        assert!(
            diagram
                .flows
                .iter()
                .any(|f| f.guard.as_deref() == Some("flow")),
            "expected flow statement edge"
        );
        assert!(
            diagram
                .flows
                .iter()
                .any(|f| f.guard.as_deref() == Some("first")),
            "expected first/then edge"
        );
        assert!(
            diagram.states.iter().any(|s| s.state_type == "merge"),
            "expected merge node"
        );
    }

    #[test]
    fn extract_activity_diagrams_with_only_interface_have_no_behavior_nodes() {
        let input = r#"
            package P {
                action def ValidateRoute {
                    in route : Route;
                    out isValid : Boolean;
                }
            }
        "#;

        let root = parse(input).expect("parse");
        let diagrams = extract_activity_diagrams(&root);
        let diagram = diagrams
            .iter()
            .find(|d| d.name == "ValidateRoute")
            .expect("diagram");

        assert!(diagram.actions.is_empty());
        assert!(diagram.flows.is_empty());
        assert!(diagram.states.is_empty());
        assert!(diagram.interface.is_some());
    }

    #[test]
    fn extract_activity_diagrams_finds_action_defs_in_library_package() {
        let input = r#"
            standard library package P {
                action def ExecuteMission {
                    perform action captureVideo : CaptureVideo;
                }
            }
        "#;

        let root = parse(input).expect("parse");
        let diagrams = extract_activity_diagrams(&root);
        assert!(
            diagrams.iter().any(|d| d.name == "ExecuteMission"),
            "expected action def inside library package to be discovered; diagrams: {:?}",
            diagrams.iter().map(|d| d.name.as_str()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn extract_activity_diagrams_synthesizes_nodes_referenced_by_first_then() {
        let input = r#"
            package P {
                action def ExecuteMission {
                    action validateRoute { out ok : Boolean; };
                    action startMission { out started : Boolean; };
                    first validateRoute then startMission;
                }
            }
        "#;

        let root = parse(input).expect("parse");
        let diagrams = extract_activity_diagrams(&root);
        let diagram = diagrams
            .iter()
            .find(|d| d.name == "ExecuteMission")
            .expect("diagram");

        assert!(
            diagram.actions.iter().any(|a| a.name == "validateRoute"),
            "expected referenced step node validateRoute to exist"
        );
        assert!(
            diagram.actions.iter().any(|a| a.name == "startMission"),
            "expected referenced step node startMission to exist"
        );
        assert!(
            diagram
                .flows
                .iter()
                .any(|f| f.guard.as_deref() == Some("first")
                    && f.from == "validateRoute"
                    && f.to == "startMission"),
            "expected first/then flow edge"
        );
    }

    #[test]
    fn extract_activity_diagrams_does_not_synthesize_interface_parameters_as_step_nodes() {
        let input = r#"
            package P {
                action def ExecutePatrol {
                    in route : String;
                    out status : String;

                    action finishMission { out missionStatus : String; };
                    bind status = finishMission::missionStatus;
                }
            }
        "#;

        let root = parse(input).expect("parse");
        let diagrams = extract_activity_diagrams(&root);
        let diagram = diagrams
            .iter()
            .find(|d| d.name == "ExecutePatrol")
            .expect("diagram");

        assert_eq!(
            diagram.interface.as_ref().map(|itf| itf.inputs.clone()),
            Some(vec!["route".to_string()])
        );
        assert_eq!(
            diagram.interface.as_ref().map(|itf| itf.outputs.clone()),
            Some(vec!["status".to_string()])
        );

        assert!(
            diagram
                .actions
                .iter()
                .all(|a| a.name != "route" && a.name != "status"),
            "interface parameters should not be synthesized into action nodes; actions={:?}",
            diagram
                .actions
                .iter()
                .map(|a| a.name.as_str())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn extract_activity_diagrams_emits_performer_context_diagrams_from_part_bodies() {
        let input = r#"
            package Mission {
                part def FlightController {
                    perform action assessVehicleState : AssessVehicleState;
                    perform action manageMissionEvents : ManageMissionEvents;
                    perform action commandVehicle : CommandVehicle;
                }
            }
        "#;

        let root = parse(input).expect("parse");
        let diagrams = extract_activity_diagrams(&root);
        let diagram = diagrams
            .iter()
            .find(|d| d.name == "FlightController" && d.source_kind == "performer")
            .expect("performer diagram");

        assert_eq!(diagram.package_path, "Mission");
        assert_eq!(diagram.id, "Mission::FlightController::performer");
        assert_eq!(diagram.actions.len(), 3);
        assert_eq!(diagram.flows.len(), 2);
        assert_eq!(diagram.flows[0].from, "assessVehicleState");
        assert_eq!(diagram.flows[0].to, "manageMissionEvents");
        assert_eq!(diagram.flows[1].from, "manageMissionEvents");
        assert_eq!(diagram.flows[1].to, "commandVehicle");
    }

    #[test]
    fn extract_activity_diagrams_include_package_metadata_for_action_defs() {
        let input = r#"
            package Mission {
                package Control {
                    action def ExecuteMission {
                        action assessVehicleState : AssessVehicleState;
                        action commandVehicle : CommandVehicle;
                        first assessVehicleState then commandVehicle;
                    }
                }
            }
        "#;

        let root = parse(input).expect("parse");
        let diagrams = extract_activity_diagrams(&root);
        let diagram = diagrams
            .iter()
            .find(|d| d.name == "ExecuteMission")
            .expect("diagram");

        assert_eq!(diagram.source_kind, "actionDef");
        assert_eq!(diagram.package_path, "Mission::Control");
        assert_eq!(diagram.id, "Mission::Control::ExecuteMission::actionDef");
    }

    #[test]
    fn extract_sequence_diagrams_emits_messages_activations_and_fragments() {
        let input = r#"
            package Demo {
                part def CheckoutFlow :> InteractionScenario {
                    part client : Lifeline;
                    part api : Lifeline;

                    part createOrder : SynchronousCall {
                        ref from : Lifeline = client;
                        ref to : Lifeline = api;
                        attribute label = "POST /orders";
                    }

                    part processing : Activation {
                        ref on : Lifeline = api;
                        ref startMessage : Message = createOrder;
                        ref finishMessage : Message = orderAccepted;
                    }

                    part validation : OptFragment {
                        part happyPath : InteractionOperand {
                            attribute guard = "valid order";
                            part orderAccepted : ReturnMessage {
                                ref from : Lifeline = api;
                                ref to : Lifeline = client;
                                attribute label = "202 Accepted";
                            }
                        }
                    }
                }
            }
        "#;

        let root = parse(input).expect("parse");
        let diagrams = extract_sequence_diagrams(&root);
        let diagram = diagrams.iter().find(|d| d.name == "CheckoutFlow").expect("sequence diagram");

        assert_eq!(diagram.package_path, "Demo");
        assert_eq!(diagram.lifelines.len(), 2);
        assert_eq!(diagram.messages.len(), 2);
        assert_eq!(diagram.messages[0].kind, "sync");
        assert_eq!(diagram.messages[1].kind, "return");
        assert_eq!(diagram.activations.len(), 1);
        assert_eq!(diagram.activations[0].on_lifeline, "Demo::CheckoutFlow::api");
        assert_eq!(diagram.fragments.len(), 1);
        assert_eq!(diagram.fragments[0].kind, "opt");
        assert_eq!(diagram.fragments[0].operands[0].guard.as_deref(), Some("valid order"));
    }

    #[test]
    fn extract_sequence_diagrams_supports_ref_fragments_and_nested_alternatives() {
        let input = r#"
            library package Demo {
                part def RetryFlow :> InteractionScenario {
                    part worker : Lifeline;
                    part queue : Lifeline;

                    part redeliver : AsynchronousMessage {
                        ref from : Lifeline = queue;
                        ref to : Lifeline = worker;
                        attribute label = "retry";
                    }
                }

                part def PaymentFlow :> InteractionScenario {
                    part customer : Lifeline;
                    part gateway : Lifeline;

                    part authorize : SynchronousCall {
                        ref from : Lifeline = customer;
                        ref to : Lifeline = gateway;
                    }

                    part outcomes : AltFragment {
                        part approved : InteractionOperand {
                            attribute guard = "approved";
                            part complete : ReturnMessage {
                                ref from : Lifeline = gateway;
                                ref to : Lifeline = customer;
                            }
                        }

                        part declined : InteractionOperand {
                            attribute guard = "declined";
                            part retryFlow : InteractionRef {
                                ref target : InteractionScenario = RetryFlow;
                            }
                        }
                    }
                }
            }
        "#;

        let root = parse(input).expect("parse");
        let diagrams = extract_sequence_diagrams(&root);
        let payment = diagrams.iter().find(|d| d.name == "PaymentFlow").expect("payment flow");

        assert_eq!(payment.fragments.len(), 1);
        assert_eq!(payment.fragments[0].kind, "alt");
        assert_eq!(payment.fragments[0].operands.len(), 2);
        assert_eq!(payment.fragments[0].operands[1].fragments.len(), 1);
        assert_eq!(payment.fragments[0].operands[1].fragments[0].kind, "ref");
        assert_eq!(
            payment.fragments[0].operands[1].fragments[0].target_ref.as_deref(),
            Some("RetryFlow")
        );
    }
}
