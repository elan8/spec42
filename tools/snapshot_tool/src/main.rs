//! Standalone source-to-snapshot harness for Spec42.
//!
//! Snapshot Markdown files are the test cases. The runner reads each file's SOURCE section,
//! builds the immutable semantic model, renders each owned derived section, and either reports
//! stale files (`check`) or rewrites them (`update`). It is intentionally a binary rather than a
//! Rust test: review happens through the normal `git diff` of the Markdown files.
//!
//! A fixture may admit the standard library by declaring `libraries=standard` in its META block.
//! The library sources are then admitted as `StandardLibrary`-role documents, so the fixture's
//! references resolve against them while the owned projections keep reporting only the fixture's
//! own authored documents.

use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};

use clap::{Parser, Subcommand};
use generator_api::{ArtifactLimits, DiagramSemanticReference, GeneratorModelView, QueryLimits};
use generator_host::{CancellationHandle, GeneratorRuntime, RuntimeLimits};
use rayon::prelude::*;
use sysml_query::resolved_slice::{
    build as build_published_model, BuildRequest, ConstructionStrategy, EditorProbe, ElementKind,
    LibraryStratum, PublishedModel, QualifiedElementReference, QualifiedReferenceOutcome,
    QualifiedReferenceProbe, SourceDocument as QuerySourceDocument, SourceKind, TextPosition,
};
use sysml_source::{SysmlDocument, SysmlDocumentSourceKind};
use workspace::PublicationCoordinator;

#[derive(Debug, Parser)]
#[command(
    name = "spec42-snapshot",
    about = "Regenerate Spec42 Markdown source snapshots"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
    /// Root directory containing Markdown snapshots.
    #[arg(long, default_value = "tests/snapshots", global = true)]
    root: PathBuf,
    /// Restrict the operation to one path relative to --root (or an explicit path).
    #[arg(long, global = true)]
    fixture: Option<PathBuf>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Compute derived sections and fail if any snapshot would change.
    Check,
    /// Rewrite all owned derived sections in place. Review with `git diff`.
    Update,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SourceDocument {
    name: String,
    text: String,
}

/// Which libraries a fixture admits alongside its authored `SOURCE` documents.
///
/// A closed set with no default beyond `None`: an unrecognised `libraries` value is an error, so a
/// typo cannot silently produce a workspace-only publication that looks like a library-admitting
/// one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LibrarySelection {
    None,
    Standard,
}

/// Closed repository-owned generator selection. Fixtures never supply filesystem paths.
#[derive(Debug, Clone, PartialEq, Eq)]
enum GeneratorPlugin {
    Conformance(String),
    RepositoryDiagram,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DiagramSelection {
    kind: String,
    document: String,
    qualified_name: String,
}

/// Generator selection parsed from fixture metadata. Execution is deliberately kept separate from
/// Markdown parsing so the runner can provide the immutable publication to whichever WASM host it
/// uses without making the snapshot format depend on that host.
#[derive(Debug, Clone, PartialEq, Eq)]
struct GenerationRequest {
    plugin: GeneratorPlugin,
    diagram_selection: Option<DiagramSelection>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FixtureMeta {
    libraries: LibrarySelection,
    repository_sources: Vec<String>,
    generation: Option<GenerationRequest>,
}

/// Complete in-memory output of a generator invocation. A sorted map makes artifact order part of
/// the snapshot contract rather than an accident of plugin emission order.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct GeneratedArtifacts {
    files: BTreeMap<String, String>,
}

impl GeneratedArtifacts {
    fn insert_utf8(&mut self, path: impl Into<String>, contents: String) -> Result<(), String> {
        let path = path.into();
        validate_artifact_path(&path)?;
        if self.files.insert(path.clone(), contents).is_some() {
            return Err(format!(
                "generator emitted duplicate artifact path {path:?}"
            ));
        }
        Ok(())
    }
}

/// The directory of the checked-in standard-library corpus, relative to the snapshot root.
///
/// The runner deliberately admits only checked-in source fixtures rather than reaching into host
/// library packaging. Library fixtures carry the pinned library text in their own `SOURCE`
/// sections, so they are the admission input as well as fixtures in their own right.
const STANDARD_LIBRARY_DIRECTORY: &str = "sysml.library";

/// Lazily loaded library sources, shared by every fixture that admits them.
struct LibraryCorpus {
    root: PathBuf,
    standard: OnceLock<Result<Vec<QuerySourceDocument>, String>>,
    standard_stratum: OnceLock<Result<LibraryStratum, String>>,
    standard_documents: OnceLock<Result<Vec<SysmlDocument>, String>>,
}

impl LibraryCorpus {
    fn new(root: PathBuf) -> Self {
        Self {
            root,
            standard: OnceLock::new(),
            standard_stratum: OnceLock::new(),
            standard_documents: OnceLock::new(),
        }
    }

    fn documents(&self, selection: LibrarySelection) -> Result<&[SysmlDocument], String> {
        match selection {
            LibrarySelection::None => Ok(&[]),
            LibrarySelection::Standard => self
                .standard_documents
                .get_or_init(|| load_standard_library_documents(&self.root))
                .as_deref()
                .map_err(Clone::clone),
        }
    }

    fn sources(&self, selection: LibrarySelection) -> Result<&[QuerySourceDocument], String> {
        match selection {
            LibrarySelection::None => Ok(&[]),
            LibrarySelection::Standard => self
                .standard
                .get_or_init(|| {
                    self.documents(LibrarySelection::Standard)?
                        .iter()
                        .map(|document| {
                            QuerySourceDocument::from_uri(
                                document.uri.as_str(),
                                document.content.clone(),
                                SourceKind::StandardLibrary,
                            )
                            .map_err(|error| format!("invalid library source: {error}"))
                        })
                        .collect()
                })
                .as_deref()
                .map_err(|error| error.clone()),
        }
    }

    fn stratum(&self, selection: LibrarySelection) -> Result<Option<&LibraryStratum>, String> {
        match selection {
            LibrarySelection::None => Ok(None),
            LibrarySelection::Standard => self
                .standard_stratum
                .get_or_init(|| {
                    LibraryStratum::build(self.sources(LibrarySelection::Standard)?.to_vec())
                        .map_err(|error| format!("standard-library stratum: {error}"))
                })
                .as_ref()
                .map(Some)
                .map_err(Clone::clone),
        }
    }
}

fn load_standard_library_documents(root: &Path) -> Result<Vec<SysmlDocument>, String> {
    let directory = root.join(STANDARD_LIBRARY_DIRECTORY);
    let mut paths = Vec::new();
    visit_markdown(&directory, &mut paths)?;
    paths.sort();
    if paths.is_empty() {
        return Err(format!(
            "no standard-library fixtures found under {}",
            directory.display()
        ));
    }
    let mut documents = Vec::new();
    for path in paths {
        let fallback_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("library.md");
        let text = fs::read_to_string(&path)
            .map_err(|error| format!("{}: read failed: {error}", path.display()))?;
        for document in parse_source_documents(&text, fallback_name)? {
            let name = format!("{STANDARD_LIBRARY_DIRECTORY}/{}", document.name);
            documents.push(SysmlDocument::from_memory_path(
                "snapshot",
                &name,
                document.text,
                SysmlDocumentSourceKind::StandardLibrary,
                None,
                None,
            )?);
        }
    }
    Ok(documents)
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();
    let paths = snapshot_paths(&cli.root, cli.fixture.as_deref())?;
    if paths.is_empty() {
        return Err(format!(
            "no Markdown snapshots found under {}",
            cli.root.display()
        ));
    }
    let libraries = LibraryCorpus::new(cli.root.clone());

    // Rayon uses its bounded global worker pool; fixture work is isolated and writes happen only
    // after every worker has completed, in deterministic path order.
    let mut results: Vec<_> = paths
        .par_iter()
        .map(|path| FixtureWorkResult {
            path: path.clone(),
            result: evaluate_fixture(path, &libraries),
        })
        .collect();
    sort_work_results(&mut results);

    let mut failures = Vec::new();
    let mut stale = Vec::new();
    let mut writes = Vec::new();
    for result in results {
        match result.result {
            Ok(FixtureOutcome::Clean) => {}
            Ok(FixtureOutcome::StaleText(updated)) => match cli.command {
                Command::Check => stale.push(result.path),
                Command::Update => writes.push((result.path, updated.into_bytes())),
            },
            Err(error) => failures.push((result.path, error)),
        }
    }

    if !failures.is_empty() {
        eprintln!("snapshot processing errors:");
        for (path, error) in failures {
            eprintln!("  {}: {error}", path.display());
        }
        return Err("snapshot processing failed".to_string());
    }

    for (path, bytes) in writes {
        fs::write(&path, bytes)
            .map_err(|error| format!("{}: write failed: {error}", path.display()))?;
    }

    if stale.is_empty() {
        return Ok(());
    }
    eprintln!("stale snapshots (run `cargo run -p spec42-snapshot -- update`):");
    for path in stale {
        eprintln!("  {}", path.display());
    }
    Err("snapshot check failed".to_string())
}

enum FixtureOutcome {
    Clean,
    StaleText(String),
}

struct FixtureWorkResult {
    path: PathBuf,
    result: Result<FixtureOutcome, String>,
}

fn sort_work_results(results: &mut [FixtureWorkResult]) {
    results.sort_by(|left, right| left.path.cmp(&right.path));
}

fn evaluate_fixture(path: &Path, libraries: &LibraryCorpus) -> Result<FixtureOutcome, String> {
    let bytes = fs::read(path).map_err(|error| format!("read failed: {error}"))?;
    let original =
        String::from_utf8(bytes).map_err(|error| format!("snapshot is not UTF-8: {error}"))?;
    let updated = regenerate_snapshot(&original, path, libraries)?;
    Ok(if updated == original {
        FixtureOutcome::Clean
    } else {
        FixtureOutcome::StaleText(updated)
    })
}

fn snapshot_paths(root: &Path, fixture: Option<&Path>) -> Result<Vec<PathBuf>, String> {
    let root = if let Some(fixture) = fixture {
        if fixture.is_absolute() {
            fixture.to_path_buf()
        } else {
            let under_root = root.join(fixture);
            if under_root.exists() {
                under_root
            } else {
                fixture.to_path_buf()
            }
        }
    } else {
        root.to_path_buf()
    };
    if !root.exists() {
        return Err(format!("snapshot path does not exist: {}", root.display()));
    }
    if root.is_file() {
        return (root.extension().is_some_and(|extension| extension == "md"))
            .then_some(vec![root.clone()])
            .ok_or_else(|| format!("snapshot is not Markdown: {}", root.display()));
    }
    let mut paths = Vec::new();
    visit_markdown(&root, &mut paths)?;
    paths.sort();
    Ok(paths)
}

fn visit_markdown(directory: &Path, paths: &mut Vec<PathBuf>) -> Result<(), String> {
    for entry in fs::read_dir(directory)
        .map_err(|error| format!("{}: read directory failed: {error}", directory.display()))?
    {
        let path = entry
            .map_err(|error| format!("{}: directory entry failed: {error}", directory.display()))?
            .path();
        if path.is_dir() {
            visit_markdown(&path, paths)?;
        } else if path.extension().is_some_and(|extension| extension == "md") {
            paths.push(path);
        }
    }
    Ok(())
}

fn regenerate_snapshot(
    fixture: &str,
    path: &Path,
    libraries: &LibraryCorpus,
) -> Result<String, String> {
    let fallback_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("snapshot.md");
    let meta = parse_fixture_meta(fixture, fallback_name)?;
    let mut documents = if meta.repository_sources.is_empty()
        || raw_section(fixture, "SOURCE")
            .and_then(fenced_block)
            .is_some()
    {
        parse_source_documents(fixture, fallback_name)?
    } else {
        Vec::new()
    };
    documents.extend(load_repository_sources(&meta.repository_sources, path)?);
    let workspace_source_documents = documents
        .iter()
        .map(|document| {
            QuerySourceDocument::from_memory_path(
                "snapshot",
                &document.name,
                document.text.clone(),
                SourceKind::Workspace,
            )
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("{}: invalid source: {error}", path.display()))?;
    let mut source_documents = workspace_source_documents.clone();
    source_documents.extend_from_slice(libraries.sources(meta.libraries)?);
    let mut admitted_documents = documents
        .iter()
        .map(|document| {
            SysmlDocument::from_memory_path(
                "snapshot",
                &document.name,
                document.text.clone(),
                SysmlDocumentSourceKind::Workspace,
                None,
                None,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    admitted_documents.extend_from_slice(libraries.documents(meta.libraries)?);
    let probes = parse_editor_probes(fixture, &documents, fallback_name)?;
    let qualified_reference_probes =
        parse_qualified_reference_probes(fixture, &documents, fallback_name)?;
    let canonical_model = PublicationCoordinator::new()
        .publish(&admitted_documents, std::iter::empty::<Box<str>>())
        .map_err(|error| {
            format!(
                "{}: canonical semantic build failed: {error}",
                path.display()
            )
        })?;
    // These direct builds are owner-internal equivalence lanes. Snapshot artifacts consume only
    // `canonical_model`, exactly as production hosts do.
    let sequential_model = Arc::new(build_model(
        &source_documents,
        ConstructionStrategy::Sequential,
        path,
    )?);
    let parallel_model = Arc::new(build_model(
        &source_documents,
        ConstructionStrategy::Parallel,
        path,
    )?);
    let sequential = render_owned_sections(
        &sequential_model,
        &documents,
        &source_documents,
        &probes,
        &qualified_reference_probes,
    )?;
    let parallel = render_owned_sections(
        &parallel_model,
        &documents,
        &source_documents,
        &probes,
        &qualified_reference_probes,
    )?;
    ensure_strategy_parity(path, &sequential, &parallel)?;
    let canonical = render_owned_sections(
        &canonical_model,
        &documents,
        &source_documents,
        &probes,
        &qualified_reference_probes,
    )?;
    ensure_strategy_parity(path, &canonical, &sequential).map_err(|error| {
        format!("{error}; canonical publication and direct equivalence lane differ")
    })?;
    let warm_models = if let Some(stratum) = libraries.stratum(meta.libraries)? {
        let warm_sequential = Arc::new(build_model_with_library(
            &workspace_source_documents,
            ConstructionStrategy::Sequential,
            stratum,
            path,
        )?);
        let warm_parallel = Arc::new(build_model_with_library(
            &workspace_source_documents,
            ConstructionStrategy::Parallel,
            stratum,
            path,
        )?);
        let warm_sequential_sections = render_owned_sections(
            &warm_sequential,
            &documents,
            &source_documents,
            &probes,
            &qualified_reference_probes,
        )?;
        let warm_parallel_sections = render_owned_sections(
            &warm_parallel,
            &documents,
            &source_documents,
            &probes,
            &qualified_reference_probes,
        )?;
        ensure_strategy_parity(path, &warm_sequential_sections, &warm_parallel_sections)?;
        ensure_strategy_parity(path, &sequential, &warm_sequential_sections).map_err(|error| {
            format!("{error}; cold/full and warm/library-stratum publications differ")
        })?;
        Some((warm_sequential, warm_parallel))
    } else {
        None
    };
    ensure_sections_balanced(&canonical).map_err(|error| format!("{}: {error}", path.display()))?;

    let fixture = replace_or_insert_section(fixture, "SMG", &canonical.smg)
        .ok_or_else(|| format!("{}: missing SOURCE/SMG section", path.display()))?;
    let fixture = replace_or_insert_section(&fixture, "DIAGNOSTICS", &canonical.diagnostics)
        .ok_or_else(|| format!("{}: missing SOURCE section", path.display()))?;
    let fixture = replace_or_insert_section(&fixture, "TYPES", &canonical.types)
        .ok_or_else(|| format!("{}: missing SOURCE section", path.display()))?;
    let fixture = replace_or_insert_section(&fixture, "NAVIGATION", &canonical.navigation)
        .ok_or_else(|| format!("{}: missing SOURCE section", path.display()))?;
    let fixture = if probes.is_empty() {
        fixture
    } else {
        replace_or_insert_section(&fixture, "EDITOR RESULTS", &canonical.editor_queries)
            .ok_or_else(|| format!("{}: missing SOURCE section", path.display()))?
    };
    let fixture = if qualified_reference_probes.is_empty() {
        fixture
    } else {
        replace_or_insert_section(
            &fixture,
            "QUALIFIED REFERENCE RESULTS",
            &canonical.qualified_references,
        )
        .ok_or_else(|| format!("{}: missing SOURCE section", path.display()))?
    };
    let fixture = if let Some(generation) = &meta.generation {
        let canonical_generated =
            execute_generation(Arc::clone(&canonical_model), generation, path)?;
        let sequential_generated =
            execute_generation(Arc::clone(&sequential_model), generation, path)?;
        let parallel_generated = execute_generation(Arc::clone(&parallel_model), generation, path)?;
        if canonical_generated != sequential_generated || sequential_generated != parallel_generated
        {
            return Err(format!(
                "{}: sequential and parallel generation differ",
                path.display()
            ));
        }
        if let Some((warm_sequential, warm_parallel)) = &warm_models {
            let warm_sequential_generated =
                execute_generation(Arc::clone(warm_sequential), generation, path)?;
            let warm_parallel_generated =
                execute_generation(Arc::clone(warm_parallel), generation, path)?;
            if sequential_generated != warm_sequential_generated
                || sequential_generated != warm_parallel_generated
            {
                return Err(format!(
                    "{}: cold/full and warm/library-stratum generation differ",
                    path.display()
                ));
            }
        }
        replace_or_insert_generated_section(&fixture, &canonical_generated)
    } else {
        fixture
    };
    Ok(canonicalize_sections(&fixture))
}

fn load_repository_sources(
    paths: &[String],
    fixture_path: &Path,
) -> Result<Vec<SourceDocument>, String> {
    let repository_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut documents = Vec::with_capacity(paths.len());
    for relative in paths {
        let relative_path = Path::new(relative);
        if relative_path.is_absolute()
            || relative_path.components().any(|component| {
                matches!(
                    component,
                    std::path::Component::ParentDir
                        | std::path::Component::RootDir
                        | std::path::Component::Prefix(_)
                )
            })
            || !relative.starts_with("examples/")
            || relative_path
                .extension()
                .is_none_or(|extension| extension != "sysml")
        {
            return Err(format!(
                "{}: repositorySources entry must be a repository-relative examples/*.sysml path: {relative:?}",
                fixture_path.display()
            ));
        }
        let text = fs::read_to_string(repository_root.join(relative_path)).map_err(|error| {
            format!(
                "{}: could not read repository source {relative:?}: {error}",
                fixture_path.display()
            )
        })?;
        documents.push(SourceDocument {
            name: relative.clone(),
            text,
        });
    }
    Ok(documents)
}

fn execute_generation(
    publication: Arc<PublishedModel>,
    request: &GenerationRequest,
    fixture_path: &Path,
) -> Result<GeneratedArtifacts, String> {
    let plugin_path = generator_plugin_path(&request.plugin);
    let module = fs::read(&plugin_path).map_err(|error| {
        format!(
            "{}: failed to read generator plugin `{}` at {}: {error}; run scripts/build-generator-plugins.sh",
            fixture_path.display(),
            generator_plugin_label(&request.plugin),
            plugin_path.display()
        )
    })?;
    let model_digest = publication.publication().model_digest();
    let model = Arc::new(GeneratorModelView::new(
        Arc::clone(&publication),
        model_digest,
        env!("CARGO_PKG_VERSION"),
        QueryLimits::default(),
    ));
    let args = generation_arguments(request, &publication, &model, fixture_path)?;
    let runtime = GeneratorRuntime::new().map_err(|error| {
        format!(
            "{}: generator runtime failed: {error}",
            fixture_path.display()
        )
    })?;
    let execution = runtime
        .execute(
            &module,
            model,
            &args,
            RuntimeLimits::default(),
            ArtifactLimits::default(),
            CancellationHandle::new(),
        )
        .map_err(|error| format!("{}: generation failed: {error}", fixture_path.display()))?;
    if !execution.diagnostics.is_empty() {
        return Err(format!(
            "{}: snapshot generator emitted diagnostics: {:?}",
            fixture_path.display(),
            execution.diagnostics
        ));
    }
    let mut artifacts = GeneratedArtifacts::default();
    for (path, bytes) in execution.artifacts.entries() {
        let contents = String::from_utf8(bytes.to_vec()).map_err(|_| {
            format!(
                "{}: generated artifact `{path}` is not UTF-8",
                fixture_path.display()
            )
        })?;
        artifacts.insert_utf8(path.to_string(), contents)?;
    }
    Ok(artifacts)
}

fn generator_plugin_label(plugin: &GeneratorPlugin) -> String {
    match plugin {
        GeneratorPlugin::Conformance(name) => format!("conformance:{name}"),
        GeneratorPlugin::RepositoryDiagram => "repository:diagram".to_string(),
    }
}

fn generator_plugin_path(plugin: &GeneratorPlugin) -> PathBuf {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    match plugin {
        GeneratorPlugin::Conformance(name) => root
            .join("generator-tests/plugins/target/wasm32-unknown-unknown/release")
            .join(format!("spec42_conformance_{name}.wasm")),
        GeneratorPlugin::RepositoryDiagram => root
            .join("generator-plugins/target/wasm32-unknown-unknown/release")
            .join("spec42_diagram_generator.wasm"),
    }
}

fn generation_arguments(
    request: &GenerationRequest,
    publication: &PublishedModel,
    model: &GeneratorModelView,
    fixture_path: &Path,
) -> Result<Vec<String>, String> {
    let Some(selection) = &request.diagram_selection else {
        return Ok(Vec::new());
    };
    let expected_kind = diagram_element_kind(&selection.kind).ok_or_else(|| {
        format!(
            "{}: unknown diagram view kind {:?}",
            fixture_path.display(),
            selection.kind
        )
    })?;
    let document = QuerySourceDocument::from_memory_path(
        "snapshot",
        &selection.document,
        String::new(),
        SourceKind::Workspace,
    )
    .map_err(|error| {
        format!(
            "{}: invalid diagram selection document {:?}: {error}",
            fixture_path.display(),
            selection.document
        )
    })?;
    let reference = QualifiedElementReference {
        document: Some(document.identity().into()),
        qualified_name: selection.qualified_name.clone().into(),
        expected_kind: Some(expected_kind),
    };
    let target = match publication
        .inspection()
        .resolve_qualified_reference(&reference)
    {
        QualifiedReferenceOutcome::Resolved(target)
        | QualifiedReferenceOutcome::Recovered(target)
        | QualifiedReferenceOutcome::UnsupportedWith(target) => target,
        outcome => {
            return Err(format!(
                "{}: diagram view reference {:?} in {:?} did not resolve: {outcome:?}",
                fixture_path.display(),
                selection.qualified_name,
                selection.document
            ))
        }
    };
    let catalog = model.diagram_views().map_err(|error| {
        format!(
            "{}: diagram view catalog failed: {error}",
            fixture_path.display()
        )
    })?;
    let matches = catalog
        .iter()
        .filter(|view| {
            matches!(
                &view.reference,
                DiagramSemanticReference::Qualified { document, qualified_name, .. }
                    if document == target.location.document.as_ref()
                        && qualified_name == target.qualified_name.as_ref()
            ) && diagram_kind_id(view.kind) == selection.kind
        })
        .collect::<Vec<_>>();
    match matches.as_slice() {
        [view] => Ok(vec![view.handle.clone()]),
        [] => Err(format!(
            "{}: selected diagram view kind {:?} with qualified reference {:?} in {:?} is not in the active publication; authored catalog entries: {}",
            fixture_path.display(),
            selection.kind,
            target.qualified_name,
            target.location.document,
            catalog
                .iter()
                .map(|view| format!("{}={:?}", diagram_kind_id(view.kind), view.reference))
                .collect::<Vec<_>>()
                .join(", ")
        )),
        _ => Err(format!(
            "{}: selected diagram identity is not unique in the active publication",
            fixture_path.display()
        )),
    }
}

fn diagram_element_kind(kind: &str) -> Option<ElementKind> {
    match kind {
        "general-view"
        | "interconnection-view"
        | "action-flow-view"
        | "state-transition-view"
        | "sequence-view"
        | "browser-view"
        | "grid-view"
        | "geometry-view" => Some(ElementKind::ViewUsage),
        _ => None,
    }
}

fn diagram_kind_id(kind: generator_api::DiagramViewKind) -> &'static str {
    match kind {
        generator_api::DiagramViewKind::GeneralView => "general-view",
        generator_api::DiagramViewKind::InterconnectionView => "interconnection-view",
        generator_api::DiagramViewKind::ActionFlowView => "action-flow-view",
        generator_api::DiagramViewKind::StateTransitionView => "state-transition-view",
        generator_api::DiagramViewKind::SequenceView => "sequence-view",
        generator_api::DiagramViewKind::BrowserView => "browser-view",
        generator_api::DiagramViewKind::GridView => "grid-view",
        generator_api::DiagramViewKind::GeometryView => "geometry-view",
    }
}

struct OwnedSections {
    smg: String,
    types: String,
    diagnostics: String,
    navigation: String,
    editor_queries: String,
    qualified_references: String,
}

fn build_model(
    source_documents: &[QuerySourceDocument],
    construction: ConstructionStrategy,
    path: &Path,
) -> Result<PublishedModel, String> {
    let request = BuildRequest::resolved(source_documents.to_vec(), construction)
        .map_err(|error| format!("{}: invalid semantic input: {error}", path.display()))?;
    build_published_model(request)
        .map_err(|error| format!("{}: semantic build failed: {error}", path.display()))
}

fn build_model_with_library(
    workspace_documents: &[QuerySourceDocument],
    construction: ConstructionStrategy,
    library: &LibraryStratum,
    path: &Path,
) -> Result<PublishedModel, String> {
    let request =
        BuildRequest::resolved_with_library(workspace_documents.to_vec(), construction, library)
            .map_err(|error| format!("{}: invalid warm semantic input: {error}", path.display()))?;
    build_published_model(request)
        .map_err(|error| format!("{}: warm semantic build failed: {error}", path.display()))
}

fn render_owned_sections(
    model: &PublishedModel,
    documents: &[SourceDocument],
    source_documents: &[QuerySourceDocument],
    probes: &[EditorProbe],
    qualified_reference_probes: &[QualifiedReferenceProbe],
) -> Result<OwnedSections, String> {
    // Both strings are complete owner-defined projections. The SMG includes publication phase,
    // completeness, evaluation state, and all owned facts; diagnostics includes canonical order.
    let smg = render_semantic_model(model)?;
    let diagnostics = render_diagnostics(model, documents, source_documents)?;
    let mut types = String::new();
    model
        .debug()
        .write_types_sexpr(&mut types)
        .map_err(|error| format!("type rendering failed: {error}"))?;
    let mut navigation = String::new();
    model
        .debug()
        .write_navigation_sexpr(&mut navigation)
        .map_err(|error| format!("navigation rendering failed: {error}"))?;
    let mut editor_queries = String::new();
    model
        .debug()
        .write_editor_queries_sexpr(probes, &mut editor_queries)
        .map_err(|error| format!("editor-query rendering failed: {error}"))?;
    let mut qualified_references = String::new();
    model
        .debug()
        .write_qualified_reference_queries_sexpr(
            qualified_reference_probes,
            &mut qualified_references,
        )
        .map_err(|error| format!("qualified-reference rendering failed: {error}"))?;
    Ok(OwnedSections {
        smg,
        types,
        diagnostics,
        navigation,
        editor_queries,
        qualified_references,
    })
}

/// Rejects an owned section whose S-expression does not close.
///
/// These sections are a contract, not decoration: a reader that parses them has to be able to.
/// Three separate producers had drifted out of balance without any check noticing, because a
/// snapshot only ever had to match its own previous text. Parentheses inside quoted strings are
/// content, not structure, so the scan tracks quoting.
fn ensure_balanced(name: &str, text: &str) -> Result<(), String> {
    let mut depth = 0i64;
    let mut quoted = false;
    let mut escaped = false;
    for character in text.chars() {
        if quoted {
            match character {
                _ if escaped => escaped = false,
                '\\' => escaped = true,
                '"' => quoted = false,
                _ => {}
            }
            continue;
        }
        match character {
            '"' => quoted = true,
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth < 0 {
                    return Err(format!("{name} section closes more elements than it opens"));
                }
            }
            _ => {}
        }
    }
    if depth != 0 {
        return Err(format!("{name} section leaves {depth} element(s) open"));
    }
    Ok(())
}

fn ensure_sections_balanced(sections: &OwnedSections) -> Result<(), String> {
    ensure_balanced("SMG", &sections.smg)?;
    ensure_balanced("TYPES", &sections.types)?;
    ensure_balanced("DIAGNOSTICS", &sections.diagnostics)?;
    ensure_balanced("NAVIGATION", &sections.navigation)?;
    ensure_balanced("EDITOR RESULTS", &sections.editor_queries).and_then(|()| {
        ensure_balanced(
            "QUALIFIED REFERENCE RESULTS",
            &sections.qualified_references,
        )
    })
}

fn ensure_strategy_parity(
    path: &Path,
    sequential: &OwnedSections,
    parallel: &OwnedSections,
) -> Result<(), String> {
    if sequential.smg != parallel.smg {
        return Err(format!(
            "{}: sequential and parallel semantic-model outputs differ",
            path.display()
        ));
    }
    if sequential.diagnostics != parallel.diagnostics {
        return Err(format!(
            "{}: sequential and parallel diagnostics outputs differ",
            path.display()
        ));
    }
    if sequential.types != parallel.types {
        return Err(format!(
            "{}: sequential and parallel type outputs differ",
            path.display()
        ));
    }
    if sequential.navigation != parallel.navigation {
        return Err(format!(
            "{}: sequential and parallel navigation outputs differ",
            path.display()
        ));
    }
    if sequential.editor_queries != parallel.editor_queries {
        return Err(format!(
            "{}: sequential and parallel editor-query outputs differ",
            path.display()
        ));
    }
    if sequential.qualified_references != parallel.qualified_references {
        return Err(format!(
            "{}: sequential and parallel qualified-reference outputs differ",
            path.display()
        ));
    }
    Ok(())
}

fn render_semantic_model(model: &PublishedModel) -> Result<String, String> {
    let mut output = String::new();
    model
        .debug()
        .write_semantic_sexpr(&mut output)
        .map_err(|error| format!("semantic-model rendering failed: {error}"))?;
    Ok(output)
}

fn render_diagnostics(
    model: &PublishedModel,
    _documents: &[SourceDocument],
    _source_documents: &[QuerySourceDocument],
) -> Result<String, String> {
    let mut rendered = String::new();
    model
        .debug()
        .write_diagnostics_sexpr(&mut rendered)
        .map_err(|error| format!("diagnostic rendering failed: {error}"))?;
    Ok(rendered)
}

fn parse_source_documents(
    fixture: &str,
    fallback_name: &str,
) -> Result<Vec<SourceDocument>, String> {
    let source = raw_section(fixture, "SOURCE")
        .ok_or_else(|| format!("{fallback_name}: missing # SOURCE section"))?;
    let mut named = Vec::new();
    let mut cursor = source;
    while let Some(index) = cursor.find("## ") {
        cursor = &cursor[index + 3..];
        let Some((name, rest)) = cursor.split_once('\n') else {
            return Err(format!("{fallback_name}: malformed named SOURCE document"));
        };
        let Some((text, after)) = fenced_block(rest) else {
            return Err(format!(
                "{fallback_name}: malformed SOURCE fence for {name}"
            ));
        };
        named.push(SourceDocument {
            name: name.trim().to_string(),
            text,
        });
        cursor = after;
    }
    if !named.is_empty() {
        return Ok(named);
    }
    fenced_block(source)
        .map(|(text, _)| {
            vec![SourceDocument {
                name: fallback_name.to_string(),
                text,
            }]
        })
        .ok_or_else(|| format!("{fallback_name}: malformed SOURCE fence"))
}

/// Reads execution-affecting META keys. Descriptive keys remain open-ended, but malformed lines,
/// duplicate execution keys, and incomplete generator declarations are rejected.
fn parse_fixture_meta(fixture: &str, fallback_name: &str) -> Result<FixtureMeta, String> {
    let Some(section) = raw_section(fixture, "META") else {
        return Ok(FixtureMeta {
            libraries: LibrarySelection::None,
            repository_sources: Vec::new(),
            generation: None,
        });
    };
    let Some((text, _)) = fenced_block(section) else {
        return Err(format!("{fallback_name}: malformed META fence"));
    };
    let mut selection = LibrarySelection::None;
    let mut fixture_type = None;
    let mut repository_sources = Vec::new();
    let mut plugin = None;
    let mut view_kind = None;
    let mut view_document = None;
    let mut view_qualified_name = None;
    let mut seen = HashSet::new();
    for (line_index, line) in text.lines().enumerate() {
        if line.trim().is_empty() || line.trim_start().starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            return Err(format!(
                "{fallback_name}: META line {} must be key=value",
                line_index + 1
            ));
        };
        let key = key.trim();
        let value = value.trim();
        if matches!(
            key,
            "libraries"
                | "repositorySources"
                | "type"
                | "plugin"
                | "viewKind"
                | "viewDocument"
                | "viewQualifiedName"
        ) && !seen.insert(key)
        {
            return Err(format!("{fallback_name}: duplicate META key {key:?}"));
        }
        match key {
            "libraries" => selection = match value {
                "none" => LibrarySelection::None,
                "standard" => LibrarySelection::Standard,
                other => return Err(format!(
                    "{fallback_name}: unknown META libraries value {other:?} (expected \"none\" or \"standard\")"
                )),
            },
            "repositorySources" => {
                repository_sources = value
                    .split(',')
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                    .map(str::to_owned)
                    .collect();
                if repository_sources.is_empty() {
                    return Err(format!("{fallback_name}: META repositorySources must not be empty"));
                }
            }
            "type" => {
                if value.is_empty() {
                    return Err(format!("{fallback_name}: META type must not be empty"));
                }
                fixture_type = Some(value.to_string());
            }
            "plugin" => {
                if value.is_empty() {
                    return Err(format!("{fallback_name}: META plugin must not be empty"));
                }
                plugin = Some(value.to_string());
            }
            "viewKind" => {
                if value.is_empty() {
                    return Err(format!("{fallback_name}: META viewKind must not be empty"));
                }
                view_kind = Some(value.to_string());
            }
            "viewDocument" => {
                if value.is_empty() {
                    return Err(format!("{fallback_name}: META viewDocument must not be empty"));
                }
                view_document = Some(value.to_string());
            }
            "viewQualifiedName" => {
                if value.is_empty() {
                    return Err(format!(
                        "{fallback_name}: META viewQualifiedName must not be empty"
                    ));
                }
                view_qualified_name = Some(value.to_string());
            }
            _ => {}
        }
    }
    let generation = match (fixture_type.as_deref(), plugin) {
        (Some("generate"), Some(plugin)) => {
            let plugin = parse_generator_plugin(&plugin, fallback_name)?;
            let diagram_selection = match (view_kind, view_document, view_qualified_name) {
                (Some(kind), Some(document), Some(qualified_name)) => Some(DiagramSelection {
                    kind,
                    document,
                    qualified_name,
                }),
                (None, None, None) => None,
                _ => {
                    return Err(format!(
                        "{fallback_name}: META viewKind, viewDocument and viewQualifiedName must be specified together"
                    ))
                }
            };
            if diagram_selection.is_some() && plugin != GeneratorPlugin::RepositoryDiagram {
                return Err(format!(
                    "{fallback_name}: typed view selection is only valid with plugin=repository:diagram"
                ));
            }
            Some(GenerationRequest {
                plugin,
                diagram_selection,
            })
        }
        (Some("generate"), None) => {
            return Err(format!(
                "{fallback_name}: META type=generate requires a plugin"
            ))
        }
        (_, Some(_)) => {
            return Err(format!(
                "{fallback_name}: META plugin is only valid with type=generate"
            ))
        }
        _ if view_kind.is_some() || view_document.is_some() || view_qualified_name.is_some() => {
            return Err(format!(
                "{fallback_name}: view selection is only valid with type=generate"
            ))
        }
        _ => None,
    };
    Ok(FixtureMeta {
        libraries: selection,
        repository_sources,
        generation,
    })
}

fn parse_generator_plugin(value: &str, fallback_name: &str) -> Result<GeneratorPlugin, String> {
    if value == "repository:diagram" {
        return Ok(GeneratorPlugin::RepositoryDiagram);
    }
    let name = value.strip_prefix("conformance:").unwrap_or(value);
    if name.is_empty()
        || !name
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
    {
        return Err(format!(
            "{fallback_name}: unknown or unsafe META plugin {value:?}"
        ));
    }
    Ok(GeneratorPlugin::Conformance(name.to_string()))
}

fn validate_artifact_path(path: &str) -> Result<(), String> {
    let candidate = Path::new(path);
    if path.is_empty()
        || candidate.is_absolute()
        || candidate.components().any(|part| {
            matches!(
                part,
                std::path::Component::ParentDir | std::path::Component::CurDir
            )
        })
    {
        return Err(format!("invalid generated artifact path {path:?}"));
    }
    Ok(())
}

fn artifact_fence_language(path: &str) -> &'static str {
    match Path::new(path)
        .extension()
        .and_then(|extension| extension.to_str())
    {
        Some("csv") => "csv",
        Some("json") => "json",
        _ => "text",
    }
}

fn render_generated_artifacts(artifacts: &GeneratedArtifacts) -> String {
    let mut output = String::new();
    for (path, contents) in &artifacts.files {
        output.push_str("## ");
        output.push_str(path);
        output.push('\n');
        output.push_str("~~~");
        output.push_str(artifact_fence_language(path));
        output.push('\n');
        output.push_str(contents);
        output.push_str("\n~~~\n");
    }
    output
}

#[cfg(test)]
fn parse_generated_artifacts(
    fixture: &str,
    fallback_name: &str,
) -> Result<Option<GeneratedArtifacts>, String> {
    let Some(section) = raw_section(fixture, "GENERATED") else {
        return Ok(None);
    };
    let mut artifacts = GeneratedArtifacts::default();
    let mut cursor = section;
    while let Some(index) = cursor.find("## ") {
        cursor = &cursor[index + 3..];
        let Some((path, rest)) = cursor.split_once('\n') else {
            return Err(format!(
                "{fallback_name}: malformed GENERATED artifact name"
            ));
        };
        let Some((contents, after)) = fenced_block(rest) else {
            return Err(format!(
                "{fallback_name}: malformed GENERATED fence for {path}"
            ));
        };
        artifacts.insert_utf8(path.trim(), contents)?;
        cursor = after;
    }
    if !section.trim().is_empty() && artifacts.files.is_empty() {
        return Err(format!(
            "{fallback_name}: GENERATED section must contain named artifacts"
        ));
    }
    Ok(Some(artifacts))
}

fn replace_or_insert_generated_section(fixture: &str, artifacts: &GeneratedArtifacts) -> String {
    let body = render_generated_artifacts(artifacts);
    if let Some(updated) = replace_raw_section(fixture, "GENERATED", &body) {
        return updated;
    }
    let mut updated = fixture.trim_end_matches('\n').to_string();
    updated.push_str("\n# GENERATED\n");
    updated.push_str(&body);
    updated
}

fn parse_editor_probes(
    fixture: &str,
    documents: &[SourceDocument],
    fallback_name: &str,
) -> Result<Vec<EditorProbe>, String> {
    let Some(section) = raw_section(fixture, "EDITOR QUERIES") else {
        return Ok(Vec::new());
    };
    let Some((text, _)) = fenced_block(section) else {
        return Err(format!("{fallback_name}: malformed EDITOR QUERIES fence"));
    };
    let mut probes = Vec::new();
    for (line_index, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut fields = line.split_whitespace();
        if fields.next() != Some("probe") {
            return Err(format!(
                "{fallback_name}: EDITOR QUERIES line {} must start with `probe`",
                line_index + 1
            ));
        }
        let document = fields
            .next()
            .ok_or_else(|| format!("{fallback_name}: missing probe document"))?;
        if !documents.iter().any(|candidate| candidate.name == document) {
            return Err(format!(
                "{fallback_name}: unknown probe document {document:?}"
            ));
        }
        let line = fields
            .next()
            .and_then(|value| value.parse().ok())
            .ok_or_else(|| format!("{fallback_name}: invalid probe line"))?;
        let character = fields
            .next()
            .and_then(|value| value.parse().ok())
            .ok_or_else(|| format!("{fallback_name}: invalid probe character"))?;
        let mut qualifier = None;
        let mut rename_to = None;
        for option in fields {
            if let Some(value) = option.strip_prefix("qualifier=") {
                qualifier = Some(value.to_string());
            } else if let Some(value) = option.strip_prefix("rename=") {
                rename_to = Some(value.to_string());
            } else {
                return Err(format!(
                    "{fallback_name}: unknown editor probe option {option:?}"
                ));
            }
        }
        probes.push(EditorProbe {
            document: format!("memory://snapshot/{document}"),
            position: TextPosition { line, character },
            qualifier,
            rename_to,
        });
    }
    Ok(probes)
}

fn parse_qualified_reference_probes(
    fixture: &str,
    documents: &[SourceDocument],
    fallback_name: &str,
) -> Result<Vec<QualifiedReferenceProbe>, String> {
    let Some(section) = raw_section(fixture, "QUALIFIED REFERENCE QUERIES") else {
        return Ok(Vec::new());
    };
    let Some((text, _)) = fenced_block(section) else {
        return Err(format!(
            "{fallback_name}: malformed QUALIFIED REFERENCE QUERIES fence"
        ));
    };
    let mut probes = Vec::new();
    for (line_index, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut fields = line.split_whitespace();
        if fields.next() != Some("resolve") {
            return Err(format!(
                "{fallback_name}: QUALIFIED REFERENCE QUERIES line {} must start with `resolve`",
                line_index + 1
            ));
        }
        let document_name = fields
            .next()
            .ok_or_else(|| format!("{fallback_name}: missing reference document"))?;
        let document = if document_name == "*" {
            None
        } else {
            if !documents
                .iter()
                .any(|candidate| candidate.name == document_name)
            {
                return Err(format!(
                    "{fallback_name}: unknown reference document {document_name:?}"
                ));
            }
            let source = QuerySourceDocument::from_memory_path(
                "snapshot",
                document_name,
                String::new(),
                SourceKind::Workspace,
            )
            .map_err(|error| format!("{fallback_name}: invalid reference document: {error}"))?;
            Some(source.identity().to_string())
        };
        let qualified_name = fields
            .next()
            .ok_or_else(|| format!("{fallback_name}: missing qualified name"))?
            .to_string();
        let expected_kind = match fields.next() {
            None | Some("*") => None,
            Some(kind) => Some(
                ElementKind::ALL
                    .iter()
                    .copied()
                    .find(|candidate| candidate.as_str() == kind)
                    .ok_or_else(|| {
                        format!("{fallback_name}: unknown expected element kind {kind:?}")
                    })?,
            ),
        };
        if fields.next().is_some() {
            return Err(format!(
                "{fallback_name}: too many qualified-reference fields on line {}",
                line_index + 1
            ));
        }
        probes.push(QualifiedReferenceProbe {
            document,
            qualified_name,
            expected_kind,
        });
    }
    Ok(probes)
}

fn raw_section<'a>(fixture: &'a str, name: &str) -> Option<&'a str> {
    let marker = format!("# {name}\n");
    let start = fixture.find(&marker)? + marker.len();
    let rest = &fixture[start..];
    let end = rest.find("\n# ").unwrap_or(rest.len());
    Some(&rest[..end])
}

fn replace_or_insert_section(fixture: &str, name: &str, replacement: &str) -> Option<String> {
    if let Some(updated) = replace_section(fixture, name, replacement) {
        return Some(updated);
    }
    let insertion = fixture.find("\n# ").unwrap_or(fixture.len());
    let section = format!("\n# {name}\n~~~sexpr\n{replacement}\n~~~");
    let mut updated = String::with_capacity(fixture.len() + section.len());
    updated.push_str(&fixture[..insertion]);
    updated.push_str(&section);
    updated.push_str(&fixture[insertion..]);
    Some(updated)
}

/// Canonical top-level Markdown order. SOURCE is authored; the other sections are owned by this
/// runner. Canonicalization drops sections outside this ownership contract.
const SECTION_ORDER: &[&str] = &[
    "META",
    "SOURCE",
    "EDITOR QUERIES",
    "QUALIFIED REFERENCE QUERIES",
    "DIAGNOSTICS",
    "SMG",
    "TYPES",
    "NAVIGATION",
    "EDITOR RESULTS",
    "QUALIFIED REFERENCE RESULTS",
    "GENERATED",
];

fn canonicalize_sections(fixture: &str) -> String {
    let mut sections = Vec::<(&str, &str, usize)>::new();
    let mut marker = None;
    for (offset, line) in fixture.split_inclusive('\n').scan(0usize, |offset, line| {
        let start = *offset;
        *offset += line.len();
        Some((start, line))
    }) {
        let name = line
            .strip_prefix("# ")
            .and_then(|line| line.strip_suffix('\n'));
        if let Some(name) = name {
            if let Some((previous_name, previous_start)) = marker.take() {
                sections.push((
                    previous_name,
                    &fixture[previous_start..offset],
                    previous_start,
                ));
            }
            marker = Some((name, offset));
        }
    }
    if let Some((previous_name, previous_start)) = marker {
        sections.push((previous_name, &fixture[previous_start..], previous_start));
    }
    if sections.len() < 2 {
        return fixture.to_string();
    }
    let prefix_end = sections[0].2;
    let prefix = &fixture[..prefix_end];
    sections.retain(|(name, _, _)| SECTION_ORDER.contains(name));
    sections.sort_by_key(|(name, _, original)| {
        (
            SECTION_ORDER
                .iter()
                .position(|candidate| candidate == name)
                .unwrap_or(SECTION_ORDER.len()),
            *original,
        )
    });
    let mut output = String::with_capacity(fixture.len());
    output.push_str(prefix);
    for (_, body, _) in sections {
        output.push_str(body.trim_end_matches('\n'));
        output.push('\n');
    }
    output
}

fn replace_section(fixture: &str, name: &str, replacement: &str) -> Option<String> {
    let marker = format!("# {name}\n");
    let section_start = fixture.find(&marker)? + marker.len();
    let section_end = fixture[section_start..]
        .find("\n# ")
        .map_or(fixture.len(), |index| section_start + index);
    let section = &fixture[section_start..section_end];
    fenced_block(section)?;
    let mut updated = String::with_capacity(fixture.len() + replacement.len() + 14);
    updated.push_str(&fixture[..section_start]);
    updated.push_str("~~~sexpr\n");
    updated.push_str(replacement.trim_end_matches('\n'));
    updated.push_str("\n~~~");
    updated.push_str(&fixture[section_end..]);
    Some(updated)
}

fn replace_raw_section(fixture: &str, name: &str, replacement: &str) -> Option<String> {
    let marker = format!("# {name}\n");
    let section_start = fixture.find(&marker)? + marker.len();
    let section_end = fixture[section_start..]
        .find("\n# ")
        .map_or(fixture.len(), |index| section_start + index);
    let mut updated = String::with_capacity(fixture.len() + replacement.len());
    updated.push_str(&fixture[..section_start]);
    updated.push_str(replacement.trim_end_matches('\n'));
    updated.push('\n');
    updated.push_str(&fixture[section_end..]);
    Some(updated)
}

fn fenced_block(input: &str) -> Option<(String, &str)> {
    let start = input.find("~~~")?;
    let after_open = &input[start + 3..];
    let (_, body) = after_open.split_once('\n')?;
    if let Some(after_close) = body.strip_prefix("~~~") {
        return Some((String::new(), after_close));
    }
    let end = body.find("\n~~~")?;
    Some((body[..end].to_string(), &body[end + 4..]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_does_not_allow_a_strategy_override() {
        assert!(
            Cli::try_parse_from(["spec42-snapshot", "check", "--strategy", "parallel"]).is_err()
        );
    }

    #[test]
    fn work_results_are_sorted_for_deterministic_reporting() {
        let mut results = vec![
            FixtureWorkResult {
                path: PathBuf::from("z.md"),
                result: Err("z failure".to_string()),
            },
            FixtureWorkResult {
                path: PathBuf::from("a.md"),
                result: Err("a failure".to_string()),
            },
        ];
        sort_work_results(&mut results);
        assert_eq!(
            results
                .iter()
                .map(|result| result.path.as_path())
                .collect::<Vec<_>>(),
            vec![Path::new("a.md"), Path::new("z.md")]
        );
    }

    fn owned_sections(smg: &str) -> OwnedSections {
        OwnedSections {
            smg: smg.to_string(),
            types: "same".to_string(),
            diagnostics: "same".to_string(),
            navigation: "same".to_string(),
            editor_queries: "same".to_string(),
            qualified_references: "same".to_string(),
        }
    }

    #[test]
    fn parity_mismatch_is_reported_before_owned_output_is_selected() {
        let error = ensure_strategy_parity(
            Path::new("fixture.md"),
            &owned_sections("sequential"),
            &owned_sections("parallel"),
        )
        .expect_err("mismatched owned output must fail parity");
        assert!(error.contains("semantic-model outputs differ"));
    }

    /// Every owned section is compared, not only the first: the editor-query section carries the
    /// inspection output, which is the one most likely to depend on construction order.
    #[test]
    fn parity_covers_every_owned_section() {
        let mut parallel = owned_sections("same");
        parallel.editor_queries = "different".to_string();
        let error =
            ensure_strategy_parity(Path::new("fixture.md"), &owned_sections("same"), &parallel)
                .expect_err("a differing editor-query section must fail parity");
        assert!(error.contains("editor-query outputs differ"));
    }

    #[test]
    fn parses_single_and_multi_source_documents() {
        let single = "# SOURCE\n~~~sysml\npackage A {}\n~~~\n";
        assert_eq!(
            parse_source_documents(single, "single.md").unwrap()[0].text,
            "package A {}"
        );
        let multi = "# SOURCE\n## A.sysml\n~~~sysml\npackage A {}\n~~~\n## B.sysml\n~~~sysml\npackage B {}\n~~~\n";
        let documents = parse_source_documents(multi, "multi.md").unwrap();
        assert_eq!(documents.len(), 2);
        assert_eq!(documents[1].name, "B.sysml");
    }

    #[test]
    fn replaces_existing_section_without_touching_neighbors() {
        let fixture = "# SOURCE\n~~~sysml\npackage A {}\n~~~\n# SMG\n~~~sexpr\nold\n~~~\n# DIAGNOSTICS\n~~~sexpr\nkeep\n~~~\n";
        let updated = replace_section(fixture, "SMG", "new").unwrap();
        assert!(updated.contains("# SMG\n~~~sexpr\nnew\n~~~"));
        assert!(updated.contains("# DIAGNOSTICS\n~~~sexpr\nkeep\n~~~"));
    }

    #[test]
    fn inserting_owned_sections_is_idempotent() {
        let fixture = "# META\n~~~ini\ntype=file\n~~~\n# SOURCE\n~~~sysml\npackage A {}\n~~~\n";
        let first = replace_or_insert_section(fixture, "SMG", "model").unwrap();
        let first = replace_or_insert_section(&first, "DIAGNOSTICS", "diagnostics").unwrap();
        let first = replace_or_insert_section(&first, "NAVIGATION", "navigation").unwrap();
        let second = replace_or_insert_section(&first, "SMG", "model").unwrap();
        let second = replace_or_insert_section(&second, "DIAGNOSTICS", "diagnostics").unwrap();
        let second = replace_or_insert_section(&second, "NAVIGATION", "navigation").unwrap();
        assert_eq!(first, second);
    }

    #[test]
    fn canonicalizes_shuffled_top_level_sections() {
        let fixture = "# SMG\nold\n# NAVIGATION\nnav\n# SOURCE\n~~~sysml\npackage A {}\n~~~\n# META\nmeta\n# DIAGNOSTICS\ndiag\n";
        let canonical = canonicalize_sections(fixture);
        assert_eq!(
            canonical,
            "# META\nmeta\n# SOURCE\n~~~sysml\npackage A {}\n~~~\n# DIAGNOSTICS\ndiag\n# SMG\nold\n# NAVIGATION\nnav\n"
        );
        assert_eq!(canonicalize_sections(&canonical), canonical);
    }

    #[test]
    fn normalizes_out_of_contract_sections_and_is_idempotent() {
        let fixture = "# META\nmeta\n# SOURCE\nsource\n# EXTRA\nextra\n# DIAGNOSTICS\ndiag\n# NOTES\nnotes\n# FORMAT\nformat\n# SMG\nsmg\n";
        let canonical = canonicalize_sections(fixture);
        assert_eq!(
            canonical,
            "# META\nmeta\n# SOURCE\nsource\n# DIAGNOSTICS\ndiag\n# SMG\nsmg\n"
        );
        assert!(!canonical.contains("# EXTRA\n"));
        assert!(!canonical.contains("# NOTES\n"));
        assert!(!canonical.contains("# FORMAT\n"));
        assert_eq!(canonicalize_sections(&canonical), canonical);
    }

    #[test]
    fn parses_generate_metadata_and_rejects_incomplete_or_conflicting_metadata() {
        let fixture = "# META\n~~~ini\ndescription=Requirements CSV\ntype=generate\nlibraries=standard\nplugin=requirements_csv\n~~~\n";
        assert_eq!(
            parse_fixture_meta(fixture, "fixture.md").unwrap(),
            FixtureMeta {
                libraries: LibrarySelection::Standard,
                repository_sources: Vec::new(),
                generation: Some(GenerationRequest {
                    plugin: GeneratorPlugin::Conformance("requirements_csv".to_string()),
                    diagram_selection: None,
                })
            }
        );

        for (meta, expected) in [
            ("type=generate", "requires a plugin"),
            ("type=file\nplugin=x", "only valid with type=generate"),
            ("type=generate\ntype=file\nplugin=x", "duplicate META key"),
            ("type=generate\nplugin=x\nplugin=y", "duplicate META key"),
            ("type=generate\nnot metadata", "must be key=value"),
        ] {
            let fixture = format!("# META\n~~~ini\n{meta}\n~~~\n");
            let error = parse_fixture_meta(&fixture, "fixture.md").unwrap_err();
            assert!(error.contains(expected), "unexpected error: {error}");
        }
    }

    #[test]
    fn parses_closed_typed_diagram_selection() {
        let diagram = "# META\n~~~ini\ntype=generate\nplugin=repository:diagram\nviewKind=general-view\nviewDocument=model.sysml\nviewQualifiedName=Example::selected\n~~~\n";
        assert_eq!(
            parse_fixture_meta(diagram, "fixture.md")
                .unwrap()
                .generation,
            Some(GenerationRequest {
                plugin: GeneratorPlugin::RepositoryDiagram,
                diagram_selection: Some(DiagramSelection {
                    kind: "general-view".to_string(),
                    document: "model.sysml".to_string(),
                    qualified_name: "Example::selected".to_string(),
                }),
            })
        );
    }

    #[test]
    fn parses_qualified_reference_probe_mechanics() {
        let fixture = "# SOURCE\n## model.sysml\n~~~sysml\npackage Example {}\n~~~\n# QUALIFIED REFERENCE QUERIES\n~~~text\nresolve model.sysml Example::selected ViewUsage\nresolve * StandardViewDefinitions::GeneralView *\n~~~\n";
        assert_eq!(
            parse_qualified_reference_probes(
                fixture,
                &[SourceDocument {
                    name: "model.sysml".to_string(),
                    text: "package Example {}".to_string(),
                }],
                "fixture.md",
            )
            .unwrap(),
            vec![
                QualifiedReferenceProbe {
                    document: Some("memory://snapshot/model.sysml".to_string()),
                    qualified_name: "Example::selected".to_string(),
                    expected_kind: Some(ElementKind::ViewUsage),
                },
                QualifiedReferenceProbe {
                    document: None,
                    qualified_name: "StandardViewDefinitions::GeneralView".to_string(),
                    expected_kind: None,
                },
            ]
        );
    }

    #[test]
    fn rejects_invalid_generator_selection_metadata() {
        for (meta, expected) in [
            (
                "type=generate\nplugin=repository:diagram\nviewKind=general-view",
                "must be specified together",
            ),
            (
                "type=generate\nplugin=requirements_csv\nviewKind=general-view\nviewDocument=model.sysml\nviewQualifiedName=Example::selected",
                "only valid with plugin=repository:diagram",
            ),
            ("type=generate\nplugin=../../escape", "unknown or unsafe"),
            (
                "type=file\nviewKind=general-view\nviewDocument=model.sysml\nviewQualifiedName=Example::selected",
                "only valid with type=generate",
            ),
        ] {
            let fixture = format!("# META\n~~~ini\n{meta}\n~~~\n");
            let error = parse_fixture_meta(&fixture, "fixture.md").unwrap_err();
            assert!(error.contains(expected), "unexpected error: {error}");
        }
    }

    #[test]
    fn repository_plugin_paths_are_closed() {
        assert!(generator_plugin_path(&GeneratorPlugin::RepositoryDiagram)
            .ends_with("generator-plugins/target/wasm32-unknown-unknown/release/spec42_diagram_generator.wasm"));
        assert!(generator_plugin_path(&GeneratorPlugin::Conformance("example".to_string()))
            .ends_with("generator-tests/plugins/target/wasm32-unknown-unknown/release/spec42_conformance_example.wasm"));
    }

    #[test]
    fn generated_artifacts_are_sorted_and_use_path_specific_fences() {
        let mut artifacts = GeneratedArtifacts::default();
        artifacts
            .insert_utf8("z/report.json", "{\"ok\":true}\n".to_string())
            .unwrap();
        artifacts
            .insert_utf8("requirements.csv", "name\nSafeStop\n".to_string())
            .unwrap();
        let rendered = render_generated_artifacts(&artifacts);
        assert_eq!(
            rendered,
            "## requirements.csv\n~~~csv\nname\nSafeStop\n\n~~~\n## z/report.json\n~~~json\n{\"ok\":true}\n\n~~~\n"
        );
        let fixture = format!("# GENERATED\n{rendered}");
        assert_eq!(
            parse_generated_artifacts(&fixture, "fixture.md").unwrap(),
            Some(artifacts)
        );
    }

    #[test]
    fn generated_section_is_inserted_last_and_replaced_as_a_complete_artifact_set() {
        let fixture = "# GENERATED\n## stale.txt\n~~~text\nstale\n~~~\n# SOURCE\n~~~sysml\npackage A {}\n~~~\n# META\n~~~ini\ntype=generate\nplugin=x\n~~~\n";
        let mut artifacts = GeneratedArtifacts::default();
        artifacts
            .insert_utf8("fresh.csv", "name\nA\n".to_string())
            .unwrap();
        let updated = replace_or_insert_generated_section(fixture, &artifacts);
        let canonical = canonicalize_sections(&updated);
        assert!(!canonical.contains("stale.txt"));
        assert!(canonical.ends_with("# GENERATED\n## fresh.csv\n~~~csv\nname\nA\n\n~~~\n"));
        assert_eq!(canonicalize_sections(&canonical), canonical);

        let without_generated = "# META\nmeta\n# SOURCE\nsource\n";
        let inserted = canonicalize_sections(&replace_or_insert_generated_section(
            without_generated,
            &artifacts,
        ));
        assert!(inserted.ends_with("# GENERATED\n## fresh.csv\n~~~csv\nname\nA\n\n~~~\n"));
    }

    #[test]
    fn generated_artifact_paths_are_safe_and_unique() {
        let mut artifacts = GeneratedArtifacts::default();
        artifacts
            .insert_utf8("ok/report.csv", String::new())
            .unwrap();
        assert!(artifacts
            .insert_utf8("ok/report.csv", String::new())
            .unwrap_err()
            .contains("duplicate"));
        for path in [
            "",
            "/absolute.csv",
            "../escape.csv",
            "a/../escape.csv",
            "./same.csv",
        ] {
            assert!(
                GeneratedArtifacts::default()
                    .insert_utf8(path, String::new())
                    .is_err(),
                "accepted {path:?}"
            );
        }
    }
}
