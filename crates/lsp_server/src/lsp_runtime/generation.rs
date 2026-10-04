use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use base64::Engine as _;
use generator_api::{
    ArtifactLimits, DiagramSemanticReference, DiagramViewKind, GeneratorModelView, QueryLimits,
};
use generator_host::{
    CancellationHandle, GeneratorRuntime, PreparedGenerator, RuntimeLimits, RuntimeOptions,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sysml_query::resolved_slice::{PublicationModelDigest, PublishedModel};

const MAX_PLUGIN_BYTES: usize = 16 * 1024 * 1024;
const MAX_PREPARED_MODULES: usize = 8;
const MAX_MODEL_VIEWS: usize = 4;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct GenerateParams {
    pub(crate) generator_base64: String,
    pub(crate) model_uri: String,
    #[serde(default)]
    pub(crate) args: Vec<String>,
    pub(crate) expected_model_digest: Option<String>,
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

#[derive(Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GeneratedArtifact {
    pub(crate) path: String,
    /// Exact artifact bytes. JSON arrays are intentionally used for this bounded spike transport;
    /// the host does not assume that a general generator artifact is UTF-8.
    pub(crate) content: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
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
    pub(crate) semantic_status: language_service::dto::SemanticResultStatus,
    pub(crate) generator_digest: String,
    pub(crate) artifacts: Vec<GeneratedArtifact>,
    pub(crate) timings: GenerationTimings,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct StateTransitionViewsParams {
    pub(crate) model_uri: String,
}

pub(crate) type DiagramViewsParams = StateTransitionViewsParams;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct DiagramParams {
    pub(crate) model_uri: String,
    /// A handle from `spec42/diagramViews` on the same publication.
    pub(crate) handle: String,
    pub(crate) expected_model_digest: Option<String>,
}

/// `spec42/diagram`: the schema-5 diagram product of one catalog view, built natively.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DiagramResult {
    pub(crate) model_digest: String,
    pub(crate) semantic_status: language_service::dto::SemanticResultStatus,
    /// The product exactly as `diagram.json` (see the `diagram_product` crate).
    pub(crate) product_json: String,
    pub(crate) duration_us: u128,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DiagramViewsResult {
    pub(crate) model_digest: String,
    pub(crate) semantic_status: language_service::dto::SemanticResultStatus,
    pub(crate) views: Vec<DiagramViewChoice>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DiagramViewChoice {
    pub(crate) handle: String,
    pub(crate) kind: DiagramViewKind,
    pub(crate) reference: DiagramReferenceChoice,
    pub(crate) name: String,
    pub(crate) source: StateTransitionSourceChoice,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "kebab-case",
    rename_all_fields = "camelCase"
)]
pub(crate) enum DiagramReferenceChoice {
    QualifiedName {
        document: String,
        qualified_name: String,
        source_domain: String,
    },
    ToolingElementId {
        element_id: String,
        source_domain: String,
    },
    SourceAnchor {
        document: String,
        owner_qualified_name: Option<String>,
        metaclass: String,
        source_domain: String,
        range: DiagramRangeChoice,
    },
    Relationship {
        document: String,
        source_qualified_name: String,
        relationship_kind: String,
        ordinal: u32,
        source_domain: String,
    },
}

#[derive(Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DiagramRangeChoice {
    start_line: u32,
    start_character: u32,
    end_line: u32,
    end_character: u32,
}

fn diagram_source_domain(value: generator_api::DiagramSourceDomain) -> String {
    match value {
        generator_api::DiagramSourceDomain::Workspace => "workspace",
        generator_api::DiagramSourceDomain::StandardLibrary => "standard-library",
        generator_api::DiagramSourceDomain::Library => "library",
        generator_api::DiagramSourceDomain::External => "external",
    }
    .to_owned()
}

fn diagram_reference(value: DiagramSemanticReference) -> DiagramReferenceChoice {
    match value {
        DiagramSemanticReference::Qualified {
            document,
            qualified_name,
            source_domain,
        } => DiagramReferenceChoice::QualifiedName {
            document,
            qualified_name,
            source_domain: diagram_source_domain(source_domain),
        },
        DiagramSemanticReference::ToolingElementId {
            element_id,
            source_domain,
        } => DiagramReferenceChoice::ToolingElementId {
            element_id,
            source_domain: diagram_source_domain(source_domain),
        },
        DiagramSemanticReference::SourceAnchor {
            document,
            owner_qualified_name,
            metaclass,
            source_domain,
            range,
        } => DiagramReferenceChoice::SourceAnchor {
            document,
            owner_qualified_name,
            metaclass: metaclass.as_str().to_owned(),
            source_domain: diagram_source_domain(source_domain),
            range: DiagramRangeChoice {
                start_line: range.start_line,
                start_character: range.start_character,
                end_line: range.end_line,
                end_character: range.end_character,
            },
        },
        DiagramSemanticReference::Relationship {
            document,
            source_qualified_name,
            relationship_kind,
            ordinal,
            source_domain,
        } => DiagramReferenceChoice::Relationship {
            document,
            source_qualified_name,
            relationship_kind: relationship_kind.as_str().to_owned(),
            ordinal,
            source_domain: diagram_source_domain(source_domain),
        },
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct StateTransitionViewsResult {
    pub(crate) model_digest: String,
    pub(crate) views: Vec<StateTransitionViewChoice>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct StateTransitionViewChoice {
    pub(crate) handle: String,
    pub(crate) semantic_id: String,
    pub(crate) name: String,
    pub(crate) exposed_machine: StateTransitionMachineChoice,
    pub(crate) source: StateTransitionSourceChoice,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct StateTransitionMachineChoice {
    pub(crate) semantic_id: String,
    pub(crate) label: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct StateTransitionSourceChoice {
    pub(crate) uri: String,
}

pub(crate) struct GeneratorService {
    /// Runs Wasm generators (`spec42/generate`). Diagrams and the view catalog do not need it, so a
    /// runtime that fails to start only disables plugin generation.
    runtime: std::result::Result<Arc<GeneratorRuntime>, String>,
    /// Entries are keyed by the digest of the exact core Wasm bytes. `PreparedGenerator` already
    /// belongs to this service's engine, so no path, timestamp, or external identity participates.
    prepared: Mutex<HashMap<String, Arc<PreparedGenerator>>>,
    /// Query handles are scoped to one immutable `GeneratorModelView`. Reusing the adapter for
    /// the same dependency-complete publication identity lets a catalog handle be consumed by a
    /// subsequent generator request without turning handles into a second semantic identity.
    models: Mutex<HashMap<PublicationModelDigest, Arc<GeneratorModelView>>>,
}

impl GeneratorService {
    pub(crate) fn new() -> Self {
        let runtime = GeneratorRuntime::with_options(RuntimeOptions {
            fuel_metering: false,
            compilation_cache: true,
        })
        .map(Arc::new)
        .map_err(|error| format!("the generator runtime is unavailable: {error}"));
        Self {
            runtime,
            prepared: Mutex::new(HashMap::new()),
            models: Mutex::new(HashMap::new()),
        }
    }

    pub(crate) fn diagram(
        &self,
        publication: Arc<PublishedModel>,
        handle: &str,
        expected_model_digest: Option<&str>,
    ) -> Result<DiagramResult, String> {
        let started = Instant::now();
        // Catalog handles are scoped to the model view that minted them: reuse the cached one.
        let model = self.model_for(publication)?;
        let model_digest = model.model_digest();
        if let Some(expected) = expected_model_digest {
            if expected != model_digest {
                return Err("the semantic publication changed while selecting a view; choose the view again".to_owned());
            }
        }
        let projection = model
            .diagram_view(handle)
            .map_err(|error| error.to_string())?;
        let product = diagram_product::diagram_product_json(&projection)?;
        Ok(DiagramResult {
            model_digest,
            semantic_status: language_service::dto::SemanticResultStatus::from_publication(
                model.publication_completeness(),
            ),
            product_json: String::from_utf8(product).expect("serde_json writes UTF-8"),
            duration_us: started.elapsed().as_micros(),
        })
    }

    fn model_for(
        &self,
        publication: Arc<PublishedModel>,
    ) -> Result<Arc<GeneratorModelView>, String> {
        let publication_identity = publication.publication().model_digest();
        {
            let models = self
                .models
                .lock()
                .map_err(|_| "generator model cache is unavailable".to_owned())?;
            if let Some(model) = models.get(&publication_identity) {
                return Ok(Arc::clone(model));
            }
        }

        // Building the boundary adapter walks the publication. Keep that work outside the cache
        // lock so unrelated generation requests can continue using already-prepared views.
        let model = Arc::new(
            GeneratorModelView::new(
                Arc::clone(&publication),
                publication_identity.to_string(),
                env!("CARGO_PKG_VERSION"),
                QueryLimits::default(),
            )
            .map_err(|error| error.to_string())?,
        );
        // A catalog handle is valid only on the model view that minted it, and this cache may
        // evict and rebuild the view of a publication that is still current. Handles are
        // deterministic per publication, so minting both catalogs on every view keeps a handle
        // from `spec42/diagramViews` or `spec42/stateTransitionViews` valid for the whole
        // publication. A catalog that cannot be listed mints nothing and reports its error when
        // requested.
        let _ = model.diagram_views();
        let _ = model.state_transition_views();
        let mut models = self
            .models
            .lock()
            .map_err(|_| "generator model cache is unavailable".to_owned())?;
        if let Some(existing) = models.get(&publication_identity) {
            return Ok(Arc::clone(existing));
        }
        if models.len() >= MAX_MODEL_VIEWS {
            evict_one(&mut models);
        }
        models.insert(publication_identity, Arc::clone(&model));
        Ok(model)
    }

    pub(crate) fn generate(
        &self,
        module_bytes: &[u8],
        publication: Arc<PublishedModel>,
        args: &[String],
        expected_model_digest: Option<&str>,
    ) -> Result<GenerateResult, String> {
        if module_bytes.len() > MAX_PLUGIN_BYTES {
            return Err(format!(
                "generator is {} bytes; LSP limit is {MAX_PLUGIN_BYTES}",
                module_bytes.len()
            ));
        }
        let runtime = self.runtime.as_ref().map_err(Clone::clone)?;
        let digest = format!("sha256:{:x}", Sha256::digest(module_bytes));
        let prepare_started = Instant::now();
        let cached = {
            let cache = self
                .prepared
                .lock()
                .map_err(|_| "generator preparation cache is unavailable".to_owned())?;
            cache.get(&digest).cloned()
        };
        let (prepared, prepared_reused) = if let Some(prepared) = cached {
            (prepared, true)
        } else {
            // Wasmtime preparation can compile the module. Do not serialize that work behind the
            // cache mutex; after construction, recheck in case another request won the race.
            let candidate = Arc::new(
                runtime
                    .prepare(module_bytes)
                    .map_err(|error| error.to_string())?,
            );
            let mut cache = self
                .prepared
                .lock()
                .map_err(|_| "generator preparation cache is unavailable".to_owned())?;
            if let Some(existing) = cache.get(&digest) {
                (Arc::clone(existing), true)
            } else {
                if cache.len() >= MAX_PREPARED_MODULES {
                    evict_one(&mut cache);
                }
                cache.insert(digest.clone(), Arc::clone(&candidate));
                (candidate, false)
            }
        };
        let module_prepare_ms = prepare_started.elapsed().as_millis();
        let model = self.model_for(publication)?;
        let model_digest = model.model_digest();
        if let Some(expected) = expected_model_digest {
            if expected != model_digest {
                return Err("the semantic publication changed while selecting a view; choose the view again".to_owned());
            }
        }
        let execution = runtime
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
            semantic_status: language_service::dto::SemanticResultStatus::from_publication(
                execution.publication_completeness,
            ),
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
                compilation_cache_enabled: runtime.compilation_cache_enabled(),
                compilation_cache_hits: runtime.compilation_cache_hits(),
                compilation_cache_misses: runtime.compilation_cache_misses(),
                compilation_cache_error: runtime.compilation_cache_error().map(str::to_owned),
            },
        })
    }

    pub(crate) fn state_transition_views(
        &self,
        publication: Arc<PublishedModel>,
    ) -> Result<StateTransitionViewsResult, String> {
        let model = self.model_for(publication)?;
        let views = model
            .state_transition_views()
            .map_err(|error| error.to_string())?
            .into_iter()
            .map(|view| StateTransitionViewChoice {
                handle: view.handle,
                semantic_id: view.semantic_id,
                name: view.name,
                exposed_machine: StateTransitionMachineChoice {
                    semantic_id: view.exposed_machine.semantic_id,
                    label: view.exposed_machine.label,
                },
                source: StateTransitionSourceChoice {
                    uri: view.source.uri,
                },
            })
            .collect();
        Ok(StateTransitionViewsResult {
            model_digest: model.model_digest(),
            views,
        })
    }

    pub(crate) fn diagram_views(
        &self,
        publication: Arc<PublishedModel>,
    ) -> Result<DiagramViewsResult, String> {
        let model = self.model_for(publication)?;
        let views = model
            .diagram_views()
            .map_err(|error| error.to_string())?
            .into_iter()
            .map(|view| DiagramViewChoice {
                handle: view.handle,
                kind: view.kind,
                reference: diagram_reference(view.reference),
                name: view.name,
                source: StateTransitionSourceChoice {
                    uri: view.source.uri,
                },
            })
            .collect();
        Ok(DiagramViewsResult {
            model_digest: model.model_digest(),
            semantic_status: language_service::dto::SemanticResultStatus::from_publication(
                model.publication_completeness(),
            ),
            views,
        })
    }
}

/// Remove one exact-identity entry without turning a capacity miss into a full-cache flush.
/// Lexicographic selection keeps eviction deterministic; cache contents never affect semantics.
fn evict_one<K: Clone + Eq + std::hash::Hash + Ord, V>(cache: &mut HashMap<K, V>) {
    if let Some(key) = cache.keys().min().cloned() {
        cache.remove(&key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spec42_generator_protocol::COMPATIBILITY_TOKEN;
    use sysml_query::source::{SourceKind, SourceService};

    #[test]
    fn bounded_cache_evicts_one_deterministic_entry() {
        let mut cache = HashMap::from([
            ("b".to_owned(), 2),
            ("a".to_owned(), 1),
            ("c".to_owned(), 3),
        ]);
        evict_one(&mut cache);
        assert_eq!(cache.len(), 2);
        assert!(!cache.contains_key("a"));
        assert_eq!(cache.get("b"), Some(&2));
        assert_eq!(cache.get("c"), Some(&3));
    }

    fn publication() -> Arc<PublishedModel> {
        sysml_query::Services::new()
            .publication
            .publish(
                &[SourceService::new()
                    .admit(
                        "file:///lsp-generator-tests/model.sysml",
                        "package P { part def Widget; }\n",
                        SourceKind::Workspace,
                    )
                    .expect("uri")],
                [],
            )
            .expect("published model")
    }

    fn state_transition_publication() -> Arc<PublishedModel> {
        let source = SourceService::new();
        let standard = source
            .admit(
                "file:///lsp-generator-tests/standard.sysml",
                "standard library package StandardViewDefinitions { view def StateTransitionView; }\n",
                SourceKind::StandardLibrary,
            )
            .expect("standard uri");
        let workspace = source
            .admit(
                "file:///lsp-generator-tests/views.sysml",
                "package P {\n\
             \tprivate import StandardViewDefinitions::*;\n\
             \tstate def Machine { then ready; state ready; final done; transition finish first ready then done; }\n\
             \tview lifecycle : StateTransitionView { expose Machine; }\n\
             }\n",
                SourceKind::Workspace,
            )
            .expect("workspace uri");
        sysml_query::Services::new()
            .publication
            .publish(&[standard, workspace], [])
            .expect("published state-transition model")
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
        let service = GeneratorService::new();
        let module = empty_generator("same");
        let cold = service
            .generate(&module, publication(), &[], None)
            .expect("cold generation");
        let warm = service
            .generate(&module, publication(), &[], None)
            .expect("warm generation");
        assert!(!cold.timings.prepared_reused);
        assert!(warm.timings.prepared_reused);
        assert!(warm.timings.compilation_cache_enabled);
        assert_eq!(cold.model_digest, warm.model_digest);
        assert_eq!(cold.generator_digest, warm.generator_digest);
        assert_eq!(cold.artifacts.len(), warm.artifacts.len());

        let stale = service
            .generate(&module, publication(), &[], Some("blake3:stale"))
            .expect_err("stale catalog selection must not execute");
        assert!(stale.contains("publication changed"));

        let changed = service
            .generate(&empty_generator("changed"), publication(), &[], None)
            .expect("changed generation");
        assert!(!changed.timings.prepared_reused);
        assert_ne!(changed.generator_digest, warm.generator_digest);
        assert_eq!(changed.model_digest, warm.model_digest);
        assert_eq!(changed.artifacts.len(), warm.artifacts.len());
    }

    #[test]
    fn catalog_handle_yields_the_native_diagram_product_on_the_same_publication() {
        let service = GeneratorService::new();
        let publication = state_transition_publication();
        let catalog = service
            .diagram_views(Arc::clone(&publication))
            .expect("diagram catalog");
        let [view] = catalog.views.as_slice() else {
            panic!(
                "expected one state-transition view, got {}",
                catalog.views.len()
            );
        };
        assert_eq!(view.kind, DiagramViewKind::StateTransitionView);

        let diagram = service
            .diagram(
                Arc::clone(&publication),
                &view.handle,
                Some(&catalog.model_digest),
            )
            .expect("a catalog handle stays valid for the diagram request");
        let product: serde_json::Value =
            serde_json::from_str(&diagram.product_json).expect("diagram JSON product");
        assert_eq!(diagram.model_digest, catalog.model_digest);
        assert_eq!(product["modelDigest"], catalog.model_digest);
        assert_eq!(product["selectedView"]["kind"], "state-transition-view");
        let selected_reference = product["selectedView"]["reference"]
            .as_u64()
            .expect("selected reference index") as usize;
        assert_eq!(
            product["references"][selected_reference]["kind"],
            "qualified-name"
        );
        assert!(product["projection"]["nodes"]
            .as_array()
            .is_some_and(|nodes| !nodes.is_empty()));

        let stale = service
            .diagram(publication, &view.handle, Some("blake3:stale"))
            .expect_err("a stale catalog selection must not project");
        assert!(stale.contains("publication changed"));
    }

    #[test]
    fn a_catalog_handle_survives_eviction_of_its_model_view() {
        let service = GeneratorService::new();
        let publication = state_transition_publication();
        let catalog = service
            .diagram_views(Arc::clone(&publication))
            .expect("diagram catalog");
        let handle = catalog.views[0].handle.clone();
        // Evict the view the handle came from. `evict_one` takes the smallest digest, so whether
        // other publications push this one out depends on digest values (which move with every
        // semantic contract version); remove it directly so the test does not depend on them.
        service
            .models
            .lock()
            .unwrap()
            .remove(&publication.publication().model_digest());
        let cached = service.models.lock().unwrap();
        assert!(
            !cached.contains_key(&publication.publication().model_digest()),
            "the handle's model view was evicted"
        );
        drop(cached);

        let diagram = service
            .diagram(publication, &handle, Some(&catalog.model_digest))
            .expect("the handle stays valid for its still-current publication");
        assert_eq!(diagram.model_digest, catalog.model_digest);
    }

    #[test]
    fn catalog_lsp_dto_uses_camel_case_at_every_level() {
        let value = serde_json::to_value(StateTransitionViewsResult {
            model_digest: "blake3:model".to_owned(),
            views: vec![StateTransitionViewChoice {
                handle: "view:one".to_owned(),
                semantic_id: "semantic:one".to_owned(),
                name: "operations".to_owned(),
                exposed_machine: StateTransitionMachineChoice {
                    semantic_id: "machine:one".to_owned(),
                    label: "Operations".to_owned(),
                },
                source: StateTransitionSourceChoice {
                    uri: "file:///workspace/model.sysml".to_owned(),
                },
            }],
        })
        .expect("catalog JSON");
        assert_eq!(value["modelDigest"], "blake3:model");
        assert_eq!(value["views"][0]["semanticId"], "semantic:one");
        assert_eq!(
            value["views"][0]["exposedMachine"]["semanticId"],
            "machine:one"
        );
        assert_eq!(
            value["views"][0]["source"]["uri"],
            "file:///workspace/model.sysml"
        );
        assert!(value["views"][0].get("semantic_id").is_none());
        assert!(value["views"][0].get("exposed_machine").is_none());
    }
}
