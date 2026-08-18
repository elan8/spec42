use crate::language::SymbolEntry;
use std::sync::Arc;
use sysml_v2_parser::RootNamespace;
use tower_lsp::lsp_types::Url;
use workspace_session::{RelinkToken, TracksRelink};

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct ParseMetadata {
    pub(crate) parse_cached: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct IndexEntry {
    pub(crate) content: String,
    pub(crate) parsed: Option<RootNamespace>,
    pub(crate) parse_metadata: ParseMetadata,
    /// When `false`, the file is indexed for `sysml/librarySearch` only and is not admitted.
    pub(crate) admitted_to_publication: bool,
}

/// A settled library stratum together with the inputs it was built from.
///
/// The key is a digest of every admitted library document's identity and content, so a library
/// file that changes on disk retires the stratum instead of silently answering from stale text.
#[derive(Clone)]
pub(crate) struct CachedLibraryStratum {
    pub(crate) key: u64,
    pub(crate) stratum: Arc<sysml_query::resolved_slice::LibraryStratum>,
}

impl std::fmt::Debug for CachedLibraryStratum {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("CachedLibraryStratum")
            .field("key", &self.key)
            .field("documents", &self.stratum.document_count())
            .finish()
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
/// `workspace_session::SessionActor<ServerState>` (see `crate::workspace::WorkspaceHandle`).
/// `Clone` is required by the actor's `Arc::make_mut` clone-on-write mutation strategy; readers
/// only ever see an `Arc<ServerState>` snapshot, never a lock guard.
#[derive(Clone, Default)]
pub(crate) struct ServerState {
    pub(crate) workspace_roots: Vec<Url>,
    pub(crate) library_paths: Vec<Url>,
    pub(crate) standard_library_paths: Vec<Url>,
    pub(crate) session: workspace::WorkspaceSession,
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
    pub(crate) published_model: Option<Arc<sysml_query::resolved_slice::PublishedModel>>,
    /// The configured libraries, parsed and solved once.
    ///
    /// Rebuilding the publication on every document change would otherwise re-parse and re-solve
    /// the whole standard library each time, which measures as the entire cost of the rebuild.
    pub(crate) library_stratum: Option<CachedLibraryStratum>,
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
    fn published_model_mut(
        &mut self,
    ) -> &mut Option<Arc<sysml_query::resolved_slice::PublishedModel>>;
    /// Configured library roots: generic libraries first, then the standard library.
    fn library_roots(&self) -> (&[Url], &[Url]);
    fn library_stratum_mut(&mut self) -> &mut Option<CachedLibraryStratum>;
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
    fn published_model_mut(
        &mut self,
    ) -> &mut Option<Arc<sysml_query::resolved_slice::PublishedModel>> {
        &mut self.published_model
    }
    fn library_roots(&self) -> (&[Url], &[Url]) {
        (&self.library_paths, &self.standard_library_paths)
    }
    fn library_stratum_mut(&mut self) -> &mut Option<CachedLibraryStratum> {
        &mut self.library_stratum
    }

    fn open_in_editor(&self) -> &std::collections::BTreeSet<Url> {
        &self.open_in_editor
    }
}

/// Rebuilds the published model for the current index.
///
/// The configured libraries are admitted so that workspace references resolve against them; a
/// model without them has no `ScalarValues::Real` and no `Base::Anything` to resolve to. They are
/// parsed and solved once into a stratum and reused by every later rebuild, because re-solving the
/// standard library on each keystroke is measurably the whole cost of a rebuild.
pub(crate) fn refresh_published_model(state: &mut impl DocumentStore) {
    use sysml_query::resolved_slice::{
        build, BuildRequest, ConstructionStrategy, SourceDocument, SourceKind,
    };

    let (library_paths, standard_library_paths) = state.library_roots();
    let mut workspace = Vec::new();
    let mut libraries = Vec::new();
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
        let Ok(document) = SourceDocument::from_uri(uri.as_str(), entry.content.clone(), kind)
        else {
            continue;
        };
        if library {
            libraries.push((uri.clone(), entry.content.clone(), document));
        } else {
            workspace.push(document);
        }
    }

    let stratum = refresh_library_stratum(state, libraries);
    let request = match stratum.as_deref() {
        Some(stratum) => {
            BuildRequest::resolved_with_library(workspace, ConstructionStrategy::Parallel, stratum)
        }
        None => BuildRequest::resolved(workspace, ConstructionStrategy::Parallel),
    };
    *state.published_model_mut() = request
        .ok()
        .map(|request| request.reporting(reported))
        .and_then(|request| build(request).ok())
        .map(Arc::new);
}

/// Replaces the symbol projection from the committed immutable publication.
pub(crate) fn refresh_symbol_table_from_publication(state: &mut ServerState) {
    let Some(model) = state.published_model.as_deref() else {
        state.symbol_table.clear();
        return;
    };
    let mut symbols = Vec::new();
    let mut uris = state.index.keys().cloned().collect::<Vec<_>>();
    uris.sort();
    for uri in uris {
        symbols.extend(crate::language::symbol_entries_for_uri(model, &uri));
        if let Some(entry) = state.index.get(&uri) {
            crate::workspace::library_search::add_short_name_symbol_entries(
                &mut symbols,
                &entry.content,
                &uri,
            );
        }
    }
    state.symbol_table = symbols;
}

/// Returns the stratum for the current library documents, rebuilding it only when they change.
///
/// A stratum that fails to build leaves the slot empty and the publication is built from the
/// workspace alone -- degraded, but explicitly so, rather than resolving against half a library.
fn refresh_library_stratum(
    state: &mut impl DocumentStore,
    libraries: Vec<(Url, String, sysml_query::resolved_slice::SourceDocument)>,
) -> Option<Arc<sysml_query::resolved_slice::LibraryStratum>> {
    use std::hash::{Hash, Hasher};

    if libraries.is_empty() {
        *state.library_stratum_mut() = None;
        return None;
    }
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    for (uri, content, _) in &libraries {
        uri.as_str().hash(&mut hasher);
        content.hash(&mut hasher);
    }
    let key = hasher.finish();
    if let Some(cached) = state.library_stratum_mut().as_ref() {
        if cached.key == key {
            return Some(Arc::clone(&cached.stratum));
        }
    }
    let documents = libraries
        .into_iter()
        .map(|(_, _, document)| document)
        .collect();
    let stratum = sysml_query::resolved_slice::LibraryStratum::build(documents)
        .ok()
        .map(Arc::new);
    *state.library_stratum_mut() = stratum.as_ref().map(|stratum| CachedLibraryStratum {
        key,
        stratum: Arc::clone(stratum),
    });
    stratum
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

    /// Without the configured libraries in the publication, every reference into them is
    /// unresolved -- which is what the workspace-only build did.
    #[test]
    fn the_published_model_resolves_against_configured_libraries() {
        let mut state = state_with(
            "standard library package Lib { part def Wheel; }",
            "package W { part w : Lib::Wheel; }",
        );
        refresh_published_model(&mut state);

        let model = state.published_model.as_ref().expect("published model");
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

    /// The stratum is the reason admitting libraries is affordable: it must survive a workspace
    /// edit and retire when a library file changes.
    #[test]
    fn the_library_stratum_survives_workspace_edits_and_retires_on_library_change() {
        let mut state = state_with(
            "standard library package Lib { part def Wheel; }",
            "package W { part w : Lib::Wheel; }",
        );
        refresh_published_model(&mut state);
        let first = state.library_stratum.as_ref().expect("stratum").key;

        let workspace_uri = Url::parse("file:///model.sysml").expect("workspace uri");
        state
            .index
            .get_mut(&workspace_uri)
            .expect("workspace entry")
            .content = "package W { part w : Lib::Wheel; part x : Lib::Wheel; }".to_string();
        refresh_published_model(&mut state);
        assert_eq!(
            state.library_stratum.as_ref().expect("stratum").key,
            first,
            "editing the workspace must not retire the library stratum"
        );

        let library_uri = Url::parse("file:///libs/lib.sysml").expect("library uri");
        state
            .index
            .get_mut(&library_uri)
            .expect("library entry")
            .content =
            "standard library package Lib { part def Wheel; part def Axle; }".to_string();
        refresh_published_model(&mut state);
        assert_ne!(
            state.library_stratum.as_ref().expect("stratum").key,
            first,
            "a changed library file must retire the stratum rather than answer from stale text"
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
