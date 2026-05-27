use std::collections::{HashMap, HashSet};

use serde_json::Value;

use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::{NodeId, SemanticNode};

mod units;

use units::{UnitError, UnitRegistry};

const EVALUATED_VALUE_KEY: &str = "evaluatedValue";
const EVALUATED_UNIT_KEY: &str = "evaluatedUnit";
const EVALUATION_STATUS_KEY: &str = "evaluationStatus";
const EVALUATION_ERROR_KEY: &str = "evaluationError";

const STATUS_OK: &str = "ok";
const STATUS_UNKNOWN: &str = "unknown";
const STATUS_INCOMPLETE: &str = "incomplete";
const STATUS_TYPE_ERROR: &str = "type_error";
const STATUS_DIV_BY_ZERO: &str = "div_by_zero";
const STATUS_UNSUPPORTED: &str = "unsupported";
const STATUS_CYCLE: &str = "cycle";

const EVALUATION_SOURCE_KEYS: [&str; 3] = ["value", "defaultValue", "literal"];
const ANALYSIS_CONSTRAINTS_KEY: &str = "analysisConstraints";
const ANALYSIS_EXPRESSION_KEY: &str = "analysisExpression";
const ANALYSIS_EVAL_STATUS_KEY: &str = "analysisEvaluationStatus";
const ANALYSIS_EVAL_VALUE_KEY: &str = "analysisEvaluationValue";
const ANALYSIS_EVAL_ERROR_KEY: &str = "analysisEvaluationError";
const ANALYSIS_CONSTRAINT_PASSED_KEY: &str = "analysisConstraintPassed";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EvalStatus {
    Ok,
    Unknown,
    Incomplete,
    TypeError,
    DivByZero,
    Unsupported,
    Cycle,
}

impl EvalStatus {
    fn as_str(self) -> &'static str {
        match self {
            EvalStatus::Ok => STATUS_OK,
            EvalStatus::Unknown => STATUS_UNKNOWN,
            EvalStatus::Incomplete => STATUS_INCOMPLETE,
            EvalStatus::TypeError => STATUS_TYPE_ERROR,
            EvalStatus::DivByZero => STATUS_DIV_BY_ZERO,
            EvalStatus::Unsupported => STATUS_UNSUPPORTED,
            EvalStatus::Cycle => STATUS_CYCLE,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AnalysisEvalError {
    status: EvalStatus,
    message: String,
}

impl AnalysisEvalError {
    fn from_status(status: EvalStatus) -> Self {
        Self {
            status,
            message: map_analysis_eval_error(status),
        }
    }

    fn with_message(status: EvalStatus, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
        }
    }

    fn with_expression(mut self, expression: &str) -> Self {
        self.message = format!("{} [expr='{}']", self.message, expression);
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
struct EvalOutcome {
    status: EvalStatus,
    value: Option<Value>,
    unit: Option<String>,
    error: Option<String>,
}

impl EvalOutcome {
    fn ok(value: Value, unit: Option<String>) -> Self {
        Self {
            status: EvalStatus::Ok,
            value: Some(value),
            unit,
            error: None,
        }
    }

    fn from_quantity(quantity: Quantity) -> Self {
        Self::ok(number_to_json(quantity.value), quantity.unit)
    }

    fn error(status: EvalStatus, message: impl Into<String>) -> Self {
        Self {
            status,
            value: None,
            unit: None,
            error: Some(message.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Quantity {
    value: f64,
    unit: Option<String>,
}

impl Quantity {
    fn scalar(value: f64) -> Self {
        Self { value, unit: None }
    }
}

pub fn evaluate_expressions(graph: &mut SemanticGraph) {
    let outcomes = {
        let mut engine = EvalEngine::new(graph);
        graph
            .node_index_by_id
            .keys()
            .filter_map(|node_id| {
                if engine.node_source_value(node_id).is_some() {
                    Some((node_id.clone(), engine.evaluate_node(node_id)))
                } else {
                    None
                }
            })
            .collect::<Vec<(NodeId, EvalOutcome)>>()
    };
    for (node_id, outcome) in outcomes {
        let Some(node) = graph.get_node_mut(&node_id) else {
            continue;
        };
        node.attributes.remove(EVALUATED_VALUE_KEY);
        node.attributes.remove(EVALUATED_UNIT_KEY);
        node.attributes.remove(EVALUATION_STATUS_KEY);
        node.attributes.remove(EVALUATION_ERROR_KEY);
        node.attributes.insert(
            EVALUATION_STATUS_KEY.to_string(),
            Value::String(outcome.status.as_str().to_string()),
        );
        if let Some(value) = outcome.value {
            node.attributes
                .insert(EVALUATED_VALUE_KEY.to_string(), value);
        }
        if let Some(unit) = outcome.unit {
            node.attributes
                .insert(EVALUATED_UNIT_KEY.to_string(), Value::String(unit));
        }
        if let Some(error) = outcome.error {
            node.attributes
                .insert(EVALUATION_ERROR_KEY.to_string(), Value::String(error));
        }
    }
    evaluate_analysis_constraints(graph);
}

fn evaluate_analysis_constraints(graph: &mut SemanticGraph) {
    let outcomes = {
        let mut engine = EvalEngine::new(graph);
        let node_ids: Vec<NodeId> = graph.node_index_by_id.keys().cloned().collect();
        let mut outcomes = Vec::new();
        for node_id in node_ids {
            let Some(node) = graph.get_node(&node_id) else {
                continue;
            };
            if is_definition_only_analysis_node(node) {
                continue;
            }
            let inline_constraints = node
                .attributes
                .get(ANALYSIS_CONSTRAINTS_KEY)
                .and_then(|value| value.as_array())
                .cloned()
                .unwrap_or_default();
            let single_expression = node
                .attributes
                .get(ANALYSIS_EXPRESSION_KEY)
                .and_then(Value::as_str)
                .map(str::to_string);

            let mut evaluated = false;
            let mut status = STATUS_UNKNOWN.to_string();
            let mut value: Option<Value> = None;
            let mut error: Option<String> = None;
            let mut passed: Option<bool> = None;
            if !inline_constraints.is_empty() {
                evaluated = true;
                for constraint in inline_constraints {
                    let Some(expr) = constraint
                        .as_object()
                        .and_then(|obj| obj.get("expression"))
                        .and_then(Value::as_str)
                    else {
                        continue;
                    };
                    match evaluate_analysis_expression(&mut engine, &node_id, expr) {
                        Ok(bool_value) => {
                            let all_pass = passed.unwrap_or(true) && bool_value;
                            passed = Some(all_pass);
                            status = if bool_value {
                                STATUS_OK.to_string()
                            } else {
                                "failed_constraint".to_string()
                            };
                            value = Some(Value::Bool(bool_value));
                        }
                        Err(err) => {
                            status = err.status.as_str().to_string();
                            error = Some(err.message);
                        }
                    }
                }
            } else if let Some(expr) = single_expression.as_deref() {
                evaluated = true;
                match evaluate_analysis_expression(&mut engine, &node_id, expr) {
                    Ok(bool_value) => {
                        status = STATUS_OK.to_string();
                        value = Some(Value::Bool(bool_value));
                        passed = Some(bool_value);
                    }
                    Err(err) => {
                        status = err.status.as_str().to_string();
                        error = Some(err.message);
                    }
                }
            }

            if evaluated {
                outcomes.push((node_id, status, value, error, passed));
            }
        }
        outcomes
    };

    for (node_id, status, value, error, passed) in outcomes {
        let Some(node_mut) = graph.get_node_mut(&node_id) else {
            continue;
        };
        node_mut.attributes.insert(
            ANALYSIS_EVAL_STATUS_KEY.to_string(),
            Value::String(status.clone()),
        );
        if let Some(v) = value {
            node_mut
                .attributes
                .insert(ANALYSIS_EVAL_VALUE_KEY.to_string(), v);
        }
        if let Some(err) = error {
            node_mut
                .attributes
                .insert(ANALYSIS_EVAL_ERROR_KEY.to_string(), Value::String(err));
        }
        if let Some(p) = passed {
            node_mut
                .attributes
                .insert(ANALYSIS_CONSTRAINT_PASSED_KEY.to_string(), Value::Bool(p));
        }
    }
}

fn is_definition_only_analysis_node(node: &SemanticNode) -> bool {
    matches!(node.element_kind.as_str(), "constraint def" | "calc def")
}

fn evaluate_analysis_expression(
    engine: &mut EvalEngine<'_>,
    context_id: &NodeId,
    expression: &str,
) -> Result<bool, AnalysisEvalError> {
    let repaired = normalize_broken_invocation_syntax(expression.trim());
    let normalized = normalize_truncated_analysis_comparison(repaired.as_str());
    let expr = normalized.as_str();
    if expr.is_empty() {
        return Err(AnalysisEvalError::with_message(
            EvalStatus::Unsupported,
            "empty analysis expression",
        ));
    }
    let mut parser = AnalysisExprParser::new(expr);
    match parser.parse_expression() {
        Ok(parsed) => {
            parser.skip_ws();
            if !parser.is_eof() {
                return Err(AnalysisEvalError::with_message(
                    EvalStatus::Unsupported,
                    "analysis expression contains unsupported trailing tokens",
                ));
            }
            evaluate_analysis_ast(engine, context_id, &parsed)
        }
        Err(_) => {
            let flattened = flatten_parenthesized_arithmetic(expr);
            if flattened != expr {
                let mut retry = AnalysisExprParser::new(flattened.as_str());
                if let Ok(parsed) = retry.parse_expression() {
                    retry.skip_ws();
                    if retry.is_eof() {
                        return evaluate_analysis_ast(engine, context_id, &parsed);
                    }
                }
            }
            // Backward-compatible fallback: accept numeric/quantity analysis expressions
            // as non-negative checks (common margin/headroom style predicates).
            let quantity = engine
                .evaluate_quantity_expression(context_id, expr)
                .map_err(|status| AnalysisEvalError::from_status(status).with_expression(expr))?;
            Ok(quantity.value >= 0.0)
        }
    }
}

fn normalize_broken_invocation_syntax(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::with_capacity(text.len());
    let mut i = 0usize;
    while i < chars.len() {
        if chars[i] == ')' {
            let mut j = i + 1;
            while j < chars.len() && chars[j].is_whitespace() {
                j += 1;
            }
            if j < chars.len() && chars[j] == '(' {
                i = j;
                continue;
            }
        }
        out.push(chars[i]);
        i += 1;
    }
    out
}

fn flatten_parenthesized_arithmetic(expr: &str) -> String {
    expr.replace(['(', ')'], " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn normalize_truncated_analysis_comparison(expr: &str) -> String {
    let mut trimmed = expr.trim().to_string();
    while let Some(stripped) = strip_outer_balanced_parens(&trimmed) {
        trimmed = stripped.to_string();
    }
    let trimmed = trimmed.trim();
    for op in ["<=", ">=", "==", "!=", "<", ">"] {
        if trimmed.ends_with(op) {
            return format!("{trimmed} 0");
        }
    }
    trimmed.to_string()
}

fn strip_outer_balanced_parens(text: &str) -> Option<&str> {
    let trimmed = text.trim();
    if !(trimmed.starts_with('(') && trimmed.ends_with(')')) {
        return None;
    }
    let mut depth = 0usize;
    for (idx, ch) in trimmed.char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => {
                depth = depth.saturating_sub(1);
                if depth == 0 && idx < trimmed.len() - 1 {
                    return None;
                }
            }
            _ => {}
        }
    }
    if depth == 0 && trimmed.len() > 2 {
        Some(&trimmed[1..trimmed.len() - 1])
    } else {
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AnalysisComparisonOp {
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Ne,
}

#[derive(Debug, Clone, PartialEq)]
enum AnalysisExpr<'s> {
    BoolLiteral(bool),
    Predicate(&'s str),
    Comparison {
        lhs: &'s str,
        op: AnalysisComparisonOp,
        rhs: &'s str,
    },
    Not(Box<AnalysisExpr<'s>>),
    And(Box<AnalysisExpr<'s>>, Box<AnalysisExpr<'s>>),
    Or(Box<AnalysisExpr<'s>>, Box<AnalysisExpr<'s>>),
}

struct AnalysisExprParser<'s> {
    src: &'s str,
    pos: usize,
}

impl<'s> AnalysisExprParser<'s> {
    fn new(src: &'s str) -> Self {
        Self { src, pos: 0 }
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.src.len()
    }

    fn skip_ws(&mut self) {
        while let Some(ch) = self.peek_char() {
            if ch.is_whitespace() {
                self.pos += ch.len_utf8();
            } else {
                break;
            }
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.src[self.pos..].chars().next()
    }

    fn eat_char(&mut self) -> Option<char> {
        let ch = self.peek_char()?;
        self.pos += ch.len_utf8();
        Some(ch)
    }

    fn parse_expression(&mut self) -> Result<AnalysisExpr<'s>, String> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<AnalysisExpr<'s>, String> {
        let mut left = self.parse_and()?;
        loop {
            self.skip_ws();
            if self.consume_symbol("||") || self.consume_keyword("or") {
                let right = self.parse_and()?;
                left = AnalysisExpr::Or(Box::new(left), Box::new(right));
            } else {
                return Ok(left);
            }
        }
    }

    fn parse_and(&mut self) -> Result<AnalysisExpr<'s>, String> {
        let mut left = self.parse_not()?;
        loop {
            self.skip_ws();
            if self.consume_symbol("&&") || self.consume_keyword("and") {
                let right = self.parse_not()?;
                left = AnalysisExpr::And(Box::new(left), Box::new(right));
            } else {
                return Ok(left);
            }
        }
    }

    fn parse_not(&mut self) -> Result<AnalysisExpr<'s>, String> {
        self.skip_ws();
        if self.consume_symbol("!") || self.consume_keyword("not") {
            return Ok(AnalysisExpr::Not(Box::new(self.parse_not()?)));
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<AnalysisExpr<'s>, String> {
        self.skip_ws();
        if self.peek_char() == Some('(') {
            let start = self.pos;
            self.consume_symbol("(");
            let inner = self.parse_expression()?;
            self.skip_ws();
            if !self.consume_symbol(")") {
                return Err("analysis expression is missing ')'".to_string());
            }
            self.skip_ws();
            if self
                .peek_char()
                .is_some_and(|ch| matches!(ch, '+' | '-' | '*' | '/' | '<' | '>' | '=' | '!'))
            {
                // Parentheses are part of an arithmetic/comparison clause.
                self.pos = start;
                return self.parse_comparison_or_bool_literal();
            }
            return Ok(inner);
        }
        self.parse_comparison_or_bool_literal()
    }

    fn parse_comparison_or_bool_literal(&mut self) -> Result<AnalysisExpr<'s>, String> {
        let start = self.pos;
        let mut paren_depth = 0usize;
        let mut bracket_depth = 0usize;
        while !self.is_eof() {
            if paren_depth == 0 && bracket_depth == 0 {
                if self.src[self.pos..].starts_with(')') {
                    break;
                }
                if self.src[self.pos..].starts_with("&&")
                    || self.src[self.pos..].starts_with("||")
                    || self.is_keyword_at_pos("and")
                    || self.is_keyword_at_pos("or")
                {
                    break;
                }
            }
            let Some(ch) = self.eat_char() else {
                break;
            };
            match ch {
                '(' => paren_depth += 1,
                ')' => {
                    if paren_depth == 0 {
                        self.pos -= 1;
                        break;
                    }
                    paren_depth -= 1;
                }
                '[' => bracket_depth += 1,
                ']' => {
                    bracket_depth = bracket_depth.saturating_sub(1);
                }
                _ => {}
            }
        }
        let clause = self.src[start..self.pos].trim();
        if clause.is_empty() {
            return Err("analysis expression is missing operand".to_string());
        }
        if clause.eq_ignore_ascii_case("true") {
            return Ok(AnalysisExpr::BoolLiteral(true));
        }
        if clause.eq_ignore_ascii_case("false") {
            return Ok(AnalysisExpr::BoolLiteral(false));
        }
        let Some((index, op, op_len)) = find_comparison_operator(clause) else {
            return Ok(AnalysisExpr::Predicate(clause));
        };
        let lhs = clause[..index].trim();
        let rhs = clause[index + op_len..].trim();
        if lhs.is_empty() || rhs.is_empty() {
            return Err(format!(
                "analysis expression '{clause}' is malformed; expected both comparison operands"
            ));
        }
        Ok(AnalysisExpr::Comparison { lhs, op, rhs })
    }

    fn consume_symbol(&mut self, symbol: &str) -> bool {
        if self.src[self.pos..].starts_with(symbol) {
            self.pos += symbol.len();
            true
        } else {
            false
        }
    }

    fn consume_keyword(&mut self, keyword: &str) -> bool {
        if !self.is_keyword_at_pos(keyword) {
            return false;
        }
        self.pos += keyword.len();
        true
    }

    fn is_keyword_at_pos(&self, keyword: &str) -> bool {
        if !self.src[self.pos..].starts_with(keyword) {
            return false;
        }
        let before = if self.pos == 0 {
            None
        } else {
            self.src[..self.pos].chars().next_back()
        };
        if before.is_some_and(is_identifier_char) {
            return false;
        }
        let end = self.pos + keyword.len();
        let after = if end >= self.src.len() {
            None
        } else {
            self.src[end..].chars().next()
        };
        !after.is_some_and(is_identifier_char)
    }
}

fn find_comparison_operator(text: &str) -> Option<(usize, AnalysisComparisonOp, usize)> {
    let mut paren_depth = 0usize;
    let mut bracket_depth = 0usize;
    let mut iter = text.char_indices().peekable();
    while let Some((index, ch)) = iter.next() {
        match ch {
            '(' => paren_depth += 1,
            ')' => paren_depth = paren_depth.saturating_sub(1),
            '[' => bracket_depth += 1,
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            _ => {}
        }
        if paren_depth != 0 || bracket_depth != 0 {
            continue;
        }
        let next = iter.peek().map(|(_, c)| *c);
        match (ch, next) {
            ('<', Some('=')) => return Some((index, AnalysisComparisonOp::Le, 2)),
            ('>', Some('=')) => return Some((index, AnalysisComparisonOp::Ge, 2)),
            ('=', Some('=')) => return Some((index, AnalysisComparisonOp::Eq, 2)),
            ('!', Some('=')) => return Some((index, AnalysisComparisonOp::Ne, 2)),
            ('<', _) => return Some((index, AnalysisComparisonOp::Lt, 1)),
            ('>', _) => return Some((index, AnalysisComparisonOp::Gt, 1)),
            _ => {}
        }
    }
    None
}

fn evaluate_analysis_ast(
    engine: &mut EvalEngine<'_>,
    context_id: &NodeId,
    expression: &AnalysisExpr<'_>,
) -> Result<bool, AnalysisEvalError> {
    match expression {
        AnalysisExpr::BoolLiteral(value) => Ok(*value),
        AnalysisExpr::Predicate(predicate) => {
            evaluate_analysis_predicate(engine, context_id, predicate)
        }
        AnalysisExpr::Not(inner) => Ok(!evaluate_analysis_ast(engine, context_id, inner)?),
        AnalysisExpr::And(left, right) => {
            let left_result = evaluate_analysis_ast(engine, context_id, left);
            if matches!(left_result, Ok(false)) {
                return Ok(false);
            }
            let right_result = evaluate_analysis_ast(engine, context_id, right);
            if matches!(right_result, Ok(false)) {
                return Ok(false);
            }
            match (left_result, right_result) {
                (Ok(true), Ok(true)) => Ok(true),
                (Ok(false), Err(_)) | (Err(_), Ok(false)) => Ok(false),
                (Err(err), Ok(true)) | (Ok(true), Err(err)) => Err(err),
                (Err(left_err), Err(right_err)) => Err(prefer_analysis_error(left_err, right_err)),
                (Ok(_), Ok(_)) => Ok(false),
            }
        }
        AnalysisExpr::Or(left, right) => {
            let left_result = evaluate_analysis_ast(engine, context_id, left);
            if matches!(left_result, Ok(true)) {
                return Ok(true);
            }
            let right_result = evaluate_analysis_ast(engine, context_id, right);
            if matches!(right_result, Ok(true)) {
                return Ok(true);
            }
            match (left_result, right_result) {
                (Ok(false), Ok(false)) => Ok(false),
                (Ok(true), Err(_)) | (Err(_), Ok(true)) => Ok(true),
                (Err(err), Ok(false)) | (Ok(false), Err(err)) => Err(err),
                (Err(left_err), Err(right_err)) => Err(prefer_analysis_error(left_err, right_err)),
                (Ok(_), Ok(_)) => Ok(true),
            }
        }
        AnalysisExpr::Comparison { lhs, op, rhs } => {
            let left = engine
                .evaluate_quantity_expression(context_id, lhs)
                .map_err(AnalysisEvalError::from_status)?;
            let right = engine
                .evaluate_quantity_expression(context_id, rhs)
                .map_err(AnalysisEvalError::from_status)?;
            compare_quantities(engine, left, *op, right)
        }
    }
}

fn evaluate_analysis_predicate(
    engine: &mut EvalEngine<'_>,
    context_id: &NodeId,
    predicate: &str,
) -> Result<bool, AnalysisEvalError> {
    if let Some((name, args)) = parse_invocation(predicate) {
        return engine.evaluate_invocation_bool(context_id, name, &args);
    }
    if let Some(identifier) = parse_standalone_identifier(predicate) {
        let outcome = engine.resolve_identifier_value(context_id, identifier);
        if outcome.status != EvalStatus::Ok {
            return Err(AnalysisEvalError::from_status(outcome.status));
        }
        if let Some(boolean) = outcome.value.as_ref().and_then(Value::as_bool) {
            return Ok(boolean);
        }
        if let Some(number) = outcome.value.as_ref().and_then(json_value_to_f64) {
            return Ok(number >= 0.0);
        }
        return Err(AnalysisEvalError::with_message(
            EvalStatus::Unsupported,
            "analysis expression predicate is not boolean",
        ));
    }
    let quantity = engine
        .evaluate_quantity_expression(context_id, predicate)
        .map_err(AnalysisEvalError::from_status)?;
    Ok(quantity.value >= 0.0)
}

fn compare_quantities(
    engine: &EvalEngine<'_>,
    left: Quantity,
    op: AnalysisComparisonOp,
    right: Quantity,
) -> Result<bool, AnalysisEvalError> {
    let right_value = match (&left.unit, &right.unit) {
        (None, None) => right.value,
        (Some(left_unit), Some(right_unit)) => {
            match engine.units.convert_value(right.value, right_unit, left_unit) {
                Ok(converted) => converted,
                Err(UnitError::IncompatibleDimension) => {
                    return Err(AnalysisEvalError::with_message(
                        EvalStatus::TypeError,
                        format!(
                        "analysis comparison has incompatible units: left='{left_unit}', right='{right_unit}'"
                        ),
                    ))
                }
                Err(other) => return Err(AnalysisEvalError::from_status(map_unit_error(other))),
            }
        }
        (Some(left_unit), None) => {
            if engine.units.has_symbol(left_unit) {
                return Err(AnalysisEvalError::with_message(
                    EvalStatus::TypeError,
                    format!(
                        "analysis comparison mixes dimensioned and unitless values: left='{left_unit}', right='<unitless>'"
                    ),
                ));
            }
            return Err(AnalysisEvalError::from_status(EvalStatus::Unknown));
        }
        (None, Some(right_unit)) => {
            if engine.units.has_symbol(right_unit) {
                return Err(AnalysisEvalError::with_message(
                    EvalStatus::TypeError,
                    format!(
                        "analysis comparison mixes unitless and dimensioned values: left='<unitless>', right='{right_unit}'"
                    ),
                ));
            }
            return Err(AnalysisEvalError::from_status(EvalStatus::Unknown));
        }
    };
    let epsilon = 1e-9;
    let result = match op {
        AnalysisComparisonOp::Lt => left.value < right_value,
        AnalysisComparisonOp::Le => left.value <= right_value,
        AnalysisComparisonOp::Gt => left.value > right_value,
        AnalysisComparisonOp::Ge => left.value >= right_value,
        AnalysisComparisonOp::Eq => (left.value - right_value).abs() < epsilon,
        AnalysisComparisonOp::Ne => (left.value - right_value).abs() >= epsilon,
    };
    Ok(result)
}

fn map_analysis_eval_error(status: EvalStatus) -> String {
    match status {
        EvalStatus::DivByZero => "analysis expression includes division by zero".to_string(),
        EvalStatus::TypeError => {
            "analysis expression has type or unit mismatch for arithmetic/comparison".to_string()
        }
        EvalStatus::Incomplete => {
            "analysis expression depends on declared value(s) that have not been assigned"
                .to_string()
        }
        EvalStatus::Unknown => {
            "analysis expression could not be resolved (unresolved variable or value)".to_string()
        }
        EvalStatus::Cycle => "analysis expression has cyclic reference".to_string(),
        EvalStatus::Unsupported | EvalStatus::Ok => {
            "analysis expression form is not supported".to_string()
        }
    }
}

fn prefer_analysis_error(left: AnalysisEvalError, right: AnalysisEvalError) -> AnalysisEvalError {
    if left.status == EvalStatus::Incomplete || right.status != EvalStatus::Incomplete {
        left
    } else {
        right
    }
}

fn is_identifier_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
}

struct EvalEngine<'a> {
    graph: &'a SemanticGraph,
    units: UnitRegistry,
    memoized: HashMap<NodeId, EvalOutcome>,
    active_stack: HashSet<NodeId>,
    parameter_bindings: Vec<HashMap<String, Quantity>>,
}

impl<'a> EvalEngine<'a> {
    fn new(graph: &'a SemanticGraph) -> Self {
        Self {
            graph,
            units: UnitRegistry::from_semantic_graph(graph),
            memoized: HashMap::new(),
            active_stack: HashSet::new(),
            parameter_bindings: Vec::new(),
        }
    }

    fn node_source_value(&self, node_id: &NodeId) -> Option<Value> {
        let node = self.graph.get_node(node_id)?;
        EVALUATION_SOURCE_KEYS
            .iter()
            .find_map(|key| node.attributes.get(*key).cloned())
    }

    fn evaluate_node(&mut self, node_id: &NodeId) -> EvalOutcome {
        if let Some(cached) = self.memoized.get(node_id) {
            return cached.clone();
        }
        if self.active_stack.contains(node_id) {
            return EvalOutcome::error(
                EvalStatus::Cycle,
                format!(
                    "cyclic dependency detected while evaluating '{}'",
                    node_id.qualified_name
                ),
            );
        }
        let Some(raw_value) = self.node_source_value(node_id) else {
            return EvalOutcome::error(
                EvalStatus::Incomplete,
                format!(
                    "no evaluable expression source found for '{}'",
                    node_id.qualified_name
                ),
            );
        };
        self.active_stack.insert(node_id.clone());
        let outcome = self.evaluate_json_value(node_id, &raw_value);
        self.active_stack.remove(node_id);
        self.memoized.insert(node_id.clone(), outcome.clone());
        outcome
    }

    fn evaluate_json_value(&mut self, node_id: &NodeId, value: &Value) -> EvalOutcome {
        match value {
            Value::Bool(v) => EvalOutcome::ok(Value::Bool(*v), None),
            Value::Number(v) => EvalOutcome::ok(Value::Number(v.clone()), None),
            Value::String(s) => self.evaluate_expression_text(node_id, s),
            Value::Null => EvalOutcome::error(EvalStatus::Unknown, "no expression value"),
            _ => EvalOutcome::error(
                EvalStatus::Unsupported,
                "expression value type is not supported",
            ),
        }
    }

    fn evaluate_expression_text(&mut self, node_id: &NodeId, raw: &str) -> EvalOutcome {
        let normalized = normalize_unit_brackets(raw.trim());
        let text = normalized.as_str();
        if text.is_empty() {
            return EvalOutcome::error(EvalStatus::Unknown, "empty expression");
        }
        if text.eq_ignore_ascii_case("true") {
            return EvalOutcome::ok(Value::Bool(true), None);
        }
        if text.eq_ignore_ascii_case("false") {
            return EvalOutcome::ok(Value::Bool(false), None);
        }
        if let Ok(parsed_string) = serde_json::from_str::<String>(text) {
            return EvalOutcome::ok(Value::String(parsed_string), None);
        }
        if let Some(identifier) = parse_standalone_identifier(text) {
            return self.resolve_identifier_value(node_id, identifier);
        }

        match self.evaluate_quantity_expression(node_id, text) {
            Ok(quantity) => EvalOutcome::from_quantity(quantity),
            Err(EvalStatus::DivByZero) => {
                EvalOutcome::error(EvalStatus::DivByZero, "division by zero")
            }
            Err(EvalStatus::Cycle) => {
                EvalOutcome::error(EvalStatus::Cycle, "cyclic reference detected")
            }
            Err(EvalStatus::TypeError) => EvalOutcome::error(
                EvalStatus::TypeError,
                "expression has type or unit mismatch for arithmetic",
            ),
            Err(EvalStatus::Unknown) => {
                EvalOutcome::error(EvalStatus::Unknown, "expression could not be resolved")
            }
            Err(EvalStatus::Incomplete) => EvalOutcome::error(
                EvalStatus::Incomplete,
                "expression depends on unevaluated value",
            ),
            Err(EvalStatus::Unsupported) | Err(EvalStatus::Ok) => {
                EvalOutcome::error(EvalStatus::Unsupported, "expression form is not supported")
            }
        }
    }

    fn evaluate_quantity_expression(
        &mut self,
        node_id: &NodeId,
        expression: &str,
    ) -> Result<Quantity, EvalStatus> {
        let units = self.units.clone();
        let mut parser = QuantityParser::new(expression, &units, |name, args| {
            if let Some(arg_list) = args {
                self.evaluate_invocation_quantity(node_id, name, arg_list)
            } else {
                self.resolve_identifier_quantity(node_id, name)
            }
        });
        let quantity = parser.parse_expression()?;
        parser.skip_ws();
        if !parser.is_eof() {
            return Err(EvalStatus::Unsupported);
        }
        Ok(quantity)
    }

    fn resolve_identifier_value(&mut self, node_id: &NodeId, identifier: &str) -> EvalOutcome {
        let referenced_id = match self.resolve_identifier_node(node_id, identifier) {
            Ok(found) => found,
            Err(outcome) => return outcome,
        };
        self.evaluate_node(&referenced_id)
    }

    fn resolve_identifier_quantity(
        &mut self,
        node_id: &NodeId,
        identifier: &str,
    ) -> Result<Quantity, EvalStatus> {
        if let Some(bound) = self.parameter_bindings.iter().rev().find_map(|scope| {
            scope.get(identifier).cloned().or_else(|| {
                identifier
                    .rsplit("::")
                    .next()
                    .and_then(|tail| scope.get(tail).cloned())
            })
        }) {
            return Ok(bound);
        }
        let referenced_id = self
            .resolve_identifier_node(node_id, identifier)
            .map_err(|outcome| outcome.status)?;
        let outcome = self.evaluate_node(&referenced_id);
        if outcome.status != EvalStatus::Ok {
            return Err(outcome.status);
        }
        let Some(value) = outcome.value else {
            return Err(EvalStatus::Unknown);
        };
        let Some(number) = json_value_to_f64(&value) else {
            return Err(EvalStatus::TypeError);
        };
        Ok(Quantity {
            value: number,
            unit: outcome.unit,
        })
    }

    fn resolve_identifier_node(
        &self,
        current_id: &NodeId,
        identifier: &str,
    ) -> Result<NodeId, EvalOutcome> {
        let Some(current) = self.graph.get_node(current_id) else {
            return Err(EvalOutcome::error(
                EvalStatus::Unknown,
                format!("unknown evaluation node '{}'", current_id.qualified_name),
            ));
        };
        let scoped_candidates = self.scoped_candidates(current, identifier);
        if !scoped_candidates.is_empty() {
            return choose_candidate(scoped_candidates, identifier);
        }
        let fallback_candidates = self.fallback_candidates(current, identifier);
        if !fallback_candidates.is_empty() {
            return choose_candidate(fallback_candidates, identifier);
        }
        Err(EvalOutcome::error(
            EvalStatus::Unknown,
            format!("unresolved reference '{identifier}'"),
        ))
    }

    fn scoped_candidates(&self, current: &SemanticNode, identifier: &str) -> Vec<NodeId> {
        let mut candidates = Vec::new();
        for scope_prefix in scope_prefixes(self.graph, current) {
            let qualified = format!("{scope_prefix}::{identifier}");
            candidates.extend(self.lookup_qualified_candidates(&qualified));
        }
        dedupe_node_ids(candidates)
    }

    fn fallback_candidates(&self, current: &SemanticNode, identifier: &str) -> Vec<NodeId> {
        let mut candidates = Vec::new();
        if identifier.contains("::") {
            candidates.extend(self.lookup_qualified_candidates(identifier));
        } else {
            let same_uri_named = self
                .graph
                .nodes_for_uri(&current.id.uri)
                .into_iter()
                .filter(|node| node.name == identifier)
                .map(|node| node.id.clone())
                .collect::<Vec<_>>();
            candidates.extend(same_uri_named);
            candidates.extend(self.lookup_qualified_candidates(identifier));
        }
        dedupe_node_ids(candidates)
    }

    fn lookup_qualified_candidates(&self, qualified_name: &str) -> Vec<NodeId> {
        self.graph
            .node_ids_by_qualified_name
            .get(qualified_name)
            .into_iter()
            .flatten()
            .cloned()
            .collect()
    }

    fn evaluate_invocation_quantity(
        &mut self,
        context_id: &NodeId,
        callable_name: &str,
        args: &[&str],
    ) -> Result<Quantity, EvalStatus> {
        let normalized_args = normalize_invocation_args(args);
        if callable_name == "sum" {
            return self.evaluate_builtin_sum(context_id, &normalized_args);
        }
        let callable_id = self
            .resolve_callable_node(context_id, callable_name)
            .ok_or(EvalStatus::Unknown)?;
        let callable = self
            .graph
            .get_node(&callable_id)
            .ok_or(EvalStatus::Unknown)?;
        if callable.element_kind != "calc def" {
            return Err(EvalStatus::TypeError);
        }
        let expression = callable
            .attributes
            .get(ANALYSIS_EXPRESSION_KEY)
            .and_then(Value::as_str)
            .ok_or(EvalStatus::Unknown)?
            .to_string();
        let mut param_names = in_parameter_names(callable);
        if param_names.is_empty() && !normalized_args.is_empty() {
            let inferred = infer_parameter_names_from_expression(&expression);
            if inferred.len() == normalized_args.len() {
                param_names = inferred;
            }
        }
        if param_names.len() != normalized_args.len() {
            return Err(EvalStatus::Unsupported);
        }
        let mut bindings = HashMap::new();
        for (name, arg_expr) in param_names.iter().zip(normalized_args.iter()) {
            let evaluated = self.evaluate_quantity_expression(context_id, arg_expr)?;
            bindings.insert(name.clone(), evaluated);
        }
        self.parameter_bindings.push(bindings);
        let result = self.evaluate_quantity_expression(&callable_id, &expression);
        self.parameter_bindings.pop();
        result
    }

    fn evaluate_builtin_sum(
        &mut self,
        context_id: &NodeId,
        normalized_args: &[&str],
    ) -> Result<Quantity, EvalStatus> {
        if normalized_args.is_empty() {
            return Err(EvalStatus::Unsupported);
        }
        let mut it = normalized_args.iter();
        let first = it
            .next()
            .expect("non-empty args")
            .to_string();
        let mut acc = self.evaluate_quantity_expression(context_id, &first)?;
        for arg in it {
            let evaluated = self.evaluate_quantity_expression(context_id, arg)?;
            acc = add_quantities_with_units(&self.units, acc, evaluated)?;
        }
        Ok(acc)
    }

    fn evaluate_invocation_bool(
        &mut self,
        context_id: &NodeId,
        callable_name: &str,
        args: &[&str],
    ) -> Result<bool, AnalysisEvalError> {
        let normalized_args = normalize_invocation_args(args);
        let callable_id = self
            .resolve_callable_node(context_id, callable_name)
            .ok_or_else(|| {
                AnalysisEvalError::with_message(
                    EvalStatus::Unknown,
                    format!("unresolved callable '{callable_name}'"),
                )
            })?;
        let callable = self.graph.get_node(&callable_id).ok_or_else(|| {
            AnalysisEvalError::with_message(
                EvalStatus::Unknown,
                format!("unresolved callable '{callable_name}'"),
            )
        })?;
        let expression = callable
            .attributes
            .get(ANALYSIS_EXPRESSION_KEY)
            .and_then(Value::as_str)
            .ok_or_else(|| {
                AnalysisEvalError::with_message(
                    EvalStatus::Incomplete,
                    format!("callable '{callable_name}' has no analysis expression"),
                )
            })?
            .to_string();
        let mut param_names = in_parameter_names(callable);
        if param_names.is_empty() && !normalized_args.is_empty() {
            let inferred = infer_parameter_names_from_expression(&expression);
            if inferred.len() == normalized_args.len() {
                param_names = inferred;
            }
        }
        if param_names.len() != normalized_args.len() {
            let arg_preview = normalized_args
                .first()
                .map(|arg| format!("; first arg='{arg}'"))
                .unwrap_or_default();
            return Err(AnalysisEvalError::with_message(
                EvalStatus::Unsupported,
                format!(
                    "callable '{callable_name}' expects {} arguments but got {}{}",
                    param_names.len(),
                    normalized_args.len(),
                    arg_preview
                ),
            ));
        }
        let mut bindings = HashMap::new();
        for (name, arg_expr) in param_names.iter().zip(normalized_args.iter()) {
            let evaluated = self
                .evaluate_quantity_expression(context_id, arg_expr)
                .map_err(AnalysisEvalError::from_status)?;
            bindings.insert(name.clone(), evaluated);
        }
        self.parameter_bindings.push(bindings);
        let result = evaluate_analysis_expression(self, &callable_id, &expression);
        self.parameter_bindings.pop();
        result
    }

    fn resolve_callable_node(&self, context_id: &NodeId, callable_name: &str) -> Option<NodeId> {
        let current = self.graph.get_node(context_id)?;
        let mut candidates = Vec::new();
        for scope_prefix in scope_prefixes(self.graph, current) {
            let qualified = format!("{scope_prefix}::{callable_name}");
            candidates.extend(self.lookup_callable_candidates(&qualified));
        }
        if callable_name.contains("::") {
            candidates.extend(self.lookup_callable_candidates(callable_name));
        } else {
            candidates.extend(
                self.graph
                    .nodes_for_uri(&current.id.uri)
                    .into_iter()
                    .filter(|node| {
                        node.name == callable_name
                            && matches!(node.element_kind.as_str(), "calc def" | "constraint def")
                    })
                    .map(|node| node.id.clone()),
            );
            candidates.extend(self.lookup_callable_candidates(callable_name));
        }
        dedupe_node_ids(candidates).into_iter().next()
    }

    fn lookup_callable_candidates(&self, qualified_name: &str) -> Vec<NodeId> {
        self.graph
            .node_ids_by_qualified_name
            .get(qualified_name)
            .into_iter()
            .flatten()
            .filter_map(|node_id| {
                let node = self.graph.get_node(node_id)?;
                matches!(node.element_kind.as_str(), "calc def" | "constraint def")
                    .then_some(node_id.clone())
            })
            .collect()
    }
}

fn scope_prefixes(graph: &SemanticGraph, current: &SemanticNode) -> Vec<String> {
    let mut prefixes = Vec::new();
    if let Some(parent) = graph.parent_of(current) {
        prefixes.push(parent.id.qualified_name.clone());
    }
    for ancestor in graph.ancestors_of(current) {
        prefixes.push(ancestor.id.qualified_name.clone());
    }
    prefixes
}

fn choose_candidate(candidates: Vec<NodeId>, identifier: &str) -> Result<NodeId, EvalOutcome> {
    if candidates.len() == 1 {
        return Ok(candidates[0].clone());
    }
    let mut sorted = candidates;
    sorted.sort_by_key(|candidate| candidate.qualified_name.len());
    let best = sorted[0].clone();
    let best_len = best.qualified_name.len();
    let second_len = sorted
        .get(1)
        .map(|candidate| candidate.qualified_name.len());
    if second_len.is_none() || second_len.unwrap_or(best_len + 1) > best_len {
        return Ok(best);
    }
    Err(EvalOutcome::error(
        EvalStatus::Unknown,
        format!("ambiguous reference '{identifier}'"),
    ))
}

fn dedupe_node_ids(ids: Vec<NodeId>) -> Vec<NodeId> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for id in ids {
        if seen.insert(id.clone()) {
            out.push(id);
        }
    }
    out
}

fn in_parameter_names(node: &SemanticNode) -> Vec<String> {
    node.attributes
        .get("parameters")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|entry| {
            let direction = entry.get("direction").and_then(Value::as_str)?;
            let name = entry.get("name").and_then(Value::as_str)?;
            matches!(direction, "in" | "inout").then_some(name.to_string())
        })
        .collect()
}

fn infer_parameter_names_from_expression(expression: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut seen = HashSet::new();
    let chars: Vec<char> = expression.chars().collect();
    let mut i = 0usize;
    while i < chars.len() {
        let ch = chars[i];
        if ch.is_ascii_alphabetic() || ch == '_' {
            let start = i;
            i += 1;
            while i < chars.len() {
                let c = chars[i];
                if c.is_ascii_alphanumeric() || c == '_' {
                    i += 1;
                    continue;
                }
                if i + 1 < chars.len() && c == ':' && chars[i + 1] == ':' {
                    i += 2;
                    continue;
                }
                break;
            }
            let token: String = chars[start..i].iter().collect();
            if token.eq_ignore_ascii_case("true") || token.eq_ignore_ascii_case("false") {
                continue;
            }
            if seen.insert(token.clone()) {
                names.push(token);
            }
            continue;
        }
        i += 1;
    }
    names
}

fn normalize_invocation_args<'a>(args: &'a [&'a str]) -> Vec<&'a str> {
    if args.len() != 1 {
        return args.to_vec();
    }
    let only = args[0].trim();
    if only.is_empty() {
        return args.to_vec();
    }
    if only.chars().any(|ch| {
        matches!(
            ch,
            '+' | '-' | '*' | '/' | '<' | '>' | '=' | '!' | '[' | ']' | '(' | ')'
        )
    }) {
        return args.to_vec();
    }
    let comma_split: Vec<&str> = only
        .split(',')
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .collect();
    if comma_split.len() > 1 && comma_split.iter().all(|token| is_identifier_like(token)) {
        return comma_split;
    }
    let split: Vec<&str> = only
        .split_whitespace()
        .filter(|token| !token.is_empty())
        .collect();
    if split.len() > 1 && split.iter().all(|token| is_identifier_like(token)) {
        split
    } else {
        args.to_vec()
    }
}

fn is_identifier_like(token: &str) -> bool {
    let mut chars = token.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first.is_ascii_alphabetic() || first == '_') {
        return false;
    }
    let mut prev_colon = false;
    for ch in chars {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            prev_colon = false;
            continue;
        }
        if ch == ':' {
            if prev_colon {
                prev_colon = false;
                continue;
            }
            prev_colon = true;
            continue;
        }
        return false;
    }
    !prev_colon
}

fn parse_standalone_identifier(text: &str) -> Option<&str> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return None;
    }
    let units = UnitRegistry::default();
    let mut parser =
        QuantityParser::new(trimmed, &units, |_name, _args| Err(EvalStatus::Unsupported));
    let identifier = parser.parse_identifier()?;
    parser.skip_ws();
    if parser.is_eof() {
        Some(identifier)
    } else {
        None
    }
}

fn json_value_to_f64(value: &Value) -> Option<f64> {
    match value {
        Value::Number(number) => number.as_f64().filter(|parsed| parsed.is_finite()),
        _ => None,
    }
}

fn number_to_json(value: f64) -> Value {
    if value.fract() == 0.0 {
        Value::Number(serde_json::Number::from(value as i64))
    } else {
        Value::Number(
            serde_json::Number::from_f64(value).unwrap_or_else(|| serde_json::Number::from(0)),
        )
    }
}

fn parse_invocation(text: &str) -> Option<(&str, Vec<&str>)> {
    let trimmed = text.trim();
    let open_idx = trimmed.find('(')?;
    if !trimmed.ends_with(')') || open_idx == 0 {
        return None;
    }
    let name = trimmed[..open_idx].trim();
    parse_standalone_identifier(name)?;
    let args_body = &trimmed[open_idx + 1..trimmed.len() - 1];
    let mut args = Vec::new();
    let mut start = 0usize;
    let mut paren_depth = 0usize;
    let mut bracket_depth = 0usize;
    for (idx, ch) in args_body.char_indices() {
        match ch {
            '(' => paren_depth += 1,
            ')' => paren_depth = paren_depth.saturating_sub(1),
            '[' => bracket_depth += 1,
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            ',' if paren_depth == 0 && bracket_depth == 0 => {
                let arg = args_body[start..idx].trim();
                if arg.is_empty() {
                    return None;
                }
                args.push(arg);
                start = idx + 1;
            }
            _ => {}
        }
    }
    let tail = args_body[start..].trim();
    if !tail.is_empty() {
        args.push(tail);
    } else if !args_body.trim().is_empty() {
        return None;
    }
    Some((name, args))
}

struct QuantityParser<'s, 'u, F>
where
    F: FnMut(&str, Option<&[&str]>) -> Result<Quantity, EvalStatus>,
{
    src: &'s str,
    units: &'u UnitRegistry,
    pos: usize,
    resolve_symbol: F,
}

impl<'s, 'u, F> QuantityParser<'s, 'u, F>
where
    F: FnMut(&str, Option<&[&str]>) -> Result<Quantity, EvalStatus>,
{
    fn new(src: &'s str, units: &'u UnitRegistry, resolve_symbol: F) -> Self {
        Self {
            src,
            units,
            pos: 0,
            resolve_symbol,
        }
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.src.len()
    }

    fn skip_ws(&mut self) {
        while let Some(ch) = self.peek_char() {
            if ch.is_whitespace() {
                self.pos += ch.len_utf8();
            } else {
                break;
            }
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.src[self.pos..].chars().next()
    }

    fn eat_char(&mut self) -> Option<char> {
        let ch = self.peek_char()?;
        self.pos += ch.len_utf8();
        Some(ch)
    }

    fn parse_expression(&mut self) -> Result<Quantity, EvalStatus> {
        let mut left = self.parse_term()?;
        loop {
            self.skip_ws();
            let Some(op) = self.peek_char() else {
                return Ok(left);
            };
            if op != '+' && op != '-' {
                return Ok(left);
            }
            self.eat_char();
            let right = self.parse_term()?;
            left = if op == '+' {
                self.add_quantities(left, right)?
            } else {
                self.sub_quantities(left, right)?
            };
        }
    }

    fn parse_term(&mut self) -> Result<Quantity, EvalStatus> {
        let mut left = self.parse_factor()?;
        loop {
            self.skip_ws();
            let Some(op) = self.peek_char() else {
                return Ok(left);
            };
            if op != '*' && op != '/' {
                return Ok(left);
            }
            self.eat_char();
            let right = self.parse_factor()?;
            if op == '/' && right.value == 0.0 {
                return Err(EvalStatus::DivByZero);
            }
            let composed = self.units.compose_product(
                left.value,
                left.unit.as_deref(),
                right.value,
                right.unit.as_deref(),
                op == '/',
            );
            left = match composed {
                Ok((value, unit)) => Quantity { value, unit },
                Err(err) => return Err(map_unit_error(err)),
            };
        }
    }

    fn parse_factor(&mut self) -> Result<Quantity, EvalStatus> {
        self.skip_ws();
        let Some(ch) = self.peek_char() else {
            return Err(EvalStatus::Unsupported);
        };
        if ch == '+' || ch == '-' {
            self.eat_char();
            let mut inner = self.parse_factor()?;
            if ch == '-' {
                inner.value = -inner.value;
            }
            return Ok(inner);
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Quantity, EvalStatus> {
        self.skip_ws();
        let Some(ch) = self.peek_char() else {
            return Err(EvalStatus::Unsupported);
        };
        if ch == '(' {
            self.eat_char();
            let value = self.parse_expression()?;
            self.skip_ws();
            if self.eat_char() != Some(')') {
                return Err(EvalStatus::Unsupported);
            }
            return Ok(value);
        }
        if ch == '[' {
            self.eat_char();
            let value = self.parse_expression()?;
            self.skip_ws();
            if self.eat_char() != Some(']') {
                return Err(EvalStatus::Unsupported);
            }
            return Ok(value);
        }
        if let Some(identifier) = self.parse_identifier() {
            self.skip_ws();
            if self.peek_char() == Some('(') {
                let args = self.parse_argument_slices()?;
                return (self.resolve_symbol)(identifier, Some(&args));
            }
            return (self.resolve_symbol)(identifier, None);
        }
        let value = self.parse_numeric_literal()?;
        let unit = self.parse_unit_suffix();
        Ok(Quantity { value, unit })
    }

    fn parse_identifier(&mut self) -> Option<&'s str> {
        self.skip_ws();
        let start = self.pos;
        let first = self.peek_char()?;
        if !(first.is_ascii_alphabetic() || first == '_') {
            return None;
        }
        self.eat_char();
        while let Some(ch) = self.peek_char() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                self.eat_char();
                continue;
            }
            if self.src[self.pos..].starts_with("::") {
                self.pos += 2;
                continue;
            }
            break;
        }
        let parsed = &self.src[start..self.pos];
        if parsed.ends_with("::") {
            return None;
        }
        Some(parsed)
    }

    fn parse_numeric_literal(&mut self) -> Result<f64, EvalStatus> {
        self.skip_ws();
        let start = self.pos;
        let mut seen_digit = false;
        let mut seen_dot = false;
        while let Some(ch) = self.peek_char() {
            if ch.is_ascii_digit() {
                seen_digit = true;
                self.eat_char();
                continue;
            }
            if ch == '.' && !seen_dot {
                seen_dot = true;
                self.eat_char();
                continue;
            }
            break;
        }
        if !seen_digit {
            return Err(EvalStatus::Unsupported);
        }
        let raw = &self.src[start..self.pos];
        raw.parse::<f64>()
            .ok()
            .filter(|v| v.is_finite())
            .ok_or(EvalStatus::Unsupported)
    }

    fn parse_argument_slices(&mut self) -> Result<Vec<&'s str>, EvalStatus> {
        if self.eat_char() != Some('(') {
            return Err(EvalStatus::Unsupported);
        }
        let mut args = Vec::new();
        loop {
            self.skip_ws();
            if self.peek_char() == Some(')') {
                self.eat_char();
                return Ok(args);
            }
            let start = self.pos;
            let mut paren_depth = 0usize;
            let mut bracket_depth = 0usize;
            while let Some(ch) = self.peek_char() {
                match ch {
                    '(' => {
                        paren_depth += 1;
                        self.eat_char();
                    }
                    ')' if paren_depth == 0 && bracket_depth == 0 => break,
                    ')' => {
                        paren_depth = paren_depth.saturating_sub(1);
                        self.eat_char();
                    }
                    '[' => {
                        bracket_depth += 1;
                        self.eat_char();
                    }
                    ']' => {
                        bracket_depth = bracket_depth.saturating_sub(1);
                        self.eat_char();
                    }
                    ',' if paren_depth == 0 && bracket_depth == 0 => break,
                    _ => {
                        self.eat_char();
                    }
                }
            }
            let arg = self.src[start..self.pos].trim();
            if arg.is_empty() {
                return Err(EvalStatus::Unsupported);
            }
            args.push(arg);
            self.skip_ws();
            match self.peek_char() {
                Some(',') => {
                    self.eat_char();
                }
                Some(')') => {
                    self.eat_char();
                    return Ok(args);
                }
                _ => return Err(EvalStatus::Unsupported),
            }
        }
    }

    fn parse_unit_suffix(&mut self) -> Option<String> {
        self.skip_ws();
        if self.peek_char() != Some('[') {
            return None;
        }
        self.eat_char();
        let start = self.pos;
        while let Some(ch) = self.peek_char() {
            if ch == ']' {
                break;
            }
            self.eat_char();
        }
        if self.eat_char() != Some(']') {
            return None;
        }
        let raw = self.src[start..self.pos - 1].trim();
        if raw.is_empty() {
            None
        } else {
            Some(trim_quotes(raw))
        }
    }

    fn add_quantities(&self, left: Quantity, right: Quantity) -> Result<Quantity, EvalStatus> {
        match (&left.unit, &right.unit) {
            (None, None) => Ok(Quantity::scalar(left.value + right.value)),
            (Some(unit), None) | (None, Some(unit)) => {
                if !self.units.has_symbol(unit) {
                    return Err(EvalStatus::Unknown);
                }
                Err(EvalStatus::TypeError)
            }
            (Some(left_unit), Some(right_unit)) => {
                let converted = self.units.convert_value(right.value, right_unit, left_unit);
                match converted {
                    Ok(v) => Ok(Quantity {
                        value: left.value + v,
                        unit: Some(left_unit.clone()),
                    }),
                    Err(err) => Err(map_unit_error(err)),
                }
            }
        }
    }

    fn sub_quantities(&self, left: Quantity, right: Quantity) -> Result<Quantity, EvalStatus> {
        self.add_quantities(
            left,
            Quantity {
                value: -right.value,
                unit: right.unit,
            },
        )
    }
}

fn trim_quotes(value: &str) -> String {
    let mut out = value.trim().to_string();
    if out.starts_with('\'') && out.ends_with('\'') && out.len() > 1 {
        out = out[1..out.len() - 1].to_string();
    }
    out
}

fn normalize_unit_brackets(text: &str) -> String {
    text.replace("[[", "[").replace("]]", "]")
}

fn map_unit_error(err: UnitError) -> EvalStatus {
    match err {
        UnitError::UnknownUnit => EvalStatus::Unknown,
        UnitError::IncompatibleDimension => EvalStatus::TypeError,
        UnitError::UnsupportedConversion | UnitError::AmbiguousMetadata => EvalStatus::Unsupported,
    }
}

fn add_quantities_with_units(
    units: &UnitRegistry,
    left: Quantity,
    right: Quantity,
) -> Result<Quantity, EvalStatus> {
    match (&left.unit, &right.unit) {
        (None, None) => Ok(Quantity::scalar(left.value + right.value)),
        (Some(unit), None) | (None, Some(unit)) => {
            if !units.has_symbol(unit) {
                return Err(EvalStatus::Unknown);
            }
            Err(EvalStatus::TypeError)
        }
        (Some(left_unit), Some(right_unit)) => {
            let converted = units.convert_value(right.value, right_unit, left_unit);
            match converted {
                Ok(v) => Ok(Quantity {
                    value: left.value + v,
                    unit: Some(left_unit.clone()),
                }),
                Err(err) => Err(map_unit_error(err)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;
    use crate::semantic::text_span::{TextPosition, TextRange};
    use url::Url;

    use crate::semantic::model::SemanticNode;

    fn range() -> TextRange {
        TextRange::new(TextPosition::new(0, 0), TextPosition::new(0, 1))
    }

    fn add_node(
        graph: &mut SemanticGraph,
        uri: &Url,
        qualified_name: &str,
        element_kind: &str,
        name: &str,
        parent_id: Option<&NodeId>,
        attributes: HashMap<String, Value>,
    ) -> NodeId {
        let id = NodeId::new(uri, qualified_name);
        let node = SemanticNode {
            id: id.clone(),
            element_kind: element_kind.to_string(),
            name: name.to_string(),
            range: range(),
            attributes,
            parent_id: parent_id.cloned(),
        };
        let idx = graph.graph.add_node(node);
        graph.node_index_by_id.insert(id.clone(), idx);
        graph
            .nodes_by_uri
            .entry(uri.clone())
            .or_default()
            .push(id.clone());
        graph
            .node_ids_by_qualified_name
            .entry(qualified_name.to_string())
            .or_default()
            .push(id.clone());
        id
    }

    fn node_attr<'a>(graph: &'a SemanticGraph, id: &NodeId, key: &str) -> Option<&'a Value> {
        graph.get_node(id).and_then(|node| node.attributes.get(key))
    }

    fn register_units_fixture(graph: &mut SemanticGraph) {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let root: PathBuf = std::env::temp_dir().join(format!("spec42-units-{unique}"));
        let path = root
            .join("sysml.library")
            .join("Domain Libraries")
            .join("Quantities and Units");
        fs::create_dir_all(&path).expect("create fixture path");
        let file = path.join("FixtureUnits.sysml");
        fs::write(
            &file,
            r#"
                attribute <m> 'metre' : LengthUnit;
                attribute <s> second : TimeUnit;
                attribute <cm> 'centimetre' : LengthUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m; :>> conversionFactor = 1E-02; } }
                attribute <ft> 'foot' : LengthUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m; :>> conversionFactor = 3.048E-01; } }
                attribute <kg> 'kilogram' : MassUnit;
                attribute <K> kelvin : ThermodynamicTemperatureUnit, TemperatureDifferenceUnit;
                attribute <'°C'> 'degree celsius (temperature difference)' : TemperatureDifferenceUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = K; :>> conversionFactor = 1; } }
                attribute <'°F'> 'degree Fahrenheit (temperature difference)' : TemperatureDifferenceUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = K; :>> conversionFactor = 5/9; } }
                attribute <'°C_abs'> 'degree celsius (absolute temperature scale)' : IntervalScale {
                    attribute :>> unit = '°C';
                    private attribute zeroDegreeCelsiusInKelvin: ThermodynamicTemperatureValue = 273.15 [K];
                }
                attribute <'°F_abs'> 'degree fahrenheit (absolute temperature scale)' : IntervalScale {
                    :>> unit = '°F';
                    private attribute zeroDegreeFahrenheitInKelvin: ThermodynamicTemperatureValue = 229835/900 [K];
                }
            "#,
        )
        .expect("write fixture");
        let uri = Url::from_file_path(&file).expect("fixture uri");
        let _ = add_node(
            graph,
            &uri,
            "Units::marker",
            "package",
            "marker",
            None,
            HashMap::new(),
        );
    }

    #[test]
    fn evaluates_reference_chain() {
        let mut graph = SemanticGraph::new();
        let uri = Url::parse("file:///C:/workspace/ref.sysml").expect("uri");
        let owner = add_node(
            &mut graph,
            &uri,
            "Demo::Rocket",
            "part def",
            "Rocket",
            None,
            HashMap::new(),
        );
        let _a = add_node(
            &mut graph,
            &uri,
            "Demo::Rocket::a",
            "attribute",
            "a",
            Some(&owner),
            HashMap::from([("value".to_string(), Value::String("2".to_string()))]),
        );
        let b = add_node(
            &mut graph,
            &uri,
            "Demo::Rocket::b",
            "attribute",
            "b",
            Some(&owner),
            HashMap::from([("value".to_string(), Value::String("a + 3".to_string()))]),
        );
        evaluate_expressions(&mut graph);
        assert_eq!(
            node_attr(&graph, &b, EVALUATED_VALUE_KEY),
            Some(&Value::Number(serde_json::Number::from(5)))
        );
    }

    #[test]
    fn evaluates_unit_conversion_addition() {
        let mut graph = SemanticGraph::new();
        register_units_fixture(&mut graph);
        let uri = Url::parse("file:///C:/workspace/unit-add.sysml").expect("uri");
        let node = add_node(
            &mut graph,
            &uri,
            "Demo::value",
            "attribute",
            "value",
            None,
            HashMap::from([(
                "value".to_string(),
                Value::String("1 [m] + 50 [cm]".to_string()),
            )]),
        );
        evaluate_expressions(&mut graph);
        assert_eq!(
            node_attr(&graph, &node, EVALUATION_STATUS_KEY),
            Some(&Value::String(STATUS_OK.to_string()))
        );
        assert_eq!(
            node_attr(&graph, &node, EVALUATED_UNIT_KEY),
            Some(&Value::String("m".to_string()))
        );
        assert_eq!(
            node_attr(&graph, &node, EVALUATED_VALUE_KEY),
            Some(&Value::Number(
                serde_json::Number::from_f64(1.5).expect("num")
            ))
        );
    }

    #[test]
    fn evaluates_double_bracket_unit_syntax() {
        let mut graph = SemanticGraph::new();
        register_units_fixture(&mut graph);
        let uri = Url::parse("file:///C:/workspace/unit-double-bracket.sysml").expect("uri");
        let node = add_node(
            &mut graph,
            &uri,
            "Demo::value",
            "attribute",
            "value",
            None,
            HashMap::from([(
                "value".to_string(),
                Value::String("1 [[m]] + 50 [[cm]]".to_string()),
            )]),
        );
        evaluate_expressions(&mut graph);
        assert_eq!(
            node_attr(&graph, &node, EVALUATION_STATUS_KEY),
            Some(&Value::String(STATUS_OK.to_string()))
        );
        assert_eq!(
            node_attr(&graph, &node, EVALUATED_UNIT_KEY),
            Some(&Value::String("m".to_string()))
        );
    }

    #[test]
    fn supports_si_to_imperial_when_registry_has_pair() {
        let mut graph = SemanticGraph::new();
        register_units_fixture(&mut graph);
        let uri = Url::parse("file:///C:/workspace/unit-imperial.sysml").expect("uri");
        let node = add_node(
            &mut graph,
            &uri,
            "Demo::value",
            "attribute",
            "value",
            None,
            HashMap::from([(
                "value".to_string(),
                Value::String("1 [m] + 1 [ft]".to_string()),
            )]),
        );
        evaluate_expressions(&mut graph);
        assert_eq!(
            node_attr(&graph, &node, EVALUATION_STATUS_KEY),
            Some(&Value::String(STATUS_OK.to_string()))
        );
    }

    #[test]
    fn rejects_incompatible_unit_addition() {
        let mut graph = SemanticGraph::new();
        register_units_fixture(&mut graph);
        let uri = Url::parse("file:///C:/workspace/unit-bad.sysml").expect("uri");
        let node = add_node(
            &mut graph,
            &uri,
            "Demo::value",
            "attribute",
            "value",
            None,
            HashMap::from([(
                "value".to_string(),
                Value::String("1 [m] + 2 [kg]".to_string()),
            )]),
        );
        evaluate_expressions(&mut graph);
        assert_eq!(
            node_attr(&graph, &node, EVALUATION_STATUS_KEY),
            Some(&Value::String(STATUS_TYPE_ERROR.to_string()))
        );
    }

    #[test]
    fn evaluates_affine_absolute_temperature_addition() {
        let mut graph = SemanticGraph::new();
        register_units_fixture(&mut graph);
        let uri = Url::parse("file:///C:/workspace/unit-affine.sysml").expect("uri");
        let node = add_node(
            &mut graph,
            &uri,
            "Demo::value",
            "attribute",
            "value",
            None,
            HashMap::from([(
                "value".to_string(),
                Value::String("0 [°C_abs] + 32 [°F_abs]".to_string()),
            )]),
        );
        evaluate_expressions(&mut graph);
        assert_eq!(
            node_attr(&graph, &node, EVALUATION_STATUS_KEY),
            Some(&Value::String(STATUS_OK.to_string()))
        );
        assert_eq!(
            node_attr(&graph, &node, EVALUATED_UNIT_KEY),
            Some(&Value::String("°C_abs".to_string()))
        );
        let value = node_attr(&graph, &node, EVALUATED_VALUE_KEY)
            .and_then(Value::as_f64)
            .expect("numeric");
        assert!((value - 0.0).abs() < 1e-9);
    }

    #[test]
    fn canonicalizes_multiply_divide_units_and_values() {
        let mut graph = SemanticGraph::new();
        register_units_fixture(&mut graph);
        let uri = Url::parse("file:///C:/workspace/unit-canonical.sysml").expect("uri");
        let area = add_node(
            &mut graph,
            &uri,
            "Demo::area",
            "attribute",
            "area",
            None,
            HashMap::from([(
                "value".to_string(),
                Value::String("2 [cm] * 3 [m]".to_string()),
            )]),
        );
        let speed = add_node(
            &mut graph,
            &uri,
            "Demo::speed",
            "attribute",
            "speed",
            None,
            HashMap::from([(
                "value".to_string(),
                Value::String("10 [m] / 2 [s]".to_string()),
            )]),
        );
        evaluate_expressions(&mut graph);

        assert_eq!(
            node_attr(&graph, &area, EVALUATED_UNIT_KEY),
            Some(&Value::String("m^2".to_string()))
        );
        let area_value = node_attr(&graph, &area, EVALUATED_VALUE_KEY)
            .and_then(Value::as_f64)
            .expect("area value");
        assert!((area_value - 0.06).abs() < 1e-9);

        assert_eq!(
            node_attr(&graph, &speed, EVALUATED_UNIT_KEY),
            Some(&Value::String("m/s".to_string()))
        );
        let speed_value = node_attr(&graph, &speed, EVALUATED_VALUE_KEY)
            .and_then(Value::as_f64)
            .expect("speed value");
        assert!((speed_value - 5.0).abs() < 1e-9);
    }

    #[test]
    fn rejects_affine_units_in_multiplication() {
        let mut graph = SemanticGraph::new();
        register_units_fixture(&mut graph);
        let uri = Url::parse("file:///C:/workspace/unit-affine-mul.sysml").expect("uri");
        let node = add_node(
            &mut graph,
            &uri,
            "Demo::value",
            "attribute",
            "value",
            None,
            HashMap::from([(
                "value".to_string(),
                Value::String("1 [°C_abs] * 2 [m]".to_string()),
            )]),
        );
        evaluate_expressions(&mut graph);
        assert_eq!(
            node_attr(&graph, &node, EVALUATION_STATUS_KEY),
            Some(&Value::String(STATUS_UNSUPPORTED.to_string()))
        );
    }

    #[test]
    fn evaluates_inline_analysis_constraint_from_owner_attributes() {
        let mut graph = SemanticGraph::new();
        let uri = Url::parse("file:///C:/workspace/analysis-inline.sysml").expect("uri");
        let requirement = add_node(
            &mut graph,
            &uri,
            "Demo::Req",
            "requirement def",
            "Req",
            None,
            HashMap::from([(
                ANALYSIS_CONSTRAINTS_KEY.to_string(),
                serde_json::json!([{
                    "kind": "require_constraint",
                    "expression": "measured <= limit",
                    "params": [],
                }]),
            )]),
        );
        let _measured = add_node(
            &mut graph,
            &uri,
            "Demo::Req::measured",
            "attribute",
            "measured",
            Some(&requirement),
            HashMap::from([("value".to_string(), Value::String("4".to_string()))]),
        );
        let _limit = add_node(
            &mut graph,
            &uri,
            "Demo::Req::limit",
            "attribute",
            "limit",
            Some(&requirement),
            HashMap::from([("value".to_string(), Value::String("5".to_string()))]),
        );
        evaluate_expressions(&mut graph);
        assert_eq!(
            node_attr(&graph, &requirement, ANALYSIS_EVAL_STATUS_KEY),
            Some(&Value::String(STATUS_OK.to_string()))
        );
        assert_eq!(
            node_attr(&graph, &requirement, ANALYSIS_CONSTRAINT_PASSED_KEY),
            Some(&Value::Bool(true))
        );
    }

    #[test]
    fn marks_analysis_constraint_as_failed_when_comparison_fails() {
        let mut graph = SemanticGraph::new();
        let uri = Url::parse("file:///C:/workspace/analysis-inline-fail.sysml").expect("uri");
        let requirement = add_node(
            &mut graph,
            &uri,
            "Demo::Req",
            "requirement def",
            "Req",
            None,
            HashMap::from([(
                ANALYSIS_CONSTRAINTS_KEY.to_string(),
                serde_json::json!([{
                    "kind": "require_constraint",
                    "expression": "measured <= limit",
                    "params": [],
                }]),
            )]),
        );
        let _measured = add_node(
            &mut graph,
            &uri,
            "Demo::Req::measured",
            "attribute",
            "measured",
            Some(&requirement),
            HashMap::from([("value".to_string(), Value::String("8".to_string()))]),
        );
        let _limit = add_node(
            &mut graph,
            &uri,
            "Demo::Req::limit",
            "attribute",
            "limit",
            Some(&requirement),
            HashMap::from([("value".to_string(), Value::String("5".to_string()))]),
        );
        evaluate_expressions(&mut graph);
        assert_eq!(
            node_attr(&graph, &requirement, ANALYSIS_EVAL_STATUS_KEY),
            Some(&Value::String("failed_constraint".to_string()))
        );
        assert_eq!(
            node_attr(&graph, &requirement, ANALYSIS_CONSTRAINT_PASSED_KEY),
            Some(&Value::Bool(false))
        );
    }

    #[test]
    fn evaluates_boolean_precedence_and_parentheses_in_analysis_expression() {
        let mut graph = SemanticGraph::new();
        let uri = Url::parse("file:///C:/workspace/analysis-precedence.sysml").expect("uri");
        let requirement = add_node(
            &mut graph,
            &uri,
            "Demo::Req",
            "requirement def",
            "Req",
            None,
            HashMap::from([(
                ANALYSIS_EXPRESSION_KEY.to_string(),
                Value::String("not measured > limit and limit == 5 or false".to_string()),
            )]),
        );
        let _measured = add_node(
            &mut graph,
            &uri,
            "Demo::Req::measured",
            "attribute",
            "measured",
            Some(&requirement),
            HashMap::from([("value".to_string(), Value::String("4".to_string()))]),
        );
        let _limit = add_node(
            &mut graph,
            &uri,
            "Demo::Req::limit",
            "attribute",
            "limit",
            Some(&requirement),
            HashMap::from([("value".to_string(), Value::String("5".to_string()))]),
        );
        evaluate_expressions(&mut graph);
        assert_eq!(
            node_attr(&graph, &requirement, ANALYSIS_EVAL_STATUS_KEY),
            Some(&Value::String(STATUS_OK.to_string()))
        );
        assert_eq!(
            node_attr(&graph, &requirement, ANALYSIS_EVAL_VALUE_KEY),
            Some(&Value::Bool(true))
        );
    }

    #[test]
    fn supports_arithmetic_and_unit_conversion_in_analysis_comparison() {
        let mut graph = SemanticGraph::new();
        register_units_fixture(&mut graph);
        let uri = Url::parse("file:///C:/workspace/analysis-units.sysml").expect("uri");
        let requirement = add_node(
            &mut graph,
            &uri,
            "Demo::Req",
            "requirement def",
            "Req",
            None,
            HashMap::from([(
                ANALYSIS_EXPRESSION_KEY.to_string(),
                Value::String("measured + margin <= limit".to_string()),
            )]),
        );
        let _measured = add_node(
            &mut graph,
            &uri,
            "Demo::Req::measured",
            "attribute",
            "measured",
            Some(&requirement),
            HashMap::from([("value".to_string(), Value::String("90 [cm]".to_string()))]),
        );
        let _margin = add_node(
            &mut graph,
            &uri,
            "Demo::Req::margin",
            "attribute",
            "margin",
            Some(&requirement),
            HashMap::from([("value".to_string(), Value::String("0.2 [m]".to_string()))]),
        );
        let _limit = add_node(
            &mut graph,
            &uri,
            "Demo::Req::limit",
            "attribute",
            "limit",
            Some(&requirement),
            HashMap::from([("value".to_string(), Value::String("1.2 [m]".to_string()))]),
        );
        evaluate_expressions(&mut graph);
        assert_eq!(
            node_attr(&graph, &requirement, ANALYSIS_EVAL_STATUS_KEY),
            Some(&Value::String(STATUS_OK.to_string()))
        );
        assert_eq!(
            node_attr(&graph, &requirement, ANALYSIS_EVAL_VALUE_KEY),
            Some(&Value::Bool(true))
        );
    }

    #[test]
    fn supports_multi_term_arithmetic_operands_in_analysis_comparison() {
        let mut graph = SemanticGraph::new();
        let uri = Url::parse("file:///C:/workspace/analysis-parenthesized.sysml").expect("uri");
        let requirement = add_node(
            &mut graph,
            &uri,
            "Demo::Req",
            "requirement def",
            "Req",
            None,
            HashMap::from([(
                ANALYSIS_EXPRESSION_KEY.to_string(),
                Value::String("allowed - estimated - uncertainty >= 0".to_string()),
            )]),
        );
        let _allowed = add_node(
            &mut graph,
            &uri,
            "Demo::Req::allowed",
            "attribute",
            "allowed",
            Some(&requirement),
            HashMap::from([("value".to_string(), Value::String("2.0".to_string()))]),
        );
        let _estimated = add_node(
            &mut graph,
            &uri,
            "Demo::Req::estimated",
            "attribute",
            "estimated",
            Some(&requirement),
            HashMap::from([("value".to_string(), Value::String("1.7".to_string()))]),
        );
        let _uncertainty = add_node(
            &mut graph,
            &uri,
            "Demo::Req::uncertainty",
            "attribute",
            "uncertainty",
            Some(&requirement),
            HashMap::from([("value".to_string(), Value::String("0.1".to_string()))]),
        );
        evaluate_expressions(&mut graph);
        assert_eq!(
            node_attr(&graph, &requirement, ANALYSIS_EVAL_STATUS_KEY),
            Some(&Value::String(STATUS_OK.to_string()))
        );
        assert_eq!(
            node_attr(&graph, &requirement, ANALYSIS_EVAL_VALUE_KEY),
            Some(&Value::Bool(true))
        );
    }

    #[test]
    fn supports_truncated_comparison_rhs_by_assuming_zero() {
        let mut graph = SemanticGraph::new();
        let uri = Url::parse("file:///C:/workspace/analysis-truncated-rhs.sysml").expect("uri");
        let requirement = add_node(
            &mut graph,
            &uri,
            "Demo::Req",
            "requirement def",
            "Req",
            None,
            HashMap::from([(
                ANALYSIS_EXPRESSION_KEY.to_string(),
                Value::String("allowed - estimated - uncertainty >=".to_string()),
            )]),
        );
        let _allowed = add_node(
            &mut graph,
            &uri,
            "Demo::Req::allowed",
            "attribute",
            "allowed",
            Some(&requirement),
            HashMap::from([("value".to_string(), Value::String("2.0".to_string()))]),
        );
        let _estimated = add_node(
            &mut graph,
            &uri,
            "Demo::Req::estimated",
            "attribute",
            "estimated",
            Some(&requirement),
            HashMap::from([("value".to_string(), Value::String("1.7".to_string()))]),
        );
        let _uncertainty = add_node(
            &mut graph,
            &uri,
            "Demo::Req::uncertainty",
            "attribute",
            "uncertainty",
            Some(&requirement),
            HashMap::from([("value".to_string(), Value::String("0.1".to_string()))]),
        );
        evaluate_expressions(&mut graph);
        assert_eq!(
            node_attr(&graph, &requirement, ANALYSIS_EVAL_STATUS_KEY),
            Some(&Value::String(STATUS_OK.to_string()))
        );
        assert_eq!(
            node_attr(&graph, &requirement, ANALYSIS_EVAL_VALUE_KEY),
            Some(&Value::Bool(true))
        );
    }

    #[test]
    fn reports_type_mismatch_for_incompatible_analysis_units() {
        let mut graph = SemanticGraph::new();
        register_units_fixture(&mut graph);
        let uri = Url::parse("file:///C:/workspace/analysis-unit-type-error.sysml").expect("uri");
        let requirement = add_node(
            &mut graph,
            &uri,
            "Demo::Req",
            "requirement def",
            "Req",
            None,
            HashMap::from([(
                ANALYSIS_EXPRESSION_KEY.to_string(),
                Value::String("measured < limit".to_string()),
            )]),
        );
        let _measured = add_node(
            &mut graph,
            &uri,
            "Demo::Req::measured",
            "attribute",
            "measured",
            Some(&requirement),
            HashMap::from([("value".to_string(), Value::String("1 [m]".to_string()))]),
        );
        let _limit = add_node(
            &mut graph,
            &uri,
            "Demo::Req::limit",
            "attribute",
            "limit",
            Some(&requirement),
            HashMap::from([("value".to_string(), Value::String("2 [kg]".to_string()))]),
        );
        evaluate_expressions(&mut graph);
        assert_eq!(
            node_attr(&graph, &requirement, ANALYSIS_EVAL_STATUS_KEY),
            Some(&Value::String(STATUS_TYPE_ERROR.to_string()))
        );
        let message = node_attr(&graph, &requirement, ANALYSIS_EVAL_ERROR_KEY)
            .and_then(Value::as_str)
            .unwrap_or_default();
        assert!(
            message.contains("type or unit mismatch")
                || message.contains("incompatible units")
                || message.contains("dimensioned and unitless")
        );
    }

    #[test]
    fn reports_unresolved_reference_for_analysis_expression() {
        let mut graph = SemanticGraph::new();
        let uri = Url::parse("file:///C:/workspace/analysis-unresolved.sysml").expect("uri");
        let requirement = add_node(
            &mut graph,
            &uri,
            "Demo::Req",
            "requirement def",
            "Req",
            None,
            HashMap::from([(
                ANALYSIS_EXPRESSION_KEY.to_string(),
                Value::String("measured <= missingLimit".to_string()),
            )]),
        );
        let _measured = add_node(
            &mut graph,
            &uri,
            "Demo::Req::measured",
            "attribute",
            "measured",
            Some(&requirement),
            HashMap::from([("value".to_string(), Value::String("4".to_string()))]),
        );
        evaluate_expressions(&mut graph);
        assert_eq!(
            node_attr(&graph, &requirement, ANALYSIS_EVAL_STATUS_KEY),
            Some(&Value::String(STATUS_UNKNOWN.to_string()))
        );
        let message = node_attr(&graph, &requirement, ANALYSIS_EVAL_ERROR_KEY)
            .and_then(Value::as_str)
            .unwrap_or_default();
        assert!(message.contains("could not be resolved"));
    }

    #[test]
    fn evaluates_calc_invocation_in_analysis_comparison() {
        let mut graph = SemanticGraph::new();
        let uri = Url::parse("file:///C:/workspace/analysis-calc-call.sysml").expect("uri");
        let _calc = add_node(
            &mut graph,
            &uri,
            "Demo::Margin",
            "calc def",
            "Margin",
            None,
            HashMap::from([
                (
                    "parameters".to_string(),
                    serde_json::json!([
                        {"direction":"in","name":"limit","type":"Real"},
                        {"direction":"in","name":"measured","type":"Real"},
                        {"direction":"in","name":"allowance","type":"Real"}
                    ]),
                ),
                (
                    ANALYSIS_EXPRESSION_KEY.to_string(),
                    Value::String("limit - measured - allowance".to_string()),
                ),
            ]),
        );
        let requirement = add_node(
            &mut graph,
            &uri,
            "Demo::Req",
            "requirement def",
            "Req",
            None,
            HashMap::from([(
                ANALYSIS_EXPRESSION_KEY.to_string(),
                Value::String("Margin(limit, measured, allowance) >= 0".to_string()),
            )]),
        );
        let _limit = add_node(
            &mut graph,
            &uri,
            "Demo::Req::limit",
            "attribute",
            "limit",
            Some(&requirement),
            HashMap::from([("value".to_string(), Value::String("2.0".to_string()))]),
        );
        let _measured = add_node(
            &mut graph,
            &uri,
            "Demo::Req::measured",
            "attribute",
            "measured",
            Some(&requirement),
            HashMap::from([("value".to_string(), Value::String("1.7".to_string()))]),
        );
        let _allowance = add_node(
            &mut graph,
            &uri,
            "Demo::Req::allowance",
            "attribute",
            "allowance",
            Some(&requirement),
            HashMap::from([("value".to_string(), Value::String("0.1".to_string()))]),
        );
        evaluate_expressions(&mut graph);
        assert_eq!(
            node_attr(&graph, &requirement, ANALYSIS_EVAL_VALUE_KEY),
            Some(&Value::Bool(true))
        );
    }

    #[test]
    fn evaluates_constraint_invocation_as_boolean_predicate() {
        let mut graph = SemanticGraph::new();
        let uri = Url::parse("file:///C:/workspace/analysis-constraint-call.sysml").expect("uri");
        let _constraint = add_node(
            &mut graph,
            &uri,
            "Demo::WithinLimit",
            "constraint def",
            "WithinLimit",
            None,
            HashMap::from([
                (
                    "parameters".to_string(),
                    serde_json::json!([
                        {"direction":"in","name":"measured","type":"Real"},
                        {"direction":"in","name":"limit","type":"Real"}
                    ]),
                ),
                (
                    ANALYSIS_EXPRESSION_KEY.to_string(),
                    Value::String("measured <= limit".to_string()),
                ),
            ]),
        );
        let requirement = add_node(
            &mut graph,
            &uri,
            "Demo::Req",
            "requirement def",
            "Req",
            None,
            HashMap::from([(
                ANALYSIS_EXPRESSION_KEY.to_string(),
                Value::String("WithinLimit(measured, limit)".to_string()),
            )]),
        );
        let _measured = add_node(
            &mut graph,
            &uri,
            "Demo::Req::measured",
            "attribute",
            "measured",
            Some(&requirement),
            HashMap::from([("value".to_string(), Value::String("4".to_string()))]),
        );
        let _limit = add_node(
            &mut graph,
            &uri,
            "Demo::Req::limit",
            "attribute",
            "limit",
            Some(&requirement),
            HashMap::from([("value".to_string(), Value::String("5".to_string()))]),
        );
        evaluate_expressions(&mut graph);
        assert_eq!(
            node_attr(&graph, &requirement, ANALYSIS_EVAL_VALUE_KEY),
            Some(&Value::Bool(true))
        );
    }

    #[test]
    fn skips_unbound_constraint_def_analysis_evaluation() {
        let mut graph = SemanticGraph::new();
        let uri = Url::parse("file:///C:/workspace/analysis-def-skip.sysml").expect("uri");
        let constraint_def = add_node(
            &mut graph,
            &uri,
            "Demo::EnduranceMargin",
            "constraint def",
            "EnduranceMargin",
            None,
            HashMap::from([(
                ANALYSIS_EXPRESSION_KEY.to_string(),
                Value::String("measured <= limit".to_string()),
            )]),
        );
        evaluate_expressions(&mut graph);
        assert_eq!(
            node_attr(&graph, &constraint_def, ANALYSIS_EVAL_STATUS_KEY),
            None
        );
        assert_eq!(
            node_attr(&graph, &constraint_def, ANALYSIS_EVAL_ERROR_KEY),
            None
        );
    }

    #[test]
    fn skips_calc_def_analysis_evaluation() {
        let mut graph = SemanticGraph::new();
        let uri = Url::parse("file:///C:/workspace/calc-def-skip.sysml").expect("uri");
        let calc_def = add_node(
            &mut graph,
            &uri,
            "Demo::MarginEstimate",
            "calc def",
            "MarginEstimate",
            None,
            HashMap::from([(
                ANALYSIS_EXPRESSION_KEY.to_string(),
                Value::String("limit - measured".to_string()),
            )]),
        );
        evaluate_expressions(&mut graph);
        assert_eq!(node_attr(&graph, &calc_def, ANALYSIS_EVAL_STATUS_KEY), None);
        assert_eq!(node_attr(&graph, &calc_def, ANALYSIS_EVAL_ERROR_KEY), None);
    }

    #[test]
    fn evaluates_builtin_sum_over_quantities() {
        let mut graph = SemanticGraph::new();
        register_units_fixture(&mut graph);
        let uri = Url::parse("file:///C:/workspace/sum.sysml").expect("uri");

        // Workspace values.
        let owner = add_node(
            &mut graph,
            &uri,
            "P",
            "package",
            "P",
            None,
            HashMap::new(),
        );
        let _a = add_node(
            &mut graph,
            &uri,
            "P::a",
            "attribute",
            "a",
            Some(&owner),
            HashMap::from([("value".to_string(), Value::String("2 [kg]".to_string()))]),
        );
        let _b = add_node(
            &mut graph,
            &uri,
            "P::b",
            "attribute",
            "b",
            Some(&owner),
            HashMap::from([("value".to_string(), Value::String("3 [kg]".to_string()))]),
        );

        let expr_id = add_node(
            &mut graph,
            &uri,
            "P::total",
            "attribute",
            "total",
            Some(&owner),
            HashMap::from([("value".to_string(), Value::String("sum(a, b)".to_string()))]),
        );

        evaluate_expressions(&mut graph);
        let evaluated = node_attr(&graph, &expr_id, EVALUATED_VALUE_KEY)
            .cloned()
            .expect("evaluated value");
        assert_eq!(evaluated, Value::Number(serde_json::Number::from(5)));
        let unit = node_attr(&graph, &expr_id, EVALUATED_UNIT_KEY)
            .and_then(Value::as_str)
            .expect("evaluated unit");
        assert_eq!(unit, "kg");
    }
}
