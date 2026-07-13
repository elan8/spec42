use super::*;

pub(crate) fn split_comparison_lhs(expression: &str) -> Option<&str> {
    let trimmed = expression.trim();
    for op in ["<=", ">=", "==", "!=", "<", ">"] {
        if let Some(index) = trimmed.find(op) {
            let lhs = trimmed[..index].trim();
            if !lhs.is_empty() {
                return Some(lhs);
            }
        }
    }
    None
}

pub(crate) fn split_comparison_rhs(expression: &str) -> Option<&str> {
    let trimmed = expression.trim();
    for op in ["<=", ">=", "==", "!=", "<", ">"] {
        if let Some(index) = trimmed.find(op) {
            let rhs = trimmed[index + op.len()..].trim().trim_end_matches(';');
            if !rhs.is_empty() {
                return Some(rhs);
            }
        }
    }
    None
}

pub(crate) fn format_quantity_display(quantity: &Quantity) -> String {
    let formatted = if (quantity.value - quantity.value.round()).abs() < f64::EPSILON {
        format!("{}", quantity.value.round() as i64)
    } else {
        format!("{}", quantity.value)
    };
    match quantity
        .unit
        .as_deref()
        .filter(|unit| !unit.trim().is_empty())
    {
        Some(unit) => format!("{formatted} {unit}"),
        None => formatted,
    }
}

pub(crate) fn evaluate_analysis_limit_quantity(
    engine: &mut EvalEngine<'_>,
    context_id: &NodeId,
    expression: &str,
) -> Option<Quantity> {
    let repaired = normalize_broken_invocation_syntax(expression.trim());
    let normalized = normalize_truncated_analysis_comparison(repaired.as_str());
    let rhs = split_comparison_rhs(normalized.as_str())?;
    engine.evaluate_quantity_expression(context_id, rhs).ok()
}

pub(crate) fn evaluate_analysis_display_quantity(
    engine: &mut EvalEngine<'_>,
    context_id: &NodeId,
    expression: &str,
) -> Option<Quantity> {
    let repaired = normalize_broken_invocation_syntax(expression.trim());
    let normalized = normalize_truncated_analysis_comparison(repaired.as_str());
    let quantity_expr = split_comparison_lhs(normalized.as_str()).unwrap_or(normalized.as_str());
    engine
        .evaluate_quantity_expression(context_id, quantity_expr)
        .ok()
}

pub(crate) fn is_definition_only_analysis_node(node: &SemanticNode) -> bool {
    matches!(node.element_kind, ElementKind::ConstraintDef | ElementKind::CalcDef)
}

pub(crate) fn parse_verdict_kind_token(expression: &str) -> Option<String> {
    let trimmed = expression.trim();
    let rest = trimmed.strip_prefix("VerdictKind::")?;
    let token = rest
        .split(|ch: char| !ch.is_ascii_alphanumeric() && ch != '_')
        .next()
        .unwrap_or_default()
        .trim()
        .to_ascii_lowercase();
    if token.is_empty() {
        None
    } else {
        Some(token)
    }
}

pub(crate) fn resolve_verdict_kind_token(
    graph: &SemanticGraph,
    node_id: &NodeId,
    expression: &str,
) -> Option<String> {
    if let Some(token) = parse_verdict_kind_token(expression) {
        return Some(token);
    }
    let node = graph.get_node(node_id)?;
    graph
        .children_of(node)
        .into_iter()
        .filter(|child| child.element_kind == ElementKind::Verdict)
        .find_map(|child| {
            child
                .attributes
                .get("rawVerdictToken")
                .and_then(|value| value.as_str())
                .map(|token| token.trim().to_ascii_lowercase())
                .filter(|token| !token.is_empty())
        })
}

pub(crate) fn evaluate_analysis_expression(
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

pub(crate) fn normalize_broken_invocation_syntax(text: &str) -> String {
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

pub(crate) fn flatten_parenthesized_arithmetic(expr: &str) -> String {
    expr.replace(['(', ')'], " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

pub(crate) fn normalize_truncated_analysis_comparison(expr: &str) -> String {
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

pub(crate) fn strip_outer_balanced_parens(text: &str) -> Option<&str> {
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
pub(crate) enum AnalysisComparisonOp {
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Ne,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum AnalysisExpr<'s> {
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

pub(crate) struct AnalysisExprParser<'s> {
    src: &'s str,
    pos: usize,
}

impl<'s> AnalysisExprParser<'s> {
    pub(crate) fn new(src: &'s str) -> Self {
        Self { src, pos: 0 }
    }

    pub(crate) fn is_eof(&self) -> bool {
        self.pos >= self.src.len()
    }

    pub(crate) fn skip_ws(&mut self) {
        while let Some(ch) = self.peek_char() {
            if ch.is_whitespace() {
                self.pos += ch.len_utf8();
            } else {
                break;
            }
        }
    }

    pub(crate) fn peek_char(&self) -> Option<char> {
        self.src[self.pos..].chars().next()
    }

    pub(crate) fn eat_char(&mut self) -> Option<char> {
        let ch = self.peek_char()?;
        self.pos += ch.len_utf8();
        Some(ch)
    }

    pub(crate) fn parse_expression(&mut self) -> Result<AnalysisExpr<'s>, String> {
        self.parse_or()
    }

    pub(crate) fn parse_or(&mut self) -> Result<AnalysisExpr<'s>, String> {
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

    pub(crate) fn parse_and(&mut self) -> Result<AnalysisExpr<'s>, String> {
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

    pub(crate) fn parse_not(&mut self) -> Result<AnalysisExpr<'s>, String> {
        self.skip_ws();
        if self.consume_symbol("!") || self.consume_keyword("not") {
            return Ok(AnalysisExpr::Not(Box::new(self.parse_not()?)));
        }
        self.parse_primary()
    }

    pub(crate) fn parse_primary(&mut self) -> Result<AnalysisExpr<'s>, String> {
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

    pub(crate) fn parse_comparison_or_bool_literal(&mut self) -> Result<AnalysisExpr<'s>, String> {
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

    pub(crate) fn consume_symbol(&mut self, symbol: &str) -> bool {
        if self.src[self.pos..].starts_with(symbol) {
            self.pos += symbol.len();
            true
        } else {
            false
        }
    }

    pub(crate) fn consume_keyword(&mut self, keyword: &str) -> bool {
        if !self.is_keyword_at_pos(keyword) {
            return false;
        }
        self.pos += keyword.len();
        true
    }

    pub(crate) fn is_keyword_at_pos(&self, keyword: &str) -> bool {
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

pub(crate) fn find_comparison_operator(text: &str) -> Option<(usize, AnalysisComparisonOp, usize)> {
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

pub(crate) fn evaluate_analysis_ast(
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
            if matches!(op, AnalysisComparisonOp::Eq | AnalysisComparisonOp::Ne) {
                if let (Some(left), Some(right)) = (
                    try_evaluate_boolean_operand(engine, context_id, lhs),
                    try_evaluate_boolean_operand(engine, context_id, rhs),
                ) {
                    return Ok(match op {
                        AnalysisComparisonOp::Eq => left == right,
                        AnalysisComparisonOp::Ne => left != right,
                        _ => unreachable!("guarded by outer match"),
                    });
                }
            }
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

/// Evaluates a comparison operand as a boolean, when it plainly is one (a `true`/`false`
/// literal, or an identifier resolving to a `Value::Bool`) — used so `s.flag == true`-style
/// Boolean equality checks in `require constraint` bodies don't fall through to the
/// numeric/quantity comparison path (which cannot parse `true`/`false` as a quantity).
pub(crate) fn try_evaluate_boolean_operand(
    engine: &mut EvalEngine<'_>,
    context_id: &NodeId,
    operand: &str,
) -> Option<bool> {
    let trimmed = operand.trim();
    if trimmed.eq_ignore_ascii_case("true") {
        return Some(true);
    }
    if trimmed.eq_ignore_ascii_case("false") {
        return Some(false);
    }
    let identifier = parse_standalone_identifier(trimmed)?;
    let outcome = engine.resolve_identifier_value(context_id, identifier);
    if outcome.status != EvalStatus::Ok {
        return None;
    }
    outcome.value.as_ref().and_then(Value::as_bool)
}

pub(crate) fn evaluate_analysis_predicate(
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

pub(crate) fn compare_quantities(
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

pub(crate) fn map_analysis_eval_error(status: EvalStatus) -> String {
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

pub(crate) fn prefer_analysis_error(left: AnalysisEvalError, right: AnalysisEvalError) -> AnalysisEvalError {
    if left.status == EvalStatus::Incomplete || right.status != EvalStatus::Incomplete {
        left
    } else {
        right
    }
}

pub(crate) fn is_identifier_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
}

