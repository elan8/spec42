//! Root package / namespace body extraction from the AST.

use sysml_parser::ast::{PackageBody, PackageBodyElement, RootElement};

use crate::ast_util::identification_name;

/// Extracts (elements, qualified, name_display, span) from Package or Namespace RootElement.
/// Returns None if body is not Brace.
pub(crate) fn root_element_body(
    re: &RootElement,
) -> Option<(
    &[sysml_parser::Node<PackageBodyElement>],
    String,
    String,
    &sysml_parser::Span,
)> {
    let (ident, body, span) = match re {
        RootElement::Package(p) => (&p.identification, &p.body, &p.span),
        RootElement::Namespace(n) => (&n.identification, &n.body, &n.span),
        RootElement::LibraryPackage(lp) => (&lp.identification, &lp.body, &lp.span),
        RootElement::Import(_) => return None,
    };
    let name = identification_name(ident);
    let qualified = if name.is_empty() {
        "(top level)".to_string()
    } else {
        name.clone()
    };
    let name_display = if name.is_empty() {
        "(top level)".to_string()
    } else {
        name
    };
    match body {
        PackageBody::Brace { elements } => Some((elements, qualified, name_display, span)),
        _ => None,
    }
}
