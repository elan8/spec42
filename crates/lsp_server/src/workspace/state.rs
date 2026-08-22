use crate::language::SymbolEntry;
use std::sync::Arc;
use sysml_resolution::syntax::SyntaxDocument;
use tower_lsp::lsp_types::Url;
use workspace_session::{
    PublishedModelSnapshot, RelinkToken, SemanticPublicationAuthority, TracksRelink,
};

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct ParseMetadata {
    pub(crate) parse_cached: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct IndexEntry {
    pub(crate) content: String,
    pub(crate) parsed: Option<SyntaxDocument>,
    pub(crate) parse_metadata: ParseMetadata,
    /// When `false`, the file is indexed for `sysml/librarySearch` only and is not admitted.
    pub(crate) admitted_to_publication: bool,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct RuntimeConfig {
    pub(crate) startup_trace_id: Option<String>,
    pub(crate) code_lens_enabled: bool,
    pub(crate) perf_logging_enabled: bool,
    /// Development-only: include library paths in the debounced workspace-wide diagnostics
    /// sweep. See `spec42.development.diagnoseLibraryPaths` and
    /// `publish_workspace_diagnostics`'s comment.
    pub(crate) diagnose_library_paths: bool,
}

/// The server's live workspace state — managed exclusively by a single
/// `workspace_session::SessionActor<ServerState>` (see `crate::workspace::WorkspaceHandle`).
/// `Clone` is required by the actor's `Arc::make_mut` clone-on-write mutation strategy; readers
/// only ever see an `Arc<ServerState>` snapshot, never a lock guard.
#[derive(Clone)]
pub(crate) struct ServerState {
    pub(crate) workspace_roots: Vec<Url>,
    pub(crate) library_paths: Vec<Url>,
    pub(crate) standard_library_paths: Vec<Url>,
    pub(crate) session: workspace::WorkspaceSession,
    /// Changes only when admitted semantic inputs/reporting change. Guards model mirroring from
    /// edits that race construction without treating search-only bookkeeping as semantic change.
    pub(crate) semantic_revision: u64,
    pub(crate) index: std::collections::HashMap<Url, IndexEntry>,
    pub(crate) symbol_table: Vec<SymbolEntry>,
    /// Documents the editor currently has open.
    ///
    /// A library file is a library by *provenance* -- that is what decides how its names resolve.
    /// Opening one makes it an authoring surface as well, which is a different fact and one only
    /// the editor knows. The publication reports diagnostics for the documents named here in
    /// addition to the workspace's own, so an author editing a library file sees its diagnostics
    /// without every workspace inheriting the whole library's.
    pub(crate) open_in_editor: std::collections::BTreeSet<Url>,
    /// Reader projection obtained only from the semantic publication authority.
    pub(crate) published_model: PublishedModelSnapshot,
    /// The single live owner of build/cache policy and atomic publication.
    pub(crate) publication_authority: Arc<SemanticPublicationAuthority>,
    /// Last explicitly observed construction failure. The last good model remains readable.
    pub(crate) publication_failure: Option<Arc<workspace::PublicationBuildFailure>>,
}

impl Default for ServerState {
    fn default() -> Self {
        Self::with_publication_coordinator(Arc::new(workspace::PublicationCoordinator::new()))
    }
}

impl ServerState {
    pub(crate) fn with_publication_coordinator(
        coordinator: Arc<workspace::PublicationCoordinator>,
    ) -> Self {
        let initial = coordinator
            .publish(&[], [])
            .expect("an empty initial semantic publication must be constructible");
        Self::with_initial_publication(coordinator, initial)
    }

    pub(crate) fn with_initial_publication(
        coordinator: Arc<workspace::PublicationCoordinator>,
        initial: Arc<sysml_query::resolved_slice::PublishedModel>,
    ) -> Self {
        let authority = Arc::new(SemanticPublicationAuthority::new(coordinator, initial));
        let published_model = authority.snapshot();
        Self {
            workspace_roots: Vec::new(),
            library_paths: Vec::new(),
            standard_library_paths: Vec::new(),
            session: workspace::WorkspaceSession::new(),
            semantic_revision: 0,
            index: std::collections::HashMap::new(),
            symbol_table: Vec::new(),
            open_in_editor: std::collections::BTreeSet::new(),
            published_model,
            publication_authority: authority,
            publication_failure: None,
        }
    }
}

impl TracksRelink for ServerState {
    fn is_token_current(&self, token: &RelinkToken) -> bool {
        self.session.is_token_current(token)
    }

    fn rekey_for_actor(&mut self) {
        self.session.rekey_for_owner();
    }
}

/// Shared accessors letting `workspace/services.rs`'s free functions stay agnostic of the
/// concrete state type (kept as a trait rather than inlined directly against `ServerState`
/// since several of those functions are also exercised in isolation by their own unit tests).
pub(crate) trait DocumentStore {
    fn index(&self) -> &std::collections::HashMap<Url, IndexEntry>;
    fn index_mut(&mut self) -> &mut std::collections::HashMap<Url, IndexEntry>;
    fn symbol_table_mut(&mut self) -> &mut Vec<SymbolEntry>;
    fn published_model(&self) -> Option<&sysml_query::resolved_slice::PublishedModel>;
    /// Configured library roots: generic libraries first, then the standard library.
    fn library_roots(&self) -> (&[Url], &[Url]);
    /// Documents the editor has open, whose diagnostics are reported whatever their provenance.
    fn open_in_editor(&self) -> &std::collections::BTreeSet<Url>;
}

impl DocumentStore for ServerState {
    fn index(&self) -> &std::collections::HashMap<Url, IndexEntry> {
        &self.index
    }
    fn index_mut(&mut self) -> &mut std::collections::HashMap<Url, IndexEntry> {
        &mut self.index
    }
    fn symbol_table_mut(&mut self) -> &mut Vec<SymbolEntry> {
        &mut self.symbol_table
    }
    fn published_model(&self) -> Option<&sysml_query::resolved_slice::PublishedModel> {
        Some(self.published_model.model())
    }
    fn library_roots(&self) -> (&[Url], &[Url]) {
        (&self.library_paths, &self.standard_library_paths)
    }
    fn open_in_editor(&self) -> &std::collections::BTreeSet<Url> {
        &self.open_in_editor
    }
}

/// Rebuilds the published model for the current index.
///
/// The configured libraries are admitted so that workspace references resolve against them; a
/// model without them has no `ScalarValues::Real` and no `Base::Anything` to resolve to. They are
/// handed to the workspace-owned publication coordinator, which alone decides whether its settled
/// library stratum can be reused.
pub(crate) fn publication_inputs(
    state: &impl DocumentStore,
) -> (Vec<sysml_query::source::SourceDocument>, Vec<Box<str>>) {
    use sysml_query::source::{SourceKind, SourceService};

    let source = SourceService::new();

    let (library_paths, standard_library_paths) = state.library_roots();
    let mut documents = Vec::new();
    let mut entries = state
        .index()
        .iter()
        .filter(|(_, entry)| entry.admitted_to_publication)
        .collect::<Vec<_>>();
    // Sorted so the stratum key is a property of the library's content rather than of hash-map
    // iteration order.
    entries.sort_by(|(left, _), (right, _)| left.as_str().cmp(right.as_str()));
    let mut reported: Vec<Box<str>> = Vec::new();
    for (uri, entry) in entries {
        let standard = crate::common::util::uri_under_any_library(uri, standard_library_paths);
        let library = standard || crate::common::util::uri_under_any_library(uri, library_paths);
        // A library file the editor has open is being authored, so its diagnostics are reported
        // even though its provenance keeps it out of the workspace's own set.
        if library && state.open_in_editor().contains(uri) {
            reported.push(uri.as_str().into());
        }
        let kind = if standard {
            SourceKind::StandardLibrary
        } else if library {
            SourceKind::Library
        } else {
            SourceKind::Workspace
        };
        documents.push(source.admit_url(uri.clone(), &entry.content, kind));
    }

    // A failed rebuild must not replace the last coherent publication. Readers may continue using
    // the older state until the coordinator can atomically produce a new one.
    (documents, reported)
}

/// Replaces the symbol projection from the committed immutable publication.
pub(crate) fn refresh_symbol_table_from_publication(state: &mut ServerState) {
    let model = state.published_model.model();
    let mut symbols = Vec::new();
    let mut uris = state.index.keys().cloned().collect::<Vec<_>>();
    uris.sort();
    for uri in uris {
        symbols.extend(crate::language::symbol_entries_for_uri(model, &uri));
    }
    state.symbol_table = symbols;
}

pub(crate) fn supports_semantic_queries(lifecycle: workspace::SessionLifecycle) -> bool {
    matches!(lifecycle, workspace::SessionLifecycle::Ready)
}

#[derive(Debug, Default)]
pub(crate) struct ScanSummary {
    pub(crate) roots_scanned: usize,
    pub(crate) roots_skipped_non_file: usize,
    pub(crate) candidate_files: usize,
    pub(crate) files_loaded: usize,
    pub(crate) read_failures: usize,
    pub(crate) uri_failures: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn server_state_is_clone() {
        fn assert_clone<T: Clone>() {}
        assert_clone::<ServerState>();
    }

    fn state_with(library: &str, workspace: &str) -> ServerState {
        let library_root = Url::parse("file:///libs/").expect("library root");
        let library_uri = Url::parse("file:///libs/lib.sysml").expect("library uri");
        let workspace_uri = Url::parse("file:///model.sysml").expect("workspace uri");
        let mut state = ServerState {
            standard_library_paths: vec![library_root],
            ..ServerState::default()
        };
        for (uri, content) in [(library_uri, library), (workspace_uri, workspace)] {
            state.index.insert(
                uri,
                IndexEntry {
                    content: content.to_string(),
                    parsed: None,
                    parse_metadata: ParseMetadata::default(),
                    admitted_to_publication: true,
                },
            );
        }
        state
    }

    async fn publish(state: &mut ServerState) {
        let (documents, reported) = publication_inputs(state);
        let build = state
            .publication_authority
            .begin_build(&documents, reported)
            .await
            .unwrap();
        let result = state
            .publication_authority
            .finish_build(build.construct())
            .await
            .unwrap();
        assert_eq!(
            result.outcome,
            workspace_session::SemanticPublicationOutcome::Published
        );
        state.published_model = state.publication_authority.snapshot();
    }

    /// Without the configured libraries in the publication, every reference into them is
    /// unresolved -- which is what the workspace-only build did.
    #[tokio::test]
    async fn the_published_model_resolves_against_configured_libraries() {
        let mut state = state_with(
            "standard library package Lib { part def Wheel; }",
            "package W { part w : Lib::Wheel; }",
        );
        publish(&mut state).await;

        let model = state.published_model.model();
        let symbols = match model.inspection().document_symbols("file:///model.sysml") {
            sysml_query::resolved_slice::QueryOutcome::Resolved(entries)
            | sysml_query::resolved_slice::QueryOutcome::Recovered(entries)
            | sysml_query::resolved_slice::QueryOutcome::UnsupportedWith(entries) => entries,
            other => panic!("expected document symbols, got: {other:?}"),
        };
        let usage = symbols
            .iter()
            .find(|entry| entry.qualified_name.as_ref() == "W::w")
            .expect("the workspace usage");
        let types = match model.types().direct_types(&usage.identity) {
            sysml_query::resolved_slice::QueryOutcome::Resolved(types)
            | sysml_query::resolved_slice::QueryOutcome::Recovered(types)
            | sysml_query::resolved_slice::QueryOutcome::UnsupportedWith(types) => types,
            other => panic!("expected settled types, got: {other:?}"),
        };
        assert_eq!(
            types.len(),
            1,
            "the usage must be typed by the library's declaration, got: {types:?}"
        );
    }

    #[tokio::test]
    async fn the_live_authority_resolves_against_a_generic_library() {
        let mut state = state_with(
            "library package Domain { part def Wheel; }",
            "package App { part w : Domain::Wheel; }",
        );
        state.library_paths = std::mem::take(&mut state.standard_library_paths);
        publish(&mut state).await;
        let model = state.published_model.clone().into_model();
        let symbols = match model.inspection().document_symbols("file:///model.sysml") {
            sysml_query::resolved_slice::QueryOutcome::Resolved(symbols)
            | sysml_query::resolved_slice::QueryOutcome::Recovered(symbols)
            | sysml_query::resolved_slice::QueryOutcome::UnsupportedWith(symbols) => symbols,
            other => panic!("workspace usage: {other:?}"),
        };
        let symbol = symbols
            .iter()
            .find(|symbol| symbol.qualified_name.as_ref() == "App::w")
            .unwrap();
        assert!(matches!(
            model.types().direct_types(&symbol.identity),
            sysml_query::resolved_slice::QueryOutcome::Resolved(ref types) if types.len() == 1
        ));
    }

    /// The configured-library path is the path used by the live editor. It must preserve public
    /// imports settled inside the reused library stratum, not merely direct library declarations.
    #[tokio::test]
    async fn the_published_model_resolves_filters_through_library_public_imports() {
        let mut state = state_with(
            concat!(
                "standard library package StandardViewDefinitions { view def GeneralView; } ",
                "standard library package SysML { public import Systems::*; ",
                "package Systems { metaclass PartUsage; } }"
            ),
            concat!(
                "package W { import StandardViewDefinitions::*; ",
                "part def Widget; part root : Widget; ",
                "view selected : GeneralView { expose root; filter @SysML::PartUsage; } }"
            ),
        );
        publish(&mut state).await;

        let model = state.published_model.model();
        let catalog = match model.diagrams().catalog() {
            sysml_query::resolved_slice::QueryOutcome::Resolved(catalog)
            | sysml_query::resolved_slice::QueryOutcome::Recovered(catalog)
            | sysml_query::resolved_slice::QueryOutcome::UnsupportedWith(catalog) => catalog,
            other => panic!("expected diagram catalog, got: {other:?}"),
        };
        let view = catalog
            .iter()
            .find(|view| {
                matches!(
                    &view.reference,
                    sysml_query::resolved_slice::DiagramSemanticReference::Qualified {
                        qualified_name,
                        ..
                    } if qualified_name.as_ref() == "W::selected"
                )
            })
            .expect("selected GeneralView");
        let projection = match model.diagrams().view(&view.semantic_id) {
            sysml_query::resolved_slice::QueryOutcome::Resolved(projection) => projection,
            other => panic!("expected resolved GeneralView projection, got: {other:?}"),
        };
        assert!(
            projection.incomplete_reasons.is_empty(),
            "settled library imports must not produce an incomplete view: {:?}",
            projection.incomplete_reasons
        );
        assert!(
            !projection.elements.is_empty(),
            "the resolved filter must retain the exposed part"
        );
    }

    #[test]
    fn tracks_relink_delegates_to_session() {
        let mut state = ServerState::default();
        state.session.begin_startup();
        state.session.complete_startup();

        let first_token = state.session.schedule_relink();
        assert!(
            state.is_token_current(&first_token),
            "freshly scheduled token must be current"
        );

        let second_token = state.session.schedule_relink();
        assert!(
            !state.is_token_current(&first_token),
            "superseded token must no longer be current"
        );
        assert!(
            state.is_token_current(&second_token),
            "the newly scheduled token must be current"
        );
    }
}
