//! Syntax-fidelity answers published by the parser authority.
//!
//! This crate is the only one that may name the parser, because it is the one that lowers the AST
//! to the semantic graph. Consumers that legitimately need a *syntactic* answer -- what packages a
//! file declares, where a token sits -- get it from here as plain data rather than by parsing the
//! same text a second time against an AST they would have to keep in step with the parser.
//!
//! Nothing here returns a parser type. That is what makes the boundary structural: a crate with no
//! parser dependency cannot name `ParsedDocument`, so it cannot hold one, cache one, or walk one.

use sysml_v2_parser::ast::{DeclarationName, QualifiedIdentification};
use sysml_v2_parser::{ParsedDocument, RootElement};

/// The declared names of every top-level package in `source`.
///
/// Strict parse: a source that does not parse yields the parser's own message rather than a
/// partial answer, because the caller (archive packing) is deciding an identity, not rendering an
/// editor view.
///
/// A package name may be a qualified path (`package A::B { ... }`). The simple alternative carries
/// its own label; the qualified one is an arena identity that only the owning document can render
/// back to authored text, which is precisely why this cannot be answered outside this crate.
pub fn package_declaration_names(source: &str) -> Result<Vec<String>, String> {
    let document = sysml_v2_parser::parse(source).map_err(|error| error.to_string())?;
    Ok(document
        .elements
        .iter()
        .filter_map(|element| match &element.value {
            RootElement::Package(package) => declaration_name(&document, &package.identification),
            RootElement::LibraryPackage(package) => {
                declaration_name(&document, &package.identification)
            }
            _ => None,
        })
        .collect())
}

fn declaration_name(
    document: &ParsedDocument,
    identification: &QualifiedIdentification,
) -> Option<String> {
    match identification.name.as_ref()? {
        DeclarationName::Simple(name) => Some(name.clone()),
        DeclarationName::Qualified(name) => document
            .qualified_declaration_name(*name)
            .map(|view| view.authored_text().to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_simple_and_qualified_package_names() {
        assert_eq!(
            package_declaration_names("package P { }").unwrap(),
            vec!["P".to_string()]
        );
        assert_eq!(
            package_declaration_names("package A::B { }").unwrap(),
            vec!["A::B".to_string()],
            "a qualified declaration name is arena-backed and must still render"
        );
        assert_eq!(
            package_declaration_names("library package L { }").unwrap(),
            vec!["L".to_string()]
        );
    }

    #[test]
    fn a_source_that_does_not_parse_is_an_error_not_an_empty_list() {
        assert!(package_declaration_names("package P { @@@ ").is_err());
    }
}
