use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Component, Path, PathBuf};
use std::process::ExitCode;
use std::sync::Arc;
use std::time::{Duration, Instant};

use generator_api::{
    ArtifactLimits, ArtifactSet, GeneratorDiagnosticLevel, GeneratorModelView, QueryLimits,
};
use generator_host::{
    CancellationHandle, GeneratorFailureCategory, GeneratorHostError, GeneratorRuntime,
    RuntimeLimits, RuntimeOptions, GENERATOR_ABI_VERSION,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub mod apply;
pub mod plan;

use crate::cli::{Cli, GenerateArgs, OutputFormat};
use crate::host_snapshot::load_snapshot_for_paths;

use generator_api::RESERVED_MANIFEST_NAME as MANIFEST_NAME;
const EXIT_MODEL_INVALID: u8 = 10;
const EXIT_API_INCOMPATIBLE: u8 = 11;
const EXIT_GENERATOR_FAILED: u8 = 12;
const EXIT_RESOURCE_EXHAUSTED: u8 = 13;
const EXIT_OUTPUT_POLICY: u8 = 14;
const EXIT_CHECK_DIFFERENT: u8 = 15;

/// Outcome of a generation run, as reported to the user and to CI.
///
/// These values are a public interface -- scripts branch on them -- so they are an enum
/// with one serialized spelling rather than string literals scattered across call sites.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GenerationStatus {
    Generated,
    Unchanged,
    Different,
    DryRun,
    ArtifactInvalid,
    ApiIncompatible,
    GeneratorError,
    GeneratorTrap,
    ResourceExhausted,
    Cancelled,
    ModelValidationFailure,
    OutputPolicyFailure,
}

impl std::fmt::Display for GenerationStatus {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // One spelling for JSON and for humans, derived from the serde name so the two
        // cannot drift apart.
        let rendered = serde_json::to_value(self)
            .ok()
            .and_then(|value| value.as_str().map(str::to_owned))
            .unwrap_or_default();
        formatter.write_str(&rendered)
    }
}

impl GenerationStatus {
    fn from_failure(category: GeneratorFailureCategory) -> Self {
        match category {
            GeneratorFailureCategory::ArtifactInvalid => Self::ArtifactInvalid,
            GeneratorFailureCategory::ApiIncompatible => Self::ApiIncompatible,
            GeneratorFailureCategory::GeneratorError => Self::GeneratorError,
            GeneratorFailureCategory::Trap => Self::GeneratorTrap,
            GeneratorFailureCategory::ResourceExhausted => Self::ResourceExhausted,
            GeneratorFailureCategory::Cancelled => Self::Cancelled,
            GeneratorFailureCategory::OutputPolicy => Self::OutputPolicyFailure,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ModelDiagnosticRecord {
    pub uri: String,
    pub line: u32,
    pub character: u32,
    pub severity: String,
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct GenerationOperations {
    pub created: Vec<String>,
    pub changed: Vec<String>,
    pub unchanged: Vec<String>,
    pub conflicting: Vec<String>,
    /// Existing unowned files whose bytes already match what the generator produced.
    /// Writing them would change nothing, but recording them as owned would silently
    /// license a future overwrite, so they need `--force` like any other unowned file.
    pub adopted: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GeneratorDiagnosticRecord {
    pub level: GeneratorDiagnosticLevel,
    pub message: String,
    pub element_id: Option<String>,
    pub source_uri: Option<String>,
    pub line: Option<u32>,
    pub character: Option<u32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GenerationTimings {
    pub module_prepare_ms: u128,
    pub workspace_load_ms: u128,
    pub validation_ms: u128,
    pub guest_execution_ms: u128,
    /// Microseconds, because a guest run routinely rounds to 0 ms.
    pub guest_execution_us: u128,
    pub output_plan_ms: u128,
    pub output_commit_ms: u128,
}

impl GenerationOperations {
    fn has_differences(&self) -> bool {
        !self.created.is_empty() || !self.changed.is_empty() || !self.conflicting.is_empty()
    }

    fn blocked(&self) -> bool {
        !self.conflicting.is_empty() || !self.adopted.is_empty()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct GenerationReport {
    pub status: GenerationStatus,
    pub model_digest: String,
    pub generator_digest: String,
    pub api_version: String,
    pub spec42_version: String,
    pub validation_errors: usize,
    pub validation_warnings: usize,
    pub model_diagnostics: Vec<ModelDiagnosticRecord>,
    pub generator_diagnostics: Vec<GeneratorDiagnosticRecord>,
    pub operations: GenerationOperations,
    pub output_files: usize,
    pub output_bytes: usize,
    pub duration_ms: u128,
    pub timings: GenerationTimings,
    pub query_count: u64,
    /// Absent unless a fuel budget was requested, which is what enables metering.
    pub fuel_consumed: Option<u64>,
    pub peak_memory_bytes: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GenerationManifest {
    schema_version: u32,
    generator_digest: String,
    model_digest: String,
    generator_api_version: String,
    spec42_version: String,
    artifacts: BTreeMap<String, String>,
}

pub fn run_generate(cli: &Cli, args: &GenerateArgs) -> Result<ExitCode, String> {
    if !matches!(args.format, OutputFormat::Text | OutputFormat::Json) {
        return Err("generate supports only text and json output".to_owned());
    }

    let module_bytes = match fs::read(&args.generator) {
        Ok(bytes) => bytes,
        Err(error) => {
            emit_simple_failure(
                args.format,
                GenerationStatus::ArtifactInvalid,
                &format!(
                    "failed to read generator {}: {error}",
                    args.generator.display()
                ),
            )?;
            return Ok(ExitCode::from(EXIT_API_INCOMPATIBLE));
        }
    };
    let module_prepare_started = Instant::now();
    // Fuel accounting is instrumentation, not policy: it is enabled only when the caller
    // asked for a budget, which is also what makes `fuel_consumed` reportable.
    let runtime = match GeneratorRuntime::with_options(RuntimeOptions {
        fuel_metering: args.max_fuel.is_some(),
    }) {
        Ok(runtime) => runtime,
        Err(error) => return emit_host_failure(args.format, &error),
    };
    // Validate the module and its core ABI imports before model analysis.
    let prepared = match runtime.prepare(&module_bytes) {
        Ok(prepared) => prepared,
        Err(error) => return emit_host_failure(args.format, &error),
    };
    let module_prepare_ms = module_prepare_started.elapsed().as_millis();

    let workspace_load_started = Instant::now();
    let snapshot = match load_snapshot_for_paths(
        cli,
        &args.path,
        args.workspace_root.as_deref(),
        args.strict_diagnostics,
    ) {
        Ok(snapshot) => snapshot,
        Err(message) => {
            emit_simple_failure(
                args.format,
                GenerationStatus::ModelValidationFailure,
                &message,
            )?;
            return Ok(ExitCode::from(EXIT_MODEL_INVALID));
        }
    };
    let workspace_load_ms = workspace_load_started.elapsed().as_millis();
    let validation_started = Instant::now();
    let validation = match snapshot.ensure_validation() {
        Ok(validation) => validation,
        Err(error) => {
            emit_simple_failure(
                args.format,
                GenerationStatus::ModelValidationFailure,
                &error.to_string(),
            )?;
            return Ok(ExitCode::from(EXIT_MODEL_INVALID));
        }
    };
    let validation_ms = validation_started.elapsed().as_millis();
    let model_diagnostics = validation
        .documents
        .iter()
        .flat_map(|document| &document.diagnostics)
        .map(|diagnostic| ModelDiagnosticRecord {
            uri: diagnostic.uri.to_string(),
            line: diagnostic.range.start.line,
            character: diagnostic.range.start.character,
            severity: format!("{:?}", diagnostic.severity).to_ascii_lowercase(),
            code: diagnostic.code.clone(),
            message: diagnostic.message.clone(),
        })
        .collect::<Vec<_>>();
    let validation_errors = validation.summary.error_count;
    let validation_warnings = validation.summary.warning_count;
    if validation_errors > 0 {
        emit_validation_failure(
            args.format,
            validation_errors,
            validation_warnings,
            &model_diagnostics,
        )?;
        return Ok(ExitCode::from(EXIT_MODEL_INVALID));
    }

    let model = Arc::new(GeneratorModelView::new(snapshot, QueryLimits::default()));
    let model_digest = model.model_digest();
    let spec42_version = model.spec42_version().to_owned();
    let execution = match runtime.execute_prepared(
        &prepared,
        Arc::clone(&model),
        &args.generator_args,
        RuntimeLimits {
            memory_bytes: args.max_memory_bytes,
            fuel: args.max_fuel,
            wall_time: args.timeout_seconds.map(Duration::from_secs),
        },
        ArtifactLimits {
            max_files: args.max_files,
            max_file_bytes: args.max_file_bytes,
            max_total_bytes: args.max_total_bytes,
        },
        CancellationHandle::new(),
    ) {
        Ok(execution) => execution,
        Err(error) => return emit_host_failure(args.format, &error),
    };

    // Carry forward ownership of files this run did not regenerate, so a generator that
    // stops emitting a path does not forfeit the right to replace it later.
    let previous_manifest = match read_manifest(&args.output) {
        Ok(manifest) => manifest,
        Err(message) => {
            emit_simple_failure(args.format, GenerationStatus::OutputPolicyFailure, &message)?;
            return Ok(ExitCode::from(EXIT_OUTPUT_POLICY));
        }
    };
    let manifest = manifest_for(
        &execution.artifacts,
        &execution.generator_digest,
        &model_digest,
        &spec42_version,
        previous_manifest.as_ref(),
    );
    let output_plan_started = Instant::now();
    let (operations, observed) = match plan_outputs(&args.output, &execution.artifacts, args.force)
    {
        Ok(planned) => planned,
        Err(message) => {
            emit_simple_failure(args.format, GenerationStatus::OutputPolicyFailure, &message)?;
            return Ok(ExitCode::from(EXIT_OUTPUT_POLICY));
        }
    };
    let output_plan_ms = output_plan_started.elapsed().as_millis();
    if operations.blocked() && !args.check && !args.dry_run {
        let message = format!(
            "refusing to take over {} unowned or locally modified file(s); use --force to authorize replacement",
            operations.conflicting.len() + operations.adopted.len()
        );
        emit_simple_failure(args.format, GenerationStatus::OutputPolicyFailure, &message)?;
        return Ok(ExitCode::from(EXIT_OUTPUT_POLICY));
    }

    let mut output_commit_ms = 0;
    let status = if args.check {
        if operations.has_differences() {
            GenerationStatus::Different
        } else {
            GenerationStatus::Unchanged
        }
    } else if args.dry_run {
        GenerationStatus::DryRun
    } else {
        let output_commit_started = Instant::now();
        if let Err(message) =
            commit_outputs(&args.output, &execution.artifacts, &manifest, &observed)
        {
            emit_simple_failure(args.format, GenerationStatus::OutputPolicyFailure, &message)?;
            return Ok(ExitCode::from(EXIT_OUTPUT_POLICY));
        }
        output_commit_ms = output_commit_started.elapsed().as_millis();
        GenerationStatus::Generated
    };

    let generator_diagnostics = execution
        .diagnostics
        .into_iter()
        .map(|diagnostic| {
            let source = diagnostic
                .element_id
                .as_deref()
                .and_then(|handle| model.element(handle).ok());
            GeneratorDiagnosticRecord {
                level: diagnostic.level,
                message: diagnostic.message,
                element_id: diagnostic.element_id,
                source_uri: source.as_ref().map(|detail| detail.source_uri.clone()),
                line: source.as_ref().map(|detail| detail.source_range.start_line),
                character: source
                    .as_ref()
                    .map(|detail| detail.source_range.start_character),
            }
        })
        .collect();
    let report = GenerationReport {
        status,
        model_digest,
        generator_digest: execution.generator_digest,
        api_version: GENERATOR_ABI_VERSION.to_string(),
        spec42_version,
        validation_errors,
        validation_warnings,
        model_diagnostics,
        generator_diagnostics,
        operations,
        output_files: execution.artifacts.len(),
        output_bytes: execution.artifacts.total_bytes(),
        duration_ms: execution.duration.as_millis(),
        timings: GenerationTimings {
            module_prepare_ms,
            workspace_load_ms,
            validation_ms,
            guest_execution_ms: execution.duration.as_millis(),
            guest_execution_us: execution.duration.as_micros(),
            output_plan_ms,
            output_commit_ms,
        },
        query_count: execution.query_count,
        fuel_consumed: execution.fuel_consumed,
        peak_memory_bytes: execution.peak_memory_bytes,
    };
    emit_report(&report, args.format)?;
    Ok(if args.check && report.operations.has_differences() {
        ExitCode::from(EXIT_CHECK_DIFFERENT)
    } else if args.dry_run && !report.operations.conflicting.is_empty() {
        ExitCode::from(EXIT_OUTPUT_POLICY)
    } else {
        ExitCode::SUCCESS
    })
}

fn manifest_for(
    artifacts: &ArtifactSet,
    generator_digest: &str,
    model_digest: &str,
    spec42_version: &str,
    retained: Option<&GenerationManifest>,
) -> GenerationManifest {
    GenerationManifest {
        schema_version: 1,
        generator_digest: generator_digest.to_owned(),
        model_digest: model_digest.to_owned(),
        generator_api_version: GENERATOR_ABI_VERSION.to_string(),
        spec42_version: spec42_version.to_owned(),
        artifacts: retained
            .iter()
            .flat_map(|manifest| manifest.artifacts.clone())
            .chain(
                artifacts
                    .entries()
                    .map(|(path, content)| (path.to_string(), digest(content))),
            )
            .collect(),
    }
}

/// Observes the output tree, then plans against it.
///
/// The observation is the only filesystem work here; every ownership and conflict decision
/// is made by [`plan::plan`], which is pure and exhaustively table-tested.
fn plan_outputs(
    output: &Path,
    artifacts: &ArtifactSet,
    force: bool,
) -> Result<(GenerationOperations, apply::ObservedVersions), String> {
    validate_output_root(output)?;
    reject_symlink(output)?;
    let previous = read_manifest(output)?;

    let mut observation = plan::Observation {
        owned: previous
            .map(|manifest| manifest.artifacts)
            .unwrap_or_default(),
        ..plan::Observation::default()
    };
    let mut entries = Vec::new();
    for (path, content) in artifacts.entries() {
        reject_symlink_chain(output, path.as_str())?;
        let target = artifact_path(output, path.as_str());
        let existing = match fs::symlink_metadata(&target) {
            Err(error) if error.kind() == io::ErrorKind::NotFound => plan::Existing::Absent,
            Err(error) => return Err(format!("failed to inspect {}: {error}", target.display())),
            Ok(metadata) if !metadata.file_type().is_file() => plan::Existing::NotAFile,
            Ok(_) => plan::Existing::File {
                content: fs::read(&target)
                    .map_err(|error| format!("failed to read {}: {error}", target.display()))?,
            },
        };
        observation.existing.insert(path.clone(), existing);
        entries.push((path.clone(), content.to_vec()));
    }

    // What planning saw, so the commit can confirm it still holds.
    let versions = apply::ObservedVersions {
        entries: observation
            .existing
            .iter()
            .map(|(path, existing)| {
                let seen = match existing {
                    plan::Existing::File { content } => Some(digest(content)),
                    _ => None,
                };
                (path.to_string(), seen)
            })
            .collect(),
    };

    let planned = plan::plan(&entries, &observation, force, &digest);
    Ok((
        GenerationOperations {
            created: planned.paths_with(plan::Operation::Create),
            changed: planned.paths_with(plan::Operation::Change),
            unchanged: planned.paths_with(plan::Operation::Unchanged),
            conflicting: planned.paths_with(plan::Operation::Conflict),
            adopted: planned.paths_with(plan::Operation::Adopt),
        },
        versions,
    ))
}

fn commit_outputs(
    output: &Path,
    artifacts: &ArtifactSet,
    manifest: &GenerationManifest,
    observed: &apply::ObservedVersions,
) -> Result<(), String> {
    validate_output_root(output)?;
    let parent = output
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent).map_err(|error| {
        format!(
            "failed to create output parent {}: {error}",
            parent.display()
        )
    })?;
    // Creating parents can change what the root resolves to, so check again now that the
    // tree exists rather than trusting the pre-mutation answer.
    validate_output_root(output)?;
    reject_symlink(output)?;
    if output.exists() && !output.is_dir() {
        return Err(format!(
            "output root {} is not a directory",
            output.display()
        ));
    }
    // Staging and the swap both go through the executor, so what production runs is exactly
    // what the fault sweep covers. A unique suffix keeps concurrent runs from colliding.
    let unique = std::process::id();
    let stage_path = parent.join(format!(".spec42-stage-{unique}"));
    let backup_root = parent.join(format!(".spec42-backup-{unique}"));
    let manifest_bytes = serde_json::to_vec_pretty(manifest)
        .map_err(|error| format!("failed to encode generation manifest: {error}"))?;
    let staged: Vec<(String, Vec<u8>)> = artifacts
        .entries()
        .map(|(path, content)| (path.to_string(), content.to_vec()))
        .collect();

    apply::stage_and_install(
        &apply::RealFileSystem,
        output,
        &stage_path,
        &backup_root,
        output.exists(),
        &staged,
        MANIFEST_NAME,
        &manifest_bytes,
        observed,
    )
    .map_err(|error| error.to_string())
}

fn copy_tree(source: &Path, destination: &Path) -> Result<(), String> {
    let mut entries = fs::read_dir(source)
        .map_err(|error| {
            format!(
                "failed to read existing output {}: {error}",
                source.display()
            )
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("failed to enumerate existing output: {error}"))?;
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        let metadata = fs::symlink_metadata(&source_path)
            .map_err(|error| format!("failed to inspect {}: {error}", source_path.display()))?;
        if metadata.file_type().is_symlink() {
            return Err(format!(
                "output transaction refuses symlink {}",
                source_path.display()
            ));
        }
        if metadata.is_dir() {
            fs::create_dir(&destination_path).map_err(|error| {
                format!(
                    "failed to stage directory {}: {error}",
                    destination_path.display()
                )
            })?;
            copy_tree(&source_path, &destination_path)?;
        } else if metadata.is_file() {
            fs::copy(&source_path, &destination_path).map_err(|error| {
                format!(
                    "failed to stage existing file {}: {error}",
                    source_path.display()
                )
            })?;
        } else {
            return Err(format!(
                "unsupported output entry {}",
                source_path.display()
            ));
        }
    }
    Ok(())
}

fn read_manifest(output: &Path) -> Result<Option<GenerationManifest>, String> {
    let path = output.join(MANIFEST_NAME);
    match fs::read(&path) {
        Ok(bytes) => serde_json::from_slice(&bytes)
            .map(Some)
            .map_err(|error| format!("invalid generation manifest {}: {error}", path.display())),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(format!(
            "failed to read generation manifest {}: {error}",
            path.display()
        )),
    }
}

fn reject_symlink(path: &Path) -> Result<(), String> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() => Err(format!(
            "output root {} must not be a symlink",
            path.display()
        )),
        Ok(_) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(format!("failed to inspect {}: {error}", path.display())),
    }
}

/// Rejects an output root that would place generated files over the workspace.
///
/// Called before any filesystem mutation *and* again after parent directories are created,
/// because the two see different things: `scratch/..` cannot be canonicalized while
/// `scratch` does not exist, so an existence-gated check passed it and the transaction then
/// created `scratch`, resolved the root to the current directory, and staged a copy of the
/// whole workspace inside the workspace.
fn validate_output_root(output: &Path) -> Result<(), String> {
    if output.as_os_str().is_empty() || output.parent().is_none() {
        return Err(format!(
            "refusing broad output root {}; choose a dedicated generation directory",
            output.display()
        ));
    }
    // Reject traversal syntactically, before anything on disk is consulted. `..` cannot be
    // resolved safely against a path that does not exist yet, and a root that needs it is
    // not a dedicated generation directory.
    for component in output.components() {
        match component {
            Component::CurDir | Component::ParentDir => {
                return Err(format!(
                    "refusing output root {} containing `.` or `..`; give an explicit path",
                    output.display()
                ));
            }
            _ => {}
        }
    }

    let current = std::env::current_dir()
        .and_then(|path| path.canonicalize())
        .map_err(|error| format!("failed to resolve current directory: {error}"))?;
    // Resolve against the nearest ancestor that exists, so the check works before the
    // directory has been created as well as after.
    let resolved = resolve_against_existing_ancestor(output)?;
    if resolved == current {
        return Err(
            "refusing to use the workspace/current directory as generated output".to_owned(),
        );
    }
    if current.starts_with(&resolved) {
        return Err(format!(
            "refusing output root {}; it contains the current directory",
            output.display()
        ));
    }
    Ok(())
}

/// Absolute form of `path`, resolved through the closest ancestor that exists.
fn resolve_against_existing_ancestor(path: &Path) -> Result<PathBuf, String> {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|error| format!("failed to resolve current directory: {error}"))?
            .join(path)
    };

    let mut existing = absolute.as_path();
    let mut trailing = Vec::new();
    loop {
        if existing.exists() {
            break;
        }
        match (existing.file_name(), existing.parent()) {
            (Some(name), Some(parent)) => {
                trailing.push(name.to_owned());
                existing = parent;
            }
            _ => return Ok(absolute),
        }
    }
    let mut resolved = existing
        .canonicalize()
        .map_err(|error| format!("failed to resolve output root {}: {error}", path.display()))?;
    for name in trailing.into_iter().rev() {
        resolved.push(name);
    }
    Ok(resolved)
}

fn reject_symlink_chain(output: &Path, artifact: &str) -> Result<(), String> {
    let mut current = output.to_path_buf();
    reject_symlink(&current)?;
    for segment in artifact.split('/') {
        current.push(segment);
        reject_symlink(&current)?;
    }
    Ok(())
}

fn artifact_path(root: &Path, normalized: &str) -> PathBuf {
    normalized
        .split('/')
        .fold(root.to_path_buf(), |mut path, segment| {
            path.push(segment);
            path
        })
}

fn digest(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn emit_report(report: &GenerationReport, format: OutputFormat) -> Result<(), String> {
    match format {
        OutputFormat::Json => println!(
            "{}",
            serde_json::to_string_pretty(report)
                .map_err(|error| format!("failed to serialize generation report: {error}"))?
        ),
        OutputFormat::Text => {
            for diagnostic in &report.model_diagnostics {
                eprintln!(
                    "model[{}] {}:{}:{}: {}",
                    diagnostic.severity,
                    diagnostic.uri,
                    diagnostic.line + 1,
                    diagnostic.character + 1,
                    diagnostic.message
                );
            }
            for diagnostic in &report.generator_diagnostics {
                eprintln!("generator[{:?}]: {}", diagnostic.level, diagnostic.message);
            }
            println!(
                "Generation {}: {} created, {} changed, {} unchanged; {} file(s), {} byte(s)",
                report.status,
                report.operations.created.len(),
                report.operations.changed.len(),
                report.operations.unchanged.len(),
                report.output_files,
                report.output_bytes
            );
            for path in &report.operations.created {
                println!("create {path}");
            }
            for path in &report.operations.changed {
                println!("change {path}");
            }
            for path in &report.operations.conflicting {
                println!("conflict {path}");
            }
        }
        _ => unreachable!("format validated by run_generate"),
    }
    Ok(())
}

fn emit_validation_failure(
    format: OutputFormat,
    errors: usize,
    warnings: usize,
    diagnostics: &[ModelDiagnosticRecord],
) -> Result<(), String> {
    if format == OutputFormat::Json {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "status": "model_validation_failure",
                "error_count": errors,
                "warning_count": warnings,
                "diagnostics": diagnostics,
            }))
            .map_err(|error| error.to_string())?
        );
    } else {
        eprintln!("generation skipped: model has {errors} error(s) and {warnings} warning(s)");
        for diagnostic in diagnostics {
            eprintln!(
                "{}:{}:{}: {}",
                diagnostic.uri,
                diagnostic.line + 1,
                diagnostic.character + 1,
                diagnostic.message
            );
        }
    }
    Ok(())
}

fn emit_simple_failure(
    format: OutputFormat,
    status: GenerationStatus,
    message: &str,
) -> Result<(), String> {
    let status = status.to_string();
    if format == OutputFormat::Json {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "status": status,
                "message": message,
            }))
            .map_err(|error| error.to_string())?
        );
    } else {
        eprintln!("{status}: {message}");
    }
    Ok(())
}

fn emit_host_failure(format: OutputFormat, error: &GeneratorHostError) -> Result<ExitCode, String> {
    let status = GenerationStatus::from_failure(error.category);
    if format == OutputFormat::Json {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "status": status,
                "category": status,
                "phase": error.phase,
                "message": error.message,
            }))
            .map_err(|serialization_error| serialization_error.to_string())?
        );
    } else {
        eprintln!("{status} during {}: {}", error.phase, error.message);
    }
    let code = match error.category {
        GeneratorFailureCategory::ArtifactInvalid | GeneratorFailureCategory::ApiIncompatible => {
            EXIT_API_INCOMPATIBLE
        }
        GeneratorFailureCategory::GeneratorError
        | GeneratorFailureCategory::Trap
        | GeneratorFailureCategory::Cancelled => EXIT_GENERATOR_FAILED,
        GeneratorFailureCategory::ResourceExhausted => EXIT_RESOURCE_EXHAUSTED,
        GeneratorFailureCategory::OutputPolicy => EXIT_OUTPUT_POLICY,
    };
    Ok(ExitCode::from(code))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Plans, discarding the observed versions the commit path needs.
    fn plan_only(
        output: &Path,
        artifacts: &ArtifactSet,
        force: bool,
    ) -> Result<GenerationOperations, String> {
        plan_outputs(output, artifacts, force).map(|(operations, _)| operations)
    }

    /// Plans and commits in one step, as `run_generate` does.
    fn commit(
        output: &Path,
        artifacts: &ArtifactSet,
        manifest: &GenerationManifest,
    ) -> Result<(), String> {
        let (_, observed) = plan_outputs(output, artifacts, true)?;
        commit_outputs(output, artifacts, manifest, &observed)
    }

    fn artifacts(entries: &[(&str, &[u8])]) -> ArtifactSet {
        let mut set = ArtifactSet::new(ArtifactLimits::default());
        for (path, bytes) in entries {
            set.emit(path, bytes.to_vec()).unwrap();
        }
        set
    }

    #[test]
    fn commit_is_transactional_and_preserves_unmentioned_files() {
        let temp = tempfile::tempdir().unwrap();
        let output = temp.path().join("generated");
        fs::create_dir(&output).unwrap();
        fs::write(output.join("keep.txt"), b"keep").unwrap();
        let set = artifacts(&[("nested/a.bin", &[0, 255]), ("b.txt", b"new")]);
        let manifest = manifest_for(&set, "generator", "model", "spec42", None);
        commit(&output, &set, &manifest).unwrap();
        assert_eq!(fs::read(output.join("keep.txt")).unwrap(), b"keep");
        assert_eq!(fs::read(output.join("nested/a.bin")).unwrap(), [0, 255]);
        assert!(output.join(MANIFEST_NAME).is_file());
    }

    #[test]
    fn unowned_changed_file_conflicts_but_force_allows_it() {
        let temp = tempfile::tempdir().unwrap();
        fs::write(temp.path().join("a.txt"), b"local").unwrap();
        let set = artifacts(&[("a.txt", b"generated")]);
        let plan = plan_only(temp.path(), &set, false).unwrap();
        assert_eq!(plan.conflicting, ["a.txt"]);
        let forced = plan_only(temp.path(), &set, true).unwrap();
        assert_eq!(forced.changed, ["a.txt"]);
    }

    #[test]
    fn matching_bytes_do_not_grant_ownership_of_an_unowned_file() {
        let temp = tempfile::tempdir().unwrap();
        // A file the user already had, which the generator happens to reproduce exactly.
        fs::write(temp.path().join("__init__.py"), b"").unwrap();
        let set = artifacts(&[("__init__.py", b"")]);

        let plan = plan_only(temp.path(), &set, false).unwrap();
        assert_eq!(plan.adopted, ["__init__.py"]);
        assert!(plan.unchanged.is_empty());
        assert!(plan.blocked(), "adoption must require --force");

        let forced = plan_only(temp.path(), &set, true).unwrap();
        assert_eq!(forced.unchanged, ["__init__.py"]);
        assert!(!forced.blocked());
    }

    #[test]
    fn ownership_survives_a_run_that_stops_emitting_a_file() {
        let temp = tempfile::tempdir().unwrap();
        let output = temp.path().join("generated");

        let first = artifacts(&[("keep.rs", b"one"), ("dropped.rs", b"two")]);
        commit(&output, &first, &manifest_for(&first, "g", "m", "s", None)).unwrap();

        // A later run emits only one of them; the other stays on disk.
        let previous = read_manifest(&output).unwrap();
        let second = artifacts(&[("keep.rs", b"one")]);
        let manifest = manifest_for(&second, "g", "m", "s", previous.as_ref());
        commit(&output, &second, &manifest).unwrap();

        // The dropped file must still be ours, so regenerating it later is not a conflict.
        let third = artifacts(&[("keep.rs", b"one"), ("dropped.rs", b"changed")]);
        let plan = plan_only(&output, &third, false).unwrap();
        assert_eq!(plan.changed, ["dropped.rs"]);
        assert!(plan.conflicting.is_empty(), "{:?}", plan.conflicting);
    }

    #[test]
    fn an_empty_run_does_not_forfeit_the_ownership_record() {
        let temp = tempfile::tempdir().unwrap();
        let output = temp.path().join("generated");

        let first = artifacts(&[("a.rs", b"one")]);
        commit(&output, &first, &manifest_for(&first, "g", "m", "s", None)).unwrap();

        let previous = read_manifest(&output).unwrap();
        let empty = artifacts(&[]);
        let manifest = manifest_for(&empty, "g", "m", "s", previous.as_ref());
        commit(&output, &empty, &manifest).unwrap();

        let again = artifacts(&[("a.rs", b"two")]);
        let plan = plan_only(&output, &again, false).unwrap();
        assert_eq!(plan.changed, ["a.rs"]);
        assert!(plan.conflicting.is_empty());
    }

    #[test]
    fn output_roots_that_escape_to_the_workspace_are_refused() {
        // `scratch/..` resolves to the current directory once `scratch` is created, so the
        // pre-mutation check must reject it syntactically rather than waiting for the path
        // to exist.
        for root in ["scratch/..", "..", "../..", "a/b/../..", "."] {
            let error = validate_output_root(Path::new(root))
                .expect_err(&format!("accepted escaping output root `{root}`"));
            assert!(
                error.contains("refusing"),
                "unexpected message for `{root}`: {error}"
            );
        }
    }

    #[test]
    fn a_dedicated_output_root_is_accepted_before_it_exists() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("generated/nested");
        assert!(!root.exists());
        validate_output_root(&root).expect("a fresh dedicated directory should be allowed");
    }

    #[cfg(unix)]
    #[test]
    fn symlink_escape_is_rejected() {
        use std::os::unix::fs::symlink;

        let temp = tempfile::tempdir().unwrap();
        let outside = tempfile::tempdir().unwrap();
        symlink(outside.path(), temp.path().join("escape")).unwrap();
        let set = artifacts(&[("escape/file.txt", b"bad")]);
        assert!(plan_only(temp.path(), &set, false)
            .unwrap_err()
            .contains("symlink"));
    }
}
