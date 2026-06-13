/// Normalizes "a.b.c" to "a::b::c" for node lookup (SysML uses dot for feature access).
pub fn normalize_for_lookup(s: &str) -> String {
    s.replace('.', "::")
}

fn strip_wrapping_quotes(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.len() >= 2
        && ((trimmed.starts_with('\'') && trimmed.ends_with('\''))
            || (trimmed.starts_with('"') && trimmed.ends_with('"')))
    {
        return trimmed[1..trimmed.len() - 1].to_string();
    }
    trimmed.to_string()
}

/// Strip conjugation prefix and optional wrapping quotes from declared type references.
pub fn normalize_declared_type_ref(type_ref: &str) -> String {
    let trimmed = type_ref
        .trim()
        .strip_prefix('~')
        .map(str::trim)
        .unwrap_or(type_ref.trim());
    strip_wrapping_quotes(trimmed)
}

/// Returns candidate qualified names for resolving an unqualified type reference.
/// If type_ref already contains "::", returns it as-is. Otherwise tries package prefixes
/// from container_prefix (e.g. "SurveillanceDrone::Propulsion" -> "SurveillanceDrone::PropulsionUnit").
pub fn type_ref_candidates(container_prefix: Option<&str>, type_ref: &str) -> Vec<String> {
    if type_ref.contains("::") {
        return vec![type_ref.to_string()];
    }
    let mut candidates = vec![type_ref.to_string()];
    if let Some(prefix) = container_prefix {
        let segments: Vec<&str> = prefix.split("::").filter(|s| !s.is_empty()).collect();
        for i in 1..=segments.len() {
            let pkg_prefix = segments[..i].join("::");
            candidates.push(format!("{}::{}", pkg_prefix, type_ref));
        }
    }
    candidates
}

/// Like type_ref_candidates but also includes #kind-suffixed variants for disambiguated nodes
/// (e.g. when a package and part def share the same name).
pub fn type_ref_candidates_with_kind(
    container_prefix: Option<&str>,
    type_ref: &str,
    kind: &str,
) -> Vec<String> {
    let base = type_ref_candidates(container_prefix, type_ref);
    let kind_suffix = kind.replace(' ', "_");
    let mut out = base.clone();
    for c in base {
        if !c.contains('#') {
            out.push(format!("{}#{}", c, kind_suffix));
        }
    }
    out
}
