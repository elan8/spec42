use std::collections::{HashMap, HashSet};

use serde_json::Value;

use crate::graph::SemanticGraph;
use crate::model::{NodeId, SemanticNode};

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

const _EVAL_STATUS_VARIANTS_USED: [EvalStatus; 2] = [EvalStatus::TypeError, EvalStatus::Cycle];

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
    let _ = EvalStatus::TypeError;
    let _ = EvalStatus::Cycle;
    let node_ids: Vec<NodeId> = graph.node_index_by_id.keys().cloned().collect();
    let outcomes = {
        let mut engine = EvalEngine::new(graph);
        node_ids
            .iter()
            .map(|node_id| {
                let outcome = if engine.node_source_value(node_id).is_some() {
                    Some(engine.evaluate_node(node_id))
                } else {
                    None
                };
                (node_id.clone(), outcome)
            })
            .collect::<Vec<(NodeId, Option<EvalOutcome>)>>()
    };
    for (node_id, outcome) in outcomes {
        let Some(node) = graph.get_node_mut(&node_id) else {
            continue;
        };
        node.attributes.remove(EVALUATED_VALUE_KEY);
        node.attributes.remove(EVALUATED_UNIT_KEY);
        node.attributes.remove(EVALUATION_STATUS_KEY);
        node.attributes.remove(EVALUATION_ERROR_KEY);
        let Some(outcome) = outcome else {
            continue;
        };
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

struct EvalEngine<'a> {
    graph: &'a SemanticGraph,
    memoized: HashMap<NodeId, EvalOutcome>,
    active_stack: HashSet<NodeId>,
}

impl<'a> EvalEngine<'a> {
    fn new(graph: &'a SemanticGraph) -> Self {
        Self {
            graph,
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
                format!("cyclic dependency detected while evaluating '{}'", node_id.qualified_name),
            );
        }
        let Some(raw_value) = self.node_source_value(node_id) else {
            return EvalOutcome::error(
                EvalStatus::Unknown,
                format!("no evaluable expression source found for '{}'", node_id.qualified_name),
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
        let text = raw.trim();
        if text.is_empty() {
            return EvalOutcome::error(EvalStatus::Unknown, "empty expression");
        }
        if let Some((value_expr, unit_text)) = split_trailing_unit(text) {
            let base = self.evaluate_expression_text(node_id, value_expr);
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
        if let Some(identifier) = parse_standalone_identifier(text) {
            return self.resolve_identifier_value(node_id, identifier);
        }

        let mut parser = ArithmeticParser::new(text, |identifier| {
            self.resolve_identifier_number(node_id, identifier)
        });
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
            Err(EvalStatus::DivByZero) => {
                EvalOutcome::error(EvalStatus::DivByZero, "division by zero")
            }
            Err(EvalStatus::Cycle) => {
                EvalOutcome::error(EvalStatus::Cycle, "cyclic reference detected")
            }
            Err(EvalStatus::TypeError) => EvalOutcome::error(
                EvalStatus::TypeError,
                "expression has type mismatch for arithmetic",
            ),
            Err(EvalStatus::Unknown) => {
                EvalOutcome::error(EvalStatus::Unknown, "expression could not be resolved")
            }
            Err(EvalStatus::Unsupported) | Err(EvalStatus::Ok) => EvalOutcome::error(
                EvalStatus::Unsupported,
                "expression form is not supported",
            ),
        }
    }

    fn resolve_identifier_value(&mut self, node_id: &NodeId, identifier: &str) -> EvalOutcome {
        let referenced_id = match self.resolve_identifier_node(node_id, identifier) {
            Ok(found) => found,
            Err(outcome) => return outcome,
        };
        self.evaluate_node(&referenced_id)
    }

    fn resolve_identifier_number(&mut self, node_id: &NodeId, identifier: &str) -> Result<f64, EvalStatus> {
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
        json_value_to_f64(&value).ok_or(EvalStatus::TypeError)
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
    let second_len = sorted.get(1).map(|candidate| candidate.qualified_name.len());
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
    let mut parser = ArithmeticParser::new(trimmed, |_identifier| Err(EvalStatus::Unsupported));
    let identifier = parser.parse_identifier()?;
    parser.skip_ws();
    if parser.is_eof() {
        Some(identifier)
    } else {
        None
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
            serde_json::Number::from_f64(value)
                .unwrap_or_else(|| serde_json::Number::from(0)),
        )
    }
}

struct ArithmeticParser<'a, F>
where
    F: FnMut(&str) -> Result<f64, EvalStatus>,
{
    src: &'a str,
    pos: usize,
    resolve_identifier: F,
}

impl<'a, F> ArithmeticParser<'a, F>
where
    F: FnMut(&str) -> Result<f64, EvalStatus>,
{
    fn new(src: &'a str, resolve_identifier: F) -> Self {
        Self {
            src,
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
        if let Some(identifier) = self.parse_identifier() {
            return (self.resolve_identifier)(identifier);
        }
        self.parse_numeric_literal()
    }

    fn parse_identifier(&mut self) -> Option<&'a str> {
        self.skip_ws();
        let start = self.pos;
        let first = self.peek_char()?;
        if !(first.is_ascii_alphabetic() || first == '_') {
            return None;
        }
        self.eat_char();
        loop {
            let Some(ch) = self.peek_char() else {
                break;
            };
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use tower_lsp::lsp_types::{Position, Range, Url};

    use crate::model::SemanticNode;

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

    fn node_attr<'a>(
        graph: &'a SemanticGraph,
        id: &NodeId,
        key: &str,
    ) -> Option<&'a Value> {
        graph.get_node(id).and_then(|node| node.attributes.get(key))
    }

    #[test]
    fn evaluates_literal_number() {
        let mut graph = SemanticGraph::new();
        let uri = Url::parse("file:///C:/workspace/literal.sysml").expect("uri");
        let attr_id = add_node(
            &mut graph,
            &uri,
            "Demo::literal",
            "attribute",
            "literal",
            None,
            HashMap::from([("value".to_string(), Value::String("42".to_string()))]),
        );
        evaluate_expressions(&mut graph);
        assert_eq!(
            node_attr(&graph, &attr_id, EVALUATION_STATUS_KEY),
            Some(&Value::String(STATUS_OK.to_string()))
        );
        assert_eq!(
            node_attr(&graph, &attr_id, EVALUATED_VALUE_KEY),
            Some(&Value::Number(serde_json::Number::from(42)))
        );
        assert_eq!(node_attr(&graph, &attr_id, EVALUATED_UNIT_KEY), None);
    }

    #[test]
    fn evaluates_literal_with_unit_passthrough() {
        let mut graph = SemanticGraph::new();
        let uri = Url::parse("file:///C:/workspace/unit.sysml").expect("uri");
        let attr_id = add_node(
            &mut graph,
            &uri,
            "Demo::mass",
            "attribute",
            "mass",
            None,
            HashMap::from([("value".to_string(), Value::String("1200 [kg]".to_string()))]),
        );
        evaluate_expressions(&mut graph);
        assert_eq!(
            node_attr(&graph, &attr_id, EVALUATION_STATUS_KEY),
            Some(&Value::String(STATUS_OK.to_string()))
        );
        assert_eq!(
            node_attr(&graph, &attr_id, EVALUATED_VALUE_KEY),
            Some(&Value::Number(serde_json::Number::from(1200)))
        );
        assert_eq!(
            node_attr(&graph, &attr_id, EVALUATED_UNIT_KEY),
            Some(&Value::String("kg".to_string()))
        );
    }

    #[test]
    fn evaluates_direct_reference_expression() {
        let mut graph = SemanticGraph::new();
        let uri = Url::parse("file:///C:/workspace/direct.sysml").expect("uri");
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
            HashMap::from([("value".to_string(), Value::String("a + 1".to_string()))]),
        );

        evaluate_expressions(&mut graph);
        assert_eq!(
            node_attr(&graph, &b, EVALUATION_STATUS_KEY),
            Some(&Value::String(STATUS_OK.to_string()))
        );
        assert_eq!(
            node_attr(&graph, &b, EVALUATED_VALUE_KEY),
            Some(&Value::Number(serde_json::Number::from(3)))
        );
    }

    #[test]
    fn evaluates_multi_hop_reference_expression() {
        let mut graph = SemanticGraph::new();
        let uri = Url::parse("file:///C:/workspace/multihop.sysml").expect("uri");
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
        let _b = add_node(
            &mut graph,
            &uri,
            "Demo::Rocket::b",
            "attribute",
            "b",
            Some(&owner),
            HashMap::from([("value".to_string(), Value::String("a + 1".to_string()))]),
        );
        let c = add_node(
            &mut graph,
            &uri,
            "Demo::Rocket::c",
            "attribute",
            "c",
            Some(&owner),
            HashMap::from([("value".to_string(), Value::String("b + 2".to_string()))]),
        );

        evaluate_expressions(&mut graph);
        assert_eq!(
            node_attr(&graph, &c, EVALUATED_VALUE_KEY),
            Some(&Value::Number(serde_json::Number::from(5)))
        );
    }

    #[test]
    fn marks_unresolved_reference_unknown() {
        let mut graph = SemanticGraph::new();
        let uri = Url::parse("file:///C:/workspace/unresolved.sysml").expect("uri");
        let owner = add_node(
            &mut graph,
            &uri,
            "Demo::Rocket",
            "part def",
            "Rocket",
            None,
            HashMap::new(),
        );
        let b = add_node(
            &mut graph,
            &uri,
            "Demo::Rocket::b",
            "attribute",
            "b",
            Some(&owner),
            HashMap::from([("value".to_string(), Value::String("missing + 1".to_string()))]),
        );

        evaluate_expressions(&mut graph);
        assert_eq!(
            node_attr(&graph, &b, EVALUATION_STATUS_KEY),
            Some(&Value::String(STATUS_UNKNOWN.to_string()))
        );
        assert!(node_attr(&graph, &b, EVALUATED_VALUE_KEY).is_none());
    }

    #[test]
    fn marks_self_reference_cycle() {
        let mut graph = SemanticGraph::new();
        let uri = Url::parse("file:///C:/workspace/selfcycle.sysml").expect("uri");
        let owner = add_node(
            &mut graph,
            &uri,
            "Demo::Rocket",
            "part def",
            "Rocket",
            None,
            HashMap::new(),
        );
        let a = add_node(
            &mut graph,
            &uri,
            "Demo::Rocket::a",
            "attribute",
            "a",
            Some(&owner),
            HashMap::from([("value".to_string(), Value::String("a + 1".to_string()))]),
        );
        evaluate_expressions(&mut graph);
        assert_eq!(
            node_attr(&graph, &a, EVALUATION_STATUS_KEY),
            Some(&Value::String(STATUS_CYCLE.to_string()))
        );
    }

    #[test]
    fn marks_mutual_reference_cycle() {
        let mut graph = SemanticGraph::new();
        let uri = Url::parse("file:///C:/workspace/mutualcycle.sysml").expect("uri");
        let owner = add_node(
            &mut graph,
            &uri,
            "Demo::Rocket",
            "part def",
            "Rocket",
            None,
            HashMap::new(),
        );
        let a = add_node(
            &mut graph,
            &uri,
            "Demo::Rocket::a",
            "attribute",
            "a",
            Some(&owner),
            HashMap::from([("value".to_string(), Value::String("b + 1".to_string()))]),
        );
        let b = add_node(
            &mut graph,
            &uri,
            "Demo::Rocket::b",
            "attribute",
            "b",
            Some(&owner),
            HashMap::from([("value".to_string(), Value::String("a + 1".to_string()))]),
        );
        evaluate_expressions(&mut graph);
        assert_eq!(
            node_attr(&graph, &a, EVALUATION_STATUS_KEY),
            Some(&Value::String(STATUS_CYCLE.to_string()))
        );
        assert_eq!(
            node_attr(&graph, &b, EVALUATION_STATUS_KEY),
            Some(&Value::String(STATUS_CYCLE.to_string()))
        );
    }

    #[test]
    fn evaluates_reference_precedence_expression() {
        let mut graph = SemanticGraph::new();
        let uri = Url::parse("file:///C:/workspace/precedence.sysml").expect("uri");
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
        let _b = add_node(
            &mut graph,
            &uri,
            "Demo::Rocket::b",
            "attribute",
            "b",
            Some(&owner),
            HashMap::from([("value".to_string(), Value::String("3".to_string()))]),
        );
        let c = add_node(
            &mut graph,
            &uri,
            "Demo::Rocket::c",
            "attribute",
            "c",
            Some(&owner),
            HashMap::from([("value".to_string(), Value::String("a + b * 2".to_string()))]),
        );
        evaluate_expressions(&mut graph);
        assert_eq!(
            node_attr(&graph, &c, EVALUATED_VALUE_KEY),
            Some(&Value::Number(serde_json::Number::from(8)))
        );
    }
}
