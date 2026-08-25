//! Shared model construction for the host's integration tests.
//!
//! Built straight from an in-memory source through the immutable `sysml_query` publication —
//! the same path `server`'s generation command and the conformance runner use. A generator
//! consumer reaches a `PublishedModel` without a host workspace, a filesystem, or a cache, so
//! neither does the harness that pins its behaviour.

use std::sync::Arc;

use generator_api::{GeneratorModelView, QueryLimits};
use sysml_query::{source::SourceKind, Services};

/// Publishes `source` and wraps it in the view the runtime serves queries from.
pub fn published_model_view(source: &str) -> Arc<GeneratorModelView> {
    let services = Services::new();
    let document = services
        .source
        .admit_memory(
            "generator-host-tests",
            "model.sysml",
            source,
            SourceKind::Workspace,
        )
        .expect("in-memory source document");
    let publication = services
        .publication
        .publish(&[document], [])
        .expect("published model");
    Arc::new(
        GeneratorModelView::new(
            Arc::clone(&publication),
            publication.publication().model_digest().to_string(),
            env!("CARGO_PKG_VERSION"),
            QueryLimits::default(),
        )
        .expect("complete generator model"),
    )
}
