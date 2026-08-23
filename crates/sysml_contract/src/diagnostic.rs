//! How a diagnostic is classified: how bad it is, who decided it, and what kind of outcome it is.
//!
//! The code that names the specific rule stays with the authority that can report it; these three
//! are the neutral vocabulary a consumer aggregates on, and they compute nothing.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    /// A fact worth surfacing that is not a fault: an unconnected port, a state machine with no
    /// finality indicator, a workspace with no library context.
    Information,
}

impl DiagnosticSeverity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Information => "information",
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

/// The neutral class of a published diagnostic.
///
/// Categories express the settled kind of outcome, while the authority's `DiagnosticCode` identifies the
/// specific rule or construct. A client may aggregate categories without guessing from code
/// strings; it must still use the code where the exact cause matters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DiagnosticCategory {
    MalformedSyntax,
    UnsupportedSyntax,
    UnsupportedSemantics,
    Unresolved,
    Ambiguous,
    NonConverged,
    Validation,
    MissingContext,
    Advisory,
    /// The upstream parser supplied no category. This is explicit rather than inferred from
    /// its code or prose.
    UnclassifiedParser,
}

impl DiagnosticCategory {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MalformedSyntax => "malformed_syntax",
            Self::UnsupportedSyntax => "unsupported_syntax",
            Self::UnsupportedSemantics => "unsupported_semantics",
            Self::Unresolved => "unresolved",
            Self::Ambiguous => "ambiguous",
            Self::NonConverged => "non_converged",
            Self::Validation => "validation",
            Self::MissingContext => "missing_context",
            Self::Advisory => "advisory",
            Self::UnclassifiedParser => "unclassified_parser",
        }
    }
}
