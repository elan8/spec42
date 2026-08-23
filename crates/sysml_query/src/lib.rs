#![recursion_limit = "256"]

//! The facade: the only crate a consumer names for anything SysML.
//!
//! A host obtains one [`Services`] value and works with its handles — [`source::SourceService`],
//! [`syntax::SyntaxService`], [`library::LibraryClosureService`],
//! [`publication::PublicationService`] and [`publication::PublicationSession`] — and with the
//! typed queries of an opaque [`resolved_slice::PublishedModel`]. Parsing, memoisation, file I/O,
//! library closure and publication lifecycle are the authorities' and are invisible behind these
//! handles; consumers cannot obtain a parser tree, the structural graph, resolver state, fact
//! collections, or query-index storage. See `design.md` at the repository root.

/// The version of the semantic vocabulary this build speaks.
///
/// Re-exported verbatim from `sysml_contract`, which owns it. Until now nothing outside the
/// authority could name the contract version at all: a host that records which vocabulary an
/// artefact was produced under had no value to record, and no test outside `sysml_resolution`
/// asserted it. It is published here because the version of a contract is part of the contract,
/// and because consumers reach the vocabulary only through this facade.
pub use sysml_contract::{SemanticContractVersion, SEMANTIC_CONTRACT_VERSION};

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
