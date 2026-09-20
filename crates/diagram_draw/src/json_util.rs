//! Nullish-coalescing helper matching JavaScript `??` (skips JSON null only).

use serde_json::{Map, Value};

pub fn as_object(value: &Value) -> Map<String, Value> {
    value.as_object().cloned().unwrap_or_default()
}

pub fn as_array(value: &Value) -> &[Value] {
    match value {
        Value::Array(items) => items.as_slice(),
        _ => &[],
    }
}

pub fn as_array_opt(value: Option<&Value>) -> &[Value] {
    match value {
        Some(value) => as_array(value),
        None => &[],
    }
}

pub fn as_string(value: &Value, fallback: &str) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        _ => fallback.to_string(),
    }
}

pub fn field<'a>(object: &'a Value, key: &str) -> &'a Value {
    object.get(key).unwrap_or(&Value::Null)
}

pub fn first_present<'a>(values: impl IntoIterator<Item = &'a Value>) -> &'a Value {
    for value in values {
        if !value.is_null() {
            return value;
        }
    }
    &Value::Null
}
