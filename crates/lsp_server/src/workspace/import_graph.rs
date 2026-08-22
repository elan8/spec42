//! Semantic dependency selection for incremental diagnostic republish.

use sysml_query::resolved_slice::{ElementSource, PublishedModel, QueryOutcome};
use tower_lsp::lsp_types::Url;

/// Workspace documents whose diagnostics may change with a provider.
///
/// The URI set comes from the publication's settled import/alias facts. Only an explicitly
/// incomplete semantic outcome permits conservative over-invalidation.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct AffectedDiagnosticDocuments {
    uris: Vec<Url>,
    pub(crate) conservative: bool,
}

impl AffectedDiagnosticDocuments {
    pub(crate) fn into_uris(self) -> Vec<Url> {
        self.uris
    }
}

pub(crate) fn affected_diagnostic_documents(
    model: &PublishedModel,
    workspace_uris: impl IntoIterator<Item = Url>,
    provider_uri: &Url,
) -> AffectedDiagnosticDocuments {
    let mut all_workspace = workspace_uris.into_iter().collect::<Vec<_>>();
    all_workspace.sort();
    all_workspace.dedup();
    all_workspace.retain(|uri| uri != provider_uri);

    let outcome = model
        .dependencies()
        .affected_documents(provider_uri.as_str());
    let (documents, conservative) = match outcome {
        QueryOutcome::Resolved(documents) => (documents, false),
        QueryOutcome::Recovered(documents) | QueryOutcome::UnsupportedWith(documents) => {
            (documents, true)
        }
        QueryOutcome::Unresolved
        | QueryOutcome::Ambiguous(_)
        | QueryOutcome::Unsupported
        | QueryOutcome::Recovery
        | QueryOutcome::Incomplete => {
            return AffectedDiagnosticDocuments {
                uris: all_workspace,
                conservative: true,
            };
        }
    };
    if conservative {
        return AffectedDiagnosticDocuments {
            uris: all_workspace,
            conservative: true,
        };
    }
    let mut uris = documents
        .iter()
        .filter(|document| document.source == ElementSource::Workspace)
        .filter_map(|document| Url::parse(&document.identity).ok())
        .collect::<Vec<_>>();
    uris.sort();
    uris.dedup();
    AffectedDiagnosticDocuments {
        uris,
        conservative: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use sysml_query::source::{SourceKind, SourceService};

    fn model(sources: &[(&str, &str)]) -> Arc<PublishedModel> {
        let source = SourceService::new();
        let documents = sources
            .iter()
            .map(|(uri, content)| source.admit(uri, content, SourceKind::Workspace).unwrap())
            .collect::<Vec<_>>();
        workspace::PublicationCoordinator::default()
            .publish(&documents, [])
            .unwrap()
    }

    fn uri(value: &str) -> Url {
        Url::parse(value).unwrap()
    }

    #[test]
    fn follows_nested_public_imports_transitively() {
        let a = "file:///workspace/a.sysml";
        let b = "file:///workspace/b.sysml";
        let c = "file:///workspace/c.sysml";
        let model = model(&[
            (a, "package A { part def T; }"),
            (b, "package B { package Nested { public import A::*; } }"),
            (c, "package C { import B::Nested::*; part p : T; }"),
        ]);
        let affected = affected_diagnostic_documents(&model, [uri(a), uri(b), uri(c)], &uri(a));
        assert!(!affected.conservative);
        assert_eq!(affected.into_uris(), vec![uri(b), uri(c)]);
    }

    #[test]
    fn alias_binding_is_a_semantic_dependency() {
        let a = "file:///workspace/a.sysml";
        let b = "file:///workspace/b.sysml";
        let model = model(&[
            (a, "package A { part def Thing; }"),
            (b, "package B { alias PublicThing for A::Thing; }"),
        ]);
        let affected = affected_diagnostic_documents(&model, [uri(a), uri(b)], &uri(a));
        assert!(!affected.conservative);
        assert_eq!(affected.into_uris(), vec![uri(b)]);
    }

    #[test]
    fn recovery_is_explicit_and_overinvalidates() {
        let a = "file:///workspace/a.sysml";
        let b = "file:///workspace/b.sysml";
        let c = "file:///workspace/c.sysml";
        let model = model(&[
            (a, "package A {}"),
            (b, "package B { import Missing::*;"),
            (c, "package C {}"),
        ]);
        let affected = affected_diagnostic_documents(&model, [uri(a), uri(b), uri(c)], &uri(a));
        assert!(affected.conservative);
        assert_eq!(affected.into_uris(), vec![uri(b), uri(c)]);
    }
}
