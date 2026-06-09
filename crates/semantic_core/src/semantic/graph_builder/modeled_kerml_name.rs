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
        if let Some(name) = name_after_definition_header(&tokens, pos) {
            return name;
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
        "def",
        "id",
        "case",
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

/// Skip `def`, legacy `id '…'`, and `use case` prefixes before the declared name.
fn name_after_definition_header(tokens: &[String], kw_pos: usize) -> Option<String> {
    let mut i = kw_pos + 1;
    while i < tokens.len() {
        let tok = &tokens[i];
        if tok.eq_ignore_ascii_case("def") {
            i += 1;
            continue;
        }
        if tok.eq_ignore_ascii_case("id") {
            i += 1;
            continue;
        }
        if tok.eq_ignore_ascii_case("case")
            && tokens
                .get(kw_pos)
                .is_some_and(|kw| kw.eq_ignore_ascii_case("use"))
        {
            i += 1;
            continue;
        }
        if tok.starts_with('\'') {
            i += 1;
            continue;
        }
        if tok.eq_ignore_ascii_case("specializes") {
            return None;
        }
        let s = sanitize_identifier(tok);
        if !s.is_empty() {
            return Some(s);
        }
        break;
    }
    None
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

    #[test]
    fn requirement_def_id_dialect_uses_declaration_name() {
        assert_eq!(
            extract_modeled_decl_name(
                "requirement",
                "requirement def id 'Req001' MaximaleMasse { doc /* x */ }",
                "_x"
            ),
            "MaximaleMasse"
        );
    }

    #[test]
    fn keyword_def_name_skips_def_token() {
        assert_eq!(
            extract_modeled_decl_name("action", "action def DoNavigate { }", "_x"),
            "DoNavigate"
        );
    }
}
