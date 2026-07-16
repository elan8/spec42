use super::*;

pub(crate) fn normalize_invocation_args<'a>(args: &'a [&'a str]) -> Vec<&'a str> {
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

pub(crate) fn is_identifier_like(token: &str) -> bool {
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

pub(crate) fn parse_standalone_identifier(text: &str) -> Option<&str> {
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

pub(crate) fn json_value_to_f64(value: &Value) -> Option<f64> {
    match value {
        Value::Number(number) => number.as_f64().filter(|parsed| parsed.is_finite()),
        _ => None,
    }
}

pub(crate) fn number_to_json(value: f64) -> Value {
    if value.fract() == 0.0 {
        Value::Number(serde_json::Number::from(value as i64))
    } else {
        Value::Number(
            serde_json::Number::from_f64(value).unwrap_or_else(|| serde_json::Number::from(0)),
        )
    }
}

pub(crate) fn parse_invocation(text: &str) -> Option<(&str, Vec<&str>)> {
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

pub(crate) enum InvocationArgs<'a> {
    Positional(Vec<&'a str>),
    Named(HashMap<String, &'a str>),
}

#[derive(Debug, Clone)]
pub(crate) enum BoundValue {
    Quantity(Quantity),
    Collection(Vec<String>),
}

pub(crate) fn parse_invocation_args<'a>(
    args: &[&'a str],
) -> Result<InvocationArgs<'a>, EvalStatus> {
    if args.is_empty() {
        return Ok(InvocationArgs::Positional(Vec::new()));
    }
    let mut any_named = false;
    let mut any_positional = false;
    let mut named = HashMap::<String, &'a str>::new();
    let mut positional = Vec::<&'a str>::new();
    for arg in args {
        if let Some((name, expr)) = parse_named_arg(arg) {
            any_named = true;
            if named.insert(name, expr).is_some() {
                return Err(EvalStatus::Unsupported);
            }
        } else {
            any_positional = true;
            positional.push(*arg);
        }
    }
    if any_named && any_positional {
        return Err(EvalStatus::Unsupported);
    }
    if any_named {
        Ok(InvocationArgs::Named(named))
    } else {
        Ok(InvocationArgs::Positional(positional))
    }
}

pub(crate) fn parse_named_arg(arg: &str) -> Option<(String, &str)> {
    let trimmed = arg.trim();
    let mut paren_depth = 0usize;
    let mut bracket_depth = 0usize;
    for (idx, ch) in trimmed.char_indices() {
        match ch {
            '(' => paren_depth += 1,
            ')' => paren_depth = paren_depth.saturating_sub(1),
            '[' => bracket_depth += 1,
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            '=' if paren_depth == 0 && bracket_depth == 0 => {
                let lhs = trimmed[..idx].trim();
                let rhs = trimmed[idx + 1..].trim();
                if rhs.is_empty() {
                    return None;
                }
                let name = parse_standalone_identifier(lhs)?.to_string();
                return Some((name, rhs));
            }
            _ => {}
        }
    }
    None
}

pub(crate) fn callable_collection_params(node: &SemanticNode) -> HashSet<String> {
    let mut names = HashSet::new();
    for key in ["parameters", "analysisParams"] {
        if let Some(array) = node.attributes.get(key).and_then(Value::as_array) {
            for entry in array {
                let Some(direction) = entry.get("direction").and_then(Value::as_str) else {
                    continue;
                };
                let Some(name) = entry.get("name").and_then(Value::as_str) else {
                    continue;
                };
                let name = name.trim();
                if name.is_empty() || !matches!(direction, "in" | "inout") {
                    continue;
                }
                let ty = entry
                    .get("type")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .trim();
                if ty.contains("[*]") || ty.contains("[0..*]") || ty.contains("[1..*]") {
                    names.insert(name.to_string());
                }
            }
        }
    }
    if let Some(param) = node
        .attributes
        .get(ANALYSIS_EXPRESSION_KEY)
        .and_then(Value::as_str)
        .and_then(collection_param_from_sum_projection)
    {
        names.insert(param);
    }
    names
}

/// Detects `sum(parts.massKg)`-style collection roll-ups (SysML calc body pattern).
pub(crate) fn collection_param_from_sum_projection(expression: &str) -> Option<String> {
    let inner = expression.trim().strip_prefix("sum(")?.trim();
    let head = inner.split_once('.')?.0.trim();
    if head.is_empty()
        || !head
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
    {
        return None;
    }
    Some(head.to_string())
}

pub(crate) fn parse_tuple_member_path(part: &str) -> Result<String, EvalStatus> {
    let trimmed = part.trim();
    if trimmed.is_empty() {
        return Err(EvalStatus::Unsupported);
    }
    for segment in trimmed.split('.') {
        parse_standalone_identifier(segment).ok_or(EvalStatus::Unsupported)?;
    }
    Ok(trimmed.to_string())
}

pub(crate) fn parse_tuple_identifier_list(expr: &str) -> Result<Vec<String>, EvalStatus> {
    let trimmed = expr.trim();
    if !trimmed.starts_with('(') || !trimmed.ends_with(')') {
        return Err(EvalStatus::Unsupported);
    }
    let inner = &trimmed[1..trimmed.len() - 1];
    let mut items = Vec::new();
    let mut start = 0usize;
    let mut paren_depth = 0usize;
    let mut bracket_depth = 0usize;
    for (idx, ch) in inner.char_indices() {
        match ch {
            '(' => paren_depth += 1,
            ')' => paren_depth = paren_depth.saturating_sub(1),
            '[' => bracket_depth += 1,
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            ',' if paren_depth == 0 && bracket_depth == 0 => {
                let part = inner[start..idx].trim();
                items.push(parse_tuple_member_path(part)?);
                start = idx + 1;
            }
            _ => {}
        }
    }
    let tail = inner[start..].trim();
    if !tail.is_empty() {
        items.push(parse_tuple_member_path(tail)?);
    }
    if items.is_empty() {
        return Err(EvalStatus::Unsupported);
    }
    Ok(items)
}
