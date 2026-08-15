//! The publication's typed diagnostic contract.
//!
//! A diagnostic is a semantic fact about one publication, not a rendering of one. A consumer of
//! this contract -- the canonical S-expression adapter today, an editor protocol or generator host
//! later -- reads these values rather than recovering a code, severity, or outcome by parsing
//! presentation text, and never re-derives a rule the publication already settled.
//!
//! # Scope: this is not yet every diagnostic Spec42 reports
//!
//! This contract covers exactly the families resolution owns:
//!
//! - parser errors, carried through with the parser's own code;
//! - constructs this publication does not model ([`DiagnosticCode::UnsupportedPackageMember`] and
//!   the other `Unsupported*Member` codes);
//! - the settled outcome of every authored reference -- unresolved, ambiguous, unsupported, or
//!   non-converged.
//!
//! The conformance families are **not** here. Kind compatibility, structural-feature conformance,
//! view metadata, behavior, connection, expression, import, and requirement-case conformance are
//! still evaluated by `sysml_diagnostics` over the mutable semantic graph, and together they own
//! roughly thirty further public codes.
//!
//! A consumer that swaps `sysml_diagnostics` for this contract as it stands would therefore stop
//! reporting those codes. Do not treat this as a drop-in replacement for the legacy diagnostic
//! path; each conformance family needs its owner-defined facts published here first.
//! `crates/sysml_query/PRODUCTION_CUTOVER.md` tracks that work.
//!
//! The states below stay distinguishable on purpose. "No diagnostic" is the absence of an entry;
//! an unresolved prerequisite, an ambiguous one, a rule this publication does not support, a
//! non-converged solve, and parser recovery are each their own [`DiagnosticCode`], which is a
//! closed enum so a consumer matches them exhaustively instead of parsing a string. Publication
//! completeness is a separate fact on [`PublishedDiagnostics`], because a publication can be
//! complete and still carry warnings.

use crate::{PublicationCompleteness, TextRange};

/// The resolution-owned diagnostics of one publication, with the phase that produced them.
///
/// "Every diagnostic" only within the families listed in the module documentation; the conformance
/// families are not represented yet.
///
/// `completeness` travels with the diagnostics rather than being a separate lookup because the two
/// are only meaningful together: an empty slice from a complete publication means the model is
/// clean, while an empty slice from a non-converged one means nothing of the sort.
///
/// Only workspace-authored documents are reported. Library and standard-library sources are
/// admitted to the same semantic system, but their diagnostics are not the authoring surface this
/// contract describes, and reporting them would make every workspace inherit the library's.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublishedDiagnostics {
    pub completeness: PublicationCompleteness,
    /// Canonically ordered: by document identity, then by range, then by code. The order is a
    /// property of the publication, not of traversal, storage, or scheduling.
    pub diagnostics: Box<[Diagnostic]>,
}

/// One published diagnostic.
///
/// `code` is both the stable public identifier consumers key on and the typed outcome: which
/// failure this is -- unresolved, ambiguous, an unsupported reference, an unsupported construct,
/// a non-converged solve -- is decided by matching it, never by reading text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub code: DiagnosticCode,
    pub severity: DiagnosticSeverity,
    pub origin: DiagnosticOrigin,
    /// Where the diagnostic is reported. This is the authored site, not a definition it names.
    pub location: DiagnosticLocation,
    /// Further sites that explain the diagnostic, in canonical order.
    ///
    /// Ambiguity reports every candidate here. An empty slice means the diagnostic has no related
    /// site, never that the related sites were unavailable.
    pub related: Box<[DiagnosticLocation]>,
}

/// A document identity and range inside it.
///
/// Deliberately not [`crate::SourceLocation`]: that type also carries an
/// [`crate::OccurrenceRole`], and a parse error or an unsupported construct is neither a
/// declaration nor a reference. Reusing it would require inventing a role.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticLocation {
    pub document: Box<str>,
    pub range: TextRange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
}

impl DiagnosticSeverity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
        }
    }
}

/// Which owner decided the diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticOrigin {
    /// Reported by the parser contract and carried through unchanged.
    Parser,
    /// Decided by semantic construction or resolution.
    Semantic,
}

impl DiagnosticOrigin {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Parser => "parser",
            Self::Semantic => "semantic",
        }
    }
}

/// The stable public identifier of a diagnostic.
///
/// Codes are public behavior: consumers key suppression, documentation, and tests on them. The
/// mapping to text below is exhaustive and is the only place a code string is produced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagnosticCode {
    /// A code owned by the parser contract, mirrored verbatim.
    ///
    /// The parser may report an error without a code; the publication substitutes `parse_error`
    /// rather than omitting the field, so every diagnostic has one.
    Parser(Box<str>),

    UnsupportedPackageMember,
    UnsupportedPartDefinitionMember,
    UnsupportedPartUsageMember,
    UnsupportedAttributeMember,
    UnsupportedRequirementDefinitionMember,
    UnsupportedPortDefinitionMember,
    UnsupportedPortUsageMember,
    UnsupportedActionDefinitionMember,
    UnsupportedActionUsageMember,
    UnsupportedStateDefinitionMember,
    UnsupportedConnectionDefinitionMember,
    UnsupportedInterfaceDefinitionMember,
    UnsupportedViewDefinitionMember,
    UnsupportedConstraintDefinitionMember,
    UnsupportedCalcDefinitionMember,
    UnsupportedRenderingDefinitionMember,
    UnsupportedOccurrenceDefinitionMember,
    UnsupportedAnalysisCaseDefinitionMember,
    UnsupportedCaseDefinitionMember,
    UnsupportedVerificationCaseDefinitionMember,
    UnsupportedUseCaseDefinitionMember,
    UnsupportedReferenceUsageMember,
    UnsupportedRelationshipBodyMember,
    UnsupportedParserConstruct,

    UnresolvedTypeReference,
    UnresolvedSpecializesReference,
    UnresolvedImportTarget,
    UnresolvedReference,
    UnsupportedFilteredImport,
    UnsupportedReference,
    NonConvergedResolution,
    AmbiguousImportTarget,
    AmbiguousReference,
}

impl DiagnosticCode {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Parser(code) => code,
            Self::UnsupportedPackageMember => "unsupported_package_member",
            Self::UnsupportedPartDefinitionMember => "unsupported_part_definition_member",
            Self::UnsupportedPartUsageMember => "unsupported_part_usage_member",
            Self::UnsupportedAttributeMember => "unsupported_attribute_member",
            Self::UnsupportedRequirementDefinitionMember => {
                "unsupported_requirement_definition_member"
            }
            Self::UnsupportedPortDefinitionMember => "unsupported_port_definition_member",
            Self::UnsupportedPortUsageMember => "unsupported_port_usage_member",
            Self::UnsupportedActionDefinitionMember => "unsupported_action_definition_member",
            Self::UnsupportedActionUsageMember => "unsupported_action_usage_member",
            Self::UnsupportedStateDefinitionMember => "unsupported_state_definition_member",
            Self::UnsupportedConnectionDefinitionMember => {
                "unsupported_connection_definition_member"
            }
            Self::UnsupportedInterfaceDefinitionMember => "unsupported_interface_definition_member",
            Self::UnsupportedViewDefinitionMember => "unsupported_view_definition_member",
            Self::UnsupportedConstraintDefinitionMember => {
                "unsupported_constraint_definition_member"
            }
            Self::UnsupportedCalcDefinitionMember => "unsupported_calc_definition_member",
            Self::UnsupportedRenderingDefinitionMember => "unsupported_rendering_definition_member",
            Self::UnsupportedOccurrenceDefinitionMember => {
                "unsupported_occurrence_definition_member"
            }
            Self::UnsupportedAnalysisCaseDefinitionMember => {
                "unsupported_analysis_case_definition_member"
            }
            Self::UnsupportedCaseDefinitionMember => "unsupported_case_definition_member",
            Self::UnsupportedVerificationCaseDefinitionMember => {
                "unsupported_verification_case_definition_member"
            }
            Self::UnsupportedUseCaseDefinitionMember => "unsupported_use_case_definition_member",
            Self::UnsupportedReferenceUsageMember => "unsupported_reference_usage_member",
            Self::UnsupportedRelationshipBodyMember => "unsupported_relationship_body_member",
            Self::UnsupportedParserConstruct => "unsupported_parser_construct",
            Self::UnresolvedTypeReference => "unresolved_type_reference",
            Self::UnresolvedSpecializesReference => "unresolved_specializes_reference",
            Self::UnresolvedImportTarget => "unresolved_import_target",
            Self::UnresolvedReference => "unresolved_reference",
            Self::UnsupportedFilteredImport => "unsupported_filtered_import",
            Self::UnsupportedReference => "unsupported_reference",
            Self::NonConvergedResolution => "non_converged_resolution",
            Self::AmbiguousImportTarget => "ambiguous_import_target",
            Self::AmbiguousReference => "ambiguous_reference",
        }
    }
}

/// The code a parser error carries when the parser reports none.
pub(crate) const UNCODED_PARSE_ERROR: &str = "parse_error";
