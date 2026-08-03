use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::Arc;
use std::time::{Duration, Instant};

use generator_api::{
    ArtifactLimits, ArtifactSet, GeneratorDiagnosticLevel, GeneratorModelView, QueryLimits,
};
use generator_host::{
    CancellationHandle, GeneratorFailureCategory, GeneratorHostError, GeneratorRuntime,
    RuntimeLimits, GENERATOR_ABI_VERSION,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::cli::{Cli, GenerateArgs, OutputFormat};
use crate::host_snapshot::load_snapshot_for_paths;

const MANIFEST_NAME: &str = ".spec42-generator-manifest.json";
const EXIT_MODEL_INVALID: u8 = 10;
const EXIT_API_INCOMPATIBLE: u8 = 11;
const EXIT_GENERATOR_FAILED: u8 = 12;
const EXIT_RESOURCE_EXHAUSTED: u8 = 13;
const EXIT_OUTPUT_POLICY: u8 = 14;
const EXIT_CHECK_DIFFERENT: u8 = 15;

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
    pub output_plan_ms: u128,
    pub output_commit_ms: u128,
}

impl GenerationOperations {
    fn has_differences(&self) -> bool {
        !self.created.is_empty() || !self.changed.is_empty() || !self.conflicting.is_empty()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct GenerationReport {
    pub status: String,
    pub model_digest: String,
    pub generator_digest: String,
    pub api_version: String,
    pub spec42_version: String,
    pub invalid_model: bool,
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
    pub fuel_consumed: u64,
    pub resource_limit_status: String,
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
                "artifact_invalid",
                &format!(
                    "failed to read generator {}: {error}",
                    args.generator.display()
                ),
            )?;
            return Ok(ExitCode::from(EXIT_API_INCOMPATIBLE));
        }
    };
    let module_prepare_started = Instant::now();
    let runtime = match GeneratorRuntime::new() {
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
            emit_simple_failure(args.format, "model_validation_failure", &message)?;
            return Ok(ExitCode::from(EXIT_MODEL_INVALID));
        }
    };
    let workspace_load_ms = workspace_load_started.elapsed().as_millis();
    let validation_started = Instant::now();
    let validation = match snapshot.ensure_validation() {
        Ok(validation) => validation,
        Err(error) => {
            emit_simple_failure(args.format, "model_validation_failure", &error.to_string())?;
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
            wall_time: Duration::from_secs(args.timeout_seconds),
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

    let manifest = manifest_for(
        &execution.artifacts,
        &execution.generator_digest,
        &model_digest,
        &spec42_version,
    );
    let output_plan_started = Instant::now();
    let operations = match plan_outputs(&args.output, &execution.artifacts, args.force) {
        Ok(operations) => operations,
        Err(message) => {
            emit_simple_failure(args.format, "output_policy_failure", &message)?;
            return Ok(ExitCode::from(EXIT_OUTPUT_POLICY));
        }
    };
    let output_plan_ms = output_plan_started.elapsed().as_millis();
    if !operations.conflicting.is_empty() && !args.check && !args.dry_run {
        let message = format!(
            "refusing to overwrite {} unowned or locally modified file(s); use --force to authorize replacement",
            operations.conflicting.len()
        );
        emit_simple_failure(args.format, "output_policy_failure", &message)?;
        return Ok(ExitCode::from(EXIT_OUTPUT_POLICY));
    }

    let mut output_commit_ms = 0;
    let status = if args.check {
        if operations.has_differences() {
            "different"
        } else {
            "unchanged"
        }
    } else if args.dry_run {
        "dry_run"
    } else {
        let output_commit_started = Instant::now();
        if let Err(message) = commit_outputs(&args.output, &execution.artifacts, &manifest) {
            emit_simple_failure(args.format, "output_policy_failure", &message)?;
            return Ok(ExitCode::from(EXIT_OUTPUT_POLICY));
        }
        output_commit_ms = output_commit_started.elapsed().as_millis();
        "generated"
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
        status: status.to_owned(),
        model_digest,
        generator_digest: execution.generator_digest,
        api_version: GENERATOR_ABI_VERSION.to_string(),
        spec42_version,
        invalid_model: false,
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
            output_plan_ms,
            output_commit_ms,
        },
        query_count: execution.query_count,
        fuel_consumed: execution.fuel_consumed,
        resource_limit_status: "within_limits".to_owned(),
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
) -> GenerationManifest {
    GenerationManifest {
        schema_version: 1,
        generator_digest: generator_digest.to_owned(),
        model_digest: model_digest.to_owned(),
        generator_api_version: GENERATOR_ABI_VERSION.to_string(),
        spec42_version: spec42_version.to_owned(),
        artifacts: artifacts
            .iter()
            .map(|artifact| (artifact.path, digest(&artifact.content)))
            .collect(),
    }
}

fn plan_outputs(
    output: &Path,
    artifacts: &ArtifactSet,
    force: bool,
) -> Result<GenerationOperations, String> {
    validate_output_root(output)?;
    reject_symlink(output)?;
    let previous = read_manifest(output)?;
    let mut operations = GenerationOperations {
        created: Vec::new(),
        changed: Vec::new(),
        unchanged: Vec::new(),
        conflicting: Vec::new(),
    };
    for artifact in artifacts.iter() {
        reject_symlink_chain(output, &artifact.path)?;
        let target = artifact_path(output, &artifact.path);
        match fs::symlink_metadata(&target) {
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                operations.created.push(artifact.path)
            }
            Err(error) => return Err(format!("failed to inspect {}: {error}", target.display())),
            Ok(metadata) if !metadata.file_type().is_file() => {
                operations.conflicting.push(artifact.path)
            }
            Ok(_) => {
                let current = fs::read(&target)
                    .map_err(|error| format!("failed to read {}: {error}", target.display()))?;
                if current == artifact.content {
                    operations.unchanged.push(artifact.path);
                    continue;
                }
                let previously_owned = previous
                    .as_ref()
                    .and_then(|manifest| manifest.artifacts.get(&artifact.path))
                    .is_some_and(|prior_hash| *prior_hash == digest(&current));
                if force || previously_owned {
                    operations.changed.push(artifact.path);
                } else {
                    operations.conflicting.push(artifact.path);
                }
            }
        }
    }
    Ok(operations)
}

fn commit_outputs(
    output: &Path,
    artifacts: &ArtifactSet,
    manifest: &GenerationManifest,
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
    reject_symlink(output)?;
    if output.exists() && !output.is_dir() {
        return Err(format!(
            "output root {} is not a directory",
            output.display()
        ));
    }
    let stage = tempfile::Builder::new()
        .prefix(".spec42-stage-")
        .tempdir_in(parent)
        .map_err(|error| format!("failed to create private output staging directory: {error}"))?;
    if output.exists() {
        copy_tree(output, stage.path())?;
    }
    for artifact in artifacts.iter() {
        let target = artifact_path(stage.path(), &artifact.path);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).map_err(|error| {
                format!(
                    "failed to create staging directory {}: {error}",
                    parent.display()
                )
            })?;
        }
        fs::write(&target, &artifact.content)
            .map_err(|error| format!("failed to stage {}: {error}", artifact.path))?;
    }
    let manifest_bytes = serde_json::to_vec_pretty(manifest)
        .map_err(|error| format!("failed to encode generation manifest: {error}"))?;
    fs::write(stage.path().join(MANIFEST_NAME), manifest_bytes)
        .map_err(|error| format!("failed to stage generation manifest: {error}"))?;

    let stage_path = stage.keep();
    if !output.exists() {
        return fs::rename(&stage_path, output)
            .map_err(|error| format!("failed to install generated output: {error}"));
    }

    let backup = tempfile::Builder::new()
        .prefix(".spec42-backup-")
        .tempdir_in(parent)
        .map_err(|error| format!("failed to create transactional backup: {error}"))?;
    let previous_path = backup.path().join("previous");
    fs::rename(output, &previous_path).map_err(|error| {
        format!("failed to move existing output into transaction backup: {error}")
    })?;
    if let Err(error) = fs::rename(&stage_path, output) {
        let rollback = fs::rename(&previous_path, output);
        return Err(match rollback {
            Ok(()) => format!("failed to install generated output; previous output restored: {error}"),
            Err(rollback_error) => format!(
                "failed to install generated output ({error}) and rollback failed ({rollback_error}); previous output remains at {}",
                previous_path.display()
            ),
        });
    }
    Ok(())
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

fn validate_output_root(output: &Path) -> Result<(), String> {
    if output.as_os_str().is_empty() || output == Path::new(".") || output.parent().is_none() {
        return Err(format!(
            "refusing broad output root {}; choose a dedicated generation directory",
            output.display()
        ));
    }
    if output.exists() {
        let canonical = output.canonicalize().map_err(|error| {
            format!(
                "failed to resolve output root {}: {error}",
                output.display()
            )
        })?;
        let current = std::env::current_dir()
            .and_then(|path| path.canonicalize())
            .map_err(|error| format!("failed to resolve current directory: {error}"))?;
        if canonical == current {
            return Err(
                "refusing to use the workspace/current directory as generated output".to_owned(),
            );
        }
    }
    Ok(())
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

fn emit_simple_failure(format: OutputFormat, status: &str, message: &str) -> Result<(), String> {
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
    let status = match error.category {
        GeneratorFailureCategory::ArtifactInvalid => "artifact_invalid",
        GeneratorFailureCategory::ApiIncompatible => "api_incompatible",
        GeneratorFailureCategory::GeneratorError => "generator_error",
        GeneratorFailureCategory::Trap => "generator_trap",
        GeneratorFailureCategory::ResourceExhausted => "resource_exhausted",
        GeneratorFailureCategory::Cancelled => "cancelled",
        GeneratorFailureCategory::OutputPolicy => "output_policy_failure",
    };
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
        let manifest = manifest_for(&set, "generator", "model", "spec42");
        commit_outputs(&output, &set, &manifest).unwrap();
        assert_eq!(fs::read(output.join("keep.txt")).unwrap(), b"keep");
        assert_eq!(fs::read(output.join("nested/a.bin")).unwrap(), [0, 255]);
        assert!(output.join(MANIFEST_NAME).is_file());
    }

    #[test]
    fn unowned_changed_file_conflicts_but_force_allows_it() {
        let temp = tempfile::tempdir().unwrap();
        fs::write(temp.path().join("a.txt"), b"local").unwrap();
        let set = artifacts(&[("a.txt", b"generated")]);
        let plan = plan_outputs(temp.path(), &set, false).unwrap();
        assert_eq!(plan.conflicting, ["a.txt"]);
        let forced = plan_outputs(temp.path(), &set, true).unwrap();
        assert_eq!(forced.changed, ["a.txt"]);
    }

    #[cfg(unix)]
    #[test]
    fn symlink_escape_is_rejected() {
        use std::os::unix::fs::symlink;

        let temp = tempfile::tempdir().unwrap();
        let outside = tempfile::tempdir().unwrap();
        symlink(outside.path(), temp.path().join("escape")).unwrap();
        let set = artifacts(&[("escape/file.txt", b"bad")]);
        assert!(plan_outputs(temp.path(), &set, false)
            .unwrap_err()
            .contains("symlink"));
    }
}
