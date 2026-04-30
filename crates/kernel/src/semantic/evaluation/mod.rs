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
    let node_ids: Vec<NodeId> = graph.node_index_by_id.keys().cloned().collect();
    for node_id in node_ids {
        let Some(node) = graph.get_node(&node_id) else {
            continue;
        };
        let mut evaluated = false;
        let mut status = STATUS_UNKNOWN.to_string();
        let mut value: Option<Value> = None;
        let mut error: Option<String> = None;
        let mut passed: Option<bool> = None;

        let inline_constraints = node
            .attributes
            .get(ANALYSIS_CONSTRAINTS_KEY)
            .and_then(|value| value.as_array())
            .cloned()
            .unwrap_or_default();
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
                match evaluate_analysis_expression(graph, node, expr) {
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
                        status = STATUS_UNKNOWN.to_string();
                        error = Some(err);
                    }
                }
            }
        } else if let Some(expr) = node
            .attributes
            .get(ANALYSIS_EXPRESSION_KEY)
            .and_then(Value::as_str)
        {
            evaluated = true;
            match evaluate_analysis_expression(graph, node, expr) {
                Ok(bool_value) => {
                    status = STATUS_OK.to_string();
                    value = Some(Value::Bool(bool_value));
                    passed = Some(bool_value);
                }
                Err(err) => {
                    status = STATUS_UNKNOWN.to_string();
                    error = Some(err);
                }
            }
        }

        if !evaluated {
            continue;
        }
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

fn evaluate_analysis_expression(
    graph: &SemanticGraph,
    context: &SemanticNode,
    expression: &str,
) -> Result<bool, String> {
    let expr = expression.trim();
    if expr.is_empty() {
        return Err("empty analysis expression".to_string());
    }
    for op in ["<=", ">=", "==", "!=", "<", ">"] {
        if let Some(index) = expr.find(op) {
            let lhs = expr[..index].trim();
            let rhs = expr[index + op.len()..].trim();
            let left = evaluate_numeric_term(graph, context, lhs)?;
            let right = evaluate_numeric_term(graph, context, rhs)?;
            let result = match op {
                "<=" => left <= right,
                ">=" => left >= right,
                "==" => (left - right).abs() < 1e-9,
                "!=" => (left - right).abs() >= 1e-9,
                "<" => left < right,
                ">" => left > right,
                _ => false,
            };
            return Ok(result);
        }
    }
    if expr.eq_ignore_ascii_case("true") {
        return Ok(true);
    }
    if expr.eq_ignore_ascii_case("false") {
        return Ok(false);
    }
    Err(format!(
        "analysis expression '{expr}' is unsupported; expected comparison"
    ))
}

fn evaluate_numeric_term(
    graph: &SemanticGraph,
    context: &SemanticNode,
    term: &str,
) -> Result<f64, String> {
    if let Ok(number) = term.parse::<f64>() {
        return Ok(number);
    }
    resolve_identifier_numeric(graph, context, term).ok_or_else(|| format!("unresolved '{term}'"))
}

fn resolve_identifier_numeric(graph: &SemanticGraph, context: &SemanticNode, identifier: &str) -> Option<f64> {
    let parent = graph
        .parent_of(context)
        .map(|node| node.id.clone())
        .unwrap_or_else(|| context.id.clone());
    let candidates = graph.child_named(&parent, identifier);
    for candidate in candidates {
        if let Some(number) = candidate
            .attributes
            .get(EVALUATED_VALUE_KEY)
            .and_then(json_value_to_f64)
            .or_else(|| {
                EVALUATION_SOURCE_KEYS.iter().find_map(|key| {
                    candidate
                        .attributes
                        .get(*key)
                        .and_then(json_value_to_f64)
                })
            })
        {
            return Some(number);
        }
    }
    context
        .attributes
        .get(identifier)
        .and_then(json_value_to_f64)
}

struct EvalEngine<'a> {
    graph: &'a SemanticGraph,
    units: UnitRegistry,
    memoized: HashMap<NodeId, EvalOutcome>,
    active_stack: HashSet<NodeId>,
}

impl<'a> EvalEngine<'a> {
    fn new(graph: &'a SemanticGraph) -> Self {
        Self {
            graph,
            units: UnitRegistry::from_semantic_graph(graph),
            memoized: HashMap::new(),
            active_stack: HashSet::new(),
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
                EvalStatus::Unknown,
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

        let units = self.units.clone();
        let mut parser = QuantityParser::new(text, &units, |identifier| {
            self.resolve_identifier_quantity(node_id, identifier)
        });
        match parser.parse_expression() {
            Ok(quantity) => {
                parser.skip_ws();
                if parser.is_eof() {
                    EvalOutcome::from_quantity(quantity)
                } else {
                    EvalOutcome::error(
                        EvalStatus::Unsupported,
                        "expression contains unsupported trailing tokens",
                    )
                }
            }
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
            Err(EvalStatus::Unsupported) | Err(EvalStatus::Ok) => {
                EvalOutcome::error(EvalStatus::Unsupported, "expression form is not supported")
            }
        }
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
            .filter(|node_id| self.node_source_value(node_id).is_some())
            .cloned()
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

fn parse_standalone_identifier(text: &str) -> Option<&str> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return None;
    }
    let units = UnitRegistry::default();
    let mut parser =
        QuantityParser::new(trimmed, &units, |_identifier| Err(EvalStatus::Unsupported));
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

struct QuantityParser<'s, 'u, F>
where
    F: FnMut(&str) -> Result<Quantity, EvalStatus>,
{
    src: &'s str,
    units: &'u UnitRegistry,
    pos: usize,
    resolve_identifier: F,
}

impl<'s, 'u, F> QuantityParser<'s, 'u, F>
where
    F: FnMut(&str) -> Result<Quantity, EvalStatus>,
{
    fn new(src: &'s str, units: &'u UnitRegistry, resolve_identifier: F) -> Self {
        Self {
            src,
            units,
            pos: 0,
            resolve_identifier,
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
            return (self.resolve_identifier)(identifier);
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

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;
    use tower_lsp::lsp_types::{Position, Range, Url};

    use crate::semantic::model::SemanticNode;

    fn range() -> Range {
        Range::new(Position::new(0, 0), Position::new(0, 1))
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
}
