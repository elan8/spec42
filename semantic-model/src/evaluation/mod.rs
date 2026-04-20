use serde_json::Value;

use crate::graph::SemanticGraph;
use crate::model::NodeId;

const EVALUATED_VALUE_KEY: &str = "evaluatedValue";
const EVALUATED_UNIT_KEY: &str = "evaluatedUnit";
const EVALUATION_STATUS_KEY: &str = "evaluationStatus";
const EVALUATION_ERROR_KEY: &str = "evaluationError";

const STATUS_OK: &str = "ok";
const STATUS_UNKNOWN: &str = "unknown";
const STATUS_TYPE_ERROR: &str = "type_error";
const STATUS_DIV_BY_ZERO: &str = "div_by_zero";
const STATUS_UNSUPPORTED: &str = "unsupported";
const STATUS_CYCLE: &str = "cycle";

const EVALUATION_SOURCE_KEYS: [&str; 3] = ["value", "defaultValue", "literal"];

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EvalStatus {
    Ok,
    Unknown,
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
            EvalStatus::TypeError => STATUS_TYPE_ERROR,
            EvalStatus::DivByZero => STATUS_DIV_BY_ZERO,
            EvalStatus::Unsupported => STATUS_UNSUPPORTED,
            EvalStatus::Cycle => STATUS_CYCLE,
        }
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

    fn error(status: EvalStatus, message: impl Into<String>) -> Self {
        Self {
            status,
            value: None,
            unit: None,
            error: Some(message.into()),
        }
    }
}

pub fn evaluate_expressions(graph: &mut SemanticGraph) {
    let node_ids: Vec<NodeId> = graph.node_index_by_id.keys().cloned().collect();
    for node_id in node_ids {
        let Some(node) = graph.get_node_mut(&node_id) else {
            continue;
        };
        node.attributes.remove(EVALUATED_VALUE_KEY);
        node.attributes.remove(EVALUATED_UNIT_KEY);
        node.attributes.remove(EVALUATION_STATUS_KEY);
        node.attributes.remove(EVALUATION_ERROR_KEY);

        let source_value = EVALUATION_SOURCE_KEYS
            .iter()
            .find_map(|key| node.attributes.get(*key).map(|value| (*key, value.clone())));
        let Some((_source_key, raw_value)) = source_value else {
            continue;
        };
        let outcome = evaluate_json_value(&raw_value);
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
}

fn evaluate_json_value(value: &Value) -> EvalOutcome {
    match value {
        Value::Bool(v) => EvalOutcome::ok(Value::Bool(*v), None),
        Value::Number(v) => EvalOutcome::ok(Value::Number(v.clone()), None),
        Value::String(s) => evaluate_expression_text(s),
        Value::Null => EvalOutcome::error(EvalStatus::Unknown, "no expression value"),
        _ => EvalOutcome::error(
            EvalStatus::Unsupported,
            "expression value type is not supported in phase 1",
        ),
    }
}

fn evaluate_expression_text(raw: &str) -> EvalOutcome {
    let text = raw.trim();
    if text.is_empty() {
        return EvalOutcome::error(EvalStatus::Unknown, "empty expression");
    }
    if let Some((value_expr, unit_text)) = split_trailing_unit(text) {
        let base = evaluate_expression_text(value_expr);
        if base.status != EvalStatus::Ok {
            return base;
        }
        let normalized_unit = unit_text.trim();
        if normalized_unit.is_empty() {
            return EvalOutcome::error(EvalStatus::Unsupported, "unit expression is empty");
        }
        return EvalOutcome::ok(
            base.value.unwrap_or(Value::Null),
            Some(normalized_unit.to_string()),
        );
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
    if let Some(n) = parse_number(text) {
        return EvalOutcome::ok(number_to_json(n), None);
    }

    let mut parser = ArithmeticParser::new(text);
    match parser.parse_expression() {
        Ok(value) => {
            parser.skip_ws();
            if parser.is_eof() {
                EvalOutcome::ok(number_to_json(value), None)
            } else {
                EvalOutcome::error(
                    EvalStatus::Unsupported,
                    "expression contains unsupported trailing tokens",
                )
            }
        }
        Err(EvalStatus::DivByZero) => EvalOutcome::error(EvalStatus::DivByZero, "division by zero"),
        Err(EvalStatus::Cycle) => EvalOutcome::error(EvalStatus::Cycle, "cyclic reference detected"),
        Err(EvalStatus::TypeError) => EvalOutcome::error(
            EvalStatus::TypeError,
            "expression has type mismatch for arithmetic",
        ),
        Err(EvalStatus::Unknown) => EvalOutcome::error(EvalStatus::Unknown, "expression could not be resolved"),
        Err(EvalStatus::Unsupported) | Err(EvalStatus::Ok) => EvalOutcome::error(
            EvalStatus::Unsupported,
            "expression form is not supported in phase 1",
        ),
    }
}

fn split_trailing_unit(text: &str) -> Option<(&str, &str)> {
    if !text.ends_with(']') {
        return None;
    }
    let mut depth_paren = 0_i32;
    let mut depth_bracket = 0_i32;
    let mut start_idx = None;
    for (idx, ch) in text.char_indices().rev() {
        match ch {
            ')' => depth_paren += 1,
            '(' => depth_paren -= 1,
            ']' => depth_bracket += 1,
            '[' => {
                depth_bracket -= 1;
                if depth_paren == 0 && depth_bracket == 0 {
                    start_idx = Some(idx);
                    break;
                }
            }
            _ => {}
        }
    }
    let start = start_idx?;
    if start == 0 {
        return None;
    }
    let before = &text[..start];
    if !before.ends_with(' ') {
        return None;
    }
    let unit = &text[start + 1..text.len() - 1];
    Some((before.trim_end(), unit))
}

fn parse_number(text: &str) -> Option<f64> {
    text.parse::<f64>().ok().filter(|v| v.is_finite())
}

fn number_to_json(value: f64) -> Value {
    if value.fract() == 0.0 {
        Value::Number(serde_json::Number::from(value as i64))
    } else {
        Value::Number(
            serde_json::Number::from_f64(value)
                .unwrap_or_else(|| serde_json::Number::from(0)),
        )
    }
}

struct ArithmeticParser<'a> {
    src: &'a str,
    pos: usize,
}

impl<'a> ArithmeticParser<'a> {
    fn new(src: &'a str) -> Self {
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

    fn parse_expression(&mut self) -> Result<f64, EvalStatus> {
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
            left = if op == '+' { left + right } else { left - right };
        }
    }

    fn parse_term(&mut self) -> Result<f64, EvalStatus> {
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
            if op == '/' && right == 0.0 {
                return Err(EvalStatus::DivByZero);
            }
            left = if op == '*' { left * right } else { left / right };
        }
    }

    fn parse_factor(&mut self) -> Result<f64, EvalStatus> {
        self.skip_ws();
        let Some(ch) = self.peek_char() else {
            return Err(EvalStatus::Unsupported);
        };
        if ch == '+' || ch == '-' {
            self.eat_char();
            let inner = self.parse_factor()?;
            return if ch == '-' { Ok(-inner) } else { Ok(inner) };
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<f64, EvalStatus> {
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
        self.parse_numeric_literal()
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluates_literal_number() {
        let out = evaluate_expression_text("42");
        assert_eq!(out.status, EvalStatus::Ok);
        assert_eq!(out.value, Some(Value::Number(serde_json::Number::from(42))));
        assert_eq!(out.unit, None);
    }

    #[test]
    fn evaluates_arithmetic_expression() {
        let out = evaluate_expression_text("(1 + 2) * 3");
        assert_eq!(out.status, EvalStatus::Ok);
        assert_eq!(out.value, Some(Value::Number(serde_json::Number::from(9))));
    }

    #[test]
    fn evaluates_bracket_expression() {
        let out = evaluate_expression_text("[(1 + 2)]");
        assert_eq!(out.status, EvalStatus::Ok);
        assert_eq!(out.value, Some(Value::Number(serde_json::Number::from(3))));
    }

    #[test]
    fn reports_division_by_zero() {
        let out = evaluate_expression_text("1 / 0");
        assert_eq!(out.status, EvalStatus::DivByZero);
        assert!(out.value.is_none());
    }

    #[test]
    fn supports_literal_with_unit_passthrough() {
        let out = evaluate_expression_text("1200 [kg]");
        assert_eq!(out.status, EvalStatus::Ok);
        assert_eq!(out.value, Some(Value::Number(serde_json::Number::from(1200))));
        assert_eq!(out.unit, Some("kg".to_string()));
    }

    #[test]
    fn marks_unsupported_reference_expression() {
        let out = evaluate_expression_text("mass + 1");
        assert_eq!(out.status, EvalStatus::Unsupported);
        assert!(out.value.is_none());
    }
}
