//! Activity and sequence diagram extraction for sysml/model response.

use crate::syntax::ast_util::identification_name;
use serde::Serialize;
use sysml_parser::ast::{ActionDefBody, PackageBody, PackageBodyElement, RootElement};
use sysml_parser::{RootNamespace, Span};

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

fn default_range_dto() -> RangeDto {
    RangeDto {
        start: PositionDto {
            line: 0,
            character: 0,
        },
        end: PositionDto {
            line: 0,
            character: 0,
        },
    }
}

// ---------------------------------------------------------------------------
// Activity diagrams
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityDiagramDto {
    pub name: String,
    pub actions: Vec<ActivityActionDto>,
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
    pub range: Option<RangeDto>,
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
    pub name: String,
    pub participants: Vec<ParticipantDto>,
    pub messages: Vec<MessageDto>,
    pub range: RangeDto,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantDto {
    pub name: String,
    #[serde(rename = "type")]
    pub participant_type: String,
    pub range: RangeDto,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageDto {
    pub name: String,
    pub from: String,
    pub to: String,
    pub payload: String,
    pub occurrence: u32,
    pub range: RangeDto,
}

// ---------------------------------------------------------------------------
// Extraction
// ---------------------------------------------------------------------------

fn collect_action_defs_from_elements(
    elements: &[sysml_parser::Node<PackageBodyElement>],
) -> Vec<&sysml_parser::Node<sysml_parser::ast::ActionDef>> {
    use sysml_parser::ast::PackageBodyElement as PBE;
    let mut out = Vec::new();
    for node in elements {
        match &node.value {
            PBE::ActionDef(ad) => out.push(ad),
            PBE::Package(p) => {
                if let PackageBody::Brace { elements: inner } = &p.body {
                    out.extend(collect_action_defs_from_elements(inner));
                }
            }
            _ => {}
        }
    }
    out
}

/// Extracts activity diagrams from ActionDef nodes.
/// Each ActionDef becomes one ActivityDiagramDto; sysml-parser ActionDefBody has InOutDecl only (no statements).
pub fn extract_activity_diagrams(root: &RootNamespace) -> Vec<ActivityDiagramDto> {
    let mut out = Vec::new();
    for node in &root.elements {
        let elements = match &node.value {
            RootElement::Package(p) => match &p.body {
                PackageBody::Brace { elements } => elements,
                _ => continue,
            },
            RootElement::Namespace(n) => match &n.body {
                PackageBody::Brace { elements } => elements,
                _ => continue,
            },
            RootElement::LibraryPackage(lp) => match &lp.body {
                PackageBody::Brace { elements } => elements,
                _ => continue,
            },
            RootElement::Import(_) => continue,
        };
        for action in collect_action_defs_from_elements(elements) {
            out.push(extract_activity_from_action(action));
        }
    }
    out
}

fn extract_activity_from_action(
    node: &sysml_parser::Node<sysml_parser::ast::ActionDef>,
) -> ActivityDiagramDto {
    let name = identification_name(&node.identification);
    let range = span_to_range_dto(&node.span);
    let mut actions = Vec::new();
    if let ActionDefBody::Brace { elements } = &node.body {
        for (i, in_out) in elements.iter().enumerate() {
            let param_name = format!("param_{}", i);
            actions.push(ActivityActionDto {
                name: param_name.clone(),
                action_type: "action".to_string(),
                kind: None,
                range: Some(span_to_range_dto(&in_out.span)),
            });
        }
    }
    let mut flows = Vec::new();
    let mut prev: Option<String> = None;
    for a in &actions {
        if let Some(ref p) = prev {
            flows.push(ControlFlowDto {
                from: p.clone(),
                to: a.name.clone(),
                condition: None,
                guard: None,
                range: default_range_dto(),
            });
        }
        prev = Some(a.name.clone());
    }
    let states = if actions.is_empty() {
        vec![]
    } else {
        vec![
            ActivityStateDto {
                name: "initial".to_string(),
                state_type: "initial".to_string(),
                range: default_range_dto(),
            },
            ActivityStateDto {
                name: "final".to_string(),
                state_type: "final".to_string(),
                range: default_range_dto(),
            },
        ]
    };
    ActivityDiagramDto {
        name: if name.is_empty() {
            "action".to_string()
        } else {
            name
        },
        actions,
        decisions: vec![],
        flows,
        states,
        range,
    }
}

/// Extracts sequence diagrams from the document (one per ActionDef; no Call/Perform in sysml-parser action body).
pub fn extract_sequence_diagrams(root: &RootNamespace) -> Vec<SequenceDiagramDto> {
    let mut out = Vec::new();
    for node in &root.elements {
        let elements = match &node.value {
            RootElement::Package(p) => match &p.body {
                PackageBody::Brace { elements } => elements,
                _ => continue,
            },
            RootElement::Namespace(n) => match &n.body {
                PackageBody::Brace { elements } => elements,
                _ => continue,
            },
            RootElement::LibraryPackage(lp) => match &lp.body {
                PackageBody::Brace { elements } => elements,
                _ => continue,
            },
            RootElement::Import(_) => continue,
        };
        for action in collect_action_defs_from_elements(elements) {
            out.push(extract_sequence_from_action(action));
        }
    }
    out
}

fn extract_sequence_from_action(
    node: &sysml_parser::Node<sysml_parser::ast::ActionDef>,
) -> SequenceDiagramDto {
    let name = identification_name(&node.identification);
    let range = span_to_range_dto(&node.span);
    let participants = vec![ParticipantDto {
        name: "self".to_string(),
        participant_type: "participant".to_string(),
        range: default_range_dto(),
    }];
    SequenceDiagramDto {
        name: if name.is_empty() {
            "action".to_string()
        } else {
            name
        },
        participants,
        messages: vec![],
        range,
    }
}
