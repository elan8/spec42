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

mod token_ranges;
mod token_util;

pub use token_ranges::semantic_token_roles;

use serde::{Deserialize, Serialize};

/// A parsed document, published as an opaque handle.
///
/// Consumers hold and cache this without naming a parser type. That is the whole point: a crate
/// with no parser dependency cannot reach inside, so it cannot walk an AST whose shape is the
/// pinned revision's business, and it cannot let an arena identity outlive the document that
/// gives it meaning.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyntaxDocument(sysml_v2_parser::ParsedDocument);

impl SyntaxDocument {
    /// The source this document was parsed from, after BOM stripping.
    pub fn source(&self) -> &str {
        self.0.source.as_str()
    }

    /// Whether the document has any root element at all.
    ///
    /// Enough for a caller asking "did anything parse", without handing out the elements
    /// themselves -- a root element is a parser type and stays behind this boundary.
    pub fn has_root_elements(&self) -> bool {
        !self.0.elements.is_empty()
    }

    pub(crate) fn inner(&self) -> &sysml_v2_parser::ParsedDocument {
        &self.0
    }
}

/// The parser's AST schema version, committed into every cache key that stores a document.
pub const SYNTAX_AST_VERSION: u32 = sysml_v2_parser::PARSE_AST_VERSION;

/// One parser diagnostic, in neutral terms.
///
/// A structural mirror rather than a re-export: the reporting policy that consumes it lives in
/// `sysml_diagnostics`, and it must not depend on the parser's own error type to say what it saw.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyntaxDiagnostic {
    pub severity: SyntaxDiagnosticSeverity,
    pub category: SyntaxDiagnosticCategory,
    pub message: String,
    pub code: Option<String>,
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub offset: Option<usize>,
    pub length: Option<usize>,
    pub expected: Option<String>,
    pub found: Option<String>,
    pub suggestion: Option<String>,
    pub is_cascade: Option<bool>,
}

impl SyntaxDiagnostic {
    /// The zero-based range this diagnostic points at, when it carries a position.
    ///
    /// One owner for the 1-based-to-0-based conversion, so no consumer has to remember which
    /// convention the parser reports in.
    pub fn range(&self) -> Option<SyntaxRange> {
        let (line, column) = (self.line?, self.column?);
        let start_line = line.saturating_sub(1);
        let start_character = column.saturating_sub(1);
        Some(SyntaxRange {
            start_line,
            start_character,
            end_line: start_line,
            end_character: start_character + self.length.unwrap_or(0) as u32,
        })
    }

    fn from_parse_error(error: &sysml_v2_parser::ParseError) -> Self {
        Self {
            severity: match error.severity {
                Some(sysml_v2_parser::DiagnosticSeverity::Warning) => {
                    SyntaxDiagnosticSeverity::Warning
                }
                _ => SyntaxDiagnosticSeverity::Error,
            },
            category: match error.category {
                Some(sysml_v2_parser::DiagnosticCategory::UnsupportedGrammarForm) => {
                    SyntaxDiagnosticCategory::UnsupportedGrammarForm
                }
                Some(sysml_v2_parser::DiagnosticCategory::UnresolvedSymbol) => {
                    SyntaxDiagnosticCategory::UnresolvedSymbol
                }
                _ => SyntaxDiagnosticCategory::ParseError,
            },
            message: error.message.clone(),
            code: error.code.clone(),
            line: error.line,
            column: error.column.map(|column| column as u32),
            offset: error.offset,
            length: error.length,
            expected: error.expected.clone(),
            found: error.found.clone(),
            suggestion: error.suggestion.clone(),
            is_cascade: error.is_cascade,
        }
    }
}

/// How severe a parser diagnostic is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyntaxDiagnosticSeverity {
    Error,
    Warning,
}

/// What kind of thing a parser diagnostic reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyntaxDiagnosticCategory {
    ParseError,
    UnsupportedGrammarForm,
    UnresolvedSymbol,
}

/// The result of an editor parse: always a document, diagnostics additive.
#[derive(Debug, Clone)]
pub struct SyntaxParse {
    pub document: SyntaxDocument,
    pub diagnostics: Vec<SyntaxDiagnostic>,
}

impl SyntaxParse {
    pub fn is_ok(&self) -> bool {
        self.diagnostics.is_empty()
    }
}

/// Editor-oriented parse: always produces a document, diagnostics are additive.
pub fn parse_for_editor(text: &str) -> SyntaxParse {
    let result = sysml_v2_parser::parse_with_diagnostics(text);
    SyntaxParse {
        diagnostics: result
            .errors
            .iter()
            .map(SyntaxDiagnostic::from_parse_error)
            .collect(),
        document: SyntaxDocument(result.document),
    }
}

/// Strict parse: all-or-nothing, no document on failure.
// The diagnostic is ~176 bytes and the success value is a whole document, so the `Err` variant is
// not the expensive half of this type. Boxing it would push a deref onto every caller to satisfy a
// lint about a value returned once per parsed file.
#[allow(clippy::result_large_err)]
pub fn parse_strict(text: &str) -> Result<SyntaxDocument, SyntaxDiagnostic> {
    sysml_v2_parser::parse(text)
        .map(SyntaxDocument)
        .map_err(|error| SyntaxDiagnostic::from_parse_error(&error))
}

/// A zero-based source range, in the LSP convention.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyntaxRange {
    pub start_line: u32,
    pub start_character: u32,
    pub end_line: u32,
    pub end_character: u32,
}

/// What a span *is*, in OMG terms -- not how an editor should paint it.
///
/// The authority owns traversal and node identification because both are the pinned grammar's
/// business. Which highlight index each role maps to is presentation policy and stays in
/// `sysml_tokens`, so this enum deliberately names elements rather than colours.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyntaxRole {
    /// A package, library package, namespace, or import target.
    Namespace,
    /// A definition that denotes a classifier.
    Class,
    /// A reference to a type.
    Type,
    /// A named feature, member, parameter, or end.
    Property,
    /// An interface definition or usage.
    Interface,
    /// A calculation or expression definition.
    Function,
}

mod closure_targets;
mod outline;

pub use closure_targets::{
    declared_package_names, import_targets, package_targets, type_reference_targets, PackageTargets,
};
pub use outline::{document_outline, folding_regions};

/// One node of a document outline, as the grammar sees it.
///
/// `kind` is the authored declaration keyword (`part def`, `feature decl`, ...), not an editor
/// symbol category: mapping it to an LSP `SymbolKind` is presentation policy and stays with the
/// host adapter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntaxOutlineNode {
    pub name: String,
    pub kind: String,
    pub range: SyntaxRange,
    pub selection_range: SyntaxRange,
    pub children: Vec<SyntaxOutlineNode>,
}

/// A multi-line region an editor may fold, named by what it is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntaxFoldingRegion {
    pub start_line: u32,
    pub end_line: u32,
    pub kind: Option<SyntaxFoldingKind>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyntaxFoldingKind {
    Region,
    Comment,
    Imports,
}

/// Whether `source` declares exactly one anonymous, non-empty package.
///
/// The shape a "wrap in package" code action needs, asked as a question about the grammar rather
/// than answered by a second AST walk in the host. `false` for a source that does not parse: an
/// unparseable document is not a document with one anonymous package.
pub fn declares_single_anonymous_package_with_members(source: &str) -> bool {
    let Ok(document) = sysml_v2_parser::parse(source) else {
        return false;
    };
    let mut packages = document
        .elements
        .iter()
        .filter_map(|node| match &node.value {
            RootElement::Package(package) => Some(package),
            _ => None,
        });
    let Some(package) = packages.next() else {
        return false;
    };
    if packages.next().is_some() {
        return false;
    }
    if declaration_name(&document, &package.identification).is_some_and(|name| !name.is_empty()) {
        return false;
    }
    matches!(
        &package.body,
        sysml_v2_parser::ast::PackageBody::Brace { elements, .. } if !elements.is_empty()
    )
}

/// Whether reformatting `source` into `candidate` provably preserves what the parser sees.
///
/// Two ways to be safe: both parse and yield the same tree modulo span movement, or neither parses
/// and both recover identically with the same diagnostics. Anything else is unproven and the
/// caller must leave the source alone.
pub fn reformatting_preserves_meaning(source: &str, candidate: &str) -> bool {
    match sysml_v2_parser::parse(source) {
        Ok(original) => sysml_v2_parser::parse(candidate).is_ok_and(|reparsed| {
            original.normalize_for_test_comparison() == reparsed.normalize_for_test_comparison()
        }),
        Err(_) => {
            let source = sysml_v2_parser::parse_for_editor(source);
            let candidate = sysml_v2_parser::parse_for_editor(candidate);
            !candidate.is_ok()
                && source.document.normalize_for_test_comparison()
                    == candidate.document.normalize_for_test_comparison()
                && recovery_signature(&source.errors) == recovery_signature(&candidate.errors)
        }
    }
}

fn recovery_signature(errors: &[sysml_v2_parser::ParseError]) -> Vec<String> {
    errors
        .iter()
        .map(|error| {
            format!(
                "{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}",
                error.category,
                error.severity,
                error.code,
                error.message,
                error.expected,
                error.found,
                error.suggestion,
            )
        })
        .collect()
}
