//! Syntax-fidelity answers published by the parser authority.
//!
//! This crate is the only one that may name the parser, because it is the one that lowers the AST
//! to the semantic graph. Consumers that legitimately need a *syntactic* answer -- what packages a
//! file declares, where a token sits -- get it from here as plain data rather than by parsing the
//! same text a second time against an AST they would have to keep in step with the parser.
//!
//! Nothing here returns a parser type. That is what makes the boundary structural: a crate with no
//! parser dependency cannot name `ParsedDocument`, so it cannot hold one, cache one, or walk one.
//!
//! [`SyntaxAuthority`] is the one place a document is parsed; [`ParsedSource`] is the memoised
//! handle every syntax query and the semantic build share.

use sysml_v2_parser::ast::{DeclarationName, QualifiedIdentification};
use sysml_v2_parser::{ParsedDocument, RootElement};

mod keywords;
mod parsed;

pub use keywords::{is_reserved_keyword, reserved_keywords, RESERVED_KEYWORDS};
pub use parsed::{ParsedSource, SyntaxAuthority};

fn top_level_package_names(document: &ParsedDocument) -> Vec<String> {
    document
        .elements
        .iter()
        .filter_map(|element| match &element.value {
            RootElement::Package(package) => declaration_name(document, &package.identification),
            RootElement::LibraryPackage(package) => {
                declaration_name(document, &package.identification)
            }
            _ => None,
        })
        .collect()
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

    fn parse(text: &str) -> ParsedSource {
        SyntaxAuthority::new().parse_text(text)
    }

    #[test]
    fn reports_simple_and_qualified_package_names() {
        assert_eq!(
            parse("package P { }").top_level_package_names(),
            vec!["P".to_string()]
        );
        assert_eq!(
            parse("package A::B { }").top_level_package_names(),
            vec!["A::B".to_string()],
            "a qualified declaration name is arena-backed and must still render"
        );
        assert_eq!(
            parse("library package L { }").top_level_package_names(),
            vec!["L".to_string()]
        );
    }

    #[test]
    fn a_source_that_does_not_parse_is_not_clean() {
        let parsed = parse("package P { @@@ ");
        assert!(!parsed.is_clean());
        assert!(parsed.first_error().is_some());
    }
}

mod token_ranges;
mod token_util;

use serde::{Deserialize, Serialize};

impl ParsedSource {
    /// The document outline, as the grammar sees it.
    pub fn outline(&self) -> Vec<SyntaxOutlineNode> {
        outline::document_outline(self.inner())
    }

    /// Multi-line regions an editor may fold.
    pub fn folding_regions(&self) -> Vec<SyntaxFoldingRegion> {
        outline::folding_regions(self.inner())
    }

    /// Every span the grammar gives a role, in source order.
    pub fn token_roles(&self) -> Vec<(SyntaxRange, SyntaxRole)> {
        token_ranges::semantic_token_roles(self.inner(), self.source())
    }

    /// The declared names of every top-level package.
    pub fn top_level_package_names(&self) -> Vec<String> {
        top_level_package_names(self.inner())
    }

    /// The declared names of every package, nested ones included, as qualified names.
    pub fn declared_package_names(&self) -> std::collections::HashSet<String> {
        closure_targets::declared_packages_from_parsed(self.inner())
    }

    /// Whether the source declares exactly one anonymous, non-empty package.
    pub fn declares_single_anonymous_package_with_members(&self) -> bool {
        declares_single_anonymous_package_in(self.inner())
    }

    /// Everything library-closure resolution needs about this source, from one parsed tree.
    pub fn closure_facts(&self) -> SyntaxClosureFacts {
        closure_targets::closure_facts(self.inner())
    }
}

/// What library-closure resolution asks of a source, answered from one parsed tree.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SyntaxClosureFacts {
    /// Every package declared, nested ones included, as qualified names.
    pub declared_packages: std::collections::HashSet<String>,
    /// Every import target authored, in source order, with its `::*`/`::**` shape suffix.
    pub import_targets: Vec<String>,
    /// Every type a declaration names, in source order.
    pub type_reference_targets: Vec<String>,
    /// The same two lists, per declared package.
    pub packages: Vec<PackageTargets>,
    /// Whether the source declares a measurement unit: an attribute with a `<short>` name typed
    /// by a `…Unit`. Unit catalogues are admitted to a closure whether or not they are imported.
    pub declares_unit_definitions: bool,
    /// Whether the source contains a value-with-unit literal such as `10 [kg]`.
    pub uses_unit_literals: bool,
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

    pub(super) fn from_parse_error(error: &sysml_v2_parser::ParseError) -> Self {
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
    /// The parser failed outright; the document carries an empty tree.
    ParserFailure,
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

pub use closure_targets::PackageTargets;

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

/// Whether the document declares exactly one anonymous, non-empty package.
///
/// The shape a "wrap in package" code action needs, asked as a question about the grammar rather
/// than answered by a second AST walk in the host.
fn declares_single_anonymous_package_in(document: &ParsedDocument) -> bool {
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
    if declaration_name(document, &package.identification).is_some_and(|name| !name.is_empty()) {
        return false;
    }
    matches!(
        &package.body,
        sysml_v2_parser::ast::PackageBody::Brace { elements, .. } if !elements.is_empty()
    )
}

/// Whether reformatting `source` into `candidate` provably preserves what the parser sees.
///
/// Two ways to be safe: both parse cleanly and yield the same tree modulo span movement, or
/// neither parses cleanly and both recover identically with the same diagnostics. Anything else
/// is unproven and the caller must leave the source alone.
pub fn reformatting_preserves_meaning_of(source: &ParsedSource, candidate: &ParsedSource) -> bool {
    if source.is_clean() {
        candidate.is_clean() && source.same_tree_as(candidate)
    } else {
        !candidate.is_clean()
            && source.same_tree_as(candidate)
            && recovery_signature(source.errors()) == recovery_signature(candidate.errors())
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
