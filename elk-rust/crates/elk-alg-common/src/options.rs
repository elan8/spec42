use std::collections::BTreeMap;

use elk_graph::{PropertyBag, PropertyValue};
use elk_meta::OptionRegistry;

#[must_use]
pub fn casefold_map<'a>(bag: &'a PropertyBag) -> BTreeMap<String, &'a PropertyValue> {
    let mut out = BTreeMap::new();
    for (k, v) in bag.iter() {
        out.insert(k.0.to_ascii_lowercase(), v);
    }
    out
}

#[must_use]
pub fn find_any<'a>(
    by_key: &'a BTreeMap<String, &'a PropertyValue>,
    keys: &[&str],
) -> Option<&'a PropertyValue> {
    for key in keys {
        if let Some(v) = by_key.get(&key.to_ascii_lowercase()) {
            return Some(*v);
        }
    }
    None
}

/// Find an option value by canonical id or any registered aliases (case-insensitive).
#[must_use]
pub fn find_option<'a>(
    reg: &OptionRegistry,
    by_key: &'a BTreeMap<String, &'a PropertyValue>,
    option_id: &str,
) -> Option<&'a PropertyValue> {
    if let Some(meta) = reg.lookup(option_id) {
        let mut keys = Vec::with_capacity(1 + meta.aliases.len());
        keys.push(meta.id.0.as_str());
        for a in &meta.aliases {
            keys.push(a.as_str());
        }
        return find_any(by_key, &keys);
    }
    find_any(by_key, &[option_id])
}

/// Parse a numeric property (int/float/string-number) into f32.
#[must_use]
pub fn get_f32(bag: &PropertyBag, key: &str) -> Option<f32> {
    let v = bag.get(&elk_graph::PropertyKey(key.to_string()))?;
    value_to_f32(v)
}

#[must_use]
pub fn value_to_f32(v: &PropertyValue) -> Option<f32> {
    match v {
        PropertyValue::Int(i) => Some(*i as f32),
        PropertyValue::Float(f) => Some(*f as f32),
        PropertyValue::String(s) => s.trim().parse::<f32>().ok(),
        _ => None,
    }
}

#[must_use]
pub fn value_to_usize(v: &PropertyValue) -> Option<usize> {
    match v {
        PropertyValue::Int(i) => (*i).try_into().ok(),
        PropertyValue::Float(f) => (*f as i64).try_into().ok(),
        PropertyValue::String(s) => s.trim().parse::<usize>().ok(),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn casefold_and_find_any_work() {
        let mut bag = PropertyBag::default();
        bag.insert("ELK.Direction", PropertyValue::String("DOWN".to_string()));
        let map = casefold_map(&bag);
        let v = find_any(&map, &["elk.direction"]).and_then(PropertyValue::as_str);
        assert_eq!(v, Some("DOWN"));
    }
}

