use crate::language::SymbolEntry;
use session_actor::TracksRelink;
use std::sync::Arc;
use sysml_query::publication::{
    PublicationBuildFailure, PublicationSession, PublicationToken, SessionLifecycle,
};
use sysml_query::source::SourceDocument;
use sysml_query::syntax::ParsedSource;
use sysml_query::Services;
use tower_lsp::lsp_types::Url;

/// One indexed document: its admitted source and the tree the syntax service parsed for it.
///
/// Both are reference-counted handles, so the actor's copy-on-write clone of the whole index
/// on every mutation costs a refcount bump per entry, not a copy of every file's text and tree.
#[derive(Debug, Clone)]
pub(crate) struct IndexEntry {
    pub(crate) document: SourceDocument,
    pub(crate) parsed: ParsedSource,
    /// When `false`, the file is indexed for `sysml/librarySearch` only and is not admitted.
    pub(crate) admitted_to_publication: bool,
}

impl IndexEntry {
    pub(crate) fn content(&self) -> &str {
        self.document.content()
    }

    #[cfg(test)]
    pub(crate) fn for_test(uri: &Url, content: &str) -> Self {
        let services = Services::default();
        let document = services.source.admit_url(
            uri.clone(),
            content,
            sysml_query::source::SourceKind::Workspace,
        );
        let parsed = services.syntax.parse(&document);
        Self {
            document,
            parsed,
            admitted_to_publication: true,
        }
    }
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
/// `session_actor::SessionActor<ServerState>` (see `crate::session::WorkspaceHandle`).
/// `Clone` is required by the actor's `Arc::make_mut` clone-on-write mutation strategy; readers
/// only ever see an `Arc<ServerState>` snapshot, never a lock guard.
#[derive(Clone)]
pub(crate) struct ServerState {
    pub(crate) workspace_roots: Vec<Url>,
    pub(crate) library_paths: Vec<Url>,
    pub(crate) standard_library_paths: Vec<Url>,
    /// The one set of services this host publishes through.
    pub(crate) services: Services,
    /// The publication lifecycle and the current immutable publication.
    pub(crate) session: PublicationSession,
    /// Changes only when admitted semantic inputs/reporting change. Guards model mirroring from
    /// edits that race construction without treating search-only bookkeeping as semantic change.
    pub(crate) semantic_revision: u64,
    pub(crate) index: std::collections::HashMap<Url, IndexEntry>,
    /// Presentation projection derived only from `session.current()` at the admission barrier.
    pub(crate) semantic_symbols: Vec<SymbolEntry>,
    /// Parser-recovery search entries for documents deliberately excluded from publication.
    pub(crate) recovery_search_symbols: Vec<SymbolEntry>,
    /// Documents the editor currently has open.
    ///
    /// A library file is a library by *provenance* -- that is what decides how its names resolve.
    /// Opening one makes it an authoring surface as well, which is a different fact and one only
    /// the editor knows. The publication reports diagnostics for the documents named here in
    /// addition to the workspace's own, so an author editing a library file sees its diagnostics
    /// without every workspace inheriting the whole library's.
    pub(crate) open_in_editor: std::collections::BTreeSet<Url>,
    /// Last explicitly observed construction failure. The last good model remains readable.
    pub(crate) publication_failure: Option<Arc<PublicationBuildFailure>>,
}

impl Default for ServerState {
    fn default() -> Self {
        Self::new(Services::new())
    }
}

impl ServerState {
    pub(crate) fn new(services: Services) -> Self {
        let session = services.publication.session_empty();
        Self::with_session(services, session)
    }

    fn with_session(services: Services, session: PublicationSession) -> Self {
        Self {
            workspace_roots: Vec::new(),
            library_paths: Vec::new(),
            standard_library_paths: Vec::new(),
            services,
            session,
            semantic_revision: 0,
            index: std::collections::HashMap::new(),
            semantic_symbols: Vec::new(),
            recovery_search_symbols: Vec::new(),
            open_in_editor: std::collections::BTreeSet::new(),
            publication_failure: None,
        }
    }

    /// The current immutable publication.
    pub(crate) fn published_model(&self) -> &Arc<sysml_query::resolved_slice::PublishedModel> {
        self.session.current()
    }
}

impl TracksRelink for ServerState {
    type Token = PublicationToken;

    fn is_token_current(&self, token: &PublicationToken) -> bool {
        self.session.is_publication_current(token)
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
    fn recovery_search_symbols_mut(&mut self) -> &mut Vec<SymbolEntry>;
    /// The services this store admits and parses through.
    fn services(&self) -> &Services;
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
    fn recovery_search_symbols_mut(&mut self) -> &mut Vec<SymbolEntry> {
        &mut self.recovery_search_symbols
    }
    fn services(&self) -> &Services {
        &self.services
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
) -> (Vec<SourceDocument>, Vec<Box<str>>) {
    use sysml_query::source::SourceKind;

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
        documents.push(entry.document.with_kind(kind));
    }

    // A failed rebuild must not replace the last coherent publication. Readers may continue using
    // the older state until the coordinator can atomically produce a new one.
    (documents, reported)
}

/// Replaces the symbol projection from the committed immutable publication.
pub(crate) fn refresh_symbol_table_from_publication(state: &mut ServerState) {
    let model = Arc::clone(state.session.current());
    let mut symbols = Vec::new();
    let mut uris = state.index.keys().cloned().collect::<Vec<_>>();
    uris.sort();
    for uri in uris {
        symbols.extend(crate::language::symbol_entries_for_uri(&model, &uri));
    }
    state.semantic_symbols = symbols;
}

pub(crate) fn supports_semantic_queries(lifecycle: SessionLifecycle) -> bool {
    matches!(
        lifecycle,
        SessionLifecycle::Ready | SessionLifecycle::Reindexing
    )
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
            let entry = IndexEntry::for_test(&uri, content);
            state.index.insert(uri, entry);
        }
        state
    }

    async fn publish(state: &mut ServerState) {
        use sysml_query::publication::{PublicationOutcome, SemanticBuild};
        let (documents, reported) = publication_inputs(state);
        let prepared = state
            .services
            .publication
            .prepare(&documents, reported)
            .unwrap();
        let token = state.session.begin_build(prepared.identity().clone());
        let completion = state
            .services
            .publication
            .construct(SemanticBuild::new(token, prepared));
        assert_eq!(
            completion.admit(&mut state.session),
            PublicationOutcome::Published
        );
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

        let model = state.session.current();
        let symbols = match model.inspection().document_symbols("file:///model.sysml") {
            sysml_query::resolved_slice::QueryOutcome {
                answer: sysml_query::resolved_slice::QueryAnswer::Resolved(entries),
                ..
            } => entries,
            other => panic!("expected document symbols, got: {other:?}"),
        };
        let usage = symbols
            .iter()
            .find(|entry| model.qualified_name(entry.identity) == Some("W::w"))
            .expect("the workspace usage");
        let types = match model.types().direct_types(usage.identity) {
            sysml_query::resolved_slice::QueryOutcome {
                answer: sysml_query::resolved_slice::QueryAnswer::Resolved(types),
                ..
            } => types,
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
        let model = Arc::clone(state.session.current());
        let symbols = match model.inspection().document_symbols("file:///model.sysml") {
            sysml_query::resolved_slice::QueryOutcome {
                answer: sysml_query::resolved_slice::QueryAnswer::Resolved(symbols),
                ..
            } => symbols,
            other => panic!("workspace usage: {other:?}"),
        };
        let symbol = symbols
            .iter()
            .find(|symbol| model.qualified_name(symbol.identity) == Some("App::w"))
            .unwrap();
        assert!(matches!(
            model.types().direct_types(symbol.identity),
            sysml_query::resolved_slice::QueryOutcome {
                answer: sysml_query::resolved_slice::QueryAnswer::Resolved(ref types),
                ..
            } if types.len() == 1
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

        let model = state.session.current();
        let catalog = match model.diagrams().catalog() {
            sysml_query::resolved_slice::QueryOutcome {
                answer: sysml_query::resolved_slice::QueryAnswer::Resolved(catalog),
                ..
            } => catalog,
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
        let projection = match model.diagrams().view(view.semantic_id) {
            sysml_query::resolved_slice::QueryOutcome {
                answer: sysml_query::resolved_slice::QueryAnswer::Resolved(projection),
                ..
            } => projection,
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
    fn actor_tokens_follow_the_publication_session() {
        let mut state = ServerState::default();
        state.session.begin_startup();
        state.session.complete_startup();

        let first_token = state.session.publication();
        assert!(
            state.is_token_current(&first_token),
            "fresh publication token must be current"
        );

        state.session.invalidate_inputs();
        let second_token = state.session.publication();
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
