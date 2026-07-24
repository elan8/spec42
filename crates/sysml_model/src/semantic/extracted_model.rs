//! Activity and sequence diagram extraction for sysml/model response.

use crate::semantic::ast_util::identification_name;
use crate::semantic::dto::{PositionDto, RangeDto};
use crate::semantic::expression_fold::{fold_expression, ExpressionAlgebra, FoldedChild};
use crate::semantic::graph_builder::expressions::expression_to_debug_string;
use serde::Serialize;
use sysml_v2_parser::ast::{
    ActionDefBody, ActionDefBodyElement, FlowUsageKind, PackageBody, PackageBodyElement,
    PartDefBody, PartDefBodyElement, PartUsageBody, PartUsageBodyElement, RootElement,
};
use sysml_v2_parser::{RootNamespace, Span};
use ts_rs::TS;

fn normalized_type_name(type_name: &str) -> String {
    type_name
        .split("::")
        .last()
        .unwrap_or(type_name)
        .replace([' ', '_'], "")
        .to_lowercase()
}

fn control_state_type(type_name: &str) -> Option<&'static str> {
    match normalized_type_name(type_name).as_str() {
        "decision" => Some("decision"),
        "merge" => Some("merge"),
        "fork" => Some("fork"),
        "join" => Some("join"),
        "terminate" => Some("terminate"),
        "accept" => Some("accept"),
        "send" => Some("send"),
        _ => None,
    }
}

fn flow_guard_for_usage(kind: FlowUsageKind) -> &'static str {
    match kind {
        FlowUsageKind::SuccessionFlow => "succession",
        FlowUsageKind::Message => "message",
        FlowUsageKind::Flow => "flow",
    }
}

/// Readable summary of a flow's `PayloadFeature` for diagram-extraction display purposes --
/// mirrors `graph_builder::flow_usage::payload_feature_debug_string`'s formatting, kept as a
/// separate local helper since this module already keeps its own `expr_to_string` rather than
/// reusing `graph_builder`'s expression-formatting helpers.
pub(super) fn payload_feature_to_string(payload: &sysml_v2_parser::ast::PayloadFeature) -> String {
    match (&payload.name, &payload.type_name) {
        (Some(name), Some(type_name)) => format!("{name} : {type_name}"),
        (Some(name), None) => name.clone(),
        (None, Some(type_name)) => type_name.clone(),
        (None, None) => String::new(),
    }
}

struct ExprToStringAlgebra;

impl ExpressionAlgebra for ExprToStringAlgebra {
    type Output = String;

    /// Mirrors the pre-0.47.0 recursive `expr_to_string` exactly, one match arm per variant, with
    /// each recursive call replaced by that child's already-folded string from `subs`/
    /// `arguments` -- see the module doc on [`crate::semantic::expression_fold`] for why. Kept as
    /// its own algebra (rather than reusing `DebugStringAlgebra` in `graph_builder::expressions`)
    /// for the same reason the original kept its own recursive function: several variants render
    /// differently here (`MemberAccess`/`Index`/`LiteralWithUnit` special-case an empty child;
    /// `CollectionOp` args drop their names where `Invocation`/`Constructor` keep them).
    fn build(
        &mut self,
        node: &sysml_v2_parser::Node<sysml_v2_parser::Expression>,
        children: Vec<FoldedChild<String>>,
    ) -> String {
        use sysml_v2_parser::Expression;
        let mut subs = Vec::new();
        let mut arguments: Vec<(Option<String>, String)> = Vec::new();
        for child in children {
            match child {
                FoldedChild::Sub(value) => subs.push(value),
                FoldedChild::Argument { name, value } => arguments.push((name, value)),
            }
        }
        let rendered_named_args = || {
            arguments
                .iter()
                .map(|(name, value)| match name {
                    Some(name) => format!("{name} = {value}"),
                    None => value.clone(),
                })
                .collect::<Vec<_>>()
                .join(", ")
        };
        match &node.value {
            Expression::FeatureRef(s) => s.clone(),
            Expression::MemberAccess(_, member) => {
                if subs[0].is_empty() {
                    member.clone()
                } else {
                    format!("{}.{member}", subs[0])
                }
            }
            Expression::Index { .. } => {
                if subs[0].is_empty() {
                    String::new()
                } else if subs[1].is_empty() {
                    format!("{}#()", subs[0])
                } else {
                    format!("{}#({})", subs[0], subs[1])
                }
            }
            Expression::Bracket(_) => subs[0].clone(),
            Expression::LiteralString(s) => s.clone(),
            Expression::LiteralInteger(i) => i.to_string(),
            Expression::LiteralReal(s) => s.clone(),
            Expression::LiteralBoolean(b) => b.to_string(),
            Expression::LiteralWithUnit { .. } => {
                if subs[1].is_empty() {
                    subs[0].clone()
                } else {
                    format!("{} [{}]", subs[0], subs[1])
                }
            }
            Expression::BinaryOp { op, .. } => format!("({} {} {})", subs[0], op.as_str(), subs[1]),
            Expression::UnaryOp { op, .. } => format!("({}{})", op.as_str(), subs[0]),
            Expression::Invocation { .. } => format!("{}({})", subs[0], rendered_named_args()),
            Expression::Tuple(_) => subs.join(", "),
            Expression::Classification { metaclass } => format!("@{metaclass}"),
            Expression::MetaCast { metaclass, .. } => format!("{} meta {metaclass}", subs[0]),
            Expression::TypeCheck {
                kind, type_name, ..
            } => {
                let op = match kind {
                    sysml_v2_parser::TypeCheckKind::Istype => "istype",
                    sysml_v2_parser::TypeCheckKind::Hastype => "hastype",
                    sysml_v2_parser::TypeCheckKind::As => "as",
                };
                match subs.first() {
                    Some(operand) => format!("{operand} {op} {type_name}"),
                    None => format!("{op} {type_name}"),
                }
            }
            Expression::Select { selector, .. } => format!("{}.?{selector}", subs[0]),
            Expression::Collect { selector, .. } => format!("{}.**{selector}", subs[0]),
            Expression::Parenthesized(_) => format!("({})", subs[0]),
            Expression::Constructor { type_name, .. } => {
                format!("new {type_name}({})", rendered_named_args())
            }
            Expression::FeatureChainRef(chain) => chain.segments.join("."),
            Expression::CollectionOp { op, .. } => {
                let rendered = arguments
                    .iter()
                    .map(|(_, value)| value.clone())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{}->{}({rendered})", subs[0], op.as_str())
            }
            Expression::MetadataAccess(_) => format!("{}.metadata", subs[0]),
            Expression::Null => String::new(),
        }
    }
}

/// Iterative, not recursive: see [`crate::semantic::expression_fold`] for why.
fn expr_to_string(n: &sysml_v2_parser::Node<sysml_v2_parser::Expression>) -> String {
    fold_expression(n, &mut ExprToStringAlgebra)
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

mod activity_dto;
mod activity_extract;
mod activity_walk;
mod sequence_dto;
mod state_dto;
pub use activity_dto::*;
pub(crate) use activity_extract::*;
pub use activity_walk::*;
pub use sequence_dto::*;
pub use state_dto::*;

#[cfg(test)]
mod tests;
