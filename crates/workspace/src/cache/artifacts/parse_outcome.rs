//! `ParseOutcome` cache artifact.
//!
//! Key inputs: the raw [`ContentDigest`] of the source text; the [`ParseMode`]; the parser
//! package/AST version (`sysml_v2_parser::next::PARSE_AST_VERSION`); the parse diagnostic schema and
//! parse algorithm versions owned by this module; and any relevant parser options (there are
//! none exposed by `sysml_v2_parser` today, but [`PARSE_OPTIONS_VERSION`] is committed into the
//! key so that adding one later is a natural invalidation rather than a silent key collision).
//!
//! Parse artifacts deliberately omit the source URI: parser ranges are content-relative, so an
//! artifact is portable across files and relocated checkouts that happen to share content.
//!
//! This fixes a real pre-existing bug: `crates/workspace/src/parse_cache.rs` stores only the
//! `ParsedDocument`, so a warm editor-recovery hit today returns an empty parse-error list even
//! when the cold parse reported diagnostics. [`ParseOutcome`] always carries its complete,
//! structured diagnostics alongside the AST.
//!
//! Construction is deliberately narrow: [`ParseOutcome::from_strict`] and
//! [`ParseOutcome::from_editor_recovery`] are the only ways to build a value, and both consume a
//! *completed* parser result. There is no constructor for a partial, cancelled, or panicked
//! parse — callers must not call `put` for those cases at all.

use serde::{Deserialize, Serialize};

use source_identity::{ArtifactKey, CanonicalEncoder, ContentDigest};
use sysml_v2_parser::next::{
    DiagnosticCategory as ParserDiagnosticCategory, DiagnosticSeverity as ParserDiagnosticSeverity,
    ParseError as ParserParseError, ParseResult as ParserParseResult, ParsedDocument,
};

use crate::cache::{ArtifactIdentity, ArtifactKind, CacheArtifact};

/// Schema of the structured diagnostic payload this module stores. Bump when
/// [`ParseDiagnostic`]'s shape or meaning changes.
pub const PARSE_DIAGNOSTIC_SCHEMA_VERSION: u32 = 1;

/// Version of the algorithm used to derive a [`ParseOutcome`] from a parser result (e.g. how
/// `status` is classified from `errors`/`is_ok()`). Independent of the parser's own AST version
/// so that a change to *this* derivation invalidates entries without requiring a parser bump.
pub const PARSE_ALGORITHM_VERSION: u32 = 1;

/// Version of the (currently empty) set of parser options committed into the key. Bump when a
/// real parser option is introduced and threaded through [`ParseOutcomeIdentity`].
pub const PARSE_OPTIONS_VERSION: u32 = 1;

/// The two parse entry points `sysml_v2_parser` exposes (plan §6.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ParseMode {
    /// `sysml_v2_parser::next::parse`: all-or-nothing, no AST on failure.
    StrictSemantic,
    /// `sysml_v2_parser::next::parse_for_editor`: always produces an AST, diagnostics are additive.
    EditorRecovery,
}

impl ParseMode {
    fn tag(self) -> u8 {
        match self {
            ParseMode::StrictSemantic => 0,
            ParseMode::EditorRecovery => 1,
        }
    }
}

/// Everything the [`ArtifactKey`] for a [`ParseOutcome`] must commit (plan §6.2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseOutcomeIdentity {
    pub content_digest: ContentDigest,
    pub mode: ParseMode,
}

impl ArtifactIdentity for ParseOutcomeIdentity {
    fn artifact_key(&self) -> ArtifactKey {
        let mut enc = CanonicalEncoder::new(ArtifactKey::DOMAIN);
        enc.field(b"parse-outcome.v1");
        enc.field(self.content_digest.as_bytes());
        enc.field(&[self.mode.tag()]);
        enc.field_u64(sysml_v2_parser::next::PARSE_AST_VERSION as u64);
        enc.field_u64(PARSE_DIAGNOSTIC_SCHEMA_VERSION as u64);
        enc.field_u64(PARSE_ALGORITHM_VERSION as u64);
        enc.field_u64(PARSE_OPTIONS_VERSION as u64);
        ArtifactKey::from_encoder(&enc)
    }
}

/// Typed classification of a completed parse (plan §6.2). There is no "partial"/"cancelled"
/// variant: those states must never be constructed into a [`ParseOutcome`] at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParseStatus {
    /// No diagnostics; the AST is complete.
    Success,
    /// `EditorRecovery` only: at least one diagnostic was reported, but an AST is still present.
    RecoveredWithDiagnostics,
    /// `StrictSemantic` only: parsing failed outright; no AST is stored.
    ExpectedSyntaxFailure,
}

/// Serializable mirror of `sysml_v2_parser::next::DiagnosticSeverity`, which itself derives
/// `Serialize`/`Deserialize` under the `serde` feature but is re-declared here so this module's
/// on-disk payload shape does not silently change if the upstream crate's derive is dropped.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParseDiagnosticSeverity {
    Error,
    Warning,
}

impl From<ParserDiagnosticSeverity> for ParseDiagnosticSeverity {
    fn from(value: ParserDiagnosticSeverity) -> Self {
        match value {
            ParserDiagnosticSeverity::Error => ParseDiagnosticSeverity::Error,
            ParserDiagnosticSeverity::Warning => ParseDiagnosticSeverity::Warning,
        }
    }
}

/// Serializable mirror of `sysml_v2_parser::next::DiagnosticCategory`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParseDiagnosticCategory {
    ParseError,
    UnsupportedGrammarForm,
    UnresolvedSymbol,
}

impl From<ParserDiagnosticCategory> for ParseDiagnosticCategory {
    fn from(value: ParserDiagnosticCategory) -> Self {
        match value {
            ParserDiagnosticCategory::ParseError => ParseDiagnosticCategory::ParseError,
            ParserDiagnosticCategory::UnsupportedGrammarForm => {
                ParseDiagnosticCategory::UnsupportedGrammarForm
            }
            ParserDiagnosticCategory::UnresolvedSymbol => ParseDiagnosticCategory::UnresolvedSymbol,
        }
    }
}

/// A stable, 0-based LSP-style range, derived from `ParseError::to_lsp_range` at construction
/// time so a warm hit never needs to re-derive it from 1-based line/column fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParseDiagnosticRange {
    pub start_line: u32,
    pub start_character: u32,
    pub end_line: u32,
    pub end_character: u32,
}

/// A complete, structured parser diagnostic (plan §6.2): every field `sysml_v2_parser::next::ParseError`
/// carries, since `ParseError` itself is not `Serialize`. This is the fix for the pre-existing
/// bug where a warm parse-cache hit silently dropped diagnostics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParseDiagnostic {
    pub message: String,
    pub range: Option<ParseDiagnosticRange>,
    pub code: Option<String>,
    pub severity: Option<ParseDiagnosticSeverity>,
    pub expected: Option<String>,
    pub found: Option<String>,
    pub suggestion: Option<String>,
    pub category: Option<ParseDiagnosticCategory>,
    pub is_cascade: Option<bool>,
}

impl From<&ParserParseError> for ParseDiagnostic {
    fn from(err: &ParserParseError) -> Self {
        let range =
            err.to_lsp_range()
                .map(|(start_line, start_character, end_line, end_character)| {
                    ParseDiagnosticRange {
                        start_line,
                        start_character,
                        end_line,
                        end_character,
                    }
                });
        ParseDiagnostic {
            message: err.message.clone(),
            range,
            code: err.code.clone(),
            severity: err.severity.map(ParseDiagnosticSeverity::from),
            expected: err.expected.clone(),
            found: err.found.clone(),
            suggestion: err.suggestion.clone(),
            category: err.category.map(ParseDiagnosticCategory::from),
            is_cascade: err.is_cascade,
        }
    }
}

/// The `ParseOutcome` payload (plan §6.2): a typed status, the AST when the mode produces one,
/// and complete structured diagnostics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParseOutcome {
    pub status: ParseStatus,
    pub ast: Option<ParsedDocument>,
    pub diagnostics: Vec<ParseDiagnostic>,
}

impl ParseOutcome {
    /// Builds a [`ParseOutcome`] from a completed `sysml_v2_parser::next::parse` result. Never call
    /// this with a result that represents a cancelled, panicked, or otherwise incomplete parse —
    /// there is no such state representable here by design.
    pub fn from_strict(result: &Result<ParsedDocument, ParserParseError>) -> Self {
        match result {
            Ok(ast) => ParseOutcome {
                status: ParseStatus::Success,
                ast: Some(ast.clone()),
                diagnostics: Vec::new(),
            },
            Err(err) => ParseOutcome {
                status: ParseStatus::ExpectedSyntaxFailure,
                ast: None,
                diagnostics: vec![ParseDiagnostic::from(err)],
            },
        }
    }

    /// Builds a [`ParseOutcome`] from a completed `sysml_v2_parser::next::parse_for_editor` result.
    /// `parse_for_editor` never fails outright; it always returns an AST, so `ast` is always
    /// `Some`.
    pub fn from_editor_recovery(result: &ParserParseResult) -> Self {
        let status = if result.is_ok() {
            ParseStatus::Success
        } else {
            ParseStatus::RecoveredWithDiagnostics
        };
        ParseOutcome {
            status,
            ast: Some(result.document.clone()),
            diagnostics: result.errors.iter().map(ParseDiagnostic::from).collect(),
        }
    }
}

impl CacheArtifact for ParseOutcome {
    type Identity = ParseOutcomeIdentity;

    const KIND: ArtifactKind = ArtifactKind::ParseOutcome;
    const SCHEMA_VERSION: u32 = 2;

    fn validate_invariants(&self) -> Result<(), String> {
        match self.status {
            ParseStatus::Success | ParseStatus::RecoveredWithDiagnostics => {
                if self.ast.is_none() {
                    return Err(format!(
                        "ParseOutcome status {:?} requires an AST but ast is None",
                        self.status
                    ));
                }
            }
            ParseStatus::ExpectedSyntaxFailure => {
                if self.ast.is_some() {
                    return Err(
                        "ParseOutcome status ExpectedSyntaxFailure must not carry an AST"
                            .to_string(),
                    );
                }
                if self.diagnostics.is_empty() {
                    return Err(
                        "ParseOutcome status ExpectedSyntaxFailure requires at least one diagnostic"
                            .to_string(),
                    );
                }
            }
        }
        if self.status == ParseStatus::Success && !self.diagnostics.is_empty() {
            // A "Success" outcome with recorded diagnostics would be ambiguous with
            // RecoveredWithDiagnostics on rehydration; reject it rather than let the two
            // statuses represent overlapping states.
            return Err(
                "ParseOutcome status Success must not carry diagnostics; use \
                 RecoveredWithDiagnostics"
                    .to_string(),
            );
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // `ParserParseError` is a wide external type; this crate does not own it and cannot change
    // its size, so its `Result` is allowed to be "large" in these test-only helpers.
    #[allow(clippy::result_large_err)]
    fn strict_ok() -> Result<ParsedDocument, ParserParseError> {
        sysml_v2_parser::next::parse("package P;")
    }

    #[allow(clippy::result_large_err)]
    fn strict_err() -> Result<ParsedDocument, ParserParseError> {
        sysml_v2_parser::next::parse("package P { this is not valid sysml @@@ ")
    }

    #[test]
    fn strict_success_has_ast_and_no_diagnostics() {
        let result = strict_ok();
        assert!(result.is_ok(), "fixture should parse cleanly");
        let outcome = ParseOutcome::from_strict(&result);
        assert_eq!(outcome.status, ParseStatus::Success);
        assert!(outcome.ast.is_some());
        assert!(outcome.diagnostics.is_empty());
        outcome.validate_invariants().unwrap();
    }

    #[test]
    fn strict_failure_has_no_ast_and_a_diagnostic() {
        let result = strict_err();
        assert!(result.is_err(), "fixture should fail to parse strictly");
        let outcome = ParseOutcome::from_strict(&result);
        assert_eq!(outcome.status, ParseStatus::ExpectedSyntaxFailure);
        assert!(outcome.ast.is_none());
        assert!(!outcome.diagnostics.is_empty());
        outcome.validate_invariants().unwrap();
    }

    #[test]
    fn editor_recovery_retains_all_diagnostics() {
        let recovered =
            sysml_v2_parser::next::parse_for_editor("package P { this is not valid sysml @@@ ");
        let outcome = ParseOutcome::from_editor_recovery(&recovered);
        assert!(
            outcome.ast.is_some(),
            "recovery mode always produces an AST"
        );
        assert_eq!(outcome.diagnostics.len(), recovered.errors.len());
        if recovered.errors.is_empty() {
            assert_eq!(outcome.status, ParseStatus::Success);
        } else {
            assert_eq!(outcome.status, ParseStatus::RecoveredWithDiagnostics);
        }
        outcome.validate_invariants().unwrap();
    }

    #[test]
    fn invariant_rejects_success_status_without_ast() {
        let bad = ParseOutcome {
            status: ParseStatus::Success,
            ast: None,
            diagnostics: Vec::new(),
        };
        assert!(bad.validate_invariants().is_err());
    }

    #[test]
    fn invariant_rejects_expected_failure_with_ast() {
        let bad = ParseOutcome {
            status: ParseStatus::ExpectedSyntaxFailure,
            ast: Some(strict_ok().unwrap()),
            diagnostics: vec![ParseDiagnostic {
                message: "x".to_string(),
                range: None,
                code: None,
                severity: None,
                expected: None,
                found: None,
                suggestion: None,
                category: None,
                is_cascade: None,
            }],
        };
        assert!(bad.validate_invariants().is_err());
    }

    #[test]
    fn invariant_rejects_expected_failure_without_diagnostics() {
        let bad = ParseOutcome {
            status: ParseStatus::ExpectedSyntaxFailure,
            ast: None,
            diagnostics: Vec::new(),
        };
        assert!(bad.validate_invariants().is_err());
    }

    #[test]
    fn identity_key_changes_with_content_digest() {
        let a = ParseOutcomeIdentity {
            content_digest: ContentDigest::of_bytes(b"package A;"),
            mode: ParseMode::StrictSemantic,
        };
        let b = ParseOutcomeIdentity {
            content_digest: ContentDigest::of_bytes(b"package B;"),
            mode: ParseMode::StrictSemantic,
        };
        assert_ne!(a.artifact_key(), b.artifact_key());
    }

    #[test]
    fn identity_key_changes_with_mode() {
        let digest = ContentDigest::of_bytes(b"package A;");
        let a = ParseOutcomeIdentity {
            content_digest: digest,
            mode: ParseMode::StrictSemantic,
        };
        let b = ParseOutcomeIdentity {
            content_digest: digest,
            mode: ParseMode::EditorRecovery,
        };
        assert_ne!(a.artifact_key(), b.artifact_key());
    }
}
