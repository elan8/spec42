use super::*;

pub(crate) struct QuantityParser<'s, 'u, F>
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
    pub(crate) fn new(src: &'s str, units: &'u UnitRegistry, resolve_symbol: F) -> Self {
        Self {
            src,
            units,
            pos: 0,
            resolve_symbol,
        }
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

    /// Iterative, not recursive. The original recursive-descent chain (`parse_expression` ->
    /// `parse_term` -> `parse_factor` -> `parse_primary` -> `(`/`[` -> `parse_expression`, plus
    /// `parse_factor`'s own self-recursion for chained unary `+`/`-`) cost one native stack frame
    /// per nesting level with no bound -- the same class of bug fixed in `sysml-v2-parser` 0.47.0,
    /// applied here to this crate's own small quantity-expression grammar. This keeps one explicit
    /// `Vec`-based operand/operator stack (precedence climbing, matching the original two fixed
    /// precedence levels: `*`/`/` bind tighter than `+`/`-`) plus a `Vec`-based frame stack that
    /// suspends the current climb and starts fresh whenever `(` or `[` opens a new group, resuming
    /// it when the matching `)`/`]` closes. Depth of input becomes `Vec` growth, not call-stack
    /// growth.
    pub(crate) fn parse_expression(&mut self) -> Result<Quantity, EvalStatus> {
        struct PendingOp {
            op: char,
            prec: u8,
        }

        #[derive(Default)]
        struct Climb {
            operands: Vec<Quantity>,
            ops: Vec<PendingOp>,
        }

        fn reduce_one(units: &UnitRegistry, climb: &mut Climb) -> Result<(), EvalStatus> {
            let pending = climb.ops.pop().ok_or(EvalStatus::Unsupported)?;
            let right = climb.operands.pop().ok_or(EvalStatus::Unsupported)?;
            let left = climb.operands.pop().ok_or(EvalStatus::Unsupported)?;
            let result = match pending.op {
                '+' => add_quantities_with_units(units, left, right)?,
                '-' => add_quantities_with_units(
                    units,
                    left,
                    Quantity {
                        value: -right.value,
                        unit: right.unit,
                    },
                )?,
                _ => {
                    if pending.op == '/' && right.value == 0.0 {
                        return Err(EvalStatus::DivByZero);
                    }
                    match units.compose_product(
                        left.value,
                        left.unit.as_deref(),
                        right.value,
                        right.unit.as_deref(),
                        pending.op == '/',
                    ) {
                        Ok((value, unit)) => Quantity { value, unit },
                        Err(err) => return Err(map_unit_error(err)),
                    }
                }
            };
            climb.operands.push(result);
            Ok(())
        }

        struct SuspendedFrame {
            close: char,
            climb: Climb,
            sign: f64,
        }

        let mut stack: Vec<SuspendedFrame> = Vec::new();
        let mut climb = Climb::default();
        let mut pending_factor: Option<Quantity> = None;

        'outer: loop {
            let factor = match pending_factor.take() {
                Some(factor) => factor,
                None => {
                    let mut sign = 1.0f64;
                    loop {
                        self.skip_ws();
                        match self.peek_char() {
                            Some('+') => {
                                self.eat_char();
                            }
                            Some('-') => {
                                self.eat_char();
                                sign = -sign;
                            }
                            _ => break,
                        }
                    }
                    self.skip_ws();
                    match self.peek_char() {
                        Some('(') => {
                            self.eat_char();
                            stack.push(SuspendedFrame {
                                close: ')',
                                climb: std::mem::take(&mut climb),
                                sign,
                            });
                            continue 'outer;
                        }
                        Some('[') => {
                            self.eat_char();
                            stack.push(SuspendedFrame {
                                close: ']',
                                climb: std::mem::take(&mut climb),
                                sign,
                            });
                            continue 'outer;
                        }
                        _ => {
                            let mut factor = if let Some(identifier) = self.parse_identifier() {
                                self.skip_ws();
                                if self.peek_char() == Some('(') {
                                    let args = self.parse_argument_slices()?;
                                    (self.resolve_symbol)(identifier, Some(&args))?
                                } else {
                                    (self.resolve_symbol)(identifier, None)?
                                }
                            } else {
                                let value = self.parse_numeric_literal()?;
                                let unit = self.parse_unit_suffix();
                                Quantity { value, unit }
                            };
                            factor.value *= sign;
                            factor
                        }
                    }
                }
            };

            climb.operands.push(factor);
            self.skip_ws();
            let next_op = match self.peek_char() {
                Some('+') => Some(('+', 0u8)),
                Some('-') => Some(('-', 0u8)),
                Some('*') => Some(('*', 1u8)),
                Some('/') => Some(('/', 1u8)),
                _ => None,
            };
            if let Some((op, prec)) = next_op {
                self.eat_char();
                while let Some(top) = climb.ops.last() {
                    if top.prec < prec {
                        break;
                    }
                    reduce_one(self.units, &mut climb)?;
                }
                climb.ops.push(PendingOp { op, prec });
                continue 'outer;
            }

            while !climb.ops.is_empty() {
                reduce_one(self.units, &mut climb)?;
            }
            let value = climb.operands.pop().ok_or(EvalStatus::Unsupported)?;

            let Some(frame) = stack.pop() else {
                return Ok(value);
            };
            self.skip_ws();
            if self.eat_char() != Some(frame.close) {
                return Err(EvalStatus::Unsupported);
            }
            climb = frame.climb;
            pending_factor = Some(Quantity {
                value: value.value * frame.sign,
                unit: value.unit,
            });
        }
    }

    pub(crate) fn parse_identifier(&mut self) -> Option<&'s str> {
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
            if ch == '.' {
                self.eat_char();
                continue;
            }
            break;
        }
        let parsed = &self.src[start..self.pos];
        if parsed.ends_with("::") {
            return None;
        }
        if parsed.ends_with('.') {
            return None;
        }
        Some(parsed)
    }

    pub(crate) fn parse_numeric_literal(&mut self) -> Result<f64, EvalStatus> {
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

    pub(crate) fn parse_argument_slices(&mut self) -> Result<Vec<&'s str>, EvalStatus> {
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

    pub(crate) fn parse_unit_suffix(&mut self) -> Option<String> {
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

}

pub(crate) fn trim_quotes(value: &str) -> String {
    let mut out = value.trim().to_string();
    if out.starts_with('\'') && out.ends_with('\'') && out.len() > 1 {
        out = out[1..out.len() - 1].to_string();
    }
    out
}

pub(crate) fn normalize_unit_brackets(text: &str) -> String {
    text.replace("[[", "[").replace("]]", "]")
}

pub(crate) fn map_unit_error(err: UnitError) -> EvalStatus {
    match err {
        UnitError::UnknownUnit => EvalStatus::Unknown,
        UnitError::IncompatibleDimension => EvalStatus::TypeError,
        UnitError::UnsupportedConversion | UnitError::AmbiguousMetadata => EvalStatus::Unsupported,
    }
}

pub(crate) fn add_quantities_with_units(
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
    use super::*;
    use crate::semantic::graph::SemanticGraph;

    #[test]
    fn deeply_nested_parentheses_do_not_overflow_the_stack() {
        const DEPTH: usize = 200_000;
        let src = format!("{}1{}", "(".repeat(DEPTH), ")".repeat(DEPTH));
        let graph = SemanticGraph::default();
        let units = UnitRegistry::from_graph(&graph);
        let mut parser = QuantityParser::new(&src, &units, |_, _| Err(EvalStatus::Unknown));
        let result = parser.parse_expression().expect("parse deeply nested parens");
        assert_eq!(result.value, 1.0);
        assert!(result.unit.is_none());
    }

    #[test]
    fn deeply_chained_unary_signs_do_not_overflow_the_stack() {
        const DEPTH: usize = 200_000;
        let src = format!("{}1", "-".repeat(DEPTH));
        let graph = SemanticGraph::default();
        let units = UnitRegistry::from_graph(&graph);
        let mut parser = QuantityParser::new(&src, &units, |_, _| Err(EvalStatus::Unknown));
        let result = parser
            .parse_expression()
            .expect("parse deeply chained unary signs");
        // An even number of `-` signs cancels out to +1.
        assert_eq!(result.value, if DEPTH.is_multiple_of(2) { 1.0 } else { -1.0 });
    }
}
