//! Activity and sequence diagram extraction for sysml/model response.

use crate::semantic::ast_util::identification_name;
use crate::semantic::dto::{PositionDto, RangeDto};
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
                op.as_str(),
                expr_to_string(right)
            )
        }
        Expression::UnaryOp { op, operand } => {
            format!("({}{})", op.as_str(), expr_to_string(operand))
        }
        Expression::Invocation { callee, args } => {
            let rendered = args
                .iter()
                .map(|argument| {
                    let value =
                        expr_to_string(crate::semantic::ast_util::argument_expression(argument));
                    argument
                        .name
                        .as_ref()
                        .map(|name| format!("{name} = {value}"))
                        .unwrap_or(value)
                })
                .collect::<Vec<_>>()
                .join(", ");
            format!("{}({rendered})", expr_to_string(callee))
        }
        Expression::Tuple(items) => items
            .iter()
            .map(expr_to_string)
            .collect::<Vec<_>>()
            .join(", "),
        Expression::Classification { metaclass } => format!("@{metaclass}"),
        Expression::MetaCast { base, metaclass } => {
            format!("{} meta {metaclass}", expr_to_string(base))
        }
        Expression::TypeCheck {
            kind,
            operand,
            type_name,
        } => {
            let op = match kind {
                sysml_v2_parser::TypeCheckKind::Istype => "istype",
                sysml_v2_parser::TypeCheckKind::Hastype => "hastype",
                sysml_v2_parser::TypeCheckKind::As => "as",
            };
            match operand {
                Some(operand) => format!("{} {op} {type_name}", expr_to_string(operand)),
                None => format!("{op} {type_name}"),
            }
        }
        Expression::Select { base, selector } => {
            format!("{}.?{selector}", expr_to_string(base))
        }
        Expression::Collect { base, selector } => {
            format!("{}.**{selector}", expr_to_string(base))
        }
        Expression::Parenthesized(inner) => format!("({})", expr_to_string(inner)),
        Expression::Constructor { type_name, args } => {
            let rendered = args
                .iter()
                .map(|argument| {
                    let value =
                        expr_to_string(crate::semantic::ast_util::argument_expression(argument));
                    argument
                        .name
                        .as_ref()
                        .map(|name| format!("{name} = {value}"))
                        .unwrap_or(value)
                })
                .collect::<Vec<_>>()
                .join(", ");
            format!("new {type_name}({rendered})")
        }
        Expression::FeatureChainRef(chain) => chain.segments.join("."),
        Expression::CollectionOp { op, base, args } => {
            let rendered = args
                .iter()
                .map(|argument| {
                    expr_to_string(crate::semantic::ast_util::argument_expression(argument))
                })
                .collect::<Vec<_>>()
                .join(", ");
            format!("{}->{}({rendered})", expr_to_string(base), op.as_str())
        }
        Expression::MetadataAccess(base) => format!("{}.metadata", expr_to_string(base)),
        Expression::Null => String::new(),
    }
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
