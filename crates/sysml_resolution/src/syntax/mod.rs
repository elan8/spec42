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
    let parsed = parse_for_editor(source);
    if let Some(error) = parsed.document.0.first_error() {
        return Err(error.message.clone());
    }
    Ok(top_level_package_names(parsed.document.inner()))
}

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
///
/// A thin wrapper over [`ParsedSource`] for callers that predate the handle; new code holds the
/// handle directly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntaxDocument(ParsedSource);

impl SyntaxDocument {
    /// The source this document was parsed from, after BOM stripping.
    pub fn source(&self) -> &str {
        self.0.source()
    }

    /// Whether the document has any root element at all.
    ///
    /// Enough for a caller asking "did anything parse", without handing out the elements
    /// themselves -- a root element is a parser type and stays behind this boundary.
    pub fn has_root_elements(&self) -> bool {
        self.0.has_root_elements()
    }

    /// The memoised handle this wrapper carries.
    pub fn parsed(&self) -> &ParsedSource {
        &self.0
    }

    pub(crate) fn inner(&self) -> &sysml_v2_parser::ParsedDocument {
        self.0.inner()
    }
}

impl From<ParsedSource> for SyntaxDocument {
    fn from(parsed: ParsedSource) -> Self {
        Self(parsed)
    }
}

// Serialised as the bare tree, which is all the on-disk parse cache ever stored. Diagnostics do
// not survive a round trip; the digest is recomputed from the tree's own source.
impl Serialize for SyntaxDocument {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.inner().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for SyntaxDocument {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        ParsedDocument::deserialize(deserializer)
            .map(|document| Self(ParsedSource::from_document(document)))
    }
}

impl ParsedSource {
    /// The document outline, as the grammar sees it.
    pub fn outline(&self) -> Vec<SyntaxOutlineNode> {
        document_outline(&SyntaxDocument(self.clone()))
    }

    /// Multi-line regions an editor may fold.
    pub fn folding_regions(&self) -> Vec<SyntaxFoldingRegion> {
        folding_regions(&SyntaxDocument(self.clone()))
    }

    /// Every span the grammar gives a role, in source order.
    pub fn token_roles(&self) -> Vec<(SyntaxRange, SyntaxRole)> {
        semantic_token_roles(&SyntaxDocument(self.clone()), self.source())
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

/// Editor-oriented parse, outside any memo: always produces a document, diagnostics are additive.
///
/// Hosts parse through a [`SyntaxAuthority`]; this is the stateless entry for callers with text
/// and no document.
pub fn parse_for_editor(text: &str) -> SyntaxParse {
    let parsed = ParsedSource::parse_text(
        text.to_owned(),
        source_identity::ContentDigest::of_bytes(text.as_bytes()),
    );
    SyntaxParse {
        diagnostics: parsed.diagnostics().to_vec(),
        document: SyntaxDocument(parsed),
    }
}

/// Strict parse: a document only when the parse is clean.
// The diagnostic is ~176 bytes and the success value is a whole document, so the `Err` variant is
// not the expensive half of this type. Boxing it would push a deref onto every caller to satisfy a
// lint about a value returned once per parsed file.
#[allow(clippy::result_large_err)]
pub fn parse_strict(text: &str) -> Result<SyntaxDocument, SyntaxDiagnostic> {
    let parsed = parse_for_editor(text);
    match parsed.document.0.first_error() {
        Some(error) => Err(error.clone()),
        None => Ok(parsed.document),
    }
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
    let parsed = parse_for_editor(source);
    if !parsed.document.0.is_clean() {
        return false;
    }
    declares_single_anonymous_package_in(parsed.document.inner())
}

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
/// Two ways to be safe: both parse and yield the same tree modulo span movement, or neither parses
/// and both recover identically with the same diagnostics. Anything else is unproven and the
/// caller must leave the source alone.
pub fn reformatting_preserves_meaning(source: &str, candidate: &str) -> bool {
    let source = parse_for_editor(source);
    reformatting_preserves_meaning_of(&source.document.0, &parse_for_editor(candidate).document.0)
}

/// [`reformatting_preserves_meaning`] over parsed handles, so a host's already-parsed document
/// is not parsed again to check a candidate against it.
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
