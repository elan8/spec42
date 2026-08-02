//! WebAssembly Component Model runtime with only explicit Spec42 capabilities linked.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::time::{Duration, Instant};

use generator_api::{
    ArtifactLimits, ArtifactSet, ElementDetail as ApiElementDetail,
    ElementSummary as ApiElementSummary, GeneratorDiagnostic, GeneratorDiagnosticLevel,
    GeneratorModelView, MultiplicitySummary as ApiMultiplicity, RelationshipSummary,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;
use wasmtime::component::{Component, HasSelf, Linker};
use wasmtime::{Config, Engine, Store, StoreLimits, StoreLimitsBuilder};

wasmtime::component::bindgen!({
    path: "wit",
    world: "generator",
});

use elan8::spec42_generator::diagnostics::Level;
use elan8::spec42_generator::model::{
    ElementDetail, ElementSummary, Multiplicity, Relationship, SourceRange,
};

pub const GENERATOR_WIT_VERSION: &str = "0.1.0";

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
    component: Component,
    generator_digest: String,
}

impl GeneratorRuntime {
    pub fn new() -> Result<Self, GeneratorHostError> {
        let mut config = Config::new();
        config.wasm_component_model(true);
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
        component_bytes: &[u8],
        model: Arc<GeneratorModelView>,
        args: &[String],
        runtime_limits: RuntimeLimits,
        artifact_limits: ArtifactLimits,
        cancellation: CancellationHandle,
    ) -> Result<GeneratorExecution, GeneratorHostError> {
        let prepared = self.prepare(component_bytes)?;
        self.execute_prepared(
            &prepared,
            model,
            args,
            runtime_limits,
            artifact_limits,
            cancellation,
        )
    }

    /// Compiles and links the component contract without loading a model snapshot.
    pub fn prepare(&self, component_bytes: &[u8]) -> Result<PreparedGenerator, GeneratorHostError> {
        let generator_digest = digest(component_bytes);
        let component =
            Component::new(&self.engine, component_bytes).map_err(|error| GeneratorHostError {
                category: GeneratorFailureCategory::ArtifactInvalid,
                phase: "component-validation",
                message: format!("not a valid WebAssembly component: {error}"),
            })?;
        let mut linker = Linker::<HostState>::new(&self.engine);
        Generator::add_to_linker::<_, HasSelf<_>>(&mut linker, |state| state).map_err(|error| {
            GeneratorHostError {
                category: GeneratorFailureCategory::ApiIncompatible,
                phase: "api-linking",
                message: error.to_string(),
            }
        })?;
        linker.instantiate_pre(&component).map_err(|error| GeneratorHostError {
            category: GeneratorFailureCategory::ApiIncompatible,
            phase: "api-compatibility",
            message: format!(
                "component does not implement elan8:spec42-generator@{GENERATOR_WIT_VERSION}: {error}"
            ),
        })?;
        Ok(PreparedGenerator {
            component,
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
        Generator::add_to_linker::<_, HasSelf<_>>(&mut linker, |state| state).map_err(|error| {
            GeneratorHostError {
                category: GeneratorFailureCategory::ApiIncompatible,
                phase: "api-linking",
                message: error.to_string(),
            }
        })?;

        let store_limits = StoreLimitsBuilder::new()
            .memory_size(runtime_limits.memory_bytes)
            .instances(100)
            .memories(1)
            .tables(32)
            .build();
        let mut store = Store::new(
            &self.engine,
            HostState {
                model,
                artifacts: ArtifactSet::new(artifact_limits),
                diagnostics: Vec::new(),
                store_limits,
                query_count: 0,
                output_policy_violation: None,
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
        let guest_result = (|| {
            let bindings = Generator::instantiate(&mut store, &prepared.component, &linker)
                .map_err(|error| {
                    classify_wasmtime_error(
                        "component-instantiation",
                        error.to_string(),
                        &cancellation,
                    )
                })?;
            bindings.call_generate(&mut store, args).map_err(|error| {
                classify_wasmtime_error("guest-execution", error.to_string(), &cancellation)
            })
        })();
        let _ = deadline_tx.send(());
        let _ = deadline_thread.join();
        let guest_result = guest_result?;
        if let Err(message) = guest_result {
            return Err(GeneratorHostError {
                category: GeneratorFailureCategory::GeneratorError,
                phase: "guest-execution",
                message,
            });
        }
        if let Some(message) = store.data().output_policy_violation.clone() {
            return Err(GeneratorHostError {
                category: GeneratorFailureCategory::OutputPolicy,
                phase: "artifact-staging",
                message,
            });
        }
        let duration = started.elapsed();
        let remaining_fuel = store.get_fuel().unwrap_or(0);
        let fuel_consumed = runtime_limits.fuel.saturating_sub(remaining_fuel);
        let state = store.into_data();
        Ok(GeneratorExecution {
            artifacts: state.artifacts,
            diagnostics: state.diagnostics,
            generator_digest: prepared.generator_digest.clone(),
            duration,
            query_count: state.query_count,
            fuel_consumed,
        })
    }
}

struct HostState {
    model: Arc<GeneratorModelView>,
    artifacts: ArtifactSet,
    diagnostics: Vec<GeneratorDiagnostic>,
    store_limits: StoreLimits,
    query_count: u64,
    output_policy_violation: Option<String>,
}

impl HostState {
    fn queried(&mut self) {
        self.query_count = self.query_count.saturating_add(1);
    }
}

impl elan8::spec42_generator::model::Host for HostState {
    fn info(&mut self) -> elan8::spec42_generator::model::ModelInfo {
        self.queried();
        elan8::spec42_generator::model::ModelInfo {
            model_digest: self.model.model_digest(),
            spec42_version: self.model.spec42_version().to_owned(),
            semantic_api_version: self.model.semantic_api_version().to_owned(),
        }
    }

    fn roots(&mut self) -> Result<Vec<ElementSummary>, String> {
        self.queried();
        self.model
            .roots()
            .map(|values| values.into_iter().map(summary).collect())
            .map_err(|error| error.to_string())
    }

    fn find(&mut self, metaclass: Option<String>) -> Result<Vec<ElementSummary>, String> {
        self.queried();
        self.model
            .find(metaclass.as_deref())
            .map(|values| values.into_iter().map(summary).collect())
            .map_err(|error| error.to_string())
    }

    fn children(&mut self, owner: String) -> Result<Vec<ElementSummary>, String> {
        self.queried();
        self.model
            .children(&owner)
            .map(|values| values.into_iter().map(summary).collect())
            .map_err(|error| error.to_string())
    }

    fn element(&mut self, handle: String) -> Result<ElementDetail, String> {
        self.queried();
        self.model
            .element(&handle)
            .map(detail)
            .map_err(|error| error.to_string())
    }

    fn typed_by(&mut self, feature: String) -> Result<Option<ElementSummary>, String> {
        self.queried();
        self.model
            .typed_by(&feature)
            .map(|value| value.map(summary))
            .map_err(|error| error.to_string())
    }

    fn relationships(&mut self, element: String) -> Result<Vec<Relationship>, String> {
        self.queried();
        self.model
            .relationships(&element)
            .map(|values| values.into_iter().map(relationship).collect())
            .map_err(|error| error.to_string())
    }

    fn effective_features(&mut self, element: String) -> Result<Vec<ElementSummary>, String> {
        self.queried();
        self.model
            .effective_features(&element)
            .map(|values| values.into_iter().map(summary).collect())
            .map_err(|error| error.to_string())
    }
}

impl elan8::spec42_generator::artifacts::Host for HostState {
    fn emit(&mut self, path: String, content: Vec<u8>) -> Result<(), String> {
        self.artifacts.emit(&path, content).map_err(|error| {
            let message = error.to_string();
            self.output_policy_violation
                .get_or_insert_with(|| message.clone());
            message
        })
    }
}

impl elan8::spec42_generator::diagnostics::Host for HostState {
    fn log(&mut self, level: Level, message: String) {
        if self.diagnostics.len() >= 10_000 {
            return;
        }
        self.diagnostics.push(GeneratorDiagnostic {
            level: diagnostic_level(level),
            message: bounded_message(message),
            element_id: None,
        });
    }

    fn report(&mut self, level: Level, message: String, element: Option<String>) {
        if self.diagnostics.len() >= 10_000 {
            return;
        }
        let element = element.filter(|handle| self.model.element(handle).is_ok());
        self.diagnostics.push(GeneratorDiagnostic {
            level: diagnostic_level(level),
            message: bounded_message(message),
            element_id: element,
        });
    }
}

fn summary(value: ApiElementSummary) -> ElementSummary {
    ElementSummary {
        handle: value.handle,
        semantic_id: value.semantic_id,
        metaclass: value.metaclass,
        name: value.name,
        qualified_name: value.qualified_name,
        library_element: value.library_element,
    }
}

fn multiplicity(value: ApiMultiplicity) -> Multiplicity {
    Multiplicity {
        lower: value.lower,
        upper: value.upper,
        ordered: value.ordered,
        unique: value.unique,
        implied: value.implied,
    }
}

fn detail(value: ApiElementDetail) -> ElementDetail {
    ElementDetail {
        summary: summary(value.summary),
        owner: value.owner.map(summary),
        declared_name: value.declared_name,
        effective_name: value.effective_name,
        source_uri: value.source_uri,
        source_range: SourceRange {
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

fn relationship(value: RelationshipSummary) -> Relationship {
    Relationship {
        kind: value.kind,
        source: summary(value.source),
        target: summary(value.target),
        implied: value.implied,
    }
}

fn diagnostic_level(level: Level) -> GeneratorDiagnosticLevel {
    match level {
        Level::Debug => GeneratorDiagnosticLevel::Debug,
        Level::Info => GeneratorDiagnosticLevel::Info,
        Level::Warning => GeneratorDiagnosticLevel::Warning,
        Level::Error => GeneratorDiagnosticLevel::Error,
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
    } else if phase == "component-instantiation" {
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
