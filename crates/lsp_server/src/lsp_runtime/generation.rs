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
use sysml_query::resolved_slice::PublishedModel;

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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DiagramViewsResult {
    pub(crate) model_digest: String,
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
    runtime: Arc<GeneratorRuntime>,
    /// Entries are keyed by the digest of the exact core Wasm bytes. `PreparedGenerator` already
    /// belongs to this service's engine, so no path, timestamp, or external identity participates.
    prepared: Mutex<HashMap<String, Arc<PreparedGenerator>>>,
    /// Query handles are scoped to one immutable `GeneratorModelView`. Reusing the adapter for
    /// the same dependency-complete publication identity lets a catalog handle be consumed by a
    /// subsequent generator request without turning handles into a second semantic identity.
    models: Mutex<HashMap<String, Arc<GeneratorModelView>>>,
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
            models: Mutex::new(HashMap::new()),
        })
    }

    fn model_for(
        &self,
        publication: Arc<PublishedModel>,
    ) -> Result<Arc<GeneratorModelView>, String> {
        let publication_identity = publication.publication().model_digest();
        let mut models = self
            .models
            .lock()
            .map_err(|_| "generator model cache is unavailable".to_owned())?;
        if let Some(model) = models.get(&publication_identity) {
            return Ok(Arc::clone(model));
        }
        let model = Arc::new(GeneratorModelView::new(
            Arc::clone(&publication),
            &publication_identity,
            env!("CARGO_PKG_VERSION"),
            QueryLimits::default(),
        ));
        if models.len() == MAX_MODEL_VIEWS {
            models.clear();
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
        let model = self.model_for(publication)?;
        let model_digest = model.model_digest();
        if let Some(expected) = expected_model_digest {
            if expected != model_digest {
                return Err("the semantic publication changed while selecting a view; choose the view again".to_owned());
            }
        }
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
            views,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spec42_generator_protocol::COMPATIBILITY_TOKEN;
    use sysml_source::{SysmlDocument, SysmlDocumentSourceKind};
    use tower_lsp::lsp_types::Url;

    fn publication() -> Arc<PublishedModel> {
        workspace::PublicationCoordinator::default()
            .publish(
                &[SysmlDocument {
                    uri: Url::parse("file:///lsp-generator-tests/model.sysml").expect("uri"),
                    content: "package P { part def Widget; }\n".to_owned(),
                    path_hint: None,
                    source_kind: SysmlDocumentSourceKind::Workspace,
                    content_digest: None,
                    byte_size: None,
                }],
                [],
            )
            .expect("published model")
    }

    fn state_transition_publication() -> Arc<PublishedModel> {
        let standard = SysmlDocument {
            uri: Url::parse("file:///lsp-generator-tests/standard.sysml").expect("standard uri"),
            content: "standard library package StandardViewDefinitions { view def StateTransitionView; }\n"
                .to_owned(),
            path_hint: None,
            source_kind: SysmlDocumentSourceKind::StandardLibrary,
            content_digest: None,
            byte_size: None,
        };
        let workspace = SysmlDocument {
            uri: Url::parse("file:///lsp-generator-tests/views.sysml").expect("workspace uri"),
            content: "package P {\n\
             \tprivate import StandardViewDefinitions::*;\n\
             \tstate def Machine { then ready; state ready; final done; transition finish first ready then done; }\n\
             \tview lifecycle : StateTransitionView { expose Machine; }\n\
             }\n"
                .to_owned(),
            path_hint: None,
            source_kind: SysmlDocumentSourceKind::Workspace,
            content_digest: None,
            byte_size: None,
        };
        workspace::PublicationCoordinator::default()
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

    fn packaged_diagram_generator() -> Vec<u8> {
        std::fs::read(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../vscode/generators/diagram.wasm"),
        )
        .expect("packaged diagram generator; run scripts/build-repository-generator-plugins.sh")
    }

    #[test]
    fn reuses_prepared_module_without_changing_results() {
        let service = GeneratorService::new().expect("generator service");
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
    fn catalog_handle_remains_valid_for_generation_on_the_same_publication() {
        let service = GeneratorService::new().expect("generator service");
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

        let generation_model = service
            .model_for(Arc::clone(&publication))
            .expect("generation model adapter");
        let projection = generation_model
            .diagram_view(&view.handle)
            .expect("catalog handle must remain valid for guest generation");

        let generated = service
            .generate(
                &packaged_diagram_generator(),
                Arc::clone(&publication),
                std::slice::from_ref(&view.handle),
                Some(&catalog.model_digest),
            )
            .expect("catalog handle must survive through actual guest execution");
        let artifact = generated
            .artifacts
            .iter()
            .find(|artifact| artifact.path == "diagram.json")
            .expect("diagram artifact");
        let product: serde_json::Value =
            serde_json::from_slice(&artifact.content).expect("diagram JSON product");

        assert_eq!(projection.model_digest, catalog.model_digest);
        assert_eq!(catalog.views.len(), 1);
        assert_eq!(catalog.views[0].kind, DiagramViewKind::StateTransitionView);
        assert_eq!(
            diagram_reference(projection.view.reference.clone()),
            view.reference
        );
        assert_eq!(product["modelDigest"], catalog.model_digest);
        assert_eq!(product["selectedView"]["kind"], "state-transition-view");
        let selected_reference = product["selectedView"]["reference"]
            .as_u64()
            .expect("selected reference index") as usize;
        assert_eq!(
            product["references"][selected_reference]["kind"],
            "qualified-name"
        );
        assert!(!String::from_utf8_lossy(&artifact.content).contains("element/v1"));
        assert!(product["projection"]["nodes"]
            .as_array()
            .is_some_and(|nodes| !nodes.is_empty()));
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
