use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

use sysml_query::Services;
use sysml_query::StandardLibraryAvailability;
use tower_lsp::lsp_types::Url;

use crate::session::state::ServerState;
use crate::session::WorkspaceHandle;

use super::lifecycle::project_boundary_for_uri;

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
                    .filter_map(|root| Url::from_directory_path(root).ok())
                    .collect(),
                admission
                    .standard_library_roots
                    .iter()
                    .filter_map(|root| Url::from_directory_path(root).ok())
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

fn boundary_root_for_uri(uri: &Url, workspace_roots: &[Url]) -> Option<PathBuf> {
    if workspace_roots.is_empty() {
        let path = uri.to_file_path().ok()?;
        // LSP permits `rootUri: null`. Such a loose document remains a useful manifestless
        // project, bounded at its containing directory rather than being rejected outright.
        return Some(canonicalize_or_self(path.parent()?));
    }
    if let Some(boundary) = project_boundary_for_uri(uri, workspace_roots) {
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
}
