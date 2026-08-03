//! Sandboxed runtime for Spec42 core WebAssembly generator modules.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::time::{Duration, Instant};

use generator_api::{
    ArtifactLimits, ArtifactSet, ElementDetail as ApiElementDetail,
    ElementSummary as ApiElementSummary, GeneratorDiagnostic, GeneratorDiagnosticLevel,
    GeneratorModelView, MultiplicitySummary as ApiMultiplicity, RelationshipSummary,
    MAX_ARTIFACT_PATH_BYTES,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sha2::{Digest, Sha256};
use spec42_generator_protocol as protocol;
use thiserror::Error;
use wasmparser::Parser;
use wasmtime::{
    Caller, Config, Engine, Extern, ExternType, Linker, Memory, Module, Store, StoreLimits,
    StoreLimitsBuilder, ValType,
};

pub const GENERATOR_ABI_VERSION: u32 = protocol::ABI_VERSION;
const MAX_QUERY_TRANSFER_BYTES: usize = 64 * 1024 * 1024;
const MAX_POSTCARD_ARTIFACT_OVERHEAD_BYTES: usize = 20;
const MAX_POSTCARD_RESULT_OVERHEAD_BYTES: usize = 16;
const MAX_GENERATOR_ERROR_RESULT_BYTES: usize = 64 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeLimits {
    pub memory_bytes: usize,
    pub fuel: u64,
    pub wall_time: Duration,
}

impl Default for RuntimeLimits {
    fn default() -> Self {
        Self {
            memory_bytes: 256 * 1024 * 1024,
            fuel: 100_000_000,
            wall_time: Duration::from_secs(30),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GeneratorFailureCategory {
    ArtifactInvalid,
    ApiIncompatible,
    GeneratorError,
    Trap,
    ResourceExhausted,
    Cancelled,
    OutputPolicy,
}

#[derive(Debug, Error)]
#[error("{category:?} during {phase}: {message}")]
pub struct GeneratorHostError {
    pub category: GeneratorFailureCategory,
    pub phase: &'static str,
    pub message: String,
}

#[derive(Debug)]
pub struct GeneratorExecution {
    pub artifacts: ArtifactSet,
    pub diagnostics: Vec<GeneratorDiagnostic>,
    pub generator_digest: String,
    pub duration: Duration,
    pub query_count: u64,
    pub fuel_consumed: u64,
}

#[derive(Debug, Clone)]
pub struct CancellationHandle(Arc<AtomicBool>);

impl CancellationHandle {
    pub fn new() -> Self {
        Self(Arc::new(AtomicBool::new(false)))
    }

    pub fn cancel(&self) {
        self.0.store(true, Ordering::Release);
    }

    pub fn is_cancelled(&self) -> bool {
        self.0.load(Ordering::Acquire)
    }
}

impl Default for CancellationHandle {
    fn default() -> Self {
        Self::new()
    }
}

pub struct GeneratorRuntime {
    engine: Engine,
}

pub struct PreparedGenerator {
    module: Module,
    generator_digest: String,
}

impl GeneratorRuntime {
    pub fn new() -> Result<Self, GeneratorHostError> {
        let mut config = Config::new();
        config.consume_fuel(true);
        config.epoch_interruption(true);
        config.max_wasm_stack(2 * 1024 * 1024);
        let engine = Engine::new(&config).map_err(|error| GeneratorHostError {
            category: GeneratorFailureCategory::ArtifactInvalid,
            phase: "runtime-configuration",
            message: error.to_string(),
        })?;
        Ok(Self { engine })
    }

    pub fn execute(
        &self,
        module_bytes: &[u8],
        model: Arc<GeneratorModelView>,
        args: &[String],
        runtime_limits: RuntimeLimits,
        artifact_limits: ArtifactLimits,
        cancellation: CancellationHandle,
    ) -> Result<GeneratorExecution, GeneratorHostError> {
        let prepared = self.prepare(module_bytes)?;
        self.execute_prepared(
            &prepared,
            model,
            args,
            runtime_limits,
            artifact_limits,
            cancellation,
        )
    }

    /// Validates and compiles a core module without loading a model snapshot.
    pub fn prepare(&self, module_bytes: &[u8]) -> Result<PreparedGenerator, GeneratorHostError> {
        let generator_digest = digest(module_bytes);
        if !Parser::is_core_wasm(module_bytes) {
            return Err(GeneratorHostError {
                category: GeneratorFailureCategory::ArtifactInvalid,
                phase: "module-validation",
                message: "not a core WebAssembly module".to_owned(),
            });
        }

        let module =
            Module::new(&self.engine, module_bytes).map_err(|error| GeneratorHostError {
                category: GeneratorFailureCategory::ArtifactInvalid,
                phase: "module-compilation",
                message: format!("failed to compile WebAssembly module: {error}"),
            })?;
        validate_module_exports(&module).map_err(|message| GeneratorHostError {
            category: GeneratorFailureCategory::ApiIncompatible,
            phase: "api-compatibility",
            message,
        })?;
        let mut linker = Linker::<HostState>::new(&self.engine);
        add_host_functions(&mut linker).map_err(|error| GeneratorHostError {
            category: GeneratorFailureCategory::ApiIncompatible,
            phase: "api-linking",
            message: error.to_string(),
        })?;
        linker
            .instantiate_pre(&module)
            .map_err(|error| GeneratorHostError {
                category: GeneratorFailureCategory::ApiIncompatible,
                phase: "api-compatibility",
                message: format!(
                    "module does not implement the Spec42 generator ABI v{GENERATOR_ABI_VERSION}: {error}"
                ),
            })?;
        Ok(PreparedGenerator {
            module,
            generator_digest,
        })
    }

    pub fn execute_prepared(
        &self,
        prepared: &PreparedGenerator,
        model: Arc<GeneratorModelView>,
        args: &[String],
        runtime_limits: RuntimeLimits,
        artifact_limits: ArtifactLimits,
        cancellation: CancellationHandle,
    ) -> Result<GeneratorExecution, GeneratorHostError> {
        if cancellation.is_cancelled() {
            return Err(GeneratorHostError {
                category: GeneratorFailureCategory::Cancelled,
                phase: "before-execution",
                message: "generation was cancelled".to_owned(),
            });
        }

        let mut linker = Linker::new(&self.engine);
        add_host_functions(&mut linker).map_err(|error| GeneratorHostError {
            category: GeneratorFailureCategory::ApiIncompatible,
            phase: "api-linking",
            message: error.to_string(),
        })?;
        let store_limits = StoreLimitsBuilder::new()
            .memory_size(runtime_limits.memory_bytes)
            .instances(10)
            .memories(1)
            .tables(8)
            .build();
        let mut store = Store::new(
            &self.engine,
            HostState {
                model,
                diagnostics: Vec::new(),
                store_limits,
                query_count: 0,
            },
        );
        store.limiter(|state| &mut state.store_limits);
        store
            .set_fuel(runtime_limits.fuel)
            .map_err(|error| GeneratorHostError {
                category: GeneratorFailureCategory::ResourceExhausted,
                phase: "resource-configuration",
                message: error.to_string(),
            })?;
        store.set_epoch_deadline(1);
        store.epoch_deadline_trap();

        let (deadline_tx, deadline_rx) = mpsc::channel();
        let engine = self.engine.clone();
        let cancelled = cancellation.clone();
        let wall_time = runtime_limits.wall_time;
        let deadline_thread = std::thread::spawn(move || {
            let deadline = Instant::now() + wall_time;
            loop {
                if cancelled.is_cancelled() {
                    engine.increment_epoch();
                    break;
                }
                let remaining = deadline.saturating_duration_since(Instant::now());
                if remaining.is_zero() {
                    engine.increment_epoch();
                    break;
                }
                match deadline_rx.recv_timeout(remaining.min(Duration::from_millis(10))) {
                    Ok(()) => break,
                    Err(mpsc::RecvTimeoutError::Timeout) => continue,
                    Err(mpsc::RecvTimeoutError::Disconnected) => break,
                }
            }
        });

        let started = Instant::now();
        let guest_result = execute_guest(
            &mut store,
            &linker,
            &prepared.module,
            args,
            artifact_limits,
            &cancellation,
        );
        let _ = deadline_tx.send(());
        let _ = deadline_thread.join();
        let artifacts = guest_result?;
        let duration = started.elapsed();
        let remaining_fuel = store.get_fuel().unwrap_or(0);
        let fuel_consumed = runtime_limits.fuel.saturating_sub(remaining_fuel);
        let state = store.into_data();
        Ok(GeneratorExecution {
            artifacts,
            diagnostics: state.diagnostics,
            generator_digest: prepared.generator_digest.clone(),
            duration,
            query_count: state.query_count,
            fuel_consumed,
        })
    }
}

fn validate_module_exports(module: &Module) -> Result<(), String> {
    match module.get_export("memory") {
        Some(ExternType::Memory(_)) => {}
        Some(_) => return Err("module export `memory` is not WebAssembly memory".to_owned()),
        None => return Err("module does not export `memory`".to_owned()),
    }

    require_function(module, "spec42_alloc", &[ValType::I32], &[ValType::I32])?;
    require_function(module, "spec42_dealloc", &[ValType::I32, ValType::I32], &[])?;
    require_function(
        module,
        "spec42_generate",
        &[ValType::I32, ValType::I32],
        &[ValType::I64],
    )
}

fn require_function(
    module: &Module,
    name: &str,
    expected_params: &[ValType],
    expected_results: &[ValType],
) -> Result<(), String> {
    let function = match module.get_export(name) {
        Some(ExternType::Func(function)) => function,
        Some(_) => return Err(format!("module export `{name}` is not a function")),
        None => return Err(format!("module does not export `{name}`")),
    };
    let params = function.params().collect::<Vec<_>>();
    let results = function.results().collect::<Vec<_>>();
    let params_match = params.len() == expected_params.len()
        && params
            .iter()
            .zip(expected_params)
            .all(|(actual, expected)| actual.matches(expected));
    let results_match = results.len() == expected_results.len()
        && results
            .iter()
            .zip(expected_results)
            .all(|(actual, expected)| actual.matches(expected));
    if !params_match || !results_match {
        return Err(format!(
            "module export `{name}` has an incompatible function signature"
        ));
    }
    Ok(())
}

fn execute_guest(
    store: &mut Store<HostState>,
    linker: &Linker<HostState>,
    module: &Module,
    args: &[String],
    artifact_limits: ArtifactLimits,
    cancellation: &CancellationHandle,
) -> Result<ArtifactSet, GeneratorHostError> {
    let instance = linker.instantiate(&mut *store, module).map_err(|error| {
        classify_wasmtime_error("guest-instantiation", error.to_string(), cancellation)
    })?;
    let memory = instance
        .get_memory(&mut *store, "memory")
        .ok_or_else(|| incompatible("guest-instantiation", "module does not export memory"))?;
    let alloc = instance
        .get_typed_func::<i32, i32>(&mut *store, "spec42_alloc")
        .map_err(|error| incompatible("guest-instantiation", error.to_string()))?;
    let dealloc = instance
        .get_typed_func::<(i32, i32), ()>(&mut *store, "spec42_dealloc")
        .map_err(|error| incompatible("guest-instantiation", error.to_string()))?;
    let generate = instance
        .get_typed_func::<(i32, i32), i64>(&mut *store, "spec42_generate")
        .map_err(|error| incompatible("guest-instantiation", error.to_string()))?;

    let args_bytes = postcard::to_allocvec(args).map_err(|error| GeneratorHostError {
        category: GeneratorFailureCategory::GeneratorError,
        phase: "guest-execution",
        message: format!("failed to encode generator arguments: {error}"),
    })?;
    let args_len = i32::try_from(args_bytes.len()).map_err(|_| {
        incompatible(
            "guest-execution",
            "generator arguments exceed the ABI limit",
        )
    })?;
    let args_ptr = alloc.call(&mut *store, args_len).map_err(|error| {
        classify_wasmtime_error("guest-execution", error.to_string(), cancellation)
    })?;
    memory
        .write(&mut *store, args_ptr as usize, &args_bytes)
        .map_err(|error| incompatible("guest-execution", error.to_string()))?;
    let packed = generate
        .call(&mut *store, (args_ptr, args_len))
        .map_err(|error| {
            classify_wasmtime_error("guest-execution", error.to_string(), cancellation)
        })? as u64;
    let output_ptr = (packed & u32::MAX as u64) as u32 as usize;
    let output_len = (packed >> 32) as usize;
    if output_len > max_artifact_result_bytes(artifact_limits) {
        return Err(GeneratorHostError {
            category: GeneratorFailureCategory::OutputPolicy,
            phase: "artifact-validation",
            message: "generator result exceeds the configured artifact limits".to_owned(),
        });
    }
    let mut output = vec![0; output_len];
    memory
        .read(&mut *store, output_ptr, &mut output)
        .map_err(|error| incompatible("guest-execution", error.to_string()))?;
    dealloc
        .call(&mut *store, (output_ptr as i32, output_len as i32))
        .map_err(|error| {
            classify_wasmtime_error("guest-execution", error.to_string(), cancellation)
        })?;
    validate_artifact_result_header(&output, artifact_limits.max_files)?;
    let (result, remaining): (Result<Vec<protocol::Artifact>, String>, _) =
        postcard::take_from_bytes(&output).map_err(|error| {
            incompatible(
                "guest-execution",
                format!("generator returned an invalid result: {error}"),
            )
        })?;
    if !remaining.is_empty() {
        return Err(incompatible(
            "guest-execution",
            "generator result contains trailing bytes",
        ));
    }
    let returned = result.map_err(|message| GeneratorHostError {
        category: GeneratorFailureCategory::GeneratorError,
        phase: "guest-execution",
        message,
    })?;
    validate_returned_artifacts(returned, artifact_limits)
}

fn max_artifact_result_bytes(limits: ArtifactLimits) -> usize {
    limits
        .max_total_bytes
        .saturating_add(
            limits
                .max_files
                .saturating_mul(MAX_ARTIFACT_PATH_BYTES + MAX_POSTCARD_ARTIFACT_OVERHEAD_BYTES),
        )
        .saturating_add(MAX_POSTCARD_RESULT_OVERHEAD_BYTES)
        .max(MAX_GENERATOR_ERROR_RESULT_BYTES)
        .min(i32::MAX as usize)
}

fn validate_artifact_result_header(
    output: &[u8],
    max_files: usize,
) -> Result<(), GeneratorHostError> {
    let Ok((variant, remaining)) = postcard::take_from_bytes::<u32>(output) else {
        return Ok(());
    };
    if variant != 0 {
        return Ok(());
    }
    let Ok((count, _)) = postcard::take_from_bytes::<u32>(remaining) else {
        return Ok(());
    };
    let count = count as usize;
    if count > max_files {
        return Err(GeneratorHostError {
            category: GeneratorFailureCategory::OutputPolicy,
            phase: "artifact-validation",
            message: format!("generator returned {count} files; the limit is {max_files}"),
        });
    }
    Ok(())
}

fn validate_returned_artifacts(
    returned: Vec<protocol::Artifact>,
    limits: ArtifactLimits,
) -> Result<ArtifactSet, GeneratorHostError> {
    let mut artifacts = ArtifactSet::new(limits);
    for artifact in returned {
        artifacts
            .emit(&artifact.file_path, artifact.contents)
            .map_err(|error| GeneratorHostError {
                category: GeneratorFailureCategory::OutputPolicy,
                phase: "artifact-validation",
                message: error.to_string(),
            })?;
    }
    Ok(artifacts)
}

struct HostState {
    model: Arc<GeneratorModelView>,
    diagnostics: Vec<GeneratorDiagnostic>,
    store_limits: StoreLimits,
    query_count: u64,
}

impl HostState {
    fn queried(&mut self) {
        self.query_count = self.query_count.saturating_add(1);
    }
}

fn add_host_functions(linker: &mut Linker<HostState>) -> wasmtime::Result<()> {
    linker.func_wrap(
        protocol::IMPORT_MODULE,
        "query",
        |mut caller: Caller<'_, HostState>,
         operation: i32,
         request_ptr: i32,
         request_len: i32,
         response_ptr: i32,
         response_capacity: i32|
         -> wasmtime::Result<i64> {
            let request = read_guest(&mut caller, request_ptr, request_len)?;
            let response = handle_query(caller.data_mut(), operation, &request)?;
            write_guest_response(&mut caller, response_ptr, response_capacity, &response)
        },
    )?;
    linker.func_wrap(
        protocol::IMPORT_MODULE,
        "diagnostic",
        |mut caller: Caller<'_, HostState>,
         level: i32,
         message_ptr: i32,
         message_len: i32,
         element_ptr: i32,
         element_len: i32|
         -> wasmtime::Result<()> {
            let message = read_guest_string(&mut caller, message_ptr, message_len)?;
            let element = if element_ptr == 0 && element_len == 0 {
                None
            } else {
                Some(read_guest_string(&mut caller, element_ptr, element_len)?)
            };
            let level = diagnostic_level(level)?;
            let state = caller.data_mut();
            if state.diagnostics.len() >= 10_000 {
                return Ok(());
            }
            let element = element.filter(|handle| state.model.element(handle).is_ok());
            state.diagnostics.push(GeneratorDiagnostic {
                level,
                message: bounded_message(message),
                element_id: element,
            });
            Ok(())
        },
    )?;
    Ok(())
}

fn handle_query(
    state: &mut HostState,
    operation: i32,
    request: &[u8],
) -> wasmtime::Result<Vec<u8>> {
    use protocol::operation;

    state.queried();
    match operation {
        operation::INFO => encode_result(Ok(protocol::ModelInfo {
            model_digest: state.model.model_digest(),
            spec42_version: state.model.spec42_version().to_owned(),
            semantic_api_version: state.model.semantic_api_version().to_owned(),
        })),
        operation::ROOTS => encode_result(
            state
                .model
                .roots()
                .map(|values| values.into_iter().map(summary).collect::<Vec<_>>())
                .map_err(|error| error.to_string()),
        ),
        operation::FIND => {
            let metaclass: Option<String> = decode_request(request)?;
            encode_result(
                state
                    .model
                    .find(metaclass.as_deref())
                    .map(|values| values.into_iter().map(summary).collect::<Vec<_>>())
                    .map_err(|error| error.to_string()),
            )
        }
        operation::CHILDREN => {
            let owner: String = decode_request(request)?;
            encode_result(
                state
                    .model
                    .children(&owner)
                    .map(|values| values.into_iter().map(summary).collect::<Vec<_>>())
                    .map_err(|error| error.to_string()),
            )
        }
        operation::ELEMENT => {
            let handle: String = decode_request(request)?;
            encode_result(
                state
                    .model
                    .element(&handle)
                    .map(detail)
                    .map_err(|error| error.to_string()),
            )
        }
        operation::TYPED_BY => {
            let feature: String = decode_request(request)?;
            encode_result(
                state
                    .model
                    .typed_by(&feature)
                    .map(|value| value.map(summary))
                    .map_err(|error| error.to_string()),
            )
        }
        operation::RELATIONSHIPS => {
            let element: String = decode_request(request)?;
            encode_result(
                state
                    .model
                    .relationships(&element)
                    .map(|values| values.into_iter().map(relationship).collect::<Vec<_>>())
                    .map_err(|error| error.to_string()),
            )
        }
        operation::EFFECTIVE_FEATURES => {
            let element: String = decode_request(request)?;
            encode_result(
                state
                    .model
                    .effective_features(&element)
                    .map(|values| values.into_iter().map(summary).collect::<Vec<_>>())
                    .map_err(|error| error.to_string()),
            )
        }
        _ => Err(wasmtime::Error::msg(format!(
            "unknown Spec42 query operation {operation}"
        ))),
    }
}

fn decode_request<T: DeserializeOwned>(bytes: &[u8]) -> wasmtime::Result<T> {
    postcard::from_bytes(bytes)
        .map_err(|error| wasmtime::Error::msg(format!("invalid Spec42 query request: {error}")))
}

fn encode_result<T: Serialize>(result: Result<T, String>) -> wasmtime::Result<Vec<u8>> {
    postcard::to_allocvec(&result)
        .map_err(|error| wasmtime::Error::msg(format!("failed to encode query response: {error}")))
}

fn guest_memory(caller: &mut Caller<'_, HostState>) -> wasmtime::Result<Memory> {
    caller
        .get_export("memory")
        .and_then(Extern::into_memory)
        .ok_or_else(|| wasmtime::Error::msg("generator does not export memory"))
}

fn transfer_range(pointer: i32, length: i32) -> wasmtime::Result<(usize, usize)> {
    let pointer = usize::try_from(pointer)
        .map_err(|_| wasmtime::Error::msg("negative guest memory pointer"))?;
    let length = usize::try_from(length)
        .map_err(|_| wasmtime::Error::msg("negative guest memory length"))?;
    if length > MAX_QUERY_TRANSFER_BYTES {
        return Err(wasmtime::Error::msg("guest transfer exceeds the ABI limit"));
    }
    Ok((pointer, length))
}

fn read_guest(
    caller: &mut Caller<'_, HostState>,
    pointer: i32,
    length: i32,
) -> wasmtime::Result<Vec<u8>> {
    let (pointer, length) = transfer_range(pointer, length)?;
    let memory = guest_memory(caller)?;
    let mut bytes = vec![0; length];
    memory
        .read(caller, pointer, &mut bytes)
        .map_err(|error| wasmtime::Error::msg(error.to_string()))?;
    Ok(bytes)
}

fn read_guest_string(
    caller: &mut Caller<'_, HostState>,
    pointer: i32,
    length: i32,
) -> wasmtime::Result<String> {
    String::from_utf8(read_guest(caller, pointer, length)?)
        .map_err(|error| wasmtime::Error::msg(format!("guest string is not UTF-8: {error}")))
}

fn write_guest_response(
    caller: &mut Caller<'_, HostState>,
    pointer: i32,
    capacity: i32,
    bytes: &[u8],
) -> wasmtime::Result<i64> {
    let (pointer, capacity) = transfer_range(pointer, capacity)?;
    if bytes.len() > MAX_QUERY_TRANSFER_BYTES {
        return Err(wasmtime::Error::msg("host response exceeds the ABI limit"));
    }
    if bytes.len() > capacity {
        return Ok(-(bytes.len() as i64));
    }
    let memory = guest_memory(caller)?;
    memory
        .write(caller, pointer, bytes)
        .map_err(|error| wasmtime::Error::msg(error.to_string()))?;
    Ok(bytes.len() as i64)
}

fn summary(value: ApiElementSummary) -> protocol::ElementSummary {
    protocol::ElementSummary {
        handle: value.handle,
        semantic_id: value.semantic_id,
        metaclass: value.metaclass,
        name: value.name,
        qualified_name: value.qualified_name,
        library_element: value.library_element,
    }
}

fn multiplicity(value: ApiMultiplicity) -> protocol::Multiplicity {
    protocol::Multiplicity {
        lower: value.lower,
        upper: value.upper,
        ordered: value.ordered,
        unique: value.unique,
        implied: value.implied,
    }
}

fn detail(value: ApiElementDetail) -> protocol::ElementDetail {
    protocol::ElementDetail {
        summary: summary(value.summary),
        owner: value.owner.map(summary),
        declared_name: value.declared_name,
        effective_name: value.effective_name,
        source_uri: value.source_uri,
        source_range: protocol::SourceRange {
            start_line: value.source_range.start_line,
            start_character: value.source_range.start_character,
            end_line: value.source_range.end_line,
            end_character: value.source_range.end_character,
        },
        definition: value.definition,
        documentation: value.documentation,
        short_name: value.short_name,
        direction: value.direction,
        derived: value.derived,
        constant: value.constant,
        abstract_flag: value.abstract_,
        variation: value.variation,
        individual: value.individual,
        conjugated: value.conjugated,
        composite: value.composite,
        reference: value.reference,
        end: value.end,
        ordered: value.ordered,
        unique: value.unique,
        multiplicity: value.multiplicity.map(multiplicity),
        evaluated_value: value.evaluated_value,
    }
}

fn relationship(value: RelationshipSummary) -> protocol::Relationship {
    protocol::Relationship {
        kind: value.kind,
        source: summary(value.source),
        target: summary(value.target),
        implied: value.implied,
    }
}

fn diagnostic_level(level: i32) -> wasmtime::Result<GeneratorDiagnosticLevel> {
    match level {
        value if value == protocol::Level::Debug as i32 => Ok(GeneratorDiagnosticLevel::Debug),
        value if value == protocol::Level::Info as i32 => Ok(GeneratorDiagnosticLevel::Info),
        value if value == protocol::Level::Warning as i32 => Ok(GeneratorDiagnosticLevel::Warning),
        value if value == protocol::Level::Error as i32 => Ok(GeneratorDiagnosticLevel::Error),
        _ => Err(wasmtime::Error::msg("invalid diagnostic level")),
    }
}

fn digest(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn bounded_message(mut message: String) -> String {
    const MAX_MESSAGE_BYTES: usize = 64 * 1024;
    if message.len() <= MAX_MESSAGE_BYTES {
        return message;
    }
    let mut boundary = MAX_MESSAGE_BYTES;
    while !message.is_char_boundary(boundary) {
        boundary -= 1;
    }
    message.truncate(boundary);
    message
}

fn incompatible(phase: &'static str, message: impl Into<String>) -> GeneratorHostError {
    GeneratorHostError {
        category: GeneratorFailureCategory::ApiIncompatible,
        phase,
        message: message.into(),
    }
}

fn classify_wasmtime_error(
    phase: &'static str,
    mut message: String,
    cancellation: &CancellationHandle,
) -> GeneratorHostError {
    let lower = message.to_ascii_lowercase();
    let category = if cancellation.is_cancelled() {
        GeneratorFailureCategory::Cancelled
    } else if lower.contains("fuel")
        || lower.contains("epoch")
        || lower.contains("deadline")
        || lower.contains("memory")
        || lower.contains("resource limit")
    {
        GeneratorFailureCategory::ResourceExhausted
    } else if phase == "guest-instantiation" {
        GeneratorFailureCategory::ApiIncompatible
    } else {
        GeneratorFailureCategory::Trap
    };
    if category == GeneratorFailureCategory::Cancelled {
        message = "generation was cancelled".to_owned();
    } else if category == GeneratorFailureCategory::Trap && lower.contains("wasm backtrace") {
        message =
            "WebAssembly guest trapped; guest backtrace omitted from normal output".to_owned();
    } else if let Some(first_line) = message.lines().next() {
        message = first_line.to_owned();
    }
    GeneratorHostError {
        category,
        phase,
        message,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_wasm_input_is_rejected_as_a_module_validation_error() {
        let runtime = GeneratorRuntime::new().expect("runtime");
        let error = match runtime.prepare(b"not wasm") {
            Ok(_) => panic!("non-wasm input unexpectedly prepared"),
            Err(error) => error,
        };
        assert_eq!(error.category, GeneratorFailureCategory::ArtifactInvalid);
        assert_eq!(error.phase, "module-validation");
        assert_eq!(error.message, "not a core WebAssembly module");
    }

    #[test]
    fn missing_abi_exports_are_rejected_during_preparation() {
        let runtime = GeneratorRuntime::new().expect("runtime");
        let empty_module = b"\0asm\x01\0\0\0";
        let error = match runtime.prepare(empty_module) {
            Ok(_) => panic!("module without ABI exports unexpectedly prepared"),
            Err(error) => error,
        };
        assert_eq!(error.category, GeneratorFailureCategory::ApiIncompatible);
        assert_eq!(error.phase, "api-compatibility");
        assert_eq!(error.message, "module does not export `memory`");
    }

    #[test]
    fn returned_artifacts_are_validated_and_sorted() {
        let artifacts = validate_returned_artifacts(
            vec![
                protocol::Artifact {
                    file_path: "z.bin".to_owned(),
                    contents: vec![0, 255],
                },
                protocol::Artifact {
                    file_path: "a/report.txt".to_owned(),
                    contents: b"report".to_vec(),
                },
            ],
            ArtifactLimits::default(),
        )
        .expect("valid returned artifacts");

        assert_eq!(
            artifacts
                .iter()
                .map(|artifact| artifact.path)
                .collect::<Vec<_>>(),
            ["a/report.txt", "z.bin"]
        );
    }

    #[test]
    fn duplicate_returned_paths_are_output_policy_failures() {
        let error = validate_returned_artifacts(
            vec![
                protocol::Artifact {
                    file_path: "report.txt".to_owned(),
                    contents: vec![],
                },
                protocol::Artifact {
                    file_path: "report.txt".to_owned(),
                    contents: vec![],
                },
            ],
            ArtifactLimits::default(),
        )
        .expect_err("duplicate path should fail");

        assert_eq!(error.category, GeneratorFailureCategory::OutputPolicy);
        assert_eq!(error.phase, "artifact-validation");
        assert!(error.message.contains("returned more than once"));
    }

    #[test]
    fn returned_artifact_count_is_bounded_before_deserialization() {
        let encoded = postcard::to_allocvec(&Ok::<_, String>(vec![
            protocol::Artifact {
                file_path: "a".to_owned(),
                contents: vec![],
            },
            protocol::Artifact {
                file_path: "b".to_owned(),
                contents: vec![],
            },
        ]))
        .expect("encode result");

        let error = validate_artifact_result_header(&encoded, 1)
            .expect_err("artifact count over the limit should fail");
        assert_eq!(error.category, GeneratorFailureCategory::OutputPolicy);
        assert_eq!(error.phase, "artifact-validation");
        assert_eq!(error.message, "generator returned 2 files; the limit is 1");
    }
}
