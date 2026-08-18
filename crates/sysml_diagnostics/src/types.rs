//! The neutral diagnostic shape every Spec42 host renders from.
//!
//! A projection of one published diagnostic, carrying its stable code, severity, message, range,
//! and related sites in types no transport owns. It is a rendering target, never a source of
//! truth: [`crate::document_diagnostics`] is the only place these values are produced, and no
//! consumer may recover a semantic fact from one.

use sysml_query::resolved_slice::TextRange;
use url::Url;

/// How a host presents one diagnostic.
///
/// The three the publication settles. There is deliberately no `Hint`: nothing produces one, and a
/// severity a host can never receive is a branch that cannot be tested.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Information,
}

/// One further site that explains a diagnostic, with the owner's note about why it is related.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticRelatedInfo {
    pub uri: Url,
    pub range: TextRange,
    pub message: String,
}

/// One diagnostic, projected for a host.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticDiagnostic {
    pub uri: Url,
    pub range: TextRange,
    pub severity: DiagnosticSeverity,
    /// Which owner decided it: `sysml` for the parser contract, `semantic` for resolution.
    ///
    /// A public value editors already key on, so it keeps the spellings the LSP contract shipped
    /// with rather than the publication's internal names.
    pub source: String,
    pub code: String,
    pub message: String,
    pub related_information: Vec<DiagnosticRelatedInfo>,
}
