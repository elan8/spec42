/// Normalizes "a.b.c" to "a::b::c" for node lookup (SysML uses dot for feature access).
pub(crate) fn normalize_for_lookup(s: &str) -> String {
    s.replace('.', "::")
}

/// Returns candidate qualified names for resolving an unqualified type reference.
/// If type_ref already contains "::", returns it as-is. Otherwise tries package prefixes
/// from container_prefix (e.g. "SurveillanceDrone::Propulsion" -> "SurveillanceDrone::PropulsionUnit").
pub(crate) fn type_ref_candidates(container_prefix: Option<&str>, type_ref: &str) -> Vec<String> {
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
pub(crate) fn type_ref_candidates_with_kind(
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
