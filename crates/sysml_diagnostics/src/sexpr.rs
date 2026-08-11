//! Stable, reviewable rendering for diagnostic golden contracts.
//!
//! The rendering deliberately records diagnostic identity and location, rather
//! than human-oriented messages. Messages may be refined without changing the
//! diagnostic contract; codes, severities, source categories, ranges, related
//! locations, and order are the stable facts asserted by compatibility
//! fixtures.

use std::fmt;

use crate::ordering::{canonical_related_information, canonicalize_diagnostics};
use crate::{DiagnosticSeverity, SemanticDiagnostic};

/// Writes a diagnostic collection as a deterministic S-expression.
///
/// This is intentionally not an interchange format. It is a compact golden
/// representation owned by `sysml_diagnostics`.
pub fn write_diagnostics_sexpr(
    diagnostics: &[SemanticDiagnostic],
    output: &mut dyn fmt::Write,
) -> fmt::Result {
    let mut diagnostics = diagnostics.to_vec();
    canonicalize_diagnostics(&mut diagnostics);

    writeln!(output, "(diagnostics")?;
    for diagnostic in diagnostics {
        writeln!(output, "  (diagnostic")?;
        writeln!(
            output,
            "    (severity {})",
            severity_name(diagnostic.severity)
        )?;
        writeln!(output, "    (code {})", quote(&diagnostic.code))?;
        writeln!(output, "    (source {})", quote(&diagnostic.source))?;
        writeln!(
            output,
            "    (range (start {} {}) (end {} {}))",
            diagnostic.range.start.line,
            diagnostic.range.start.character,
            diagnostic.range.end.line,
            diagnostic.range.end.character,
        )?;
        render_related_information(output, &diagnostic)?;
        writeln!(output, "  )")?;
    }
    write!(output, ")")
}

fn render_related_information(
    output: &mut dyn fmt::Write,
    diagnostic: &SemanticDiagnostic,
) -> fmt::Result {
    let related = canonical_related_information(&diagnostic.related_information);
    if related.is_empty() {
        return Ok(());
    }

    writeln!(output, "    (related-information")?;
    for related in related {
        writeln!(output, "      (related")?;
        writeln!(output, "        (uri {})", quote(related.uri.as_str()))?;
        writeln!(
            output,
            "        (range (start {} {}) (end {} {}))",
            related.range.start.line,
            related.range.start.character,
            related.range.end.line,
            related.range.end.character,
        )?;
        writeln!(output, "      )")?;
    }
    writeln!(output, "    )")?;
    Ok(())
}

fn severity_name(severity: DiagnosticSeverity) -> &'static str {
    match severity {
        DiagnosticSeverity::Error => "error",
        DiagnosticSeverity::Warning => "warning",
        DiagnosticSeverity::Information => "information",
        DiagnosticSeverity::Hint => "hint",
    }
}

fn quote(value: &str) -> String {
    format!("{:?}", value)
}

#[cfg(test)]
mod tests {
    use sysml_model::semantic::text_span::{TextPosition, TextRange};
    use url::Url;

    use super::*;
    use crate::DiagnosticRelatedInfo;

    fn diagnostic(
        code: &str,
        severity: DiagnosticSeverity,
        range: TextRange,
    ) -> SemanticDiagnostic {
        SemanticDiagnostic {
            uri: Url::parse("memory://diagnostics/model.sysml").expect("URL"),
            range,
            severity,
            source: "semantic".to_string(),
            code: code.to_string(),
            message: "human wording is intentionally not rendered".to_string(),
            related_information: Vec::new(),
        }
    }

    #[test]
    fn renderer_sorts_by_range_then_severity_and_omits_messages() {
        let mut later = diagnostic(
            "later",
            DiagnosticSeverity::Warning,
            TextRange::new(TextPosition::new(4, 3), TextPosition::new(4, 4)),
        );
        later.related_information.push(DiagnosticRelatedInfo {
            uri: Url::parse("memory://diagnostics/related.sysml").expect("URL"),
            range: TextRange::new(TextPosition::new(0, 0), TextPosition::new(0, 1)),
            message: "related wording".to_string(),
        });
        let early_warning = diagnostic(
            "early_warning",
            DiagnosticSeverity::Warning,
            TextRange::new(TextPosition::new(1, 2), TextPosition::new(1, 3)),
        );
        let early_error = diagnostic(
            "early_error",
            DiagnosticSeverity::Error,
            TextRange::new(TextPosition::new(1, 2), TextPosition::new(1, 3)),
        );

        let mut rendered = String::new();
        write_diagnostics_sexpr(&[later, early_warning, early_error], &mut rendered)
            .expect("render diagnostics");
        assert_eq!(
            rendered,
            "(diagnostics\n  (diagnostic\n    (severity error)\n    (code \"early_error\")\n    (source \"semantic\")\n    (range (start 1 2) (end 1 3))\n  )\n  (diagnostic\n    (severity warning)\n    (code \"early_warning\")\n    (source \"semantic\")\n    (range (start 1 2) (end 1 3))\n  )\n  (diagnostic\n    (severity warning)\n    (code \"later\")\n    (source \"semantic\")\n    (range (start 4 3) (end 4 4))\n    (related-information\n      (related\n        (uri \"memory://diagnostics/related.sysml\")\n        (range (start 0 0) (end 0 1))\n      )\n    )\n  )\n)"
        );
        assert!(!rendered.contains("human wording"));
        assert!(!rendered.contains("related wording"));
    }
}
