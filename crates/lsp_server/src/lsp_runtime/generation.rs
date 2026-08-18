use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use base64::Engine as _;
use generator_api::{ArtifactLimits, GeneratorModelView, QueryLimits};
use generator_host::{
    CancellationHandle, GeneratorRuntime, PreparedGenerator, RuntimeLimits, RuntimeOptions,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sysml_query::resolved_slice::PublishedModel;

const MAX_PLUGIN_BYTES: usize = 16 * 1024 * 1024;
const MAX_PREPARED_MODULES: usize = 8;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct GenerateParams {
    pub(crate) generator_base64: String,
    pub(crate) model_uri: String,
    #[serde(default)]
    pub(crate) args: Vec<String>,
}

impl GenerateParams {
    pub(crate) fn module_bytes(&self) -> Result<Vec<u8>, String> {
        let max_encoded = MAX_PLUGIN_BYTES.saturating_mul(4).saturating_add(2) / 3 + 4;
        if self.generator_base64.len() > max_encoded {
            return Err(format!(
                "encoded generator is {} bytes; LSP module limit is {MAX_PLUGIN_BYTES}",
                self.generator_base64.len()
            ));
        }
        base64::engine::general_purpose::STANDARD
            .decode(&self.generator_base64)
            .map_err(|error| format!("generator is not valid base64: {error}"))
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GeneratedArtifact {
    pub(crate) path: String,
    /// Exact artifact bytes. JSON arrays are intentionally used for this bounded spike transport;
    /// the host does not assume that a general generator artifact is UTF-8.
    pub(crate) content: Vec<u8>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GenerationTimings {
    pub(crate) module_prepare_ms: u128,
    pub(crate) guest_execution_us: u128,
    pub(crate) prepared_reused: bool,
    pub(crate) compilation_cache_enabled: bool,
    pub(crate) compilation_cache_hits: usize,
    pub(crate) compilation_cache_misses: usize,
    pub(crate) compilation_cache_error: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GenerateResult {
    pub(crate) model_digest: String,
    pub(crate) generator_digest: String,
    pub(crate) artifacts: Vec<GeneratedArtifact>,
    pub(crate) timings: GenerationTimings,
}

pub(crate) struct GeneratorService {
    runtime: Arc<GeneratorRuntime>,
    /// Entries are keyed by the digest of the exact core Wasm bytes. `PreparedGenerator` already
    /// belongs to this service's engine, so no path, timestamp, or external identity participates.
    prepared: Mutex<HashMap<String, Arc<PreparedGenerator>>>,
}

impl GeneratorService {
    pub(crate) fn new() -> Result<Self, String> {
        let runtime = GeneratorRuntime::with_options(RuntimeOptions {
            fuel_metering: false,
            compilation_cache: true,
        })
        .map_err(|error| error.to_string())?;
        Ok(Self {
            runtime: Arc::new(runtime),
            prepared: Mutex::new(HashMap::new()),
        })
    }

    pub(crate) fn generate(
        &self,
        module_bytes: &[u8],
        publication: Arc<PublishedModel>,
        args: &[String],
    ) -> Result<GenerateResult, String> {
        if module_bytes.len() > MAX_PLUGIN_BYTES {
            return Err(format!(
                "generator is {} bytes; LSP limit is {MAX_PLUGIN_BYTES}",
                module_bytes.len()
            ));
        }
        let digest = format!("sha256:{:x}", Sha256::digest(module_bytes));
        let prepare_started = Instant::now();
        let (prepared, prepared_reused) = {
            let mut cache = self
                .prepared
                .lock()
                .map_err(|_| "generator preparation cache is unavailable".to_owned())?;
            if let Some(prepared) = cache.get(&digest) {
                (Arc::clone(prepared), true)
            } else {
                let prepared = Arc::new(
                    self.runtime
                        .prepare(module_bytes)
                        .map_err(|error| error.to_string())?,
                );
                if cache.len() == MAX_PREPARED_MODULES {
                    cache.clear();
                }
                cache.insert(digest.clone(), Arc::clone(&prepared));
                (prepared, false)
            }
        };
        let module_prepare_ms = prepare_started.elapsed().as_millis();
        let model = Arc::new(GeneratorModelView::new(
            Arc::clone(&publication),
            publication.publication().source_digest(),
            env!("CARGO_PKG_VERSION"),
            QueryLimits::default(),
        ));
        let model_digest = model.model_digest();
        let execution = self
            .runtime
            .execute_prepared(
                &prepared,
                model,
                args,
                RuntimeLimits {
                    memory_bytes: 256 * 1024 * 1024,
                    fuel: None,
                    wall_time: Some(Duration::from_secs(30)),
                },
                ArtifactLimits {
                    max_files: 16,
                    max_file_bytes: 16 * 1024 * 1024,
                    max_total_bytes: 16 * 1024 * 1024,
                },
                CancellationHandle::new(),
            )
            .map_err(|error| error.to_string())?;
        Ok(GenerateResult {
            model_digest,
            generator_digest: execution.generator_digest,
            artifacts: execution
                .artifacts
                .entries()
                .map(|(path, content)| GeneratedArtifact {
                    path: path.to_string(),
                    content: content.to_vec(),
                })
                .collect(),
            timings: GenerationTimings {
                module_prepare_ms,
                guest_execution_us: execution.duration.as_micros(),
                prepared_reused,
                compilation_cache_enabled: self.runtime.compilation_cache_enabled(),
                compilation_cache_hits: self.runtime.compilation_cache_hits(),
                compilation_cache_misses: self.runtime.compilation_cache_misses(),
                compilation_cache_error: self.runtime.compilation_cache_error().map(str::to_owned),
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spec42_generator_protocol::COMPATIBILITY_TOKEN;
    use sysml_query::resolved_slice::{
        build, BuildRequest, ConstructionStrategy, SourceDocument, SourceKind,
    };

    fn publication() -> Arc<PublishedModel> {
        let document = SourceDocument::from_memory_path(
            "lsp-generator-tests",
            "model.sysml",
            "package P { part def Widget; }\n".to_owned(),
            SourceKind::Workspace,
        )
        .expect("source document");
        let request = BuildRequest::resolved(vec![document], ConstructionStrategy::Sequential)
            .expect("build request");
        Arc::new(build(request).expect("published model"))
    }

    fn empty_generator(name: &str) -> Vec<u8> {
        let packed_result = 2_u64 << 32 | 1024;
        wat::parse_str(format!(
            r#"(module ${name}
              (import "spec42" "query" (func $query (param i32 i32 i32 i32 i32) (result i64)))
              (import "spec42" "diagnostic" (func $diagnostic (param i32 i32 i32 i32 i32)))
              (memory (export "memory") 1)
              (data (i32.const 1024) "\00\00")
              (func (export "spec42_abi_version") (result i64) (i64.const {COMPATIBILITY_TOKEN}))
              (func (export "spec42_alloc") (param i32) (result i32) (i32.const 2048))
              (func (export "spec42_generate") (param i32 i32) (result i64)
                (i64.const {packed_result})))"#
        ))
        .expect("valid guest")
    }

    #[test]
    fn reuses_prepared_module_without_changing_results() {
        let service = GeneratorService::new().expect("generator service");
        let module = empty_generator("same");
        let cold = service
            .generate(&module, publication(), &[])
            .expect("cold generation");
        let warm = service
            .generate(&module, publication(), &[])
            .expect("warm generation");
        assert!(!cold.timings.prepared_reused);
        assert!(warm.timings.prepared_reused);
        assert!(warm.timings.compilation_cache_enabled);
        assert_eq!(cold.model_digest, warm.model_digest);
        assert_eq!(cold.generator_digest, warm.generator_digest);
        assert_eq!(cold.artifacts.len(), warm.artifacts.len());

        let changed = service
            .generate(&empty_generator("changed"), publication(), &[])
            .expect("changed generation");
        assert!(!changed.timings.prepared_reused);
        assert_ne!(changed.generator_digest, warm.generator_digest);
        assert_eq!(changed.model_digest, warm.model_digest);
        assert_eq!(changed.artifacts.len(), warm.artifacts.len());
    }
}
