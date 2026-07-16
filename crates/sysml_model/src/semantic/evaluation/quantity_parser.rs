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

    pub(crate) fn parse_expression(&mut self) -> Result<Quantity, EvalStatus> {
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

    pub(crate) fn parse_term(&mut self) -> Result<Quantity, EvalStatus> {
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

    pub(crate) fn parse_factor(&mut self) -> Result<Quantity, EvalStatus> {
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

    pub(crate) fn parse_primary(&mut self) -> Result<Quantity, EvalStatus> {
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

    pub(crate) fn add_quantities(
        &self,
        left: Quantity,
        right: Quantity,
    ) -> Result<Quantity, EvalStatus> {
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

    pub(crate) fn sub_quantities(
        &self,
        left: Quantity,
        right: Quantity,
    ) -> Result<Quantity, EvalStatus> {
        self.add_quantities(
            left,
            Quantity {
                value: -right.value,
                unit: right.unit,
            },
        )
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
