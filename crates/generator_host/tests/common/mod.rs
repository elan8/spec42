//! Shared model construction for the host's integration tests.
//!
//! Built straight from an in-memory source through the immutable `sysml_query` publication —
//! the same path `server`'s generation command and the conformance runner use. A generator
//! consumer reaches a `PublishedModel` without a host workspace, a filesystem, or a cache, so
//! neither does the harness that pins its behaviour.

use std::sync::Arc;

use generator_api::{GeneratorModelView, QueryLimits};
use sysml_query::resolved_slice::{
    build, AdmittedSource, BuildRequest, ConstructionStrategy, SourceKind,
};

/// Publishes `source` and wraps it in the view the runtime serves queries from.
pub fn published_model_view(source: &str) -> Arc<GeneratorModelView> {
    let document = AdmittedSource::from_memory_path(
        "generator-host-tests",
        "model.sysml",
        source.to_owned(),
        SourceKind::Workspace,
    )
    .expect("in-memory source document");
    let request = BuildRequest::resolved(vec![document], ConstructionStrategy::Sequential)
        .expect("resolved build request");
    let publication = Arc::new(build(request).expect("published model"));
    Arc::new(GeneratorModelView::new(
        Arc::clone(&publication),
        publication.publication().model_digest(),
        env!("CARGO_PKG_VERSION"),
        QueryLimits::default(),
    ))
}
