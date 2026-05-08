//! Derive stable graph node names from KerML modeled declarations (`KermlSemanticDecl`, etc.).
//! The parser stores the full declaration in `text` and the leading keyword in `bnf_production`.

/// Returns a short name suitable for `qualified_name_for_node` (e.g. `Real` for
/// `datatype Real specializes Complex;`).
pub(super) fn extract_modeled_decl_name(
    bnf_production: &str,
    text: &str,
    fallback: &str,
) -> String {
    let t = text.trim().trim_end_matches(';').trim();
    let tokens: Vec<String> = t
        .split_whitespace()
        .map(|s| {
            s.trim_end_matches(';')
                .trim_end_matches(',')
                .trim_end_matches(')')
                .to_string()
        })
        .filter(|s| !s.is_empty())
        .collect();
    let kw = bnf_production.trim();
    if let Some(pos) = tokens.iter().position(|tok| tok.eq_ignore_ascii_case(kw)) {
        if pos + 1 < tokens.len() {
            let name = &tokens[pos + 1];
            if !name.is_empty() && !name.eq_ignore_ascii_case("specializes") {
                let s = sanitize_identifier(name);
                if !s.is_empty() {
                    return s;
                }
            }
        }
    }
    const SKIP: &[&str] = &[
        "abstract",
        "public",
        "private",
        "protected",
        "inv",
        "specializes",
        "subsets",
    ];
    for tok in &tokens {
        if SKIP.iter().any(|s| tok.eq_ignore_ascii_case(s)) {
            continue;
        }
        let s = sanitize_identifier(tok);
        if !s.is_empty() {
            return s;
        }
    }
    fallback.to_string()
}

fn sanitize_identifier(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect()
}

#[cfg(test)]
mod tests {
    use super::extract_modeled_decl_name;

    #[test]
    fn datatype_real() {
        assert_eq!(
            extract_modeled_decl_name("datatype", "datatype Real specializes Complex;", "_x"),
            "Real"
        );
    }

    #[test]
    fn abstract_datatype() {
        assert_eq!(
            extract_modeled_decl_name(
                "datatype",
                "abstract datatype ScalarValue specializes DataValue;",
                "_x"
            ),
            "ScalarValue"
        );
    }

    #[test]
    fn fallback_when_no_match() {
        assert_eq!(extract_modeled_decl_name("unknown", "???;", "_fb"), "_fb");
    }
}
