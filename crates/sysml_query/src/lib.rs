#![recursion_limit = "256"]

//! The only supported consumer facade over Spec42's semantic model implementation.
//!
//! [`resolved_slice::PublishedModel`] is opaque. Consumers select a cohesive service and receive
//! typed answers or stream an owner-defined debug projection; they cannot obtain the structural
//! graph, resolver state, fact collections, or query-index storage.

pub mod library;
pub mod publication;
pub mod resolved_slice;
pub mod source;
pub mod syntax;

/// Every service a host works with, sharing one set of authorities.
///
/// A host process constructs exactly one of these and hands clones of the handles to whatever
/// needs them; the memo, the library-stratum reuse, and admission policy are then one per process.
#[derive(Debug, Clone)]
pub struct Services {
    pub source: source::SourceService,
    pub syntax: syntax::SyntaxService,
    pub library: library::LibraryClosureService,
    pub publication: publication::PublicationService,
}

impl Services {
    pub fn new() -> Self {
        let source = source::SourceService::new();
        let syntax = syntax::SyntaxService::new();
        let library = library::LibraryClosureService::new(&source, &syntax);
        let publication = publication::PublicationService::new(&syntax);
        Self {
            source,
            syntax,
            library,
            publication,
        }
    }
}

impl Default for Services {
    fn default() -> Self {
        Self::new()
    }
}
