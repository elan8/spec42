use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

use sysml_query::Services;
use sysml_query::StandardLibraryAvailability;
use tower_lsp::lsp_types::Url;

use crate::session::state::ServerState;
use crate::session::WorkspaceHandle;

use super::lifecycle::{project_boundary_for_uri, project_source_selected};

/// Owns the independent semantic publication for every editor-visible SysML project.
///
/// The editor folders are discovery ceilings. They are not combined into one semantic model:
/// the nearest `.project.json` (or the most-specific containing ceiling as a fallback) selects
/// exactly one handle for a document.
#[derive(Clone)]
pub(crate) struct ProjectRegistry {
    inner: Arc<RwLock<RegistryState>>,
    services: Services,
    library_catalog: Option<Arc<library_catalog::LibraryCatalog>>,
    standard_library_availability: StandardLibraryAvailability,
}

struct RegistryState {
    workspace_roots: Vec<Url>,
    library_paths: Vec<Url>,
    standard_library_paths: Vec<Url>,
    handles: BTreeMap<ProjectRoot, WorkspaceHandle>,
    admission_errors: BTreeMap<ProjectRoot, String>,
}

/// Canonical identity of one independently published editor project.
///
/// This is deliberately distinct from a document path: registry maps are keyed by semantic
/// publication owner, not by individual source documents.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ProjectRoot(PathBuf);

impl ProjectRegistry {
    pub(crate) fn new(
        services: Services,
        library_catalog: Option<Arc<library_catalog::LibraryCatalog>>,
        standard_library_availability: StandardLibraryAvailability,
    ) -> Self {
        Self {
            inner: Arc::new(RwLock::new(RegistryState {
                workspace_roots: Vec::new(),
                library_paths: Vec::new(),
                standard_library_paths: Vec::new(),
                handles: BTreeMap::new(),
                admission_errors: BTreeMap::new(),
            })),
            services,
            library_catalog,
            standard_library_availability,
        }
    }

    pub(crate) async fn configure(
        &self,
        workspace_roots: Vec<Url>,
        library_paths: Vec<Url>,
        standard_library_paths: Vec<Url>,
    ) {
        // Capture editor-owned revisions before replacing project sessions. Manifest creation,
        // deletion, or dependency edits may change the owning boundary, but must never revert an
        // unsaved buffer to the filesystem revision during rediscovery.
        let open_documents = {
            let state = self.inner.read().expect("project registry poisoned");
            let mut documents = BTreeMap::new();
            for handle in state.handles.values() {
                let snapshot = handle.snapshot();
                for uri in &snapshot.open_in_editor {
                    if let Some(entry) = snapshot.index.get(uri) {
                        documents.insert(uri.clone(), entry.clone());
                    }
                }
            }
            documents
        };
        let fallback_roots = workspace_roots
            .iter()
            .filter_map(|uri| uri.to_file_path().ok())
            .map(|path| canonicalize_or_self(&path))
            .collect::<Vec<_>>();
        let mut roots = fallback_roots.clone();
        roots.extend(sysml_query::source::discover_project_roots(&fallback_roots));
        roots.extend(
            open_documents
                .keys()
                .filter_map(|uri| boundary_root_for_uri(uri, &workspace_roots)),
        );
        roots.sort();
        roots.dedup();
        let created = roots
            .iter()
            .map(|root| {
                (
                    root.clone(),
                    WorkspaceHandle::spawn(ServerState::new(self.services.clone())),
                )
            })
            .collect::<Vec<_>>();
        let mut admission_errors = BTreeMap::new();
        for (root, handle) in &created {
            if let Ok(uri) = Url::from_directory_path(root) {
                let (project_library_paths, project_standard_library_paths, availability, error) =
                    self.admitted_library_paths(root, &library_paths, &standard_library_paths);
                if let Some(error) = error {
                    admission_errors.insert(ProjectRoot(root.clone()), error);
                }
                let _ = handle
                    .set_startup_config(
                        vec![uri],
                        project_library_paths,
                        project_standard_library_paths,
                        availability,
                    )
                    .await;
            }
        }
        let created_by_root = created
            .iter()
            .cloned()
            .map(|(root, handle)| (ProjectRoot(root), handle))
            .collect::<BTreeMap<_, _>>();
        let mut adopted_by_root: BTreeMap<PathBuf, Vec<_>> = BTreeMap::new();
        for (uri, entry) in open_documents {
            if let Some(root) = boundary_root_for_uri(&uri, &workspace_roots) {
                adopted_by_root.entry(root).or_default().push((uri, entry));
            }
        }
        for (root, documents) in adopted_by_root {
            if let Some(handle) = created_by_root.get(&ProjectRoot(root)) {
                let _ = handle.adopt_open_documents(documents).await;
            }
        }
        let mut state = self.inner.write().expect("project registry poisoned");
        state.workspace_roots = workspace_roots;
        state.library_paths = library_paths;
        state.standard_library_paths = standard_library_paths;
        state.handles = created
            .into_iter()
            .map(|(root, handle)| (ProjectRoot(root), handle))
            .collect();
        state.admission_errors = admission_errors;
    }

    pub(crate) async fn rediscover(&self) {
        let (workspace_roots, library_paths, standard_library_paths) = {
            let state = self.inner.read().expect("project registry poisoned");
            (
                state.workspace_roots.clone(),
                state.library_paths.clone(),
                state.standard_library_paths.clone(),
            )
        };
        self.configure(workspace_roots, library_paths, standard_library_paths)
            .await;
    }

    pub(crate) async fn handle_for_uri(&self, uri: &Url) -> Option<WorkspaceHandle> {
        if let Some(handle) = self.existing_handle_for_uri(uri) {
            let boundary_root = self.boundary_root(uri)?;
            let existing_root = handle
                .snapshot()
                .workspace_roots
                .first()
                .and_then(|root| root.to_file_path().ok())
                .map(|root| canonicalize_or_self(&root));
            if existing_root.as_ref() == Some(&boundary_root) {
                return Some(handle);
            }
        }
        let root = self.boundary_root(uri)?;
        let (library_paths, standard_library_paths) = {
            let state = self.inner.read().expect("project registry poisoned");
            (
                state.library_paths.clone(),
                state.standard_library_paths.clone(),
            )
        };
        let handle = WorkspaceHandle::spawn(ServerState::new(self.services.clone()));
        let root_uri = Url::from_directory_path(&root).ok()?;
        let (library_paths, standard_library_paths, availability, admission_error) =
            self.admitted_library_paths(&root, &library_paths, &standard_library_paths);
        let _ = handle
            .set_startup_config(
                vec![root_uri],
                library_paths,
                standard_library_paths,
                availability,
            )
            .await;
        // Lazily-created loose projects did not participate in the initialize-time scan. They
        // must nevertheless cross the publication lifecycle barrier before didOpen diagnostics
        // and semantic requests can observe their first authored document.
        let _ = handle.complete_startup().await;
        let mut state = self.inner.write().expect("project registry poisoned");
        if let Some(error) = admission_error {
            state
                .admission_errors
                .insert(ProjectRoot(root.clone()), error);
        }
        Some(
            state
                .handles
                .entry(ProjectRoot(root))
                .or_insert_with(|| handle.clone())
                .clone(),
        )
    }

    pub(crate) fn existing_handle_for_uri(&self, uri: &Url) -> Option<WorkspaceHandle> {
        let root = self.boundary_root(uri)?;
        self.inner
            .read()
            .expect("project registry poisoned")
            .handles
            .get(&ProjectRoot(root))
            .cloned()
    }

    pub(crate) fn handles(&self) -> Vec<WorkspaceHandle> {
        let state = self.inner.read().expect("project registry poisoned");
        state
            .handles
            .iter()
            .filter(|(root, _)| !state.admission_errors.contains_key(*root))
            .map(|(_, handle)| handle.clone())
            .collect()
    }

    /// Route reads to an owning publication, including admitted documents outside its root.
    /// Shared libraries need an explicit project context rather than an arbitrary consumer.
    pub(crate) fn query_handle_for_uri(
        &self,
        uri: &Url,
        project_uri: Option<&Url>,
    ) -> Result<WorkspaceHandle, String> {
        if let Some(error) = self.admission_error_for_uri(uri) {
            return Err(error);
        }
        // Workspace and loose documents already have an owner. Project provenance only
        // disambiguates library reads; a previous editor's context cannot override ownership.
        if let Some(handle) = self.existing_handle_for_uri(uri) {
            return Ok(handle);
        }
        if let Some(project_uri) = project_uri {
            if let Some(error) = self.admission_error_for_uri(project_uri) {
                return Err(error);
            }
            let handle = self.existing_handle_for_uri(project_uri).ok_or_else(|| {
                "project context URI is outside the configured workspace projects".to_owned()
            })?;
            let admits_library = self
                .handles_admitting_library_uri(uri)
                .iter()
                .any(|consumer| consumer.same_session(&handle));
            return if admits_library {
                Ok(handle)
            } else {
                Err("document URI is not admitted by the requested project".to_owned())
            };
        }
        let handles = self.handles_admitting_library_uri(uri);
        match handles.as_slice() {
            [handle] => Ok(handle.clone()),
            [] => Err("document URI is outside the configured workspace projects".to_owned()),
            _ => Err("shared library document requires project provenance".to_owned()),
        }
    }

    /// Every project publication that explicitly admits this library document.
    ///
    /// A library can be shared by several projects, so editor mutations to an opened library
    /// document fan out to each consumer instead of being assigned to an arbitrary project.
    pub(crate) fn handles_admitting_library_uri(&self, uri: &Url) -> Vec<WorkspaceHandle> {
        let Ok(document_path) = uri.to_file_path() else {
            return Vec::new();
        };
        let document_path = canonicalize_or_self(&document_path);
        self.handles()
            .into_iter()
            .filter(|handle| {
                let snapshot = handle.snapshot();
                snapshot
                    .library_paths
                    .iter()
                    .chain(&snapshot.standard_library_paths)
                    .filter_map(|root| root.to_file_path().ok())
                    .map(|root| canonicalize_or_self(&root))
                    .any(|root| document_path.starts_with(root))
            })
            .collect()
    }

    pub(crate) fn admission_errors(&self) -> Vec<(PathBuf, String)> {
        self.inner
            .read()
            .expect("project registry poisoned")
            .admission_errors
            .iter()
            .map(|(root, error)| (root.0.clone(), error.clone()))
            .collect()
    }

    pub(crate) fn admission_error_for_uri(&self, uri: &Url) -> Option<String> {
        let root = self.boundary_root(uri)?;
        self.inner
            .read()
            .expect("project registry poisoned")
            .admission_errors
            .get(&ProjectRoot(root))
            .cloned()
    }

    fn boundary_root(&self, uri: &Url) -> Option<PathBuf> {
        let state = self.inner.read().expect("project registry poisoned");
        let roots = state.workspace_roots.clone();
        if roots.is_empty() {
            let path = uri.to_file_path().ok()?;
            let parent = canonicalize_or_self(path.parent()?);
            if parent
                .join(sysml_query::source::PROJECT_MANIFEST_FILE)
                .is_file()
            {
                return Some(parent);
            }
            // With no editor root there is no safe filesystem ceiling to walk toward. Preserve
            // the historical loose-document workspace by sharing the first manifestless session;
            // explicit manifests still create their own publication above.
            if let Some(root) = state.handles.keys().next() {
                return Some(root.0.clone());
            }
            return Some(parent);
        }
        drop(state);
        boundary_root_for_uri(uri, &roots)
    }

    fn admitted_library_paths(
        &self,
        root: &Path,
        fallback_library_paths: &[Url],
        fallback_standard_library_paths: &[Url],
    ) -> (
        Vec<Url>,
        Vec<Url>,
        StandardLibraryAvailability,
        Option<String>,
    ) {
        if let Err(error) = sysml_query::source::ProjectSourceScope::load(root) {
            return (
                Vec::new(),
                Vec::new(),
                self.standard_library_availability,
                Some(error.to_string()),
            );
        }
        let Some(catalog) = &self.library_catalog else {
            let manifest = root.join(sysml_query::source::PROJECT_MANIFEST_FILE);
            if manifest.is_file() {
                let resolutions =
                    match library_catalog::resolve_project_manifest_dependencies(&manifest, &[]) {
                        Ok(resolutions) => resolutions,
                        Err(error) => {
                            return (
                                Vec::new(),
                                Vec::new(),
                                self.standard_library_availability,
                                Some(error),
                            )
                        }
                    };
                if resolutions.is_empty() {
                    return (
                        fallback_standard_library_paths.to_vec(),
                        fallback_standard_library_paths.to_vec(),
                        self.standard_library_availability,
                        None,
                    );
                }
                return (
                    Vec::new(),
                    Vec::new(),
                    self.standard_library_availability,
                    Some(format!(
                        "Project dependencies from {} were not satisfied because this LSP host supplied no library catalog: {}.",
                        manifest.display(),
                        serde_json::to_string(&resolutions)
                            .unwrap_or_else(|_| "unresolved dependencies".to_string())
                    )),
                );
            }
            return (
                fallback_library_paths.to_vec(),
                fallback_standard_library_paths.to_vec(),
                self.standard_library_availability,
                None,
            );
        };
        match library_catalog::resolve_project_dependency_admission(root, catalog) {
            Ok(admission) if admission.manifest_present => (
                admission
                    .library_roots
                    .iter()
                    .filter_map(|root| library_root_url(root))
                    .collect(),
                admission
                    .standard_library_roots
                    .iter()
                    .filter_map(|root| library_root_url(root))
                    .collect(),
                admission.standard_library_availability,
                None,
            ),
            Ok(_) => (
                fallback_library_paths.to_vec(),
                fallback_standard_library_paths.to_vec(),
                self.standard_library_availability,
                None,
            ),
            Err(error) => {
                tracing::error!(project_root = %root.display(), %error, "project dependency admission failed");
                // A manifest dependency failure is explicit and admits no fallback libraries.
                (
                    Vec::new(),
                    Vec::new(),
                    self.standard_library_availability,
                    Some(error),
                )
            }
        }
    }
}

fn canonicalize_or_self(path: &Path) -> PathBuf {
    std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
}

/// A manifest-admitted library root as a URL normalised the same way indexed document URIs are.
///
/// `Url::from_directory_path` alone keeps the platform's raw drive-letter casing (`file:///C:/…`),
/// while scanned document URIs are lowercased by `normalize_file_uri` (`file:///c:/…`). Without
/// this the two disagree and `uri_under_any` — the library/workspace classifier used by the
/// diagnostics sweep and `publication_inputs` — never matches, so every admitted-project stdlib
/// file is treated as a workspace document and diagnosed.
fn library_root_url(path: &Path) -> Option<Url> {
    Url::from_directory_path(path)
        .ok()
        .map(|uri| crate::common::util::normalize_file_uri(&uri))
}

fn boundary_root_for_uri(uri: &Url, workspace_roots: &[Url]) -> Option<PathBuf> {
    if workspace_roots.is_empty() {
        let path = uri.to_file_path().ok()?;
        // LSP permits `rootUri: null`. Such a loose document remains a useful manifestless
        // project, bounded at its containing directory rather than being rejected outright.
        return Some(canonicalize_or_self(path.parent()?));
    }
    if let Some(boundary) = project_boundary_for_uri(uri, workspace_roots) {
        let path = uri.to_file_path().ok()?;
        if sysml_query::source::is_sysml_like(&path) {
            match project_source_selected(uri, workspace_roots) {
                Ok(false) => return None,
                _ => {} // Admission owns configuration-error reporting; never hide that state.
            }
        }
        return Some(boundary.project_root().to_path_buf());
    }
    // Editors may open a document outside every advertised workspace folder. It is not allowed
    // to leak into an unrelated project, but remains useful as an independent loose project.
    let path = uri.to_file_path().ok()?;
    Some(canonicalize_or_self(path.parent()?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[tokio::test]
    async fn installed_manifests_cannot_create_workspace_sessions_but_keep_library_provenance() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        let library = root.join(".sysand/lib/types");
        fs::create_dir_all(&library).unwrap();
        fs::write(root.join("Model.sysml"), "package Model;").unwrap();
        fs::write(
            library.join(".project.json"),
            r#"{"name":"Types","version":"1.0.0"}"#,
        )
        .unwrap();
        fs::write(library.join("Types.sysml"), "package Types;").unwrap();
        let model_uri = Url::from_file_path(root.join("Model.sysml")).unwrap();
        let library_uri = Url::from_file_path(library.join("Types.sysml")).unwrap();
        let registry = ProjectRegistry::new(
            Services::default(),
            None,
            StandardLibraryAvailability::Unavailable,
        );
        registry
            .configure(
                vec![Url::from_directory_path(root).unwrap()],
                vec![library_root_url(&library).unwrap()],
                Vec::new(),
            )
            .await;
        assert_eq!(registry.handles().len(), 1);
        assert!(registry.handle_for_uri(&library_uri).await.is_none());
        assert_eq!(registry.handles().len(), 1);
        assert!(registry
            .query_handle_for_uri(&library_uri, Some(&model_uri))
            .is_ok());
    }

    #[tokio::test]
    async fn source_scope_applies_to_open_watch_and_query_routing_after_rediscovery() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        fs::create_dir(root.join("scratch")).unwrap();
        fs::write(root.join("Model.sysml"), "package Model;").unwrap();
        fs::write(root.join("scratch/Draft.sysml"), "package Draft;").unwrap();
        let model = Url::from_file_path(root.join("Model.sysml")).unwrap();
        let draft = Url::from_file_path(root.join("scratch/Draft.sysml")).unwrap();
        let registry = ProjectRegistry::new(
            Services::default(),
            None,
            StandardLibraryAvailability::Unavailable,
        );
        registry
            .configure(
                vec![Url::from_directory_path(root).unwrap()],
                Vec::new(),
                Vec::new(),
            )
            .await;
        let original = registry.handle_for_uri(&model).await.unwrap();
        original
            .store_document_text_fast(model.clone(), "package Unsaved;".into())
            .await
            .unwrap();
        original
            .set_document_open(model.clone(), true)
            .await
            .unwrap();
        assert!(registry.handle_for_uri(&draft).await.is_some());
        fs::write(root.join("syside.toml"), "exclude = ['scratch/**']").unwrap();
        registry.rediscover().await;
        assert!(registry.handle_for_uri(&draft).await.is_none());
        assert!(registry.query_handle_for_uri(&draft, None).is_err());
        let replacement = registry.handle_for_uri(&model).await.unwrap();
        assert_eq!(
            replacement.snapshot().index.get(&model).unwrap().content(),
            "package Unsaved;"
        );
        fs::remove_file(root.join("syside.toml")).unwrap();
        registry.rediscover().await;
        assert!(registry.handle_for_uri(&draft).await.is_some());
    }

    #[tokio::test]
    async fn malformed_source_configuration_is_an_explicit_project_admission_error() {
        let temp = tempfile::tempdir().unwrap();
        fs::write(temp.path().join("syside.toml"), "include = 4").unwrap();
        let uri = Url::from_file_path(temp.path().join("Model.sysml")).unwrap();
        let registry = ProjectRegistry::new(
            Services::default(),
            None,
            StandardLibraryAvailability::Unavailable,
        );
        registry
            .configure(
                vec![Url::from_directory_path(temp.path()).unwrap()],
                Vec::new(),
                Vec::new(),
            )
            .await;
        assert!(registry.admission_error_for_uri(&uri).is_some());
        assert!(registry.handles().is_empty());
    }

    #[tokio::test]
    async fn library_queries_use_admission_and_require_context_when_shared() {
        let temp = tempfile::tempdir().unwrap();
        let a = temp.path().join("a");
        let b = temp.path().join("b");
        let library = temp.path().join("library");
        for path in [&a, &b, &library] {
            fs::create_dir_all(path).unwrap();
        }
        let a_uri = Url::from_file_path(a.join("App.sysml")).unwrap();
        let b_uri = Url::from_file_path(b.join("App.sysml")).unwrap();
        let library_uri = Url::from_file_path(library.join("ScalarValues.kerml")).unwrap();
        fs::write(
            library.join("ScalarValues.kerml"),
            "library package ScalarValues;",
        )
        .unwrap();
        let foreign_uri = Url::from_file_path(temp.path().join("unadmitted.kerml")).unwrap();
        let registry = ProjectRegistry::new(
            Services::default(),
            None,
            StandardLibraryAvailability::Unavailable,
        );
        let library_root = Url::from_directory_path(&library).unwrap();
        for (libraries, standard_libraries) in [
            (vec![library_root.clone()], Vec::new()),
            (Vec::new(), vec![library_root.clone()]),
        ] {
            registry
                .configure(
                    vec![Url::from_directory_path(&a).unwrap()],
                    libraries,
                    standard_libraries,
                )
                .await;
            let owner = registry.existing_handle_for_uri(&a_uri).unwrap();
            assert!(registry
                .query_handle_for_uri(&library_uri, None)
                .unwrap()
                .same_session(&owner));
            assert!(registry.query_handle_for_uri(&foreign_uri, None).is_err());
            assert!(registry
                .query_handle_for_uri(&foreign_uri, Some(&a_uri))
                .is_err());
        }

        registry
            .configure(
                vec![
                    Url::from_directory_path(&a).unwrap(),
                    Url::from_directory_path(&b).unwrap(),
                ],
                vec![library_root.clone()],
                vec![library_root],
            )
            .await;
        assert!(registry.query_handle_for_uri(&library_uri, None).is_err());
        for project_uri in [&a_uri, &b_uri] {
            let owner = registry.existing_handle_for_uri(project_uri).unwrap();
            assert!(registry
                .query_handle_for_uri(&library_uri, Some(project_uri))
                .unwrap()
                .same_session(&owner));
        }
        assert!(registry
            .query_handle_for_uri(&a_uri, Some(&b_uri))
            .unwrap()
            .same_session(&registry.existing_handle_for_uri(&a_uri).unwrap()));
    }

    #[tokio::test]
    async fn sibling_manifest_projects_receive_distinct_handles() {
        let temp = tempfile::tempdir().unwrap();
        let a = temp.path().join("a");
        let b = temp.path().join("b");
        fs::create_dir_all(&a).unwrap();
        fs::create_dir_all(&b).unwrap();
        fs::write(a.join(".project.json"), r#"{"name":"a","version":"1.0.0"}"#).unwrap();
        fs::write(b.join(".project.json"), r#"{"name":"b","version":"1.0.0"}"#).unwrap();
        let a_source = a.join("A.sysml");
        let b_source = b.join("B.sysml");
        fs::write(&a_source, "package A;").unwrap();
        fs::write(&b_source, "package B;").unwrap();

        let registry = ProjectRegistry::new(
            Services::default(),
            None,
            StandardLibraryAvailability::Unavailable,
        );
        assert_eq!(
            sysml_query::source::discover_project_roots(&[temp.path().to_path_buf()]).len(),
            2
        );
        registry
            .configure(
                vec![Url::from_directory_path(temp.path()).unwrap()],
                Vec::new(),
                Vec::new(),
            )
            .await;
        assert_eq!(registry.handles().len(), 3);
        let a_handle = registry
            .handle_for_uri(&Url::from_file_path(&a_source).unwrap())
            .await
            .unwrap();
        let b_handle = registry
            .handle_for_uri(&Url::from_file_path(&b_source).unwrap())
            .await
            .unwrap();
        assert!(!Arc::ptr_eq(&a_handle.snapshot(), &b_handle.snapshot()));
        let a_uri = Url::from_file_path(a_source).unwrap();
        a_handle
            .store_document_text_fast(a_uri.clone(), "package A;".to_owned())
            .await
            .unwrap();
        assert!(a_handle.snapshot().index.contains_key(&a_uri));
        assert!(!b_handle.snapshot().index.contains_key(&a_uri));
    }

    #[tokio::test]
    async fn rediscovery_migrates_open_unsaved_document_to_replacement_session() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        let source = root.join("Model.sysml");
        fs::write(&source, "package Saved;").unwrap();
        let uri = Url::from_file_path(&source).unwrap();
        let registry = ProjectRegistry::new(
            Services::default(),
            None,
            StandardLibraryAvailability::Unavailable,
        );
        registry
            .configure(
                vec![Url::from_directory_path(root).unwrap()],
                Vec::new(),
                Vec::new(),
            )
            .await;
        let original = registry.handle_for_uri(&uri).await.unwrap();
        original
            .store_document_text_fast(uri.clone(), "package Unsaved;".to_owned())
            .await
            .unwrap();
        original.set_document_open(uri.clone(), true).await.unwrap();

        fs::write(
            root.join(".project.json"),
            r#"{"name":"model","version":"1.0.0"}"#,
        )
        .unwrap();
        registry.rediscover().await;

        let replacement = registry.handle_for_uri(&uri).await.unwrap();
        assert!(!Arc::ptr_eq(&original.snapshot(), &replacement.snapshot()));
        let (disk_entries, _) = crate::session::scan_sysml_files(
            vec![Url::from_directory_path(root).unwrap()],
            &registry.services.source,
        );
        let parsed_disk =
            crate::session::parse_scanned_documents(disk_entries, false, &registry.services);
        replacement.ingest_startup_scan(parsed_disk).await.unwrap();
        let snapshot = replacement.snapshot();
        assert!(snapshot.open_in_editor.contains(&uri));
        assert_eq!(
            snapshot.index.get(&uri).unwrap().content(),
            "package Unsaved;"
        );
    }

    /// A watcher event whose drive letter differs in case from the workspace folder stays in that
    /// project. VS Code opens `file:///c:/…` while an agent write is often reported as
    /// `file:///C:/…`. Windows path prefixes compare the drive letter case-insensitively, so the
    /// two URIs must not become two projects.
    #[cfg(windows)]
    #[tokio::test]
    async fn opposite_drive_letter_stays_in_the_workspace_project() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        let models = root.join("models");
        fs::create_dir_all(&models).unwrap();
        fs::write(
            root.join(".project.json"),
            r#"{"name":"model","version":"1.0.0"}"#,
        )
        .unwrap();
        let source = models.join("Model.sysml");
        fs::write(&source, "package Model;").unwrap();

        let workspace = with_drive_letter_case(&Url::from_directory_path(root).unwrap(), false);
        let lowercase_document =
            with_drive_letter_case(&Url::from_file_path(&source).unwrap(), false);
        let uppercase_document =
            with_drive_letter_case(&Url::from_file_path(&source).unwrap(), true);
        assert!(
            workspace.as_str().contains("file:///c:"),
            "workspace URI kept an uppercase drive letter: {workspace}"
        );
        assert!(
            uppercase_document.as_str().contains("file:///C:"),
            "watcher URI lost its uppercase drive letter: {uppercase_document}"
        );

        let registry = ProjectRegistry::new(
            Services::default(),
            None,
            StandardLibraryAvailability::Unavailable,
        );
        registry
            .configure(vec![workspace], Vec::new(), Vec::new())
            .await;
        assert_eq!(registry.handles().len(), 1);

        let from_editor = registry.handle_for_uri(&lowercase_document).await.unwrap();
        let from_watcher = registry.handle_for_uri(&uppercase_document).await.unwrap();
        assert!(
            from_editor.same_session(&from_watcher),
            "uppercase watcher URI was admitted as a second project"
        );
        assert_eq!(registry.handles().len(), 1);
    }

    /// A watcher event whose directory casing differs from the workspace folder stays in that
    /// project. An agent write is often reported with a different spelling of a path component
    /// (`Workspace` versus `workspace`). The file must not be admitted again as a loose project.
    #[cfg(windows)]
    #[tokio::test]
    async fn different_directory_case_stays_in_the_workspace_project() {
        let temp = tempfile::tempdir().unwrap();
        let folder = temp.path().join("Workspace");
        let models = folder.join("models");
        fs::create_dir_all(&models).unwrap();
        fs::write(
            folder.join(".project.json"),
            r#"{"name":"model","version":"1.0.0"}"#,
        )
        .unwrap();
        let source = models.join("Model.sysml");
        fs::write(&source, "package Model;").unwrap();

        let workspace = Url::from_directory_path(&folder).unwrap();
        let document = Url::from_file_path(&source).unwrap();
        let flipped = Url::parse(&document.as_str().replacen("/Workspace/", "/workspace/", 1))
            .expect("flipped directory case");
        assert_ne!(document.as_str(), flipped.as_str());

        let registry = ProjectRegistry::new(
            Services::default(),
            None,
            StandardLibraryAvailability::Unavailable,
        );
        registry
            .configure(vec![workspace], Vec::new(), Vec::new())
            .await;
        assert_eq!(registry.handles().len(), 1);

        let from_scan = registry.handle_for_uri(&document).await.unwrap();
        let from_watcher = registry.handle_for_uri(&flipped).await.unwrap();
        assert!(
            from_scan.same_session(&from_watcher),
            "differently cased watcher URI was admitted as a second project"
        );
        assert_eq!(registry.handles().len(), 1);
    }

    /// A manifest-admitted library root must classify its own files as library files. Before the
    /// fix, `library_root_url` was a bare `Url::from_directory_path` whose drive-letter casing
    /// (`file:///C:/…`) disagreed with the lowercased casing of scanned document URIs
    /// (`file:///c:/…` from `normalize_file_uri`), so `uri_under_any` never matched and every
    /// stdlib file under an admitted project was diagnosed as a workspace document.
    #[test]
    fn library_root_url_matches_normalized_document_uris_under_it() {
        let temp = tempfile::tempdir().unwrap();
        let lib_dir = temp.path().join("Kernel_Semantic_Library-1.0.0");
        fs::create_dir(&lib_dir).unwrap();
        let lib_file = lib_dir.join("Links.kerml");
        fs::write(&lib_file, "package Links;").unwrap();

        let root_url = library_root_url(&lib_dir).expect("library root url");
        let file_url = crate::common::util::normalize_file_uri(
            &Url::from_file_path(&lib_file).expect("file url"),
        );

        assert!(
            sysml_query::source::uri_under_any(&file_url, std::slice::from_ref(&root_url)),
            "normalized document URI {file_url} must be recognised under library root {root_url}",
        );
    }

    /// Rebuild a file URL with an explicit Windows drive-letter case.
    ///
    /// `Url::from_file_path` asks the path parser for the drive letter, and that parser reports
    /// it in uppercase, so rewriting the `Path` and converting again cannot produce `file:///c:/`.
    /// The letter has to be edited in the URL text.
    #[cfg(windows)]
    fn with_drive_letter_case(url: &Url, uppercase: bool) -> Url {
        let text = url.as_str().replacen("file:///C:", "file:///c:", 1);
        let text = if uppercase {
            text.replacen("file:///c:", "file:///C:", 1)
        } else {
            text
        };
        Url::parse(&text).expect("rewritten file URL")
    }
}
